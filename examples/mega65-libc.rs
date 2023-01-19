//! MEGA65 libc example
//!
//! This shows how to use mega65-libc which is exposed to rust using bindgen.
//! A limited, but growing number of these are wrapped in _safe_ rust functions
//! found in `mega65::`.
//!
//! ## Notes
//! - `goto_xy()`, `go_home()`, `text_color()` etc. do not affect `println!` output
//! - `get_real_time_clock()` returns zero values

#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use mos_hardware::mega65::*;
use mos_hardware::screen_codes_null;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    conio_init();
    set_border_color(libc::COLOUR_BROWN as u8);
    clear_screen();
    set_upper_case();

    go_home();
    set_text_color(libc::COLOUR_BLACK as u8);
    // raw null-terminated array of screen codes
    cputs([8, 5, 12, 12, 15, 0].as_slice());
    // convert unicode to null-terminated screen code array at compile time (no overhead!)
    cputs_xy(4, 4, screen_codes_null!("hello from rust!").as_slice());

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

    let rtc = get_real_time_clock();
    println!("TIME = {}:{}:{}", rtc.tm_hour, rtc.tm_min, rtc.tm_sec);

    // Hardware math accelerator
    let (mul, whole, fraction) = math_accelerator().multiply_divide(0xa, 3);
    println!("10 MUL 3 = {}", mul);
    println!("10 DIV 3 = {} W FRACTION {}", whole, fraction);

    // libc user input
    println!("OK TO CONTINUE?");
    flush_keyboard_buffer();
    let pressed_key = cgetc().to_char().to_ascii_lowercase();
    let message = match pressed_key {
        'y' => "LET US CONTINUE",
        _ => "LET US STOP",
    };
    println!("{}", message);

    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("PANIC!");
    loop {}
}
