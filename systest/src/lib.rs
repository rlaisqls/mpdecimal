//! Compile-time checks of the C ABI of mpdecimal-sys and mpdecimal-core
//! against mpdecimal.h, see gen.py. `cargo check` is the test.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all)]

mod rs {
    pub use mpdecimal_core::{common::*, constants::*, context::*, io::*, mpalloc::*, mpdecimal::*, mpsignal::*};
}

/// `a` and `b` agree in the low `bytes` bytes.
const fn same_bits(a: u64, b: u64, bytes: usize) -> bool {
    let mask = if bytes >= 8 { u64::MAX } else { (1 << (bytes * 8)) - 1 };
    a & mask == b & mask
}

#[cfg(target_pointer_width = "64")]
include!("abi64.rs");
#[cfg(target_pointer_width = "32")]
include!("abi32.rs");
