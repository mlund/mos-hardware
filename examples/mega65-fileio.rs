//! MEGA65 libc example
//!
//! Preliminary and unstable test of file I/O

#![no_std]
#![feature(error_in_core)]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::ffi::CString;
use core::error::Error;
use core::ffi::CStr;
use core::fmt;
use core::panic::PanicInfo;
use mos_hardware::cbm_kernal::{
    cbm_k_basin, cbm_k_chkin, cbm_k_close, cbm_k_load, cbm_k_open, cbm_k_readst, cbm_k_setlfs,
    cbm_k_setnam,
};
use mos_hardware::mega65::*;
use ufmt_stdio::*;

#[derive(Debug)]
pub struct FileError;

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FILE ERROR")
    }
}

/// Loads `filename` from `device` into `address` or to the file load address is `address == None`
/// Returns number of loaded bytes.
fn cbm_load(filename: &CStr, device: u8, load_address: Option<u16>) -> u16 {
    // logical file numner, lfn, is set to 0; but, it's not needed for loading
    // (BASIC V2 sets it to the value of the SA for LOAD).
    let lfn = 0u8;
    let (address, secondary_address) = match load_address {
        Some(address) => (address, 0u8),
        None => (0, 1), // use file load address (first two bytes)
    };
    unsafe {
        cbm_k_setlfs(lfn, device, secondary_address);
        cbm_k_setnam(filename.to_bytes_with_nul().as_ptr());
        cbm_k_load(lfn, address) - address
    }
}

/// Safe wrapper for libc::open (fileio.h)
/// Ideally we would later wrap this in a struct
/// that implements a `Read` trait.
pub fn open(filename: &CStr) -> Result<u8, FileError> {
    unsafe { libc::closeall() };
    match unsafe { libc::open(filename.as_ptr()) } {
        0xff => Err(FileError {}),
        file_handle => Ok(file_handle),
    }
}

impl Error for FileError {}

// enum OSError {
//     /// OK
//     OK = 0,
//     /// Too many open files
//     EMFILE = 1,
//     /// File is open
//     EINVAL = 2,
//     /// File not open
//     EINVAL = 3,
//     /// File not found
//     ENOENT = 4,
//     /// Device not present
//     ENODEV = 5,
//     /// File not input
//     EINVAL = 6,
//     /// File not output
//     EINVAL = 7,
//     /// Filename missing
//     EINVAL = 8,
//     /// Illegal device
//     ENODEV = 9,
//     /// No sector header
//     EBUSY = 20,
//     /// No sync mark
//     EBUSY = 21,
//     /// No sector data
//     EIO = 22,
//     /// Checksum error
//     EIO = 23,
//     /// Decode error
//     EIO = 24,
//     /// Verify error
//     EIO = 25,
//     /// Write protected
//     EACCES = 26,
//     /// Checksum error
//     EIO = 27,
//     /// Write overrun
//     EIO = 28,
//     /// Disk ID mismatch
//     EBUSY = 29,
//     /// Command not recognized
//     EINVAL = 30,
//     /// Command not implemented
//     ENOSYS = 31,
//     /// Command too long
//     EINVAL = 32,
//     /// Invalid write filename
//     EINVAL = 33,
//     /// No file given
//     EINVAL = 34,
//     /// System file not found
//     ENOENT = 39,
//     /// Invalid format
//     EACCES = 49,
//     /// Record not present
//     ESPIPE = 50,
//     /// Overflow in record
//     ENOSPC = 51,
//     /// File too large
//     ENOSPC = 52,
//     /// Write file open
//     EBUSY = 60,
//     /// File not open
//     EINVAL = 61,
//     /// File not found
//     ENOENT = 62,
//     /// File exists
//     EEXIST = 63,
//     /// File type mismatch
//     EINVAL = 64,
//     /// No block
//     ESPIPE = 65,
//     /// Illegal track or sector
//     EINVAL = 66,
//     /// Illegal system track or sector
//     EIO = 67,
//     /// No channel
//     EBUSY = 70,
//     /// BAM error
//     EIO = 71,
//     /// Disk full
//     ENOSPC = 72,
//     /// DOS version mismatch
//     EACCES = 73,
//     /// Drive not ready
//     ENODEV = 74,
//     /// Format error
//     EIO = 75,
//     /// Illegal partition
//     EINVAL = 77,
//     /// Bad system area
//     EIO = 78,
//}

struct File {
    logical_file_number: u8,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { cbm_k_close(self.logical_file_number) };
    }
}

enum Device {
    Keyboard = 0,
    Tape = 1,
    RC232 = 2,
    CRT = 3,
    Printer = 4,
    Disk = 8,
    Other,
}

impl File {
    /// Attempts to open a file in read-only mode.
    pub fn open(filename: &CStr, device: u8, logical_file_number: u8) -> Result<Self, FileError> {
        unsafe {
            cbm_k_setlfs(logical_file_number, device, 0);
            cbm_k_setnam(filename.to_bytes_with_nul().as_ptr());
        }
        match unsafe { cbm_k_open() } {
            0 => Ok(File {
                logical_file_number,
            }),
            _ => Err(FileError {}),
        }
    }

    // Reads up to `size` bytes into `buffer`.
    // Returns the number of actually read bytes, 0 if there are no bytes left
    // (EOF) or -1 in case of an error. __oserror contains an errorcode then (see
    // table below).
    fn read(&self, buffer: *mut u8, size: usize) -> Result<usize, FileError> {
        // if we can't change to the input channel #lfn then return an error
        if unsafe { cbm_k_chkin(self.logical_file_number) } != 0 {
            return Err(FileError {});
        }
        let mut bytes_read: usize = 0;
        unsafe {
            while (bytes_read < size) && (cbm_k_readst() == 0) {
                let byte = cbm_k_basin();
                // the kernal routine BASIN sets ST to EOF if the end of file
                // is reached the first time, then we have store tmp.
                // every subsequent call returns EOF and READ ERROR in ST, then
                // we have to exit the loop here immediatly.
                if (cbm_k_readst() & 0xbf) == 0 {
                    break;
                }
                buffer.add(bytes_read).write_volatile(byte);
                bytes_read += 1;
            }
        }
        Ok(bytes_read)
    }
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    // we could likely make a const version of this using a macro
    unsafe { libc::closeall() };
    let filename = CString::new("charrom.rom").unwrap();
    match open(&filename) {
        Ok(file) => unsafe { libc::close(file) },
        Err(_) => {
            let fil = &filename.into_string().unwrap();
            println!("FILENAME = {}", fil.as_str());
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
