use crate::common::*;
use crate::constants::mpd_pow10;
use crate::context::mpd_maxcontext;
use crate::mpalloc::mpd_alloc;
use crate::mpalloc::mpd_free;
use crate::mpalloc::mpd_realloc;
use crate::mpdecimal::mpd_del;
use crate::mpdecimal::mpd_isinfinite;
use crate::mpdecimal::mpd_isnan;
use crate::mpdecimal::mpd_isnegative;
use crate::mpdecimal::mpd_isqnan;
use crate::mpdecimal::mpd_isspecial;
use crate::mpdecimal::mpd_iszero;
use crate::mpdecimal::mpd_msword;
use crate::mpdecimal::mpd_qcopy;
use crate::mpdecimal::mpd_qfinalize;
use crate::mpdecimal::mpd_qrescale;
use crate::mpdecimal::mpd_qrescale_fmt;
use crate::mpdecimal::mpd_qresize;
use crate::mpdecimal::mpd_set_flags;
use crate::mpdecimal::mpd_set_negative;
use crate::mpdecimal::mpd_setdigits;
use crate::mpdecimal::mpd_seterror;
use crate::mpdecimal::mpd_setspecial;
use crate::mpdecimal::mpd_word_digits;
use crate::shim::mpd_c2rust_assert_fail;
use crate::shim::mpd_c2rust_errno;
use crate::shim::mpd_c2rust_stderr;
#[cfg(target_pointer_width = "32")]
use crate::shim::mpd_c2rust_strtol;
#[cfg(target_pointer_width = "64")]
use crate::shim::mpd_c2rust_strtoll;
extern "C" {
    fn isdigit(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tolower(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn localeconv() -> *mut lconv;
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn printf(format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        s: *mut ::core::ffi::c_char,
        n: size_t,
        format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(s: *const ::core::ffi::c_char, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn memmove(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(s: *const ::core::ffi::c_char) -> size_t;
    fn strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut ::core::ffi::c_char,
    pub thousands_sep: *mut ::core::ffi::c_char,
    pub grouping: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_spec_t {
    pub min_width: mpd_ssize_t,
    pub prec: mpd_ssize_t,
    pub type_0: ::core::ffi::c_char,
    pub align: ::core::ffi::c_char,
    pub sign: ::core::ffi::c_char,
    pub sign_coerce: ::core::ffi::c_char,
    pub fill: [::core::ffi::c_char; 5],
    pub dot: *const ::core::ffi::c_char,
    pub sep: *const ::core::ffi::c_char,
    pub grouping: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_mbstr_t {
    pub nbytes: mpd_ssize_t,
    pub nchars: mpd_ssize_t,
    pub cur: mpd_ssize_t,
    pub data: *mut ::core::ffi::c_char,
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const MPD_NUM_FLAGS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
unsafe extern "C" fn mod_mpd_ssize_t(a: mpd_ssize_t, m: mpd_ssize_t) -> mpd_ssize_t {
    let r: mpd_ssize_t = a % m;
    if r < 0 as mpd_ssize_t {
        r + m
    } else {
        r
    }
}
pub const SCHAR_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const CHAR_MAX: ::core::ffi::c_int = ::core::ffi::c_char::MAX as ::core::ffi::c_int;
unsafe extern "C" fn _mpd_strneq(
    mut s: *const ::core::ffi::c_char,
    mut l: *const ::core::ffi::c_char,
    mut u: *const ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    loop {
        n = n.wrapping_sub(1);
        if !(n != SIZE_MAX as size_t) {
            break;
        }
        if *s as ::core::ffi::c_int != *l as ::core::ffi::c_int
            && *s as ::core::ffi::c_int != *u as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        s = s.offset(1);
        u = u.offset(1);
        l = l.offset(1);
    }
    1 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn strtoexp(s: *const ::core::ffi::c_char) -> mpd_ssize_t {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
    let retval: mpd_ssize_t =
        mpd_c2rust_strtoll(s, &raw mut end, 10 as ::core::ffi::c_int) as mpd_ssize_t;
    if *mpd_c2rust_errno() == 0 as ::core::ffi::c_int
        && !(*s as ::core::ffi::c_int != '\0' as i32 && *end as ::core::ffi::c_int == '\0' as i32)
    {
        *mpd_c2rust_errno() = EINVAL;
    }
    retval
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn strtoexp(s: *const ::core::ffi::c_char) -> mpd_ssize_t {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
    let retval: mpd_ssize_t =
        mpd_c2rust_strtol(s, &raw mut end, 10 as ::core::ffi::c_int) as mpd_ssize_t;
    if *mpd_c2rust_errno() == 0 as ::core::ffi::c_int
        && !(*s as ::core::ffi::c_int != '\0' as i32 && *end as ::core::ffi::c_int == '\0' as i32)
    {
        *mpd_c2rust_errno() = EINVAL;
    }
    retval
}
unsafe extern "C" fn string_to_coeff(
    data: *mut mpd_uint_t,
    mut s: *const ::core::ffi::c_char,
    dpoint: *const ::core::ffi::c_char,
    r: ::core::ffi::c_int,
    mut len: size_t,
) {
    let mut j: ::core::ffi::c_int;
    if r > 0 as ::core::ffi::c_int {
        len = len.wrapping_sub(1);
        *data.add(len) = 0 as mpd_uint_t;
        j = 0 as ::core::ffi::c_int;
        while j < r {
            if s == dpoint {
                s = s.offset(1);
            }
            *data.add(len) = (10 as mpd_uint_t)
                .wrapping_mul(*data.add(len))
                .wrapping_add((*s as ::core::ffi::c_int - '0' as i32) as mpd_uint_t);
            j += 1;
            s = s.offset(1);
        }
    }
    loop {
        len = len.wrapping_sub(1);
        if !(len != SIZE_MAX as size_t) {
            break;
        }
        *data.add(len) = 0 as mpd_uint_t;
        j = 0 as ::core::ffi::c_int;
        while j < MPD_RDIGITS {
            if s == dpoint {
                s = s.offset(1);
            }
            *data.add(len) = (10 as mpd_uint_t)
                .wrapping_mul(*data.add(len))
                .wrapping_add((*s as ::core::ffi::c_int - '0' as i32) as mpd_uint_t);
            j += 1;
            s = s.offset(1);
        }
    }
}
unsafe extern "C" fn scan_dpoint_exp(
    mut s: *const ::core::ffi::c_char,
    dpoint: *mut *const ::core::ffi::c_char,
    exp: *mut *const ::core::ffi::c_char,
    end: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut coeff: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    *dpoint = ::core::ptr::null::<::core::ffi::c_char>();
    *exp = ::core::ptr::null::<::core::ffi::c_char>();
    while *s as ::core::ffi::c_int != '\0' as i32 {
        match *s as ::core::ffi::c_int {
            46 => {
                if !(*dpoint).is_null() || !(*exp).is_null() {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                *dpoint = s;
            }
            69 | 101 => {
                if !(*exp).is_null() {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                *exp = s;
                if *s.offset(1_isize) as ::core::ffi::c_int == '+' as i32
                    || *s.offset(1_isize) as ::core::ffi::c_int == '-' as i32
                {
                    s = s.offset(1);
                }
            }
            _ => {
                if isdigit(*s as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                if coeff.is_null() && (*exp).is_null() {
                    if *s as ::core::ffi::c_int == '0' as i32 {
                        if isdigit(*s.offset(1_isize) as ::core::ffi::c_uchar as ::core::ffi::c_int)
                            == 0
                            && !(*s.offset(1_isize) as ::core::ffi::c_int == '.' as i32
                                && isdigit(*s.offset(2_isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int)
                                    != 0)
                        {
                            coeff = s;
                        }
                    } else {
                        coeff = s;
                    }
                }
            }
        }
        s = s.offset(1);
    }
    *end = s;
    coeff
}
unsafe extern "C" fn scan_payload(
    mut s: *const ::core::ffi::c_char,
    end: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    while *s as ::core::ffi::c_int == '0' as i32 {
        s = s.offset(1);
    }
    let coeff: *const ::core::ffi::c_char = s;
    while isdigit(*s as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    *end = s;
    if *s as ::core::ffi::c_int == '\0' as i32 {
        coeff
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    }
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_qset_string(
    dec: *mut mpd_t,
    mut s: *const ::core::ffi::c_char,
    ctx: *const mpd_context_t,
    status: *mut uint32_t,
) {
    let mut q: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    let coeff: *const ::core::ffi::c_char;
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dpoint: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut exp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut digits: size_t;
    let mut sign: uint8_t = MPD_POS;
    mpd_set_flags(dec, 0 as uint8_t);
    (*dec).len = 0 as mpd_ssize_t;
    (*dec).exp = 0 as mpd_ssize_t;
    if *s as ::core::ffi::c_int == '+' as i32 {
        s = s.offset(1);
    } else if *s as ::core::ffi::c_int == '-' as i32 {
        mpd_set_negative(dec);
        sign = MPD_NEG;
        s = s.offset(1);
    }
    'conversion_error: {
        if _mpd_strneq(
            s,
            b"nan\0" as *const u8 as *const ::core::ffi::c_char,
            b"NAN\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0
        {
            // NaN
            s = s.offset(3_isize);
            mpd_setspecial(dec, sign, MPD_NAN);
            if *s as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            // validate payload: digits only
            coeff = scan_payload(s, &raw mut end);
            if coeff.is_null() {
                break 'conversion_error;
            }
            // payload consists entirely of zeros
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as i64 as size_t;
            // prec >= 1, clamp is 0 or 1
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                break 'conversion_error;
            }
        } else if _mpd_strneq(
            s,
            b"snan\0" as *const u8 as *const ::core::ffi::c_char,
            b"SNAN\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0
        {
            // sNaN
            s = s.offset(4_isize);
            mpd_setspecial(dec, sign, MPD_SNAN);
            if *s as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            // validate payload: digits only
            coeff = scan_payload(s, &raw mut end);
            if coeff.is_null() {
                break 'conversion_error;
            }
            // payload consists entirely of zeros
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as i64 as size_t;
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                break 'conversion_error;
            }
        } else if _mpd_strneq(
            s,
            b"inf\0" as *const u8 as *const ::core::ffi::c_char,
            b"INF\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0
        {
            s = s.offset(3_isize);
            if *s as ::core::ffi::c_int == '\0' as i32
                || _mpd_strneq(
                    s,
                    b"inity\0" as *const u8 as *const ::core::ffi::c_char,
                    b"INITY\0" as *const u8 as *const ::core::ffi::c_char,
                    6 as size_t,
                ) != 0
            {
                // numeric-value: infinity
                mpd_setspecial(dec, sign, MPD_INF);
                return;
            }
            break 'conversion_error;
        } else {
            // scan for start of coefficient, decimal point, indicator, end
            coeff = scan_dpoint_exp(s, &raw mut dpoint, &raw mut exp, &raw mut end);
            if coeff.is_null() {
                break 'conversion_error;
            }
            // numeric-value: [exponent-part]
            if !exp.is_null() {
                // exponent-part
                end = exp;
                exp = exp.offset(1);
                (*dec).exp = strtoexp(exp);
                if *mpd_c2rust_errno() != 0
                    && !(*mpd_c2rust_errno() == ERANGE
                        && ((*dec).exp == MPD_SSIZE_MAX as mpd_ssize_t
                            || (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t))
                {
                    break 'conversion_error;
                }
            }
            digits = end.offset_from(coeff) as i64 as size_t;
            if !dpoint.is_null() {
                let fracdigits: size_t = (end.offset_from(dpoint) as i64 - 1_i64) as size_t;
                if dpoint > coeff {
                    digits = digits.wrapping_sub(1);
                }
                if fracdigits as u64 > MPD_MAX_PREC as u64 {
                    break 'conversion_error;
                }
                if (*dec).exp < MPD_SSIZE_MIN as mpd_ssize_t + fracdigits as mpd_ssize_t {
                    (*dec).exp = MPD_SSIZE_MIN as mpd_ssize_t;
                } else {
                    (*dec).exp -= fracdigits as mpd_ssize_t;
                }
            }
            if digits as u64 > MPD_MAX_PREC as u64 {
                break 'conversion_error;
            }
            if (*dec).exp > MPD_EXP_INF as mpd_ssize_t {
                (*dec).exp = MPD_EXP_INF as mpd_ssize_t;
            }
            if (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t {
                (*dec).exp = (MPD_SSIZE_MIN + 1_i64) as mpd_ssize_t;
            }
        }
        _mpd_idiv_word(
            &raw mut q,
            &raw mut r,
            digits as mpd_ssize_t,
            MPD_RDIGITS as mpd_ssize_t,
        );
        let len: mpd_ssize_t = if r == 0 as mpd_ssize_t {
            q
        } else {
            q + 1 as mpd_ssize_t
        };
        if len == 0 as mpd_ssize_t {
            break 'conversion_error;
        }
        if mpd_qresize(dec, len, status) == 0 {
            mpd_seterror(dec, MPD_Malloc_error as uint32_t, status);
            return;
        }
        (*dec).len = len;
        string_to_coeff(
            (*dec).data,
            coeff,
            dpoint,
            r as ::core::ffi::c_int,
            len as size_t,
        );
        mpd_setdigits(dec);
        mpd_qfinalize(dec, ctx, status);
        return;
    }
    // standard wants a positive NaN
    mpd_seterror(dec, MPD_Conversion_syntax as uint32_t, status);
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_qset_string(
    dec: *mut mpd_t,
    mut s: *const ::core::ffi::c_char,
    ctx: *const mpd_context_t,
    status: *mut uint32_t,
) {
    let mut q: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    let coeff: *const ::core::ffi::c_char;
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dpoint: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut exp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut digits: size_t;
    let mut sign: uint8_t = MPD_POS;
    mpd_set_flags(dec, 0 as uint8_t);
    (*dec).len = 0 as mpd_ssize_t;
    (*dec).exp = 0 as mpd_ssize_t;
    if *s as ::core::ffi::c_int == '+' as i32 {
        s = s.offset(1);
    } else if *s as ::core::ffi::c_int == '-' as i32 {
        mpd_set_negative(dec);
        sign = MPD_NEG;
        s = s.offset(1);
    }
    'conversion_error: {
        if _mpd_strneq(
            s,
            b"nan\0" as *const u8 as *const ::core::ffi::c_char,
            b"NAN\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0
        {
            // NaN
            s = s.offset(3_isize);
            mpd_setspecial(dec, sign, MPD_NAN);
            if *s as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            // validate payload: digits only
            coeff = scan_payload(s, &raw mut end);
            if coeff.is_null() {
                break 'conversion_error;
            }
            // payload consists entirely of zeros
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as ::core::ffi::c_int as size_t;
            // prec >= 1, clamp is 0 or 1
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                break 'conversion_error;
            }
        } else if _mpd_strneq(
            s,
            b"snan\0" as *const u8 as *const ::core::ffi::c_char,
            b"SNAN\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0
        {
            // sNaN
            s = s.offset(4_isize);
            mpd_setspecial(dec, sign, MPD_SNAN);
            if *s as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            // validate payload: digits only
            coeff = scan_payload(s, &raw mut end);
            if coeff.is_null() {
                break 'conversion_error;
            }
            // payload consists entirely of zeros
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as ::core::ffi::c_int as size_t;
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                break 'conversion_error;
            }
        } else if _mpd_strneq(
            s,
            b"inf\0" as *const u8 as *const ::core::ffi::c_char,
            b"INF\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) != 0
        {
            s = s.offset(3_isize);
            if *s as ::core::ffi::c_int == '\0' as i32
                || _mpd_strneq(
                    s,
                    b"inity\0" as *const u8 as *const ::core::ffi::c_char,
                    b"INITY\0" as *const u8 as *const ::core::ffi::c_char,
                    6 as size_t,
                ) != 0
            {
                // numeric-value: infinity
                mpd_setspecial(dec, sign, MPD_INF);
                return;
            }
            break 'conversion_error;
        } else {
            // scan for start of coefficient, decimal point, indicator, end
            coeff = scan_dpoint_exp(s, &raw mut dpoint, &raw mut exp, &raw mut end);
            if coeff.is_null() {
                break 'conversion_error;
            }
            // numeric-value: [exponent-part]
            if !exp.is_null() {
                // exponent-part
                end = exp;
                exp = exp.offset(1);
                (*dec).exp = strtoexp(exp);
                if *mpd_c2rust_errno() != 0
                    && !(*mpd_c2rust_errno() == ERANGE
                        && ((*dec).exp == MPD_SSIZE_MAX as mpd_ssize_t
                            || (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t))
                {
                    break 'conversion_error;
                }
            }
            digits = end.offset_from(coeff) as ::core::ffi::c_int as size_t;
            if !dpoint.is_null() {
                let fracdigits: size_t = (end.offset_from(dpoint) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as size_t;
                if dpoint > coeff {
                    digits = digits.wrapping_sub(1);
                }
                if fracdigits > MPD_MAX_PREC as size_t {
                    break 'conversion_error;
                }
                if (*dec).exp < MPD_SSIZE_MIN as mpd_ssize_t + fracdigits as mpd_ssize_t {
                    (*dec).exp = MPD_SSIZE_MIN as mpd_ssize_t;
                } else {
                    (*dec).exp -= fracdigits as mpd_ssize_t;
                }
            }
            if digits > MPD_MAX_PREC as size_t {
                break 'conversion_error;
            }
            if (*dec).exp > MPD_EXP_INF as mpd_ssize_t {
                (*dec).exp = MPD_EXP_INF as mpd_ssize_t;
            }
            if (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t {
                (*dec).exp = (MPD_SSIZE_MIN + 1 as ::core::ffi::c_int) as mpd_ssize_t;
            }
        }
        _mpd_idiv_word(
            &raw mut q,
            &raw mut r,
            digits as mpd_ssize_t,
            MPD_RDIGITS as mpd_ssize_t,
        );
        let len: mpd_ssize_t = if r == 0 as mpd_ssize_t {
            q
        } else {
            q + 1 as mpd_ssize_t
        };
        if len == 0 as mpd_ssize_t {
            break 'conversion_error;
        }
        if mpd_qresize(dec, len, status) == 0 {
            mpd_seterror(dec, MPD_Malloc_error as uint32_t, status);
            return;
        }
        (*dec).len = len;
        string_to_coeff(
            (*dec).data,
            coeff,
            dpoint,
            r as ::core::ffi::c_int,
            len as size_t,
        );
        mpd_setdigits(dec);
        mpd_qfinalize(dec, ctx, status);
        return;
    }
    // standard wants a positive NaN
    mpd_seterror(dec, MPD_Conversion_syntax as uint32_t, status);
}
pub unsafe extern "C" fn mpd_qset_string_exact(
    dec: *mut mpd_t,
    s: *const ::core::ffi::c_char,
    status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qset_string(dec, s, &raw mut maxcontext, &raw mut workstatus);
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(dec, MPD_Invalid_operation as uint32_t, status);
    }
    *status = (*status as ::core::ffi::c_uint
        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint) as uint32_t;
}
/// Print word x with n decimal digits to string s. dot is either NULL or the
/// location of a decimal point. For n outside of 2..=MPD_RDIGITS+1, only the
/// last digit is printed, as the cases of the C switch do.
unsafe extern "C" fn word_to_string(
    mut s: *mut ::core::ffi::c_char,
    mut x: mpd_uint_t,
    n: ::core::ffi::c_int,
    dot: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let first: ::core::ffi::c_int = if (2..=MPD_RDIGITS + 1).contains(&n) {
        n - 1
    } else {
        0
    };
    for k in (1..=first).rev() {
        let d: mpd_uint_t = *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize);
        if s == dot {
            *s = '.' as i32 as ::core::ffi::c_char;
            s = s.add(1);
        }
        *s = ('0' as i32 + x.wrapping_div(d) as ::core::ffi::c_char as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        s = s.add(1);
        x = x.wrapping_rem(d);
    }
    if s == dot {
        *s = '.' as i32 as ::core::ffi::c_char;
        s = s.add(1);
    }
    *s = ('0' as i32 + x as ::core::ffi::c_char as ::core::ffi::c_int) as ::core::ffi::c_char;
    s = s.add(1);
    *s = '\0' as i32 as ::core::ffi::c_char;
    s
}
unsafe extern "C" fn exp_to_string(
    mut s: *mut ::core::ffi::c_char,
    mut x: mpd_ssize_t,
) -> *mut ::core::ffi::c_char {
    let mut sign: ::core::ffi::c_char = '+' as i32 as ::core::ffi::c_char;
    if x < 0 as mpd_ssize_t {
        sign = '-' as i32 as ::core::ffi::c_char;
        x = -x;
    }
    *s = sign;
    s = s.add(1);
    word_to_string(
        s,
        x as mpd_uint_t,
        mpd_word_digits(x as mpd_uint_t),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    )
}
unsafe extern "C" fn coeff_to_string(
    mut s: *mut ::core::ffi::c_char,
    dec: *const mpd_t,
) -> *mut ::core::ffi::c_char {
    let mut x: mpd_uint_t;
    let mut i: mpd_ssize_t;
    x = mpd_msword(dec);
    s = word_to_string(
        s,
        x,
        mpd_word_digits(x),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    i = (*dec).len - 2 as mpd_ssize_t;
    while i >= 0 as mpd_ssize_t {
        x = *(*dec).data.offset(i as isize);
        s = word_to_string(
            s,
            x,
            MPD_RDIGITS,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        i -= 1;
    }
    s
}
unsafe extern "C" fn coeff_to_string_dot(
    mut s: *mut ::core::ffi::c_char,
    dot: *mut ::core::ffi::c_char,
    dec: *const mpd_t,
) -> *mut ::core::ffi::c_char {
    let mut x: mpd_uint_t;
    let mut i: mpd_ssize_t;
    x = mpd_msword(dec);
    s = word_to_string(s, x, mpd_word_digits(x), dot);
    i = (*dec).len - 2 as mpd_ssize_t;
    while i >= 0 as mpd_ssize_t {
        x = *(*dec).data.offset(i as isize);
        s = word_to_string(s, x, MPD_RDIGITS, dot);
        i -= 1;
    }
    s
}
pub const MPD_FMT_LOWER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MPD_FMT_UPPER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MPD_FMT_TOSCI: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MPD_FMT_TOENG: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MPD_FMT_EXP: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MPD_FMT_FIXED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MPD_FMT_PERCENT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MPD_FMT_SIGN_SPACE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const MPD_FMT_SIGN_PLUS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MPD_FMT_SIGN_COERCE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const MPD_DEFAULT_DOTPLACE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_to_string(
    result: *mut *mut ::core::ffi::c_char,
    dec: *const mpd_t,
    flags: ::core::ffi::c_int,
    mut dplace: mpd_ssize_t,
) -> mpd_ssize_t {
    let decstring: *mut ::core::ffi::c_char;
    let mut cp: *mut ::core::ffi::c_char;
    let ldigits: mpd_ssize_t;
    let mut mem: mpd_ssize_t;
    let mut k: mpd_ssize_t;
    if mpd_isspecial(dec) != 0 {
        mem = ::core::mem::size_of::<[::core::ffi::c_char; 11]>() as mpd_ssize_t;
        if mpd_isnan(dec) != 0 && (*dec).len > 0 as mpd_ssize_t {
            mem += (*dec).digits;
        }
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as ::core::ffi::c_int) as mpd_ssize_t;
        }
        if mpd_isnegative(dec) != 0 {
            *cp = '-' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            *cp = ' ' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            *cp = '+' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        }
        if mpd_isnan(dec) != 0 {
            if mpd_isqnan(dec) != 0 {
                strcpy(cp, b"NaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(3_isize);
            } else {
                strcpy(cp, b"sNaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(4_isize);
            }
            if (*dec).len > 0 as mpd_ssize_t {
                cp = coeff_to_string(cp, dec);
            }
        } else if mpd_isinfinite(dec) != 0 {
            strcpy(cp, b"Infinity\0" as *const u8 as *const ::core::ffi::c_char);
            cp = cp.offset(8_isize);
        } else {
            abort();
        }
    } else {
        if (*dec).len > 0 as mpd_ssize_t {
        } else {
            mpd_c2rust_assert_fail(b"dec->len > 0\0" as *const u8 as *const ::core::ffi::c_char);
        };
        ldigits = (*dec).digits + (*dec).exp;
        if !(flags & MPD_FMT_EXP != 0) {
            if flags & MPD_FMT_FIXED != 0
                || (*dec).exp <= 0 as mpd_ssize_t
                    && ldigits > -(6 as ::core::ffi::c_int) as mpd_ssize_t
            {
                dplace = ldigits;
            } else if flags & MPD_FMT_TOENG != 0 {
                if mpd_iszero(dec) != 0 {
                    dplace = -(1 as ::core::ffi::c_int) as mpd_ssize_t
                        + mod_mpd_ssize_t((*dec).exp + 2 as mpd_ssize_t, 3 as mpd_ssize_t);
                } else {
                    dplace += mod_mpd_ssize_t(ldigits - 1 as mpd_ssize_t, 3 as mpd_ssize_t);
                }
            }
        }
        if dplace <= 0 as mpd_ssize_t {
            mem = -dplace + (*dec).digits + 2 as mpd_ssize_t;
        } else if dplace >= (*dec).digits {
            mem = dplace;
        } else {
            mem = (*dec).digits;
        }
        mem += (MPD_EXPDIGITS + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as mpd_ssize_t;
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as ::core::ffi::c_int) as mpd_ssize_t;
        }
        if mpd_isnegative(dec) != 0 && !(flags & MPD_FMT_SIGN_COERCE != 0 && mpd_iszero(dec) != 0) {
            *cp = '-' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            *cp = ' ' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            *cp = '+' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        }
        if dplace <= 0 as mpd_ssize_t {
            *cp = '0' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
            *cp = '.' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
            k = 0 as mpd_ssize_t;
            while k < -dplace {
                *cp = '0' as i32 as ::core::ffi::c_char;
                cp = cp.add(1);
                k += 1;
            }
            cp = coeff_to_string(cp, dec);
        } else if dplace >= (*dec).digits {
            cp = coeff_to_string(cp, dec);
            k = 0 as mpd_ssize_t;
            while k < dplace - (*dec).digits {
                *cp = '0' as i32 as ::core::ffi::c_char;
                cp = cp.add(1);
                k += 1;
            }
        } else {
            cp = coeff_to_string_dot(cp, cp.offset(dplace as isize), dec);
        }
        if ldigits != dplace || flags & MPD_FMT_EXP != 0 {
            *cp = (if flags & MPD_FMT_UPPER != 0 {
                'E' as i32
            } else {
                'e' as i32
            }) as ::core::ffi::c_char;
            cp = cp.add(1);
            cp = exp_to_string(cp, ldigits - dplace);
        }
    }
    if flags & MPD_FMT_PERCENT != 0 {
        *cp = '%' as i32 as ::core::ffi::c_char;
        cp = cp.add(1);
    }
    if cp < decstring.offset(mem as isize) {
    } else {
        mpd_c2rust_assert_fail(b"cp < decstring+mem\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if (cp.offset_from(decstring) as i64) < 9223372036854775807_i64 {
    } else {
        mpd_c2rust_assert_fail(
            b"cp-decstring < MPD_SSIZE_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *cp = '\0' as i32 as ::core::ffi::c_char;
    *result = decstring;
    cp.offset_from(decstring) as i64 as mpd_ssize_t
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_to_string(
    result: *mut *mut ::core::ffi::c_char,
    dec: *const mpd_t,
    flags: ::core::ffi::c_int,
    mut dplace: mpd_ssize_t,
) -> mpd_ssize_t {
    let decstring: *mut ::core::ffi::c_char;
    let mut cp: *mut ::core::ffi::c_char;
    let ldigits: mpd_ssize_t;
    let mut mem: mpd_ssize_t;
    let mut k: mpd_ssize_t;
    if mpd_isspecial(dec) != 0 {
        mem = ::core::mem::size_of::<[::core::ffi::c_char; 11]>() as mpd_ssize_t;
        if mpd_isnan(dec) != 0 && (*dec).len > 0 as mpd_ssize_t {
            mem += (*dec).digits;
        }
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as mpd_ssize_t);
        }
        if mpd_isnegative(dec) != 0 {
            *cp = '-' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            *cp = ' ' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            *cp = '+' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        }
        if mpd_isnan(dec) != 0 {
            if mpd_isqnan(dec) != 0 {
                strcpy(cp, b"NaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(3_isize);
            } else {
                strcpy(cp, b"sNaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(4_isize);
            }
            if (*dec).len > 0 as mpd_ssize_t {
                cp = coeff_to_string(cp, dec);
            }
        } else if mpd_isinfinite(dec) != 0 {
            strcpy(cp, b"Infinity\0" as *const u8 as *const ::core::ffi::c_char);
            cp = cp.offset(8_isize);
        } else {
            abort();
        }
    } else {
        if (*dec).len > 0 as mpd_ssize_t {
        } else {
            mpd_c2rust_assert_fail(b"dec->len > 0\0" as *const u8 as *const ::core::ffi::c_char);
        };
        ldigits = (*dec).digits + (*dec).exp;
        if !(flags & MPD_FMT_EXP != 0) {
            if flags & MPD_FMT_FIXED != 0
                || (*dec).exp <= 0 as mpd_ssize_t && ldigits > -(6 as mpd_ssize_t)
            {
                dplace = ldigits;
            } else if flags & MPD_FMT_TOENG != 0 {
                if mpd_iszero(dec) != 0 {
                    dplace = -(1 as mpd_ssize_t)
                        + mod_mpd_ssize_t((*dec).exp + 2 as mpd_ssize_t, 3 as mpd_ssize_t);
                } else {
                    dplace += mod_mpd_ssize_t(ldigits - 1 as mpd_ssize_t, 3 as mpd_ssize_t);
                }
            }
        }
        if dplace <= 0 as mpd_ssize_t {
            mem = -dplace + (*dec).digits + 2 as mpd_ssize_t;
        } else if dplace >= (*dec).digits {
            mem = dplace;
        } else {
            mem = (*dec).digits;
        }
        mem = (mem as ::core::ffi::c_int
            + (MPD_EXPDIGITS + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int))
            as mpd_ssize_t;
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as mpd_ssize_t);
        }
        if mpd_isnegative(dec) != 0 && !(flags & MPD_FMT_SIGN_COERCE != 0 && mpd_iszero(dec) != 0) {
            *cp = '-' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            *cp = ' ' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            *cp = '+' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
        }
        if dplace <= 0 as mpd_ssize_t {
            *cp = '0' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
            *cp = '.' as i32 as ::core::ffi::c_char;
            cp = cp.add(1);
            k = 0 as mpd_ssize_t;
            while k < -dplace {
                *cp = '0' as i32 as ::core::ffi::c_char;
                cp = cp.add(1);
                k += 1;
            }
            cp = coeff_to_string(cp, dec);
        } else if dplace >= (*dec).digits {
            cp = coeff_to_string(cp, dec);
            k = 0 as mpd_ssize_t;
            while k < dplace - (*dec).digits {
                *cp = '0' as i32 as ::core::ffi::c_char;
                cp = cp.add(1);
                k += 1;
            }
        } else {
            cp = coeff_to_string_dot(cp, cp.offset(dplace as isize), dec);
        }
        if ldigits != dplace || flags & MPD_FMT_EXP != 0 {
            *cp = (if flags & MPD_FMT_UPPER != 0 {
                'E' as i32
            } else {
                'e' as i32
            }) as ::core::ffi::c_char;
            cp = cp.add(1);
            cp = exp_to_string(cp, ldigits - dplace);
        }
    }
    if flags & MPD_FMT_PERCENT != 0 {
        *cp = '%' as i32 as ::core::ffi::c_char;
        cp = cp.add(1);
    }
    if cp < decstring.offset(mem as isize) {
    } else {
        mpd_c2rust_assert_fail(b"cp < decstring+mem\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if (cp.offset_from(decstring) as ::core::ffi::c_int) < 2147483647 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"cp-decstring < MPD_SSIZE_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *cp = '\0' as i32 as ::core::ffi::c_char;
    *result = decstring;
    cp.offset_from(decstring) as ::core::ffi::c_int as mpd_ssize_t
}
pub unsafe extern "C" fn mpd_to_sci(
    dec: *const mpd_t,
    fmt: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOSCI;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    _mpd_to_string(
        &raw mut res,
        dec,
        flags,
        MPD_DEFAULT_DOTPLACE as mpd_ssize_t,
    );
    res
}
pub unsafe extern "C" fn mpd_to_eng(
    dec: *const mpd_t,
    fmt: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOENG;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    _mpd_to_string(
        &raw mut res,
        dec,
        flags,
        MPD_DEFAULT_DOTPLACE as mpd_ssize_t,
    );
    res
}
pub unsafe extern "C" fn mpd_to_sci_size(
    res: *mut *mut ::core::ffi::c_char,
    dec: *const mpd_t,
    fmt: ::core::ffi::c_int,
) -> mpd_ssize_t {
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOSCI;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    _mpd_to_string(res, dec, flags, MPD_DEFAULT_DOTPLACE as mpd_ssize_t)
}
pub unsafe extern "C" fn mpd_to_eng_size(
    res: *mut *mut ::core::ffi::c_char,
    dec: *const mpd_t,
    fmt: ::core::ffi::c_int,
) -> mpd_ssize_t {
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOENG;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    _mpd_to_string(res, dec, flags, MPD_DEFAULT_DOTPLACE as mpd_ssize_t)
}
unsafe extern "C" fn _mpd_copy_utf8(
    dest: *mut ::core::ffi::c_char,
    s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *const ::core::ffi::c_uchar = s as *const ::core::ffi::c_uchar;
    'error: {
        if *cp == 0 {
            // empty string
            *dest = '\0' as i32 as ::core::ffi::c_char;
            return 0 as ::core::ffi::c_int;
        }
        if *cp <= 0x7f {
            // ascii
            *dest = *cp as ::core::ffi::c_char;
            *dest.offset(1_isize) = '\0' as i32 as ::core::ffi::c_char;
            return 1 as ::core::ffi::c_int;
        }
        let (lb, ub, count): (
            ::core::ffi::c_uchar,
            ::core::ffi::c_uchar,
            ::core::ffi::c_int,
        ) = if 0xc2 <= *cp && *cp <= 0xdf {
            (0x80, 0xbf, 2)
        } else if *cp == 0xe0 {
            (0xa0, 0xbf, 3)
        } else if *cp <= 0xec {
            (0x80, 0xbf, 3)
        } else if *cp == 0xed {
            (0x80, 0x9f, 3)
        } else if *cp <= 0xef {
            (0x80, 0xbf, 3)
        } else if *cp == 0xf0 {
            (0x90, 0xbf, 4)
        } else if *cp <= 0xf3 {
            (0x80, 0xbf, 4)
        } else if *cp == 0xf4 {
            (0x80, 0x8f, 4)
        } else {
            // invalid
            break 'error;
        };
        *dest = *cp as ::core::ffi::c_char;
        cp = cp.add(1);
        if *cp < lb || ub < *cp {
            break 'error;
        }
        *dest.offset(1_isize) = *cp as ::core::ffi::c_char;
        cp = cp.add(1);
        let mut i: ::core::ffi::c_int = 2;
        while i < count {
            if *cp < 0x80 || 0xbf < *cp {
                break 'error;
            }
            *dest.offset(i as isize) = *cp as ::core::ffi::c_char;
            cp = cp.add(1);
            i += 1;
        }
        *dest.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
        return count;
    }
    *dest = '\0' as i32 as ::core::ffi::c_char;
    -(1 as ::core::ffi::c_int)
}
pub unsafe extern "C" fn mpd_validate_lconv(spec: *mut mpd_spec_t) -> ::core::ffi::c_int {
    let mut cp: *const ::core::ffi::c_char = (*spec).grouping;
    while *cp as ::core::ffi::c_int != '\0' as i32 {
        if (*cp as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        cp = cp.add(1);
    }
    let n: size_t = strlen((*spec).dot);
    if n == 0 as size_t || n > 4 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if strlen((*spec).sep) > 4 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    0 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_parse_fmt_str(
    spec: *mut mpd_spec_t,
    fmt: *const ::core::ffi::c_char,
    caps: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = fmt as *mut ::core::ffi::c_char;
    let mut have_align: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*spec).min_width = 0 as mpd_ssize_t;
    (*spec).prec = -(1 as ::core::ffi::c_int) as mpd_ssize_t;
    (*spec).type_0 = (if caps != 0 { 'G' as i32 } else { 'g' as i32 }) as ::core::ffi::c_char;
    (*spec).align = '>' as i32 as ::core::ffi::c_char;
    (*spec).sign = '-' as i32 as ::core::ffi::c_char;
    (*spec).sign_coerce = 0 as ::core::ffi::c_char;
    (*spec).dot = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).sep = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).grouping = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let n: ::core::ffi::c_int =
        _mpd_copy_utf8(&raw mut (*spec).fill as *mut ::core::ffi::c_char, cp);
    if n < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if *cp as ::core::ffi::c_int != 0
        && (*cp.offset(n as isize) as ::core::ffi::c_int == '<' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '>' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '=' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '^' as i32)
    {
        cp = cp.offset(n as isize);
        (*spec).align = *cp;
        cp = cp.add(1);
        have_align = 1 as ::core::ffi::c_int;
    } else {
        (*spec).fill[0_usize] = ' ' as i32 as ::core::ffi::c_char;
        (*spec).fill[1_usize] = '\0' as i32 as ::core::ffi::c_char;
        if *cp as ::core::ffi::c_int == '<' as i32
            || *cp as ::core::ffi::c_int == '>' as i32
            || *cp as ::core::ffi::c_int == '=' as i32
            || *cp as ::core::ffi::c_int == '^' as i32
        {
            (*spec).align = *cp;
            cp = cp.add(1);
            have_align = 1 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == '+' as i32
        || *cp as ::core::ffi::c_int == '-' as i32
        || *cp as ::core::ffi::c_int == ' ' as i32
    {
        (*spec).sign = *cp;
        cp = cp.add(1);
    }
    if *cp as ::core::ffi::c_int == 'z' as i32 {
        (*spec).sign_coerce = 1 as ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '0' as i32 {
        if have_align != 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*spec).align = 'z' as i32 as ::core::ffi::c_char;
        (*spec).fill[0_usize] = *cp;
        cp = cp.add(1);
        (*spec).fill[1_usize] = '\0' as i32 as ::core::ffi::c_char;
    }
    if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        if *cp as ::core::ffi::c_int == '0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).min_width =
            mpd_c2rust_strtoll(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == ',' as i32 {
        (*spec).dot = b".\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).sep = b",\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).grouping = b"\x03\x03\0" as *const u8 as *const ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '.' as i32 {
        cp = cp.offset(1);
        if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).prec = mpd_c2rust_strtoll(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == 'E' as i32
        || *cp as ::core::ffi::c_int == 'e' as i32
        || *cp as ::core::ffi::c_int == 'F' as i32
        || *cp as ::core::ffi::c_int == 'f' as i32
        || *cp as ::core::ffi::c_int == 'G' as i32
        || *cp as ::core::ffi::c_int == 'g' as i32
        || *cp as ::core::ffi::c_int == '%' as i32
    {
        (*spec).type_0 = *cp;
        cp = cp.add(1);
    } else if *cp as ::core::ffi::c_int == 'N' as i32 || *cp as ::core::ffi::c_int == 'n' as i32 {
        if *(*spec).sep != 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*spec).type_0 = *cp;
        cp = cp.add(1);
        (*spec).type_0 = (if (*spec).type_0 as ::core::ffi::c_int == 'N' as i32 {
            'G' as i32
        } else {
            'g' as i32
        }) as ::core::ffi::c_char;
        let lc: *mut lconv = localeconv();
        (*spec).dot = (*lc).decimal_point;
        (*spec).sep = (*lc).thousands_sep;
        (*spec).grouping = (*lc).grouping;
        if mpd_validate_lconv(spec) < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int != '\0' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_parse_fmt_str(
    spec: *mut mpd_spec_t,
    fmt: *const ::core::ffi::c_char,
    caps: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = fmt as *mut ::core::ffi::c_char;
    let mut have_align: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*spec).min_width = 0 as mpd_ssize_t;
    (*spec).prec = -(1 as ::core::ffi::c_int) as mpd_ssize_t;
    (*spec).type_0 = (if caps != 0 { 'G' as i32 } else { 'g' as i32 }) as ::core::ffi::c_char;
    (*spec).align = '>' as i32 as ::core::ffi::c_char;
    (*spec).sign = '-' as i32 as ::core::ffi::c_char;
    (*spec).sign_coerce = 0 as ::core::ffi::c_char;
    (*spec).dot = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).sep = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).grouping = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let n: ::core::ffi::c_int =
        _mpd_copy_utf8(&raw mut (*spec).fill as *mut ::core::ffi::c_char, cp);
    if n < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if *cp as ::core::ffi::c_int != 0
        && (*cp.offset(n as isize) as ::core::ffi::c_int == '<' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '>' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '=' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '^' as i32)
    {
        cp = cp.offset(n as isize);
        (*spec).align = *cp;
        cp = cp.add(1);
        have_align = 1 as ::core::ffi::c_int;
    } else {
        (*spec).fill[0_usize] = ' ' as i32 as ::core::ffi::c_char;
        (*spec).fill[1_usize] = '\0' as i32 as ::core::ffi::c_char;
        if *cp as ::core::ffi::c_int == '<' as i32
            || *cp as ::core::ffi::c_int == '>' as i32
            || *cp as ::core::ffi::c_int == '=' as i32
            || *cp as ::core::ffi::c_int == '^' as i32
        {
            (*spec).align = *cp;
            cp = cp.add(1);
            have_align = 1 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == '+' as i32
        || *cp as ::core::ffi::c_int == '-' as i32
        || *cp as ::core::ffi::c_int == ' ' as i32
    {
        (*spec).sign = *cp;
        cp = cp.add(1);
    }
    if *cp as ::core::ffi::c_int == 'z' as i32 {
        (*spec).sign_coerce = 1 as ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '0' as i32 {
        if have_align != 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*spec).align = 'z' as i32 as ::core::ffi::c_char;
        (*spec).fill[0_usize] = *cp;
        cp = cp.add(1);
        (*spec).fill[1_usize] = '\0' as i32 as ::core::ffi::c_char;
    }
    if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        if *cp as ::core::ffi::c_int == '0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).min_width =
            mpd_c2rust_strtol(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == ',' as i32 {
        (*spec).dot = b".\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).sep = b",\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).grouping = b"\x03\x03\0" as *const u8 as *const ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '.' as i32 {
        cp = cp.offset(1);
        if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).prec = mpd_c2rust_strtol(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == 'E' as i32
        || *cp as ::core::ffi::c_int == 'e' as i32
        || *cp as ::core::ffi::c_int == 'F' as i32
        || *cp as ::core::ffi::c_int == 'f' as i32
        || *cp as ::core::ffi::c_int == 'G' as i32
        || *cp as ::core::ffi::c_int == 'g' as i32
        || *cp as ::core::ffi::c_int == '%' as i32
    {
        (*spec).type_0 = *cp;
        cp = cp.add(1);
    } else if *cp as ::core::ffi::c_int == 'N' as i32 || *cp as ::core::ffi::c_int == 'n' as i32 {
        if *(*spec).sep != 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*spec).type_0 = *cp;
        cp = cp.add(1);
        (*spec).type_0 = (if (*spec).type_0 as ::core::ffi::c_int == 'N' as i32 {
            'G' as i32
        } else {
            'g' as i32
        }) as ::core::ffi::c_char;
        let lc: *mut lconv = localeconv();
        (*spec).dot = (*lc).decimal_point;
        (*spec).sep = (*lc).thousands_sep;
        (*spec).grouping = (*lc).grouping;
        if mpd_validate_lconv(spec) < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int != '\0' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    1 as ::core::ffi::c_int
}
unsafe extern "C" fn _mpd_bcopy(
    dest: *mut ::core::ffi::c_char,
    src: *const ::core::ffi::c_char,
    mut n: mpd_ssize_t,
) {
    loop {
        n -= 1;
        if !(n >= 0 as mpd_ssize_t) {
            break;
        }
        *dest.offset(n as isize) = *src.offset(n as isize);
    }
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mbstr_copy_char(
    dest: *mut mpd_mbstr_t,
    src: *const ::core::ffi::c_char,
    n: mpd_ssize_t,
) {
    (*dest).nbytes += n;
    (*dest).nchars += (if n > 0 as mpd_ssize_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        _mpd_bcopy((*dest).data.offset((*dest).cur as isize), src, n);
    }
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mbstr_copy_char(
    dest: *mut mpd_mbstr_t,
    src: *const ::core::ffi::c_char,
    n: mpd_ssize_t,
) {
    (*dest).nbytes += n;
    (*dest).nchars = ((*dest).nchars as ::core::ffi::c_int
        + if n > 0 as mpd_ssize_t {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as mpd_ssize_t;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        _mpd_bcopy((*dest).data.offset((*dest).cur as isize), src, n);
    }
}
unsafe extern "C" fn _mbstr_copy_ascii(
    dest: *mut mpd_mbstr_t,
    src: *const ::core::ffi::c_char,
    n: mpd_ssize_t,
) {
    (*dest).nbytes += n;
    (*dest).nchars += n;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        _mpd_bcopy((*dest).data.offset((*dest).cur as isize), src, n);
    }
}
unsafe extern "C" fn _mbstr_copy_pad(dest: *mut mpd_mbstr_t, mut n: mpd_ssize_t) {
    (*dest).nbytes += n;
    (*dest).nchars += n;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        let cp: *mut ::core::ffi::c_char = (*dest).data.offset((*dest).cur as isize);
        loop {
            n -= 1;
            if !(n >= 0 as mpd_ssize_t) {
                break;
            }
            *cp.offset(n as isize) = '0' as i32 as ::core::ffi::c_char;
        }
    }
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_add_sep_dot(
    dest: *mut mpd_mbstr_t,
    sign: *const ::core::ffi::c_char,
    src: *const ::core::ffi::c_char,
    mut n_src: mpd_ssize_t,
    dot: *const ::core::ffi::c_char,
    rest: *const ::core::ffi::c_char,
    n_rest: mpd_ssize_t,
    spec: *const mpd_spec_t,
) {
    let mut consume: mpd_ssize_t;
    let mut g: *const ::core::ffi::c_char;
    let mut pad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let n_sign: mpd_ssize_t = (if !sign.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    let n_sep: mpd_ssize_t = strlen((*spec).sep) as mpd_ssize_t;
    (*dest).cur = (*dest).nbytes;
    (*dest).nchars = 0 as mpd_ssize_t;
    (*dest).nbytes = (*dest).nchars;
    _mbstr_copy_ascii(dest, rest, n_rest);
    if !dot.is_null() {
        _mbstr_copy_char(dest, dot, strlen(dot) as mpd_ssize_t);
    }
    g = (*spec).grouping;
    consume = *g as mpd_ssize_t;
    loop {
        if *g as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *g as ::core::ffi::c_int == CHAR_MAX
            || consume > n_src
        {
            consume = n_src;
        }
        n_src -= consume;
        if pad != 0 {
            _mbstr_copy_pad(dest, consume);
        } else {
            _mbstr_copy_ascii(dest, src.offset(n_src as isize), consume);
        }
        if n_src == 0 as mpd_ssize_t {
            if !((*spec).align as ::core::ffi::c_int == 'z' as i32
                && (*dest).nchars + n_sign < (*spec).min_width)
            {
                break;
            }
            n_src = (*spec).min_width - ((*dest).nchars + n_sign);
            consume = *g as mpd_ssize_t - consume;
            pad = 1 as ::core::ffi::c_int;
        } else {
            if n_sep > 0 as mpd_ssize_t {
                if pad != 0 && n_src > 1 as mpd_ssize_t {
                    n_src -= 1 as mpd_ssize_t;
                }
                _mbstr_copy_char(dest, (*spec).sep, n_sep);
            }
            if *g as ::core::ffi::c_int != 0 && *g.offset(1_isize) as ::core::ffi::c_int != 0 {
                g = g.offset(1);
            }
            consume = *g as mpd_ssize_t;
        }
    }
    if !sign.is_null() {
        _mbstr_copy_ascii(dest, sign, 1 as mpd_ssize_t);
    }
    if !(*dest).data.is_null() {
        *(*dest).data.offset((*dest).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_add_sep_dot(
    dest: *mut mpd_mbstr_t,
    sign: *const ::core::ffi::c_char,
    src: *const ::core::ffi::c_char,
    mut n_src: mpd_ssize_t,
    dot: *const ::core::ffi::c_char,
    rest: *const ::core::ffi::c_char,
    n_rest: mpd_ssize_t,
    spec: *const mpd_spec_t,
) {
    let mut consume: mpd_ssize_t;
    let mut g: *const ::core::ffi::c_char;
    let mut pad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let n_sign: mpd_ssize_t = (if !sign.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    let n_sep: mpd_ssize_t = strlen((*spec).sep) as mpd_ssize_t;
    (*dest).cur = (*dest).nbytes;
    (*dest).nchars = 0 as mpd_ssize_t;
    (*dest).nbytes = (*dest).nchars;
    _mbstr_copy_ascii(dest, rest, n_rest);
    if !dot.is_null() {
        _mbstr_copy_char(dest, dot, strlen(dot) as mpd_ssize_t);
    }
    g = (*spec).grouping;
    consume = *g as mpd_ssize_t;
    loop {
        if *g as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *g as ::core::ffi::c_int == CHAR_MAX
            || consume > n_src
        {
            consume = n_src;
        }
        n_src -= consume;
        if pad != 0 {
            _mbstr_copy_pad(dest, consume);
        } else {
            _mbstr_copy_ascii(dest, src.offset(n_src as isize), consume);
        }
        if n_src == 0 as mpd_ssize_t {
            if !((*spec).align as ::core::ffi::c_int == 'z' as i32
                && (*dest).nchars + n_sign < (*spec).min_width)
            {
                break;
            }
            n_src = (*spec).min_width - ((*dest).nchars + n_sign);
            consume = *g as mpd_ssize_t - consume;
            pad = 1 as ::core::ffi::c_int;
        } else {
            if n_sep > 0 as mpd_ssize_t {
                if pad != 0 && n_src > 1 as mpd_ssize_t {
                    n_src = (n_src as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as mpd_ssize_t;
                }
                _mbstr_copy_char(dest, (*spec).sep, n_sep);
            }
            if *g as ::core::ffi::c_int != 0 && *g.offset(1_isize) as ::core::ffi::c_int != 0 {
                g = g.offset(1);
            }
            consume = *g as mpd_ssize_t;
        }
    }
    if !sign.is_null() {
        _mbstr_copy_ascii(dest, sign, 1 as mpd_ssize_t);
    }
    if !(*dest).data.is_null() {
        *(*dest).data.offset((*dest).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_apply_lconv(
    result: *mut mpd_mbstr_t,
    spec: *const mpd_spec_t,
    status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut sign: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dp: *const ::core::ffi::c_char;
    dp = (*result).data;
    if *dp as ::core::ffi::c_int == '+' as i32
        || *dp as ::core::ffi::c_int == '-' as i32
        || *dp as ::core::ffi::c_int == ' ' as i32
    {
        sign = dp;
        dp = dp.add(1);
    }
    if isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"isdigit((unsigned char)*dp)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let intpart: *const ::core::ffi::c_char = dp;
    dp = dp.add(1);
    while isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        dp = dp.offset(1);
    }
    let n_int: mpd_ssize_t = dp.offset_from(intpart) as i64 as mpd_ssize_t;
    if *dp as ::core::ffi::c_int == '.' as i32 {
        dp = dp.offset(1);
        dot = (*spec).dot;
    }
    let rest: *const ::core::ffi::c_char = dp;
    let n_rest: mpd_ssize_t =
        (*result).nbytes - dp.offset_from((*result).data) as i64 as mpd_ssize_t;
    if dot.is_null()
        && (*(*spec).sep as ::core::ffi::c_int == '\0' as i32
            || *(*spec).grouping as ::core::ffi::c_int == '\0' as i32)
    {
        return 1 as ::core::ffi::c_int;
    }
    let decstring: *mut ::core::ffi::c_char = (*result).data;
    (*result).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    (*result).data = mpd_alloc(
        ((*result).nbytes + 1 as mpd_ssize_t) as mpd_size_t,
        1 as mpd_size_t,
    ) as *mut ::core::ffi::c_char;
    if (*result).data.is_null() {
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    1 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_apply_lconv(
    result: *mut mpd_mbstr_t,
    spec: *const mpd_spec_t,
    status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut sign: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dp: *const ::core::ffi::c_char;
    dp = (*result).data;
    if *dp as ::core::ffi::c_int == '+' as i32
        || *dp as ::core::ffi::c_int == '-' as i32
        || *dp as ::core::ffi::c_int == ' ' as i32
    {
        sign = dp;
        dp = dp.add(1);
    }
    if isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"isdigit((unsigned char)*dp)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let intpart: *const ::core::ffi::c_char = dp;
    dp = dp.add(1);
    while isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        dp = dp.offset(1);
    }
    let n_int: mpd_ssize_t = dp.offset_from(intpart) as ::core::ffi::c_int as mpd_ssize_t;
    if *dp as ::core::ffi::c_int == '.' as i32 {
        dp = dp.offset(1);
        dot = (*spec).dot;
    }
    let rest: *const ::core::ffi::c_char = dp;
    let n_rest: mpd_ssize_t =
        (*result).nbytes - dp.offset_from((*result).data) as ::core::ffi::c_int as mpd_ssize_t;
    if dot.is_null()
        && (*(*spec).sep as ::core::ffi::c_int == '\0' as i32
            || *(*spec).grouping as ::core::ffi::c_int == '\0' as i32)
    {
        return 1 as ::core::ffi::c_int;
    }
    let decstring: *mut ::core::ffi::c_char = (*result).data;
    (*result).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    (*result).data = mpd_alloc(
        ((*result).nbytes + 1 as mpd_ssize_t) as mpd_size_t,
        1 as mpd_size_t,
    ) as *mut ::core::ffi::c_char;
    if (*result).data.is_null() {
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    1 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_add_pad(
    result: *mut mpd_mbstr_t,
    spec: *const mpd_spec_t,
    status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if (*result).nchars < (*spec).min_width {
        let mut lpad: size_t = 0 as size_t;
        let mut rpad: size_t = 0 as size_t;
        let mut len: size_t;
        let mut i: size_t;
        let mut j: size_t;
        let mut align: ::core::ffi::c_char = (*spec).align;
        let mut err: uint8_t = 0 as uint8_t;
        let mut cp: *mut ::core::ffi::c_char;
        let n_fill: size_t = strlen(&raw const (*spec).fill as *const ::core::ffi::c_char);
        let add_chars: mpd_ssize_t = (*spec).min_width - (*result).nchars;
        let add_bytes: mpd_ssize_t = add_chars * n_fill as mpd_ssize_t;
        (*result).data = mpd_realloc(
            (*result).data as *mut ::core::ffi::c_void,
            ((*result).nbytes + add_bytes + 1 as mpd_ssize_t) as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
            &raw mut err,
        ) as *mut ::core::ffi::c_char;
        cp = (*result).data;
        if err != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            mpd_free.expect("non-null function pointer")(
                (*result).data as *mut ::core::ffi::c_void,
            );
            return 0 as ::core::ffi::c_int;
        }
        if align as ::core::ffi::c_int == 'z' as i32 {
            align = '=' as i32 as ::core::ffi::c_char;
        }
        if align as ::core::ffi::c_int == '<' as i32 {
            rpad = add_chars as size_t;
        } else if align as ::core::ffi::c_int == '>' as i32
            || align as ::core::ffi::c_int == '=' as i32
        {
            lpad = add_chars as size_t;
        } else {
            lpad = (add_chars / 2 as mpd_ssize_t) as size_t;
            rpad = (add_chars as u64).wrapping_sub(lpad as u64) as size_t;
        }
        len = (*result).nbytes as size_t;
        if align as ::core::ffi::c_int == '=' as i32
            && (*cp as ::core::ffi::c_int == '-' as i32
                || *cp as ::core::ffi::c_int == '+' as i32
                || *cp as ::core::ffi::c_int == ' ' as i32)
        {
            cp = cp.offset(1);
            len = len.wrapping_sub(1);
        }
        memmove(
            cp.add(n_fill.wrapping_mul(lpad)) as *mut ::core::ffi::c_void,
            cp as *const ::core::ffi::c_void,
            len,
        );
        i = 0 as size_t;
        while i < lpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.add(i.wrapping_mul(n_fill).wrapping_add(j)) = (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        cp = cp.add(n_fill.wrapping_mul(lpad).wrapping_add(len));
        i = 0 as size_t;
        while i < rpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.add(i.wrapping_mul(n_fill).wrapping_add(j)) = (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*result).nbytes += add_bytes;
        (*result).nchars += add_chars;
        *(*result).data.offset((*result).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    1 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_add_pad(
    result: *mut mpd_mbstr_t,
    spec: *const mpd_spec_t,
    status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if (*result).nchars < (*spec).min_width {
        let mut lpad: size_t = 0 as size_t;
        let mut rpad: size_t = 0 as size_t;
        let mut len: size_t;
        let mut i: size_t;
        let mut j: size_t;
        let mut align: ::core::ffi::c_char = (*spec).align;
        let mut err: uint8_t = 0 as uint8_t;
        let mut cp: *mut ::core::ffi::c_char;
        let n_fill: size_t = strlen(&raw const (*spec).fill as *const ::core::ffi::c_char);
        let add_chars: mpd_ssize_t = (*spec).min_width - (*result).nchars;
        let add_bytes: mpd_ssize_t = add_chars * n_fill as mpd_ssize_t;
        (*result).data = mpd_realloc(
            (*result).data as *mut ::core::ffi::c_void,
            ((*result).nbytes + add_bytes + 1 as mpd_ssize_t) as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
            &raw mut err,
        ) as *mut ::core::ffi::c_char;
        cp = (*result).data;
        if err != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            mpd_free.expect("non-null function pointer")(
                (*result).data as *mut ::core::ffi::c_void,
            );
            return 0 as ::core::ffi::c_int;
        }
        if align as ::core::ffi::c_int == 'z' as i32 {
            align = '=' as i32 as ::core::ffi::c_char;
        }
        if align as ::core::ffi::c_int == '<' as i32 {
            rpad = add_chars as size_t;
        } else if align as ::core::ffi::c_int == '>' as i32
            || align as ::core::ffi::c_int == '=' as i32
        {
            lpad = add_chars as size_t;
        } else {
            lpad = (add_chars / 2 as mpd_ssize_t) as size_t;
            rpad = (add_chars as size_t).wrapping_sub(lpad);
        }
        len = (*result).nbytes as size_t;
        if align as ::core::ffi::c_int == '=' as i32
            && (*cp as ::core::ffi::c_int == '-' as i32
                || *cp as ::core::ffi::c_int == '+' as i32
                || *cp as ::core::ffi::c_int == ' ' as i32)
        {
            cp = cp.offset(1);
            len = len.wrapping_sub(1);
        }
        memmove(
            cp.add(n_fill.wrapping_mul(lpad)) as *mut ::core::ffi::c_void,
            cp as *const ::core::ffi::c_void,
            len,
        );
        i = 0 as size_t;
        while i < lpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.add(i.wrapping_mul(n_fill).wrapping_add(j)) = (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        cp = cp.add(n_fill.wrapping_mul(lpad).wrapping_add(len));
        i = 0 as size_t;
        while i < rpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.add(i.wrapping_mul(n_fill).wrapping_add(j)) = (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*result).nbytes += add_bytes;
        (*result).nchars += add_chars;
        *(*result).data.offset((*result).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    1 as ::core::ffi::c_int
}
unsafe extern "C" fn _mpd_round(
    result: *mut mpd_t,
    a: *const mpd_t,
    prec: mpd_ssize_t,
    ctx: *const mpd_context_t,
    status: *mut uint32_t,
) {
    let exp: mpd_ssize_t = (*a).exp + (*a).digits - prec;
    if prec <= 0 as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_isspecial(a) != 0 || mpd_iszero(a) != 0 {
        mpd_qcopy(result, a, status);
        return;
    }
    mpd_qrescale_fmt(result, a, exp, ctx, status);
    if (*result).digits > prec {
        mpd_qrescale_fmt(result, result, exp + 1 as mpd_ssize_t, ctx, status);
    }
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_qformat_spec(
    mut dec: *const mpd_t,
    mut spec: *const mpd_spec_t,
    ctx: *const mpd_context_t,
    status: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut dt: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_STATIC_DATA as ::core::ffi::c_int)
            as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut dt as *mut mpd_uint_t,
    };
    let mut dplace: mpd_ssize_t = MPD_DEFAULT_DOTPLACE as mpd_ssize_t;
    let mut result: mpd_mbstr_t = mpd_mbstr_t {
        nbytes: 0,
        nchars: 0,
        cur: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut stackspec: mpd_spec_t;
    let mut type_0: ::core::ffi::c_char = (*spec).type_0;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*spec).min_width > MPD_MAX_PREC as mpd_ssize_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if isupper(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        type_0 =
            tolower(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) as ::core::ffi::c_char;
        flags |= MPD_FMT_UPPER;
    }
    if (*spec).sign_coerce != 0 {
        flags |= MPD_FMT_SIGN_COERCE;
    }
    if (*spec).sign as ::core::ffi::c_int == ' ' as i32 {
        flags |= MPD_FMT_SIGN_SPACE;
    } else if (*spec).sign as ::core::ffi::c_int == '+' as i32 {
        flags |= MPD_FMT_SIGN_PLUS;
    }
    'error: {
        if mpd_isspecial(dec) != 0 {
            if (*spec).align as ::core::ffi::c_int == 'z' as i32 {
                stackspec = *spec;
                stackspec.fill[0_usize] = ' ' as i32 as ::core::ffi::c_char;
                stackspec.fill[1_usize] = '\0' as i32 as ::core::ffi::c_char;
                stackspec.align = '>' as i32 as ::core::ffi::c_char;
                spec = &raw mut stackspec;
            }
            if type_0 as ::core::ffi::c_int == '%' as i32 {
                flags |= MPD_FMT_PERCENT;
            }
        } else {
            let mut workstatus: uint32_t = 0 as uint32_t;
            match type_0 as ::core::ffi::c_int {
                103 => {
                    // 'g'
                    flags |= MPD_FMT_TOSCI;
                }
                101 => {
                    // 'e'
                    flags |= MPD_FMT_EXP;
                }
                37 => {
                    // '%', falls through to 'f'
                    flags |= MPD_FMT_PERCENT;
                    if mpd_qcopy(&raw mut tmp, dec, status) == 0 {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    tmp.exp += 2 as mpd_ssize_t;
                    dec = &raw mut tmp;
                    type_0 = 'f' as i32 as ::core::ffi::c_char;
                    flags |= MPD_FMT_FIXED;
                }
                102 => {
                    // 'f'
                    flags |= MPD_FMT_FIXED;
                }
                _ => {
                    abort();
                }
            }
            if (*spec).prec >= 0 as mpd_ssize_t {
                if (*spec).prec > MPD_MAX_PREC as mpd_ssize_t {
                    *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                    break 'error;
                }
                match type_0 as ::core::ffi::c_int {
                    103 => {
                        let prec: mpd_ssize_t = if (*spec).prec == 0 as mpd_ssize_t {
                            1 as mpd_ssize_t
                        } else {
                            (*spec).prec
                        };
                        if (*dec).digits > prec {
                            _mpd_round(&raw mut tmp, dec, prec, ctx, &raw mut workstatus);
                            dec = &raw mut tmp;
                        }
                    }
                    101 => {
                        if mpd_iszero(dec) != 0 {
                            dplace = 1 as mpd_ssize_t - (*spec).prec;
                        } else {
                            _mpd_round(
                                &raw mut tmp,
                                dec,
                                (*spec).prec + 1 as mpd_ssize_t,
                                ctx,
                                &raw mut workstatus,
                            );
                            dec = &raw mut tmp;
                        }
                    }
                    102 => {
                        mpd_qrescale(&raw mut tmp, dec, -(*spec).prec, ctx, &raw mut workstatus);
                        dec = &raw mut tmp;
                    }
                    _ => {}
                }
            }
            if type_0 as ::core::ffi::c_int == 'f' as i32
                && mpd_iszero(dec) != 0
                && (*dec).exp > 0 as mpd_ssize_t
            {
                mpd_qrescale(
                    &raw mut tmp,
                    dec,
                    0 as mpd_ssize_t,
                    ctx,
                    &raw mut workstatus,
                );
                dec = &raw mut tmp;
            }
            if workstatus & MPD_Errors as uint32_t != 0 {
                *status = (*status as ::core::ffi::c_uint
                    | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                    as uint32_t;
                break 'error;
            }
        }
        result.nbytes = _mpd_to_string(&raw mut result.data, dec, flags, dplace);
        result.nchars = result.nbytes;
        if result.nbytes < 0 as mpd_ssize_t {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            break 'error;
        }
        if *(*spec).dot as ::core::ffi::c_int != '\0' as i32 && mpd_isspecial(dec) == 0 {
            if result.nchars > MPD_MAX_PREC as mpd_ssize_t + 36 as mpd_ssize_t {
                // Since a group length of one is not explicitly disallowed, ensure
                // that it is always possible to insert a four byte separator after
                // each digit.
                *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                mpd_free.expect("non-null function pointer")(
                    result.data as *mut ::core::ffi::c_void,
                );
                break 'error;
            }
            if _mpd_apply_lconv(&raw mut result, spec, status) == 0 {
                break 'error;
            }
        }
        if (*spec).min_width != 0 && _mpd_add_pad(&raw mut result, spec, status) == 0 {
            break 'error;
        }
        mpd_del(&raw mut tmp);
        return result.data;
    }
    mpd_del(&raw mut tmp);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_qformat_spec(
    mut dec: *const mpd_t,
    mut spec: *const mpd_spec_t,
    ctx: *const mpd_context_t,
    status: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut dt: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_STATIC_DATA as ::core::ffi::c_int)
            as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut dt as *mut mpd_uint_t,
    };
    let mut dplace: mpd_ssize_t = MPD_DEFAULT_DOTPLACE as mpd_ssize_t;
    let mut result: mpd_mbstr_t = mpd_mbstr_t {
        nbytes: 0,
        nchars: 0,
        cur: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut stackspec: mpd_spec_t;
    let mut type_0: ::core::ffi::c_char = (*spec).type_0;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*spec).min_width > MPD_MAX_PREC as mpd_ssize_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if isupper(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        type_0 =
            tolower(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) as ::core::ffi::c_char;
        flags |= MPD_FMT_UPPER;
    }
    if (*spec).sign_coerce != 0 {
        flags |= MPD_FMT_SIGN_COERCE;
    }
    if (*spec).sign as ::core::ffi::c_int == ' ' as i32 {
        flags |= MPD_FMT_SIGN_SPACE;
    } else if (*spec).sign as ::core::ffi::c_int == '+' as i32 {
        flags |= MPD_FMT_SIGN_PLUS;
    }
    'error: {
        if mpd_isspecial(dec) != 0 {
            if (*spec).align as ::core::ffi::c_int == 'z' as i32 {
                stackspec = *spec;
                stackspec.fill[0_usize] = ' ' as i32 as ::core::ffi::c_char;
                stackspec.fill[1_usize] = '\0' as i32 as ::core::ffi::c_char;
                stackspec.align = '>' as i32 as ::core::ffi::c_char;
                spec = &raw mut stackspec;
            }
            if type_0 as ::core::ffi::c_int == '%' as i32 {
                flags |= MPD_FMT_PERCENT;
            }
        } else {
            let mut workstatus: uint32_t = 0 as uint32_t;
            match type_0 as ::core::ffi::c_int {
                103 => {
                    // 'g'
                    flags |= MPD_FMT_TOSCI;
                }
                101 => {
                    // 'e'
                    flags |= MPD_FMT_EXP;
                }
                37 => {
                    // '%', falls through to 'f'
                    flags |= MPD_FMT_PERCENT;
                    if mpd_qcopy(&raw mut tmp, dec, status) == 0 {
                        return ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    tmp.exp =
                        (tmp.exp as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as mpd_ssize_t;
                    dec = &raw mut tmp;
                    type_0 = 'f' as i32 as ::core::ffi::c_char;
                    flags |= MPD_FMT_FIXED;
                }
                102 => {
                    // 'f'
                    flags |= MPD_FMT_FIXED;
                }
                _ => {
                    abort();
                }
            }
            if (*spec).prec >= 0 as mpd_ssize_t {
                if (*spec).prec > MPD_MAX_PREC as mpd_ssize_t {
                    *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                    break 'error;
                }
                match type_0 as ::core::ffi::c_int {
                    103 => {
                        let prec: mpd_ssize_t = if (*spec).prec == 0 as mpd_ssize_t {
                            1 as mpd_ssize_t
                        } else {
                            (*spec).prec
                        };
                        if (*dec).digits > prec {
                            _mpd_round(&raw mut tmp, dec, prec, ctx, &raw mut workstatus);
                            dec = &raw mut tmp;
                        }
                    }
                    101 => {
                        if mpd_iszero(dec) != 0 {
                            dplace = 1 as mpd_ssize_t - (*spec).prec;
                        } else {
                            _mpd_round(
                                &raw mut tmp,
                                dec,
                                (*spec).prec + 1 as mpd_ssize_t,
                                ctx,
                                &raw mut workstatus,
                            );
                            dec = &raw mut tmp;
                        }
                    }
                    102 => {
                        mpd_qrescale(&raw mut tmp, dec, -(*spec).prec, ctx, &raw mut workstatus);
                        dec = &raw mut tmp;
                    }
                    _ => {}
                }
            }
            if type_0 as ::core::ffi::c_int == 'f' as i32
                && mpd_iszero(dec) != 0
                && (*dec).exp > 0 as mpd_ssize_t
            {
                mpd_qrescale(
                    &raw mut tmp,
                    dec,
                    0 as mpd_ssize_t,
                    ctx,
                    &raw mut workstatus,
                );
                dec = &raw mut tmp;
            }
            if workstatus & MPD_Errors as uint32_t != 0 {
                *status = (*status as ::core::ffi::c_uint
                    | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                    as uint32_t;
                break 'error;
            }
        }
        result.nbytes = _mpd_to_string(&raw mut result.data, dec, flags, dplace);
        result.nchars = result.nbytes;
        if result.nbytes < 0 as mpd_ssize_t {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            break 'error;
        }
        if *(*spec).dot as ::core::ffi::c_int != '\0' as i32 && mpd_isspecial(dec) == 0 {
            if result.nchars > MPD_MAX_PREC as mpd_ssize_t + 36 as mpd_ssize_t {
                // Since a group length of one is not explicitly disallowed, ensure
                // that it is always possible to insert a four byte separator after
                // each digit.
                *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                mpd_free.expect("non-null function pointer")(
                    result.data as *mut ::core::ffi::c_void,
                );
                break 'error;
            }
            if _mpd_apply_lconv(&raw mut result, spec, status) == 0 {
                break 'error;
            }
        }
        if (*spec).min_width != 0 && _mpd_add_pad(&raw mut result, spec, status) == 0 {
            break 'error;
        }
        mpd_del(&raw mut tmp);
        return result.data;
    }
    mpd_del(&raw mut tmp);
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
pub unsafe extern "C" fn mpd_qformat(
    dec: *const mpd_t,
    fmt: *const ::core::ffi::c_char,
    ctx: *const mpd_context_t,
    status: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut spec: mpd_spec_t = mpd_spec_t {
        min_width: 0,
        prec: 0,
        type_0: 0,
        align: 0,
        sign: 0,
        sign_coerce: 0,
        fill: [0; 5],
        dot: ::core::ptr::null::<::core::ffi::c_char>(),
        sep: ::core::ptr::null::<::core::ffi::c_char>(),
        grouping: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if mpd_parse_fmt_str(&raw mut spec, fmt, 1 as ::core::ffi::c_int) == 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    mpd_qformat_spec(dec, &raw mut spec, ctx, status)
}
static mut mpd_flag_string: [*const ::core::ffi::c_char; 15] = [
    b"Clamped\0" as *const u8 as *const ::core::ffi::c_char,
    b"Conversion_syntax\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_by_zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_impossible\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_undefined\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fpu_error\0" as *const u8 as *const ::core::ffi::c_char,
    b"Inexact\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid_context\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Malloc_error\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not_implemented\0" as *const u8 as *const ::core::ffi::c_char,
    b"Overflow\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rounded\0" as *const u8 as *const ::core::ffi::c_char,
    b"Subnormal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Underflow\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut mpd_signal_string: [*const ::core::ffi::c_char; 15] = [
    b"Clamped\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_by_zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Inexact\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not_implemented\0" as *const u8 as *const ::core::ffi::c_char,
    b"Overflow\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rounded\0" as *const u8 as *const ::core::ffi::c_char,
    b"Subnormal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Underflow\0" as *const u8 as *const ::core::ffi::c_char,
];
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_snprint_flags(
    dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    flags: uint32_t,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    if nmemb >= 208 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *dest = '\0' as i32 as ::core::ffi::c_char;
    cp = dest;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s \0" as *const u8 as *const ::core::ffi::c_char,
                mpd_flag_string[j as usize],
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest {
        cp = cp.offset(-1);
        *cp = '\0' as i32 as ::core::ffi::c_char;
    }
    cp.offset_from(dest) as i64 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_snprint_flags(
    dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    flags: uint32_t,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    if nmemb >= 208 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *dest = '\0' as i32 as ::core::ffi::c_char;
    cp = dest;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s \0" as *const u8 as *const ::core::ffi::c_char,
                mpd_flag_string[j as usize],
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest {
        cp = cp.offset(-1);
        *cp = '\0' as i32 as ::core::ffi::c_char;
    }
    cp.offset_from(dest) as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_lsnprint_flags(
    dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    flags: uint32_t,
    mut flag_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    if nmemb >= 208 as ::core::ffi::c_int + 18 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if flag_string.is_null() {
        flag_string = &raw mut mpd_flag_string as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1_isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1_isize);
    nmemb -= 1;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                *flag_string.offset(j as isize),
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest.offset(1_isize) {
        cp = cp.offset(-2_isize);
    }
    *cp = ']' as i32 as ::core::ffi::c_char;
    cp = cp.add(1);
    *cp = '\0' as i32 as ::core::ffi::c_char;
    cp.offset_from(dest) as i64 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_lsnprint_flags(
    dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    flags: uint32_t,
    mut flag_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    if nmemb >= 208 as ::core::ffi::c_int + 18 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if flag_string.is_null() {
        flag_string = &raw mut mpd_flag_string as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1_isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1_isize);
    nmemb -= 1;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                *flag_string.offset(j as isize),
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest.offset(1_isize) {
        cp = cp.offset(-2_isize);
    }
    *cp = ']' as i32 as ::core::ffi::c_char;
    cp = cp.add(1);
    *cp = '\0' as i32 as ::core::ffi::c_char;
    cp.offset_from(dest) as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_lsnprint_signals(
    dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    flags: uint32_t,
    mut signal_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char;
    let mut ieee_invalid_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nmemb >= 121 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_SIGNAL_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if signal_string.is_null() {
        signal_string = &raw mut mpd_signal_string as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1_isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1_isize);
    nmemb -= 1;
    for j in 0..MPD_NUM_FLAGS {
        let f: uint32_t = flags & (1 as uint32_t) << j;
        if f != 0 {
            if f & MPD_IEEE_Invalid_operation as uint32_t != 0 {
                if ieee_invalid_done != 0 {
                    continue;
                }
                ieee_invalid_done = 1 as ::core::ffi::c_int;
            }
            let n: ::core::ffi::c_int = snprintf(
                cp,
                nmemb as size_t,
                b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                *signal_string.offset(j as isize),
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
    }
    if cp != dest.offset(1_isize) {
        cp = cp.offset(-2_isize);
    }
    *cp = ']' as i32 as ::core::ffi::c_char;
    cp = cp.add(1);
    *cp = '\0' as i32 as ::core::ffi::c_char;
    cp.offset_from(dest) as i64 as ::core::ffi::c_int
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_lsnprint_signals(
    dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    flags: uint32_t,
    mut signal_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char;
    let mut ieee_invalid_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nmemb >= 121 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_SIGNAL_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if signal_string.is_null() {
        signal_string = &raw mut mpd_signal_string as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1_isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1_isize);
    nmemb -= 1;
    for j in 0..MPD_NUM_FLAGS {
        let f: uint32_t = flags & (1 as uint32_t) << j;
        if f != 0 {
            if f & MPD_IEEE_Invalid_operation as uint32_t != 0 {
                if ieee_invalid_done != 0 {
                    continue;
                }
                ieee_invalid_done = 1 as ::core::ffi::c_int;
            }
            let n: ::core::ffi::c_int = snprintf(
                cp,
                nmemb as size_t,
                b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                *signal_string.offset(j as isize),
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
    }
    if cp != dest.offset(1_isize) {
        cp = cp.offset(-2_isize);
    }
    *cp = ']' as i32 as ::core::ffi::c_char;
    cp = cp.add(1);
    *cp = '\0' as i32 as ::core::ffi::c_char;
    cp.offset_from(dest) as ::core::ffi::c_int
}
pub unsafe extern "C" fn mpd_fprint(file: *mut FILE, dec: *const mpd_t) {
    let decstring: *mut ::core::ffi::c_char = mpd_to_sci(dec, 1 as ::core::ffi::c_int);
    if !decstring.is_null() {
        fprintf(
            file,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            decstring,
        );
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    } else {
        fputs(
            b"mpd_fprint: output error\n\0" as *const u8 as *const ::core::ffi::c_char,
            file,
        );
    };
}
pub unsafe extern "C" fn mpd_print(dec: *const mpd_t) {
    let decstring: *mut ::core::ffi::c_char = mpd_to_sci(dec, 1 as ::core::ffi::c_int);
    if !decstring.is_null() {
        printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            decstring,
        );
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    } else {
        fputs(
            b"mpd_fprint: output error\n\0" as *const u8 as *const ::core::ffi::c_char,
            mpd_c2rust_stderr(),
        );
    };
}
