//! MEGA65 memory example
//!
//! Illustrates DMA copy etc.

#![no_std]
#![feature(start)]
extern crate alloc;
extern crate mos_alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::str;
use mos_hardware::mega65::*;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    set_upper_case();

    // Memory allocation in bank 5 (0x40000 - 0x4ffff)
    let address = 0x40000;
    let mut alloc = memory::Allocator::new(address);

    // Copy bytes to upper mem, then back again
    let a = Vec::<u8>::from([7, 9, 13]);
    let ptr = alloc.push(a.as_slice());
    let b = Vec::<u8>::from(ptr);
    assert_eq!(a, b);
    println!("ADDRESS = 0X{:x} LEN = {}", ptr.address, ptr.len);

    // Copy string to upper mem, then back again
    let a = String::from("some LARGE string");
    let ptr: memory::Fat28 = alloc.push(a.as_bytes()); // dma send
    let b = String::from(ptr); // dma get
    assert_eq!(a, b);
    println!("ADDRESS = 0X{:x} LEN = {}", ptr.address, ptr.len);

    // Test memory iterator; memory is already filled from above.
    let bytes: Vec<u8> = memory::MemoryIterator::new(address)
        .skip(3 + 5)
        .take(5)
        .collect();
    let s = unsafe { str::from_utf8_unchecked(bytes.as_slice()) };
    assert_eq!(s, "LARGE");
    println!("EXTRACTED STRING = {}", s);

    // Loop over vector of Fat28 pointers as if strings
    // (transparent DMA copying)
    let v = Vec::from([alloc.push(b"first"), alloc.push(b"second")]);
    let cnt = v
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
