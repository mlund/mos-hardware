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

//! MEGA65 memory example
//!
//! Illustrates DMA copy etc.

#![no_std]
#![feature(start)]
extern crate alloc;
extern crate mos_alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::mem::size_of;
use core::str;
use mos_hardware::mega65::*;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    set_upper_case();

    // Integer sizes
    println!("USIZE = {}", size_of::<usize>());
    println!("ISIZE = {}", size_of::<isize>());
    println!("CLONG = {}", size_of::<core::ffi::c_long>());
    println!("CUINT = {}", size_of::<core::ffi::c_uint>());
    println!("MAX_28  = {}", MAX_28_BIT_ADDRESS);
    println!("MAX U32 = {}", u32::MAX);
    println!("MAX I32 = {}", i32::MAX);

    // Memory allocation in bank 4 (0x40000 - 0x4ffff)
    const ADDRESS: u32 = 0x40000;
    let mut mem = Allocator::new(ADDRESS);

    // Copy bytes to upper mem, then back again
    let ptr: Fat28 = mem.write([7, 9, 13].as_slice());
    assert_eq!(Vec::<u8>::from(ptr), [7, 9, 13]);
    println!("ADDRESS = 0X{:x} LEN = {}", ptr.address, ptr.len);

    // Copy string to upper mem, then back again
    let ptr: Fat28 = mem.write("some LARGE string".as_bytes());
    assert_eq!(String::from(ptr), "some LARGE string");
    println!("ADDRESS = 0X{:x} LEN = {}", ptr.address, ptr.len);

    // Test memory iterator; memory is already filled from above.
    let bytes: Vec<u8> = MemoryIterator::new(ADDRESS).skip(3 + 5).take(5).collect();
    let s = unsafe { str::from_utf8_unchecked(bytes.as_slice()) };
    assert_eq!(s, "LARGE");
    println!("EXTRACTED STRING = {}", s);

    // Loop over vector of fat pointers as if strings
    // (transparent DMA copying)
    let cnt = Vec::from([mem.write(b"first"), mem.write(b"second")])
        .iter()
        .copied()
        .map(String::from)
        .filter(|s| s.starts_with('s'))
        .count();
    assert_eq!(cnt, 1);

    print!("DONE!");
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    print!("PANIC!");
    loop {}
}
