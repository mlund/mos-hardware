//! MEGA65 libc example
//!
//! Preliminary and unstable test of file I/O

#![no_std]
#![feature(start)]

extern crate alloc;
extern crate mos_alloc;

use alloc::ffi::CString;
use core::ffi::CStr;
use core::panic::PanicInfo;
use mos_hardware::mega65::*;
use ufmt_stdio::*;

/// Safe wrapper for libc::open (fileio.h)
/// Ideally we would later wrap this in a struct
/// that implements a `Read` trait.
fn open(filename: &CStr) -> Option<u8> {
    match unsafe { libc::open(filename.as_ptr()) } {
        0xff => None,
        file_handle => Some(file_handle),
    }
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    // we could likely make a const version of this using a macro
    let filename = CString::new("CHARSET.ROM").unwrap();
    match open(&filename) {
        Some(file) => unsafe { libc::close(file) },
        None => {
            println!("Load error");
        }
    }
    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    print!("PANIC!");
    loop {}
}
