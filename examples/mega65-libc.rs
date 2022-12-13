//! MEGA65 libc example
//!
//! This shows how to use mega65-libc which is exposed to rust using bindgen.
//! A limited, but growing number of these are wrapped in _safe_ rust functions
//! found in `mega65::`. 
//!
//! ## TODO
//! As of writing these functions do not seem to work as expected - at least not in xemu.
//! Likely text manipulation is incompatible with rusts `print` macros and should instead
//! be used with the functions found in `libc`.
//! - `goto_xy()`
//! - `go_home()`
//! - `set_text_color()`
//! - `get_real_time_clock()`

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mos_hardware::mega65::*;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    conio_init();
    clear_screen();
    go_home();
    goto_xy(1, 1);
    set_upper_case();

    let resolution = get_screen_size();
    println!("SCREEN SIZE = {} x {}", resolution.width, resolution.height);

    print!("RANDUM BYTES FROM LIBC: ");
    for _ in 0..10 {
        print!("{} ", rand8(u8::MAX));
    }
    print!("\nRANDUM BYTES FROM SID:  ");
    sid0().start_random_generator();
    for _ in 0..10 {
        print!("{} ", sid0().rand8(u8::MAX));
    }
    println!();

    set_border_color(libc::COLOUR_BROWN as u8);
    set_text_color(libc::COLOUR_BLACK as u8);

    let rtc = get_real_time_clock();
    println!("TIME = {}:{}:{}", rtc.tm_hour, rtc.tm_min, rtc.tm_sec);
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("PANIC!");
    loop {}
}