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
#![feature(default_alloc_error_handler)]
extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::mega65::*;
use mos_hardware::screen_codes_null;
use rand::{seq::SliceRandom, SeedableRng};
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
        print!("{} ", random::rand8(u8::MAX));
    }

    print!("\nRANDOM SHUFFLE USING RNGCORE TRAIT:  ");
    let mut rng = random::LibcRng::seed_from_u64(1);
    let mut seq = [0, 1, 2, 3, 4, 5, 6, 7, 9];
    seq.shuffle(&mut rng);
    println!("{:?}", &seq);

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
    print!("PANIC!");
    loop {}
}
