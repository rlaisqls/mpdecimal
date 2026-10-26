use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::shim::mpd_c2rust_stderr;
use crate::mpalloc::mpd_free;
use crate::mpalloc::mpd_calloc;
use crate::constants::mpd_bits;
extern "C" {
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputc(c: ::core::ffi::c_int, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub const BACKWARD_CYCLE: C2RustUnnamed = 1;
pub const FORWARD_CYCLE: C2RustUnnamed = 0;
unsafe extern "C" fn pointerswap(a: *mut *mut mpd_uint_t, b: *mut *mut mpd_uint_t) {
    ::core::ptr::swap(a, b);
}
unsafe extern "C" fn mul_size_t(a: mpd_size_t, b: mpd_size_t) -> mpd_size_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, a as mpd_uint_t, b as mpd_uint_t);
    if hi != 0 {
        fprintf(
            mpd_c2rust_stderr(),
            b"%s:%d: error: \0" as *const u8 as *const ::core::ffi::c_char,
            b"libmpdec/typearith.h\0" as *const u8 as *const ::core::ffi::c_char,
            617 as ::core::ffi::c_int,
        );
        fprintf(
            mpd_c2rust_stderr(),
            b"mul_size_t(): overflow: check the context\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fputc('\n' as i32, mpd_c2rust_stderr());
        abort();
    }
    lo as mpd_size_t
}
unsafe extern "C" fn mulmod_size_t(
    a: mpd_size_t,
    b: mpd_size_t,
    m: mpd_size_t,
) -> mpd_size_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut q: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, a as mpd_uint_t, b as mpd_uint_t);
    _mpd_div_words(&raw mut q, &raw mut r, hi, lo, m as mpd_uint_t);
    r as mpd_size_t
}
pub const BUFSIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const SIDE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub unsafe extern "C" fn std_trans(
    dest: *mut mpd_uint_t,
    src: *mut mpd_uint_t,
    rows: mpd_size_t,
    cols: mpd_size_t,
) {
    let mut idest: mpd_size_t;
    let mut isrc: mpd_size_t;
    let mut r: mpd_size_t;
    let mut c: mpd_size_t;
    r = 0 as mpd_size_t;
    while r < rows {
        isrc = r.wrapping_mul(cols);
        idest = r;
        c = 0 as mpd_size_t;
        while c < cols {
            *dest.add(idest) = *src.add(isrc);
            isrc = isrc.wrapping_add(1 as mpd_size_t);
            idest = idest.wrapping_add(rows);
            c = c.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
}
unsafe extern "C" fn swap_halfrows_pow2(
    matrix: *mut mpd_uint_t,
    rows: mpd_size_t,
    cols: mpd_size_t,
    dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf1: [mpd_uint_t; 4096] = [0; 4096];
    let mut buf2: [mpd_uint_t; 4096] = [0; 4096];
    let mut readbuf: *mut mpd_uint_t;
    let mut writebuf: *mut mpd_uint_t;
    let mut hp: *mut mpd_uint_t;
    let b: mpd_size_t = BUFSIZE as mpd_size_t;
    let mut stride: mpd_size_t;
    let mut hn: mpd_size_t;
    let mut offset: mpd_size_t;
    let mut next: mpd_size_t;
    if cols == mul_size_t(2 as mpd_size_t, rows) {
    } else {
        mpd_c2rust_assert_fail(
            b"cols == mul_size_t(2, rows)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let r: mpd_size_t = if dir == FORWARD_CYCLE as ::core::ffi::c_int {
        rows
    } else if dir == BACKWARD_CYCLE as ::core::ffi::c_int {
        2 as mpd_size_t
    } else {
        abort();
    };
    let m: mpd_size_t = cols.wrapping_sub(1 as mpd_size_t);
    let hmax: mpd_size_t = rows;
    let dbits: mpd_size_t = 8_usize.wrapping_mul(::core::mem::size_of::<mpd_size_t>() as usize) as mpd_size_t;
    let done: *mut mpd_size_t = mpd_calloc(
        hmax.wrapping_div(::core::mem::size_of::<mpd_size_t>() as mpd_size_t)
            .wrapping_add(1 as mpd_size_t),
        ::core::mem::size_of::<mpd_size_t>() as mpd_size_t,
    ) as *mut mpd_size_t;
    if done.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    hn = 1 as mpd_size_t;
    while hn <= hmax {
        if !(*done.add(hn.wrapping_div(dbits))
            & *(&raw const mpd_bits as *const mpd_size_t).add(hn.wrapping_rem(dbits))
            != 0)
        {
            readbuf = &raw mut buf1 as *mut mpd_uint_t;
            writebuf = &raw mut buf2 as *mut mpd_uint_t;
            offset = 0 as mpd_size_t;
            while offset < cols.wrapping_div(2 as mpd_size_t) {
                stride = if offset.wrapping_add(b) < cols.wrapping_div(2 as mpd_size_t) {
                    b
                } else {
                    cols.wrapping_div(2 as mpd_size_t).wrapping_sub(offset)
                };
                hp = matrix.add(hn.wrapping_mul(cols).wrapping_div(2 as mpd_size_t));
                memcpy(
                    readbuf as *mut ::core::ffi::c_void,
                    hp.add(offset) as *const ::core::ffi::c_void,
                    (stride as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                );
                pointerswap(&raw mut readbuf, &raw mut writebuf);
                next = mulmod_size_t(hn, r, m);
                hp = matrix.add(next.wrapping_mul(cols).wrapping_div(2 as mpd_size_t));
                while next != hn {
                    memcpy(
                        readbuf as *mut ::core::ffi::c_void,
                        hp.add(offset) as *const ::core::ffi::c_void,
                        (stride as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    memcpy(
                        hp.add(offset) as *mut ::core::ffi::c_void,
                        writebuf as *const ::core::ffi::c_void,
                        (stride as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    pointerswap(&raw mut readbuf, &raw mut writebuf);
                    *done.add(next.wrapping_div(dbits)) |= *(&raw const mpd_bits
                        as *const mpd_size_t)
                        .add(next.wrapping_rem(dbits));
                    next = mulmod_size_t(next, r, m);
                    hp = matrix
                        .add(next.wrapping_mul(cols).wrapping_div(2 as mpd_size_t));
                }
                memcpy(
                    hp.add(offset) as *mut ::core::ffi::c_void,
                    writebuf as *const ::core::ffi::c_void,
                    (stride as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                );
                *done.add(hn.wrapping_div(dbits)) |= *(&raw const mpd_bits
                    as *const mpd_size_t)
                    .add(hn.wrapping_rem(dbits));
                offset = offset.wrapping_add(b);
            }
        }
        hn = hn.wrapping_add(2 as mpd_size_t);
    }
    mpd_free.expect("non-null function pointer")(done as *mut ::core::ffi::c_void);
    1 as ::core::ffi::c_int
}
unsafe extern "C" fn squaretrans(buf: *mut mpd_uint_t, cols: mpd_size_t) {
    let mut tmp: mpd_uint_t;
    let mut idest: mpd_size_t;
    let mut isrc: mpd_size_t;
    let mut r: mpd_size_t;
    let mut c: mpd_size_t;
    r = 0 as mpd_size_t;
    while r < cols {
        c = r.wrapping_add(1 as mpd_size_t);
        isrc = r.wrapping_mul(cols).wrapping_add(c);
        idest = c.wrapping_mul(cols).wrapping_add(r);
        c = r.wrapping_add(1 as mpd_size_t);
        while c < cols {
            tmp = *buf.add(isrc);
            *buf.add(isrc) = *buf.add(idest);
            *buf.add(idest) = tmp;
            isrc = isrc.wrapping_add(1 as mpd_size_t);
            idest = idest.wrapping_add(cols);
            c = c.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
}
unsafe extern "C" fn squaretrans_pow2(matrix: *mut mpd_uint_t, size: mpd_size_t) {
    let mut buf1: [mpd_uint_t; 16384] = [0; 16384];
    let mut buf2: [mpd_uint_t; 16384] = [0; 16384];
    let mut to: *mut mpd_uint_t;
    let mut from: *mut mpd_uint_t;
    let mut b: mpd_size_t = size;
    let mut r: mpd_size_t;
    let mut c: mpd_size_t;
    let mut i: mpd_size_t;
    while b > SIDE as mpd_size_t {
        b >>= 1 as ::core::ffi::c_int;
    }
    r = 0 as mpd_size_t;
    while r < size {
        c = r;
        while c < size {
            from = matrix
                .add(r.wrapping_mul(size))
                .add(c);
            to = &raw mut buf1 as *mut mpd_uint_t;
            i = 0 as mpd_size_t;
            while i < b {
                memcpy(
                    to as *mut ::core::ffi::c_void,
                    from as *const ::core::ffi::c_void,
                    (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                );
                from = from.add(size);
                to = to.add(b);
                i = i.wrapping_add(1);
            }
            squaretrans(&raw mut buf1 as *mut mpd_uint_t, b);
            if r == c {
                to = matrix
                    .add(r.wrapping_mul(size))
                    .add(c);
                from = &raw mut buf1 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.add(b);
                    to = to.add(size);
                    i = i.wrapping_add(1);
                }
            } else {
                from = matrix
                    .add(c.wrapping_mul(size))
                    .add(r);
                to = &raw mut buf2 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.add(size);
                    to = to.add(b);
                    i = i.wrapping_add(1);
                }
                squaretrans(&raw mut buf2 as *mut mpd_uint_t, b);
                to = matrix
                    .add(c.wrapping_mul(size))
                    .add(r);
                from = &raw mut buf1 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.add(b);
                    to = to.add(size);
                    i = i.wrapping_add(1);
                }
                to = matrix
                    .add(r.wrapping_mul(size))
                    .add(c);
                from = &raw mut buf2 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.add(b);
                    to = to.add(size);
                    i = i.wrapping_add(1);
                }
            }
            c = c.wrapping_add(b);
        }
        r = r.wrapping_add(b);
    }
}
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub unsafe extern "C" fn transpose_pow2(
    matrix: *mut mpd_uint_t,
    rows: mpd_size_t,
    cols: mpd_size_t,
) -> ::core::ffi::c_int {
    let size: mpd_size_t = mul_size_t(rows, cols);
    if ispower2(rows) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(rows)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if ispower2(cols) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(cols)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if cols == rows {
        squaretrans_pow2(matrix, rows);
    } else if cols == mul_size_t(2 as mpd_size_t, rows) {
        if swap_halfrows_pow2(matrix, rows, cols, FORWARD_CYCLE as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        squaretrans_pow2(matrix, rows);
        squaretrans_pow2(
            matrix.add(size.wrapping_div(2 as mpd_size_t)),
            rows,
        );
    } else if rows == mul_size_t(2 as mpd_size_t, cols) {
        squaretrans_pow2(matrix, cols);
        squaretrans_pow2(
            matrix.add(size.wrapping_div(2 as mpd_size_t)),
            cols,
        );
        if swap_halfrows_pow2(matrix, cols, rows, BACKWARD_CYCLE as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        abort();
    }
    1 as ::core::ffi::c_int
}
