// copyright 2022 mikael lund aka wombat
//
// licensed under the apache license, version 2.0 (the "license");
// you may not use this file except in compliance with the license.
// you may obtain a copy of the license at
//
//     http://www.apache.org/licenses/license-2.0
//
// unless required by applicable law or agreed to in writing, software
// distributed under the license is distributed on an "as is" basis,
// without warranties or conditions of any kind, either express or implied.
// see the license for the specific language governing permissions and
// limitations under the license.

//! MEGA65 file io example
//!
//! Preliminary and unstable test of file I/O

#![no_std]
#![feature(start)]

extern crate alloc;
extern crate mos_alloc;

use alloc::ffi::CString;
use core::ffi::CStr;
use core::panic::PanicInfo;
use mos_hardware::cbm_kernal::FileError;
use mos_hardware::mega65::*;
use ufmt_stdio::*;

/// Safe wrapper for libc::open (fileio.h)
/// Ideally we would later wrap this in a struct
/// that implements a `Read` trait.
pub fn open_sd(filename: &CStr) -> Result<u8, FileError> {
    unsafe { libc::closeall() };
    match unsafe { libc::open(filename.as_ptr() as *mut i8) } {
        0xff => Err(FileError::IOError),
        file_handle => Ok(file_handle),
    }
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    set_lower_case();

    let filename = CString::new("galaxy").unwrap();

    match open_sd(&filename) {
        Ok(file) => unsafe { libc::close(file) },
        Err(_) => {
            let _file = &filename.into_string().unwrap();
            println!("FILE ERROR");
        }
    }
    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    print!("PANIC!");
    loop {}
}
