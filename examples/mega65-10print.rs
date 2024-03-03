//! Classic 10print Maze in Rust
//!
//! "8-bit Show and Tell" has a great movie on this:
//! https://www.youtube.com/watch?v=IPP-EMBQPhE
//!
//! We here use the `SIDRng` random number back-end which implements
//! the [`rand::RngCore`](https://docs.rs/rand/latest/rand/trait.RngCore.html) trait
//! and can therefore be used with slices, iterators etc. It is of course overkill
//! for merely picking two characters (`/`, end `\`), and it would be more efficient
//! to manually comparing with a random byte.

#![no_std]
#![feature(start)]

extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::mega65;
use rand::seq::SliceRandom;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    mega65::set_upper_case();
    let mut rng = mega65::random::LibcRng::default();
    //   const SCREEN_MEMORY: *mut [u8; 25 * 80] = (0x0800) as *mut [u8; 25 * 80];
    const SCREEN_MEMORY: *mut u8 = (0x0800) as *mut u8;
    for offset in 0..80 * 25 {
        let random_char = [77u8, 78u8].choose(&mut rng).copied().unwrap();
        unsafe { SCREEN_MEMORY.add(offset).write_volatile(random_char) };
    }

    // unsafe { *SCREEN_MEMORY }.iter_mut().for_each(|i| unsafe {
    //     core::ptr::write_volatile(i, [77u8, 78u8].choose(&mut rng).copied().unwrap());
    // });
    println!("10print maze in rust!");
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
