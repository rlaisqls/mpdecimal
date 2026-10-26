use crate::common::*;
use crate::context::mpd_addstatus_raise;
use crate::io::mpd_qformat;
use crate::io::mpd_qset_string;
use crate::mpdecimal::mpd_qabs;
use crate::mpdecimal::mpd_qabs_uint;
use crate::mpdecimal::mpd_qadd;
use crate::mpdecimal::mpd_qadd_i32;
use crate::mpdecimal::mpd_qadd_i64;
use crate::mpdecimal::mpd_qadd_ssize;
use crate::mpdecimal::mpd_qadd_u32;
use crate::mpdecimal::mpd_qadd_u64;
use crate::mpdecimal::mpd_qadd_uint;
use crate::mpdecimal::mpd_qand;
use crate::mpdecimal::mpd_qceil;
use crate::mpdecimal::mpd_qcheck_nan;
use crate::mpdecimal::mpd_qcheck_nans;
use crate::mpdecimal::mpd_qcmp;
use crate::mpdecimal::mpd_qcompare;
use crate::mpdecimal::mpd_qcompare_signal;
use crate::mpdecimal::mpd_qcopy;
use crate::mpdecimal::mpd_qcopy_abs;
use crate::mpdecimal::mpd_qcopy_negate;
use crate::mpdecimal::mpd_qcopy_sign;
use crate::mpdecimal::mpd_qdiv;
use crate::mpdecimal::mpd_qdiv_i32;
use crate::mpdecimal::mpd_qdiv_i64;
use crate::mpdecimal::mpd_qdiv_ssize;
use crate::mpdecimal::mpd_qdiv_u32;
use crate::mpdecimal::mpd_qdiv_u64;
use crate::mpdecimal::mpd_qdiv_uint;
use crate::mpdecimal::mpd_qdivint;
use crate::mpdecimal::mpd_qdivmod;
use crate::mpdecimal::mpd_qexp;
use crate::mpdecimal::mpd_qexport_u16;
use crate::mpdecimal::mpd_qexport_u32;
use crate::mpdecimal::mpd_qfinalize;
use crate::mpdecimal::mpd_qfloor;
use crate::mpdecimal::mpd_qfma;
use crate::mpdecimal::mpd_qget_i32;
use crate::mpdecimal::mpd_qget_i64;
use crate::mpdecimal::mpd_qget_ssize;
use crate::mpdecimal::mpd_qget_u32;
use crate::mpdecimal::mpd_qget_u64;
use crate::mpdecimal::mpd_qget_uint;
use crate::mpdecimal::mpd_qimport_u16;
use crate::mpdecimal::mpd_qimport_u32;
use crate::mpdecimal::mpd_qinvert;
use crate::mpdecimal::mpd_qinvroot;
use crate::mpdecimal::mpd_qln;
use crate::mpdecimal::mpd_qlog10;
use crate::mpdecimal::mpd_qlogb;
use crate::mpdecimal::mpd_qmax;
use crate::mpdecimal::mpd_qmax_mag;
use crate::mpdecimal::mpd_qmaxcoeff;
use crate::mpdecimal::mpd_qmin;
use crate::mpdecimal::mpd_qmin_mag;
use crate::mpdecimal::mpd_qminus;
use crate::mpdecimal::mpd_qmul;
use crate::mpdecimal::mpd_qmul_i32;
use crate::mpdecimal::mpd_qmul_i64;
use crate::mpdecimal::mpd_qmul_ssize;
use crate::mpdecimal::mpd_qmul_u32;
use crate::mpdecimal::mpd_qmul_u64;
use crate::mpdecimal::mpd_qmul_uint;
use crate::mpdecimal::mpd_qnext_minus;
use crate::mpdecimal::mpd_qnext_plus;
use crate::mpdecimal::mpd_qnext_toward;
use crate::mpdecimal::mpd_qor;
use crate::mpdecimal::mpd_qplus;
use crate::mpdecimal::mpd_qpow;
use crate::mpdecimal::mpd_qpowmod;
use crate::mpdecimal::mpd_qquantize;
use crate::mpdecimal::mpd_qreduce;
use crate::mpdecimal::mpd_qrem;
use crate::mpdecimal::mpd_qrem_near;
use crate::mpdecimal::mpd_qrescale;
use crate::mpdecimal::mpd_qrotate;
use crate::mpdecimal::mpd_qround_to_int;
use crate::mpdecimal::mpd_qround_to_intx;
use crate::mpdecimal::mpd_qscaleb;
use crate::mpdecimal::mpd_qset_i32;
use crate::mpdecimal::mpd_qset_i64;
use crate::mpdecimal::mpd_qset_ssize;
use crate::mpdecimal::mpd_qset_u32;
use crate::mpdecimal::mpd_qset_u64;
use crate::mpdecimal::mpd_qset_uint;
use crate::mpdecimal::mpd_qshift;
use crate::mpdecimal::mpd_qshiftl;
use crate::mpdecimal::mpd_qshiftn;
use crate::mpdecimal::mpd_qshiftr;
use crate::mpdecimal::mpd_qsqrt;
use crate::mpdecimal::mpd_qsset_i32;
#[cfg(target_pointer_width = "64")]
use crate::mpdecimal::mpd_qsset_i64;
use crate::mpdecimal::mpd_qsset_ssize;
use crate::mpdecimal::mpd_qsset_u32;
#[cfg(target_pointer_width = "64")]
use crate::mpdecimal::mpd_qsset_u64;
use crate::mpdecimal::mpd_qsset_uint;
use crate::mpdecimal::mpd_qsub;
use crate::mpdecimal::mpd_qsub_i32;
use crate::mpdecimal::mpd_qsub_i64;
use crate::mpdecimal::mpd_qsub_ssize;
use crate::mpdecimal::mpd_qsub_u32;
use crate::mpdecimal::mpd_qsub_u64;
use crate::mpdecimal::mpd_qsub_uint;
use crate::mpdecimal::mpd_qtrunc;
use crate::mpdecimal::mpd_qxor;
pub unsafe extern "C" fn mpd_format(
    dec: *const mpd_t,
    fmt: *const ::core::ffi::c_char,
    ctx: *mut mpd_context_t,
) -> *mut ::core::ffi::c_char {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: *mut ::core::ffi::c_char = mpd_qformat(dec, fmt, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_import_u16(
    result: *mut mpd_t,
    srcdata: *const uint16_t,
    srclen: size_t,
    srcsign: uint8_t,
    base: uint32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qimport_u16(result, srcdata, srclen, srcsign, base, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_import_u32(
    result: *mut mpd_t,
    srcdata: *const uint32_t,
    srclen: size_t,
    srcsign: uint8_t,
    base: uint32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qimport_u32(result, srcdata, srclen, srcsign, base, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_export_u16(
    rdata: *mut *mut uint16_t,
    rlen: size_t,
    base: uint32_t,
    src: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> size_t {
    let mut status: uint32_t = 0 as uint32_t;
    let n: size_t = mpd_qexport_u16(rdata, rlen, base, src, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    n
}
pub unsafe extern "C" fn mpd_export_u32(
    rdata: *mut *mut uint32_t,
    rlen: size_t,
    base: uint32_t,
    src: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> size_t {
    let mut status: uint32_t = 0 as uint32_t;
    let n: size_t = mpd_qexport_u32(rdata, rlen, base, src, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    n
}
pub unsafe extern "C" fn mpd_finalize(result: *mut mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qfinalize(result, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_check_nan(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcheck_nan(result, a, ctx, &raw mut status) != 0 {
        mpd_addstatus_raise(ctx, status);
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
pub unsafe extern "C" fn mpd_check_nans(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcheck_nans(result, a, b, ctx, &raw mut status) != 0 {
        mpd_addstatus_raise(ctx, status);
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
pub unsafe extern "C" fn mpd_set_string(
    result: *mut mpd_t,
    s: *const ::core::ffi::c_char,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_string(result, s, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_maxcoeff(result: *mut mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmaxcoeff(result, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sset_ssize(
    result: *mut mpd_t,
    a: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_ssize(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sset_i32(result: *mut mpd_t, a: int32_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_i32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_sset_i64(result: *mut mpd_t, a: int64_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_i64(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sset_uint(result: *mut mpd_t, a: mpd_uint_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_uint(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sset_u32(result: *mut mpd_t, a: uint32_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_u32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_sset_u64(result: *mut mpd_t, a: uint64_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_u64(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_set_ssize(
    result: *mut mpd_t,
    a: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_ssize(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_set_i32(result: *mut mpd_t, a: int32_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_i32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_set_i64(result: *mut mpd_t, a: int64_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_i64(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_set_uint(result: *mut mpd_t, a: mpd_uint_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_uint(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_set_u32(result: *mut mpd_t, a: uint32_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_u32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_set_u64(result: *mut mpd_t, a: uint64_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_u64(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_get_ssize(a: *const mpd_t, ctx: *mut mpd_context_t) -> mpd_ssize_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: mpd_ssize_t = mpd_qget_ssize(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_get_i32(a: *const mpd_t, ctx: *mut mpd_context_t) -> int32_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: int32_t = mpd_qget_i32(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_get_i64(a: *const mpd_t, ctx: *mut mpd_context_t) -> int64_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: int64_t = mpd_qget_i64(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_get_uint(a: *const mpd_t, ctx: *mut mpd_context_t) -> mpd_uint_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: mpd_uint_t = mpd_qget_uint(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_abs_uint(a: *const mpd_t, ctx: *mut mpd_context_t) -> mpd_uint_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: mpd_uint_t = mpd_qabs_uint(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_get_u32(a: *const mpd_t, ctx: *mut mpd_context_t) -> uint32_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: uint32_t = mpd_qget_u32(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_get_u64(a: *const mpd_t, ctx: *mut mpd_context_t) -> uint64_t {
    let mut status: uint32_t = 0 as uint32_t;
    let ret: uint64_t = mpd_qget_u64(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    ret
}
pub unsafe extern "C" fn mpd_and(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qand(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_copy(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy(result, a, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
pub unsafe extern "C" fn mpd_canonical(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    mpd_copy(result, a, ctx);
}
pub unsafe extern "C" fn mpd_copy_abs(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy_abs(result, a, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
pub unsafe extern "C" fn mpd_copy_negate(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy_negate(result, a, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
pub unsafe extern "C" fn mpd_copy_sign(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy_sign(result, a, b, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
pub unsafe extern "C" fn mpd_invert(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qinvert(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_logb(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qlogb(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_or(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qor(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_rotate(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrotate(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_scaleb(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qscaleb(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_shiftl(
    result: *mut mpd_t,
    a: *const mpd_t,
    n: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qshiftl(result, a, n, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_shiftr(
    result: *mut mpd_t,
    a: *const mpd_t,
    n: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) -> mpd_uint_t {
    let mut status: uint32_t = 0 as uint32_t;
    let rnd: mpd_uint_t = mpd_qshiftr(result, a, n, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    rnd
}
pub unsafe extern "C" fn mpd_shiftn(
    result: *mut mpd_t,
    a: *const mpd_t,
    n: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qshiftn(result, a, n, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_shift(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qshift(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_xor(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qxor(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_abs(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qabs(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_cmp(
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    let c: ::core::ffi::c_int = mpd_qcmp(a, b, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    c
}
pub unsafe extern "C" fn mpd_compare(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    let c: ::core::ffi::c_int = mpd_qcompare(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    c
}
pub unsafe extern "C" fn mpd_compare_signal(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    let c: ::core::ffi::c_int = mpd_qcompare_signal(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    c
}
pub unsafe extern "C" fn mpd_add(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_add_ssize(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_add_i32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_add_i64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_add_uint(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_uint_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_add_u32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_add_u64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub_ssize(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub_i32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub_i64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub_uint(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_uint_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub_u32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sub_u64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div(
    q: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv(q, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div_ssize(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div_i32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div_i64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div_uint(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_uint_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div_u32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_div_u64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_divmod(
    q: *mut mpd_t,
    r: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdivmod(q, r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_divint(
    q: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdivint(q, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_exp(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qexp(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_fma(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    c: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qfma(result, a, b, c, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_ln(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qln(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_log10(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qlog10(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_max(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmax(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_max_mag(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmax_mag(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_min(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmin(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_min_mag(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmin_mag(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_minus(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qminus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul_ssize(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul_i32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul_i64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: int64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul_uint(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: mpd_uint_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul_u32(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint32_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_mul_u64(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: uint64_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_next_minus(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qnext_minus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_next_plus(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qnext_plus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_next_toward(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qnext_toward(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_plus(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qplus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_pow(
    result: *mut mpd_t,
    base: *const mpd_t,
    exp: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qpow(result, base, exp, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_powmod(
    result: *mut mpd_t,
    base: *const mpd_t,
    exp: *const mpd_t,
    mod_0: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qpowmod(result, base, exp, mod_0, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_quantize(
    result: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qquantize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_rescale(
    result: *mut mpd_t,
    a: *const mpd_t,
    exp: mpd_ssize_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrescale(result, a, exp, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_reduce(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qreduce(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_rem(
    r: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrem(r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_rem_near(
    r: *mut mpd_t,
    a: *const mpd_t,
    b: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrem_near(r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_round_to_intx(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qround_to_intx(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_round_to_int(
    result: *mut mpd_t,
    a: *const mpd_t,
    ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qround_to_int(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_trunc(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qtrunc(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_floor(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qfloor(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_ceil(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qceil(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_sqrt(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsqrt(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub unsafe extern "C" fn mpd_invroot(result: *mut mpd_t, a: *const mpd_t, ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qinvroot(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
