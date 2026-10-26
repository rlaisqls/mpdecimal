#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(clippy::ptr_offset_with_cast, clippy::not_unsafe_ptr_arg_deref)]
// The functions are those of libmpdec, documented in its manual.
#![allow(clippy::missing_safety_doc)]

pub mod common;
pub mod shim;
pub mod basearith;
pub mod constants;
pub mod context;
pub mod convolute;
pub mod crt;
pub mod difradix2;
pub mod fnt;
pub mod fourstep;
pub mod io;
pub mod mpalloc;
pub mod mpdecimal;
pub mod mpsignal;
pub mod numbertheory;
pub mod sixstep;
pub mod transpose;
