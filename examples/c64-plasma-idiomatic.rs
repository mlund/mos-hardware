//! C64 Plasma Example
//!
//! - (w)2001 by groepaz; sourced from the CC65 /samples/cbm directory
//! - Cleanup and porting to CC65 by Ullrich von Bassewitz.
//! - Porting to Rust by Mikael Lund aka Wombat (2022)

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use itertools::{chain, iproduct, izip};
use mos_hardware::*;
use ufmt_stdio::*;

/// Generate randomized charset
unsafe fn make_charset(charset: *mut u8) {
    const BITS: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];

    (*c64::SID).start_random_generator();

    repeat_element(mos_hardware::SINETABLE.iter(), 8)
        .enumerate()
        .for_each(|(cnt, sine)| {
            let mut char_pattern = 0;
            BITS.iter()
                .filter(|_| (*c64::SID).random_byte() > *sine)
                .for_each(|bit| {
                    char_pattern |= bit;
                });
            poke!(charset.offset(cnt as isize), char_pattern);
            if cnt % 64 == 0 {
                print!(".");
            }
        });
}

/**
 * Sine tables are handled with a cyclic iterator and custom step-size.
 * The performance is inferior to indexed access to SINUSTABLE which by
 * design is already cyclic by taking up exactly 256 bytes (relying on
 * u8 index overflow).
 *
 * @todo Rename to meaningful variable names
 */
unsafe fn render_plasma(screen: *mut u8) {
    static mut C1A: u8 = 0;
    static mut C1B: u8 = 0;
    static mut C2A: u8 = 0;
    static mut C2B: u8 = 0;
    static mut XBUF: [u8; 40] = [0; 40];
    static mut YBUF: [u8; 25] = [0; 25];

    let xsine1 = mos_hardware::SINETABLE
        .iter()
        .copied()
        .cycle()
        .skip(C1A as usize)
        .step_by(4)
        .take(XBUF.len());
    let xsine2 = mos_hardware::SINETABLE
        .iter()
        .copied()
        .cycle()
        .skip(C1B as usize)
        .step_by(9)
        .take(XBUF.len());
    let ysine1 = mos_hardware::SINETABLE
        .iter()
        .copied()
        .cycle()
        .skip(C2A as usize)
        .step_by(3)
        .take(YBUF.len());
    let ysine2 = mos_hardware::SINETABLE
        .iter()
        .copied()
        .cycle()
        .skip(C2B as usize)
        .step_by(7)
        .take(YBUF.len());

    let zipped_x = izip!(XBUF.iter_mut(), xsine1, xsine2);
    let zipped_y = izip!(YBUF.iter_mut(), ysine1, ysine2);
    chain(zipped_y, zipped_x).for_each(|(buffer, sine1, sine2)| {
        *buffer = add!(sine1, sine2);
    });

    C1A = add!(C1A, 3);
    C1B = sub!(C1B, 5);
    C2A = add!(C2A, 2);
    C2B = sub!(C2B, 3);

    iproduct!(YBUF.iter().copied(), XBUF.iter().copied())
        .enumerate()
        .for_each(|(cnt, (y, x))| {
            poke!(screen.offset(cnt as isize), add!(y, x));
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
