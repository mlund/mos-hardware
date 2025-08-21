//! Classic 10print Maze in Rust
//!
//! "8-bit Show and Tell" has a great movie on this:
//! https://www.youtube.com/watch?v=IPP-EMBQPhE
//!
//! We here use the `SIDRng` random number back-end which implements
//! the [`rand::RngCore`](https://docs.rs/rand/latest/rand/trait.RngCore.html) trait
//! and can therefore be used with slices, iterators etc. It is of course overkill
//! for merely picking two characters (`╲` or `╱`), and it would be more efficient
//! to manually comparing with a random byte.
//!
//! PETSCII characters are written directly to screen memory and thus
//! needs to be converted using the `screen_codes!` macro. This has no overhead as it is
//! done at compile time using Rust's const evaluation features.

#![no_std]
#![no_main]

extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::{c64, screen_codes, sid::SIDRng};
use rand::seq::SliceRandom;
use ufmt_stdio::*;

#[no_mangle]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    const SCREEN_SIZE: usize = 40 * 25;
    const LEFT_OR_RIGHT: [u8; 2] = screen_codes!("MN"); // ╲ or ╱
    let mut rng = SIDRng::new(c64::sid());
    c64::set_upper_case();
    for offset in 0..SCREEN_SIZE {
        let random_char = LEFT_OR_RIGHT.choose(&mut rng).copied().unwrap();
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
