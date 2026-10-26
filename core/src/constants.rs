use crate::common::*;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_moduli: [mpd_uint_t; 3] = [
    18446744069414584321_u64,
    18446744056529682433_u64,
    18446742974197923841_u64,
];
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_moduli: [mpd_uint_t; 3] = [
    2113929217_u32 as mpd_uint_t,
    2013265921_u32 as mpd_uint_t,
    1811939329_u32 as mpd_uint_t,
];
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_roots: [mpd_uint_t; 3] = [7_u64, 10_u64, 19_u64];
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_roots: [mpd_uint_t; 3] = [
    5_u32 as mpd_uint_t,
    31_u32 as mpd_uint_t,
    13_u32 as mpd_uint_t,
];
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_invmoduli: [[uint32_t; 3]; 3] = [
    [
        4293885170 as ::core::ffi::c_uint,
        2181570688 as ::core::ffi::c_uint,
        16352 as ::core::ffi::c_uint,
    ],
    [
        1698898177 as ::core::ffi::c_uint,
        2290649223 as ::core::ffi::c_uint,
        16352 as ::core::ffi::c_uint,
    ],
    [
        2716021846 as ::core::ffi::c_uint,
        2545165803 as ::core::ffi::c_uint,
        16352 as ::core::ffi::c_uint,
    ],
];
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut MPD_TWO63: ::core::ffi::c_float = 9223372036854775808.0f32;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut INV_P1_MOD_P2: mpd_uint_t = 18446744055098026669 as mpd_uint_t;
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut INV_P1_MOD_P2: mpd_uint_t = 2013265901 as mpd_uint_t;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut INV_P1P2_MOD_P3: mpd_uint_t = 287064143708160 as mpd_uint_t;
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut INV_P1P2_MOD_P3: mpd_uint_t = 54 as mpd_uint_t;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut LH_P1P2: mpd_uint_t = 18446744052234715137 as mpd_uint_t;
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut LH_P1P2: mpd_uint_t = 4127195137 as mpd_uint_t;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut UH_P1P2: mpd_uint_t = 18446744052234715141 as mpd_uint_t;
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut UH_P1P2: mpd_uint_t = 990904320 as mpd_uint_t;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_bits: [mpd_size_t; 64] = [
    1 as mpd_size_t,
    2 as mpd_size_t,
    4 as mpd_size_t,
    8 as mpd_size_t,
    16 as mpd_size_t,
    32 as mpd_size_t,
    64 as mpd_size_t,
    128 as mpd_size_t,
    256 as mpd_size_t,
    512 as mpd_size_t,
    1024 as mpd_size_t,
    2048 as mpd_size_t,
    4096 as mpd_size_t,
    8192 as mpd_size_t,
    16384 as mpd_size_t,
    32768 as mpd_size_t,
    65536 as mpd_size_t,
    131072 as mpd_size_t,
    262144 as mpd_size_t,
    524288 as mpd_size_t,
    1048576 as mpd_size_t,
    2097152 as mpd_size_t,
    4194304 as mpd_size_t,
    8388608 as mpd_size_t,
    16777216 as mpd_size_t,
    33554432 as mpd_size_t,
    67108864 as mpd_size_t,
    134217728 as mpd_size_t,
    268435456 as mpd_size_t,
    536870912 as mpd_size_t,
    1073741824 as mpd_size_t,
    2147483648_u64 as mpd_size_t,
    4294967296_u64 as mpd_size_t,
    8589934592_u64 as mpd_size_t,
    17179869184_u64 as mpd_size_t,
    34359738368_u64 as mpd_size_t,
    68719476736_u64 as mpd_size_t,
    137438953472_u64 as mpd_size_t,
    274877906944_u64 as mpd_size_t,
    549755813888_u64 as mpd_size_t,
    1099511627776_u64 as mpd_size_t,
    2199023255552_u64 as mpd_size_t,
    4398046511104_i64 as mpd_size_t,
    8796093022208_u64 as mpd_size_t,
    17592186044416_u64 as mpd_size_t,
    35184372088832_u64 as mpd_size_t,
    70368744177664_u64 as mpd_size_t,
    140737488355328_u64 as mpd_size_t,
    281474976710656_u64 as mpd_size_t,
    562949953421312_u64 as mpd_size_t,
    1125899906842624_u64 as mpd_size_t,
    2251799813685248_u64 as mpd_size_t,
    4503599627370496_u64 as mpd_size_t,
    9007199254740992_u64 as mpd_size_t,
    18014398509481984_u64 as mpd_size_t,
    36028797018963968_u64 as mpd_size_t,
    72057594037927936_u64 as mpd_size_t,
    144115188075855872_u64 as mpd_size_t,
    288230376151711744_u64 as mpd_size_t,
    576460752303423488_u64 as mpd_size_t,
    1152921504606846976_u64 as mpd_size_t,
    2305843009213693952_u64 as mpd_size_t,
    4611686018427387904_u64 as mpd_size_t,
    9223372036854775808_u64 as mpd_size_t,
];
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_bits: [mpd_size_t; 32] = [
    1 as mpd_size_t,
    2 as mpd_size_t,
    4 as mpd_size_t,
    8 as mpd_size_t,
    16 as mpd_size_t,
    32 as mpd_size_t,
    64 as mpd_size_t,
    128 as mpd_size_t,
    256 as mpd_size_t,
    512 as mpd_size_t,
    1024 as mpd_size_t,
    2048 as mpd_size_t,
    4096 as mpd_size_t,
    8192 as mpd_size_t,
    16384 as mpd_size_t,
    32768 as mpd_size_t,
    65536 as mpd_size_t,
    131072 as mpd_size_t,
    262144 as mpd_size_t,
    524288 as mpd_size_t,
    1048576 as mpd_size_t,
    2097152 as mpd_size_t,
    4194304 as mpd_size_t,
    8388608 as mpd_size_t,
    16777216 as mpd_size_t,
    33554432 as mpd_size_t,
    67108864 as mpd_size_t,
    134217728 as mpd_size_t,
    268435456 as mpd_size_t,
    536870912 as mpd_size_t,
    1073741824 as mpd_size_t,
    2147483648_u32 as mpd_size_t,
];
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_pow10: [mpd_uint_t; 20] = [
    1 as mpd_uint_t,
    10 as mpd_uint_t,
    100 as mpd_uint_t,
    1000 as mpd_uint_t,
    10000 as mpd_uint_t,
    100000 as mpd_uint_t,
    1000000 as mpd_uint_t,
    10000000 as mpd_uint_t,
    100000000 as mpd_uint_t,
    1000000000 as mpd_uint_t,
    10000000000_u64,
    100000000000_u64,
    1000000000000_u64,
    10000000000000_u64,
    100000000000000_u64,
    1000000000000000_u64,
    10000000000000000_u64,
    100000000000000000_u64,
    1000000000000000000_u64,
    10000000000000000000_u64,
];
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mpd_pow10: [mpd_uint_t; 10] = [
    1 as mpd_uint_t,
    10 as mpd_uint_t,
    100 as mpd_uint_t,
    1000 as mpd_uint_t,
    10000 as mpd_uint_t,
    100000 as mpd_uint_t,
    1000000 as mpd_uint_t,
    10000000 as mpd_uint_t,
    100000000 as mpd_uint_t,
    1000000000 as mpd_uint_t,
];
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "test-hooks", no_mangle)]
pub static mut mprime_rdx: mpd_uint_t = 15581492618384294730 as mpd_uint_t;
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut mpd_round_string: [*const ::core::ffi::c_char; 9] = [
    b"ROUND_UP\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_DOWN\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_CEILING\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_FLOOR\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_HALF_UP\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_HALF_DOWN\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_HALF_EVEN\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_05UP\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_TRUNC\0" as *const u8 as *const ::core::ffi::c_char,
];
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut mpd_clamp_string: [*const ::core::ffi::c_char; 2] = [
    b"CLAMP_DEFAULT\0" as *const u8 as *const ::core::ffi::c_char,
    b"CLAMP_IEEE_754\0" as *const u8 as *const ::core::ffi::c_char,
];
