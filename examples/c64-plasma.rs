//! C64 Plasma Example
//!
//! - (w)2001 by groepaz; sourced from the CC65 /samples/cbm directory
//! - Cleanup and porting to CC65 by Ullrich von Bassewitz.
//! - Porting to Rust by Mikael Lund aka Wombat (2022)

#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use mos_hardware::vic2::{BLACK, RED};
use mos_hardware::*;
use ufmt_stdio::*;

/// Class for rendering a character mode plasma effect
struct Plasma {
    yindex1: u8,
    yindex2: u8,
    xindex1: u8,
    xindex2: u8,
    xbuffer: [u8; 40],
    ybuffer: [u8; 25],
}

impl Plasma {
    /// Create new instance and initialize character set at given address
    pub fn new(charset_address: u16) -> Plasma {
        Plasma::make_charset(charset_address as *mut u8);
        Plasma {
            yindex1: 0,
            yindex2: 0,
            xindex1: 0,
            xindex2: 0,
            xbuffer: [0; 40],
            ybuffer: [0; 25],
        }
    }
    /// Generate stochastic character set
    fn make_charset(charset_address: *mut u8) {
        c64::sid().start_random_generator();

        let generate_char = |sine| {
            const BITS: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
            let mut char_pattern: u8 = 0;
            BITS.iter()
                .filter(|_| c64::sid().random_byte() > sine)
                .for_each(|bit| {
                    char_pattern |= bit;
                });
            char_pattern
        };

        repeat_element(SINETABLE.iter().copied(), 8)
            .enumerate()
            .for_each(|(offset, sine)| {
                let character = generate_char(sine);
                unsafe {
                    charset_address.add(offset).write_volatile(character);
                }
                if offset % 64 == 0 {
                    print!(".");
                }
            });
    }

    /// Render entire screen at given address
    pub fn render(&mut self, screen_address: *mut u8) {
        let mut i = self.yindex1;
        let mut j = self.yindex2;
        for y in self.ybuffer.iter_mut() {
            *y = sine(i).wrapping_add(sine(j));
            i = i.wrapping_add(4);
            j = j.wrapping_add(9);
        }
        self.yindex1 = self.yindex1.wrapping_add(3);
        self.yindex2 = self.yindex2.wrapping_sub(5);

        i = self.xindex1;
        j = self.xindex2;
        for x in self.xbuffer.iter_mut() {
            *x = sine(i).wrapping_add(sine(j));
            i = i.wrapping_add(3);
            j = j.wrapping_add(7);
        }
        self.xindex1 = self.xindex1.wrapping_add(2);
        self.xindex2 = self.xindex2.wrapping_sub(3);

        let mut offset: usize = 0; // screen memory offset
        for y in self.ybuffer.iter().copied() {
            for x in self.xbuffer.iter().copied() {
                let sum = x.wrapping_add(y);
                unsafe {
                    screen_address.add(offset).write_volatile(sum);
                }
                offset += 1;
            }
        }
    }
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

    let mut plasma = Plasma::new(CHARSET);

    loop {
        plasma.render(SCREEN1 as *mut u8);
        unsafe { c64::vic2().screen_and_charset_bank.write(PAGE1) };
        plasma.render(SCREEN2 as *mut u8);
        unsafe { c64::vic2().screen_and_charset_bank.write(PAGE2) };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            c64::vic2().border_color.write(RED);
            c64::vic2().border_color.write(BLACK);
        }
    }
}
