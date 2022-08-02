//! MEGA65 libc example
//!
//! This shows how to use mega65-libc which is exposed to rust using bindgen.
//! As you may notice, the interface is often not idiomatic and several functions
//! should be manually wrapped in future releases.
//!
//! TODO: gotoxy() doesn't seem to work (at least not in xemu)

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mos_hardware::mega65::*;
use ufmt_stdio::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        // play with screen
        libc::conioinit();
        libc::clrscr();
        libc::gohome();
        libc::gotoxy(1, 1);
        let mut width: u8 = 0;
        let mut height: u8 = 0;
        libc::getscreensize(&mut width, &mut height);
        println!("SCREEN SIZE = {} x {}", width, height);

        // random numbers
        println!("RANDUM BYTES FROM SID:");
        (*SID0).start_random_generator();
        for _i in 0..3 {
            println!("{}", (*SID0).rand8(u8::MAX));
        }
        println!("RANDUM BYTES FROM LIBC:");
        for _i in 0..3 {
            println!("{}", libc::rand8(u8::MAX));
        }

        // play with colors
        libc::bordercolor(libc::COLOUR_BROWN as u8);

        // real-time-clock
        let mut rtc = libc::m65_tm::default();
        libc::getrtc(&mut rtc);
        println!("TIME = {}:{}:{}", rtc.tm_hour, rtc.tm_min, rtc.tm_sec);
    }
    0
}
