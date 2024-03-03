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
use mos_hardware::{c64, sid::SIDRng};
use rand::seq::SliceRandom;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    c64::set_upper_case();
    let mut rng = SIDRng::new(c64::sid());
    for offset in 0..40 * 25 {
        let random_char = [77u8, 78u8].choose(&mut rng).copied().unwrap();
        unsafe {
            c64::DEFAULT_VIDEO_MEMORY
                .add(offset)
                .write_volatile(random_char)
        };
    }
    println!("10print maze in rust!");
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
