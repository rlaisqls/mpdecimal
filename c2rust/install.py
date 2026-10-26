#!/usr/bin/env python3
"""Install the merged crate as the crate core/ of the repository.

usage: install.py CRATE

Writes core/Cargo.toml, core/src/lib.rs and core/src/*.rs, and
rust-toolchain.toml in the repository root:

  - the modules of CRATE/src are installed as they are, except that the
    module paths crate::src::MOD become crate::MOD, since src/lib.rs
    declares the modules directly (c2rust's lib.rs declares them in a
    module src), and except for the exported symbols (see exports);
  - core/Cargo.toml names the package mpdecimal-core and declares the
    features capi and test-hooks.

The installed files are recorded with their SHA-256 in
c2rust/out/installed.sha256. The installation stops if an installed file was
changed since, or if a file of core/ is not one installed here, so that
changes made in core/ are not overwritten.
"""

import hashlib
import re
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
MANIFEST = HERE / "out" / "installed.sha256"

HEADER = ROOT / "libmpdec" / "mpdecimal.h.in"

CARGO_TOML = """\
[package]
name = "mpdecimal-core"
version = "0.0.0"
edition = "2021"
publish = false
autobins = false
autoexamples = false
autotests = false
autobenches = false

[features]
# Exports the globals of mpdecimal.h, which C code assigns (mpd_mallocfunc,
# mpd_traphandler, ...). Enabled by mpdecimal-sys, which exports the
# functions.
capi = []
# The test hooks of mpdecimal-testit (c2rust/test-hooks/README.md): extra
# symbols and fault injection used by the long tests. Never enable it
# outside of these tests.
test-hooks = []
"""

LIB_ATTRS = re.compile(r"^((?:#!\[.*\]\n)+)\npub mod src \{\n((?:    pub mod \w+;\n)+)\} // mod src\n$")


def header_names():
    """The functions and the globals declared in mpdecimal.h."""
    text = re.sub(r"^\s*#.*?(?<!\\)$", "", HEADER.read_text(), flags=re.M | re.S)
    functions = set(re.findall(r"\b(mpd_\w+)\s*\(", text))
    globals_ = set(re.findall(r"^extern\b[^;(]*?\b(\w+)\s*(?:\[[^\]]*\])?;", text, re.M))
    globals_ |= set(re.findall(r"^extern\b[^;]*\(\s*\*\s*(\w+)\s*\)", text, re.M))
    return functions, globals_


def exports(text, functions, globals_):
    """Replace #[no_mangle] so that the crate exports no symbols by itself.

    The functions of mpdecimal.h are exported by mpdecimal-sys, which wraps
    them; they lose #[no_mangle]. The globals of mpdecimal.h cannot be
    wrapped, since C code assigns them; they are exported with the feature
    capi. Everything else is exported with the feature test-hooks, as the
    long tests use internal symbols (_mpd_kmul, mpd_moduli, ...)."""
    out = []
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if line != "#[no_mangle]":
            out.append(line)
            continue
        m = re.match(r"pub (?:unsafe )?(?:extern \"C\" fn|static mut|static) (\w+)", lines[i + 1])
        if not m:
            raise SystemExit(f"unexpected item after #[no_mangle]: {lines[i + 1]!r}")
        if m.group(1) in functions and " fn " in lines[i + 1]:
            continue
        feature = "capi" if m.group(1) in globals_ else "test-hooks"
        out.append(f'#[cfg_attr(feature = "{feature}", no_mangle)]')
    return "\n".join(out)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_root():
    recorded = {}
    if MANIFEST.exists():
        for line in MANIFEST.read_text().splitlines():
            digest, name = line.split("  ", 1)
            recorded[name] = digest
    present = ["core/Cargo.toml", "rust-toolchain.toml"] + [
        str(p.relative_to(ROOT)) for p in sorted((ROOT / "core" / "src").glob("**/*")) if p.is_file()]
    for name in present:
        path = ROOT / name
        if not path.exists():
            continue
        if name not in recorded:
            raise SystemExit(f"{name} exists and was not installed by {Path(__file__).name}")
        if sha256(path) != recorded[name]:
            raise SystemExit(f"{name} was changed since the last installation")


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__.split("\n\n")[1])
    crate = Path(sys.argv[1])
    check_root()

    m = LIB_ATTRS.match((crate / "lib.rs").read_text())
    if not m:
        raise SystemExit(f"unexpected {crate / 'lib.rs'}")
    attrs, mods = m.groups()

    files = {
        "core/Cargo.toml": CARGO_TOML,
        "rust-toolchain.toml": (crate / "rust-toolchain.toml").read_text(),
        "core/src/lib.rs": attrs + "\n" + mods.replace("    pub mod", "pub mod"),
    }
    functions, globals_ = header_names()
    names = re.findall(r"pub mod (\w+);", mods)
    if sorted(names) != sorted(p.stem for p in (crate / "src").glob("*.rs")):
        raise SystemExit("lib.rs does not declare exactly the modules in src/")
    for name in names:
        text = (crate / "src" / f"{name}.rs").read_text()
        files[f"core/src/{name}.rs"] = exports(text.replace("crate::src::", "crate::"), functions, globals_)
    if "crate::src" in "".join(files.values()):
        raise SystemExit("crate::src remains")

    if (ROOT / "core" / "src").exists():
        shutil.rmtree(ROOT / "core" / "src")
    (ROOT / "core" / "src").mkdir(parents=True)
    manifest = []
    for name, text in files.items():
        path = ROOT / name
        path.write_text(text)
        manifest.append(f"{sha256(path)}  {name}")
    MANIFEST.write_text("\n".join(manifest) + "\n")
    print(f"installed {len(files)} files")


if __name__ == "__main__":
    main()
