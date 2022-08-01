#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Bindings for the MEGA65-libc library
//!
//! This exposes the [mega65-libc](https://github.com/MEGA65/mega65-libc) library
//! to Rust via automatically generated bindings.
//!
//! Example:
//! ~~~
//! unsafe {
//!     mega65::libc::mega65_fast();
//!     let address = mega65::libc::getscreenaddr();
//! }
//! ~~~
include!("bindings.rs");
