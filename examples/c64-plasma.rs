//! C64 Plasma Example
//!
//! - (w)2001 by groepaz; sourced from the CC65 /samples/cbm directory
//! - Cleanup and porting to CC65 by Ullrich von Bassewitz.
//! - Porting to Rust by Mikael Lund aka Wombat (2022)

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use itertools::iproduct;
use mos_hardware::*;
use ufmt_stdio::*;

/// Generate stochastic character set
fn make_charset(charset_ptr: *mut u8) {
    const BITS: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];
    unsafe {
        (*c64::SID).start_random_generator();
    }

    repeat_element(mos_hardware::SINETABLE.iter().copied(), 8)
        .enumerate()
        .for_each(|(cnt, sine)| {
            let mut char_pattern = 0;
            unsafe {
                BITS.iter()
                    .filter(|_| (*c64::SID).random_byte() > sine)
                    .for_each(|bit| {
                        char_pattern |= bit;
                    });
                poke!(charset_ptr.offset(cnt as isize), char_pattern);
            }
            if cnt % 64 == 0 {
                print!(".");
            }
        });
}

/// Render entire 40x25 screen
/// @todo Rename to meaningful variable names (reminiscence from C)
unsafe fn render_plasma(screen_ptr: *mut u8) {
    static mut C1A: u8 = 0;
    static mut C1B: u8 = 0;
    static mut C2A: u8 = 0;
    static mut C2B: u8 = 0;
    static mut XBUF: [u8; 40] = [0; 40];
    static mut YBUF: [u8; 25] = [0; 25];

    let mut c1a = C1A;
    let mut c1b = C1B;
    YBUF.iter_mut().for_each(|y| {
        *y = add!(
            mos_hardware::SINETABLE[c1a as usize],
            mos_hardware::SINETABLE[c1b as usize]
        );
        c1a = add!(c1a, 4);
        c1b = add!(c1b, 9);
    });
    C1A = add!(C1A, 3);
    C1B = sub!(C1B, 5);

    let mut c2a = C2A;
    let mut c2b = C2B;
    XBUF.iter_mut().for_each(|x| {
        *x = add!(
            mos_hardware::SINETABLE[c2a as usize],
            mos_hardware::SINETABLE[c2b as usize]
        );
        c2a = add!(c2a, 3);
        c2b = add!(c2b, 7);
    });
    C2A = add!(C2A, 2);
    C2B = sub!(C2B, 3);

    iproduct!(YBUF.iter().copied(), XBUF.iter().copied())
        .enumerate()
        .for_each(|(cnt, (y, x))| {
            poke!(screen_ptr.offset(cnt as isize), add!(y, x));
        })
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    const CHARSET: u16 = 0x2000; // Custom charset
    const SCREEN1: u16 = 0x2800; // Set up two character screens...
    const SCREEN2: u16 = 0x2c00; // ...for double buffering
    const PAGE1: u8 =
        vic2::ScreenBank::from_address(SCREEN1).bits() | vic2::CharsetBank::from(CHARSET).bits();
    const PAGE2: u8 =
        vic2::ScreenBank::from_address(SCREEN2).bits() | vic2::CharsetBank::from(CHARSET).bits();

    unsafe {
        make_charset(CHARSET as *mut u8);
        loop {
            render_plasma(SCREEN1 as *mut u8);
            (*c64::VIC).screen_and_charset_bank.write(PAGE1);
            render_plasma(SCREEN2 as *mut u8);
            (*c64::VIC).screen_and_charset_bank.write(PAGE2);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}
