//! Plasma Example (80 x 25 mode for mega65)
//!
//! - (w)2001 by groepaz; sourced from the CC65 /samples/cbm directory
//! - Cleanup and porting to CC65 by Ullrich von Bassewitz.
//! - Porting to Rust by Mikael Lund aka Wombat (2022)

#![no_std]
#![no_main]

extern crate mos_alloc;
extern crate mos_hardware;

use core::ops::BitOr;
use core::panic::PanicInfo;
use mos_hardware::mega65::random::HardwareRng;
use mos_hardware::{mega65, repeat_element, sine, SINETABLE};
use rand::Rng;

/// Class for rendering a character mode plasma effect
struct Plasma {
    yindex1: u8,
    yindex2: u8,
    xindex1: u8,
    xindex2: u8,
    xbuffer: [u8; 80],
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
            xbuffer: [0; 80],
            ybuffer: [0; 25],
        }
    }
    /// Generate stochastic character set
    fn make_charset(charset_address: *mut u8) {
        let mut rng = HardwareRng::default();
        let make_char = |sine| {
            [1, 2, 4, 8, 16, 32, 64, 128]
                .iter()
                .filter(|_| rng.gen::<u8>() > sine)
                .fold(0, |pattern, i| pattern.bitor(i))
        };

        repeat_element(SINETABLE.iter().copied(), 8)
            .map(make_char)
            .enumerate()
            .for_each(|(i, pattern)| unsafe {
                charset_address.add(i).write_volatile(pattern);
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

        itertools::iproduct!(self.ybuffer.iter().copied(), self.xbuffer.iter().copied())
            .map(|(y, x)| x.wrapping_add(y))
            .enumerate()
            .for_each(|(i, sum)| unsafe { screen_address.add(i).write_volatile(sum) });
    }
}

#[no_mangle]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    const CHARSET_ADDRESS: u16 = 0x3000;
    let mut plasma = Plasma::new(CHARSET_ADDRESS);
    mega65::set_charset_address(CHARSET_ADDRESS);
    mega65::CPUSpeed::Medium.set();
    loop {
        plasma.render(mega65::DEFAULT_SCREEN);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        mega65::set_border_color(0);
        mega65::set_border_color(2);
    }
}
