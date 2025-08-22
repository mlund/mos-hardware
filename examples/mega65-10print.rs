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
#![no_main]

extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::mega65;
use rand::seq::SliceRandom;
use ufmt_stdio::*;

#[no_mangle]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    mega65::set_upper_case();
    let mut rng = mega65::random::HardwareRng::default();
    for _ in 0..1000 {
        let c = "MN".as_bytes().choose(&mut rng).copied().unwrap() as char;
        print!("{}", c);
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
