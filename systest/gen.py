#!/usr/bin/env python3
"""Generate the ABI checks of systest from mpdecimal.h.

usage: gen.py

For CONFIG_64 (x86_64) and CONFIG_32 (i686), mpdecimal.h is generated from
libmpdec/mpdecimal.h.in as c2rust/prepare.py does and translated by bindgen
(against the neutral libc headers, so the output does not depend on the
host). The output is written to src/abi64.rs and src/abi32.rs:

  - mod c: the bindgen declarations, i.e. the C side;
  - mod check: one item per function, global, struct field and typedef of
    the header that assigns the item of the crate to its C type. Type names
    resolve to the types of the crate, so a mismatch in any position fails
    to compile;
  - size, alignment and field offsets of every struct, and the value of
    every integer constant that the crate defines, compared with mod c.

bindgen needs libclang; set LIBCLANG_PATH if it is not found, e.g.
LIBCLANG_PATH=/Library/Developer/CommandLineTools/usr/lib on macOS.
Run again when mpdecimal.h changes or the crate gains constants.
"""

import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT / "c2rust"))
from prepare import MACHINES, header_config  # noqa: E402

CONFIGS = [
    ("abi64.rs", "uint128", "x86_64-unknown-linux-gnu"),
    ("abi32.rs", "ansi32", "i686-unknown-linux-gnu"),
]

CONST = re.compile(r"^pub const (\w+): ([\w:]+) = (-?\d+);$", re.M)
ALIAS = re.compile(r"^pub type (mpd_\w+) = (.+);$", re.M)
STRUCT = re.compile(r"^pub struct (\w+) \{\n(.*?)^\}", re.M | re.S)
FIELD = re.compile(r"^\s+pub (\w+): (.+),$", re.M)
STATIC = re.compile(r"^    pub static (?:mut )?(\w+): (.+);$", re.M)
FN = re.compile(r"^    pub fn (\w+)\(", re.M)


def bindgen(machine, target):
    var, flags = MACHINES[machine]
    template = (ROOT / "libmpdec" / "mpdecimal.h.in").read_text()
    with tempfile.TemporaryDirectory() as tmp:
        header = Path(tmp) / "mpdecimal.h"
        header.write_text(template.replace("@MPD_HEADER_CONFIG@", header_config(var)))
        return subprocess.run(
            ["bindgen", str(header), "--allowlist-file", r".*/mpdecimal\.h",
             "--no-layout-tests", "--no-doc-comments", "--use-core", "--",
             f"--target={target}", f"-I{ROOT / 'c2rust' / 'neutral-libc'}", *flags],
            check=True, capture_output=True, text=True).stdout


def functions(text):
    """(name, parameters, return type) of every function declaration."""
    for m in FN.finditer(text):
        depth, i = 1, m.end()
        while depth:
            depth += {"(": 1, ")": -1}.get(text[i], 0)
            i += 1
        params = " ".join(text[m.end():i - 1].split()).rstrip(",")
        ret = re.match(r"\s*(?:-> (.+?))?;", text[i:], re.S).group(1)
        yield m.group(1), params, " ".join(ret.split()) if ret else None


def crate_constants(width):
    """name -> module of the constants of the crate that exist for the
    pointer width, with and without the feature test-hooks."""
    consts = {}
    for f in sorted((ROOT / "src").glob("*.rs")):
        lines = f.read_text().split("\n")
        for i, line in enumerate(lines):
            m = re.match(r"pub const (\w+):", line)
            if not m:
                continue
            attrs = []
            while i and lines[i - 1].startswith("#["):
                i -= 1
                attrs.append(lines[i])
            if any("test-hooks" in a for a in attrs):
                continue
            widths = re.findall(r'target_pointer_width = "(\d+)"', " ".join(attrs))
            if not widths or width in widths:
                consts.setdefault(m.group(1), f.stem)
    return consts


def crate_field(field):
    """c2rust renames the field `type` to type_0, bindgen to type_."""
    return field[:-1] + "_0" if field.endswith("_") else field


def checks(text, consts):
    out = ["pub mod check {", "    use crate::rs::*;", "    use mpdecimal::shim::FILE;"]
    for name, params, ret in functions(text):
        arrow = f" -> {ret}" if ret else ""
        out.append(f'    const _: unsafe extern "C" fn({params}){arrow} = {name};')
    for name, ty in STATIC.findall(text):
        out.append(f"    const _: *const {ty} = &raw const {name};")
    for name, ty in ALIAS.findall(text):
        out.append(f"    const _: fn({name}) -> {ty} = |v| v;")
    for struct, body in STRUCT.findall(text):
        for field, ty in FIELD.findall(body):
            out.append(f"    const _: fn(&{struct}) -> &{ty} = |s| &s.{crate_field(field)};")
    out.append("}")

    out += ["", "const _: () = {", "    use crate::rs;", "    use core::mem::{align_of, offset_of, size_of};"]
    for struct, body in STRUCT.findall(text):
        fields = [f for f, _ in FIELD.findall(body)]
        if not fields:  # opaque (FILE)
            continue
        out.append(f"    assert!(size_of::<rs::{struct}>() == size_of::<c::{struct}>());")
        out.append(f"    assert!(align_of::<rs::{struct}>() == align_of::<c::{struct}>());")
        for f in fields:
            out.append(f"    assert!(offset_of!(rs::{struct}, {crate_field(f)}) == offset_of!(c::{struct}, {f}));")
    # bindgen infers the types of macro constants (MPD_UINT_MAX: i32 = -1), so
    # the values are compared as bit patterns of the width of the crate's type.
    for name, _, _ in CONST.findall(text):
        if name in consts:
            rs = f"mpdecimal::{consts[name]}::{name}"
            out.append(f"    assert!(crate::same_bits({rs} as u64, c::{name} as u64, size_of_val(&{rs})));")
    out.append("};")
    return "\n".join(out)


def main():
    for out, machine, target in CONFIGS:
        text = bindgen(machine, target)
        consts = crate_constants(out[3:5])
        indented = "\n".join("    " + l if l else l for l in text.splitlines())
        (HERE / "src" / out).write_text(
            f"// @generated by systest/gen.py from mpdecimal.h ({MACHINES[machine][0]}, {target})\n\n"
            f"pub mod c {{\n{indented}\n}}\n\n{checks(text, consts)}\n")
        print(f"systest/src/{out}")


if __name__ == "__main__":
    main()
