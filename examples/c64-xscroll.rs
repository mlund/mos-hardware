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

//! Classic C64 Sprite and scroll example

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mos_hardware::{c64, peek, poke, vic2};
use ufmt_stdio::*;
use vic2::*;

/// Classic, smooth x-scroll using VIC2's 0xd016 register
struct SmoothScroll {
    /// Current position in scroll text
    text_index: usize,
    /// horizontal pixel position (0 to 7)
    displacement: u8,
}

impl SmoothScroll {
    /// Vertical position of scroll
    const YPOSITION: u16 = 8;
    /// Pointer to first character of scroll line (beginning of character row)
    const LEFTMOST_CHAR: *mut u8 = (0x0400 + (40 * SmoothScroll::YPOSITION as u16)) as *mut u8;
    /// Pointer to last character of scroll line (end of character row)
    const RIGHTMOST_CHAR: *mut u8 =
        (0x0400 + (40 * SmoothScroll::YPOSITION + 39 as u16)) as *mut u8;
    /// PETSCII encoded scroll text
    const SCROLL_TEXT: [u8; 17] = [
        8, 5, 12, 12, 15, 0x20, 6, 18, 15, 13, 0x20, 18, 21, 19, 20, 0x21, 0x20,
    ];

    const fn new() -> SmoothScroll {
        SmoothScroll {
            text_index: 0,
            displacement: 7,
        }
    }

    pub fn init(&self) {
        // set 48 column mode for smooth x-scrolling
        let mut mask = c64::vic2().control_x.read();
        mask.set(ControlXFlags::COLUMN_SELECT, false);
        unsafe {
            c64::vic2().control_x.write(mask);
        }
    }

    /// Move screen ONE pixel to the left (cycle in the interval 7..0)
    #[inline]
    fn move_pixel(&mut self) {
        self.displacement = match self.displacement.checked_sub(1) {
            Some(x) => x,
            None => 7,
        };
        let mut mask = c64::vic2().control_x.read();
        mask.set(ControlXFlags::XSCROLL, false);
        mask = ControlXFlags::from_bits(mask.bits() + self.displacement).unwrap();
        unsafe {
            c64::vic2().control_x.write(mask);
        }
    }

    // Copy all characters on scroll line ONE character to the left (0-39 <-- 1-40)
    #[inline]
    fn leftcopy_chars(&self) {
        // faster than core::ptr::copy(LEFTMOST_CHAR.offset(1), LEFTMOST_CHAR, 39)
        for i in 1..40 {
            unsafe {
                let character = peek!(SmoothScroll::LEFTMOST_CHAR.offset(i));
                poke!(SmoothScroll::LEFTMOST_CHAR.offset(i - 1), character);
            }
        }
    }

    /// Place new character at (invisible) right-most position and left-shift line
    #[inline]
    fn update_chars(&mut self) {
        if self.text_index == SmoothScroll::SCROLL_TEXT.len() {
            self.text_index = 0;
        }
        unsafe {
            poke!(
                SmoothScroll::RIGHTMOST_CHAR,
                SmoothScroll::SCROLL_TEXT[self.text_index]
            );
        }
        self.leftcopy_chars();
        self.text_index += 1;
    }

    pub fn update(&mut self) {
        self.move_pixel();
        if self.displacement == 7 {
            self.update_chars();
        }
    }
}

/// Move single sprite in sine pattern in x-y direction
struct SpriteMove {
    counter_y: u8,
}

impl SpriteMove {
    const OFFSET: u8 = 30;

    const fn new() -> SpriteMove {
        SpriteMove { counter_y: 0 }
    }

    fn update(&mut self, counter: u8) {
        const XSINE: [u8; 256] = mos_hardware::make_sine(1, 0);
        const YSINE: [u8; 256] = mos_hardware::make_sine(4, 70);
        const MSB_THRESHOLD: u8 = 255 - SpriteMove::OFFSET;

        let x = XSINE[counter as usize];
        let y = YSINE[self.counter_y as usize];

        let (offset, msb) = match x > MSB_THRESHOLD {
            true => (SpriteMove::OFFSET.wrapping_sub(255), Sprites::SPRITE0),
            false => (SpriteMove::OFFSET, Sprites::empty()),
        };

        unsafe {
            c64::vic2()
                .sprite_positions_most_significant_bit_of_x
                .write(msb);
        }
        c64::vic2().set_sprite_pos(0, x + offset, y);
        self.counter_y += 1;
    }
}

/// Global since the interrupt wrapper currently do not take arguments
static mut SCROLL: SmoothScroll = SmoothScroll::new();
static mut SPRITE_MOVE: SpriteMove = SpriteMove::new();

/// IRQ wrapper; called at every triggering event.
#[no_mangle]
pub extern "C" fn called_every_frame() {
    static mut COUNTER: u8 = 0;
    unsafe {
        (&*c64::VIC).border_color.write(vic2::LIGHT_GREEN);
        SPRITE_MOVE.update(COUNTER);
        COUNTER += 2;
        if COUNTER % 2 == 0 {
            SCROLL.update();
        }
        (&*c64::VIC).border_color.write(vic2::BLACK);
    }
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    c64::clear_screen();
    unsafe {
        SCROLL.init();
    }

    // Copy Rust logo to sprite address and set sprite shape pointers
    const SPRITE_ADDRESS: u16 = 0x2000;
    const SPRITE_PTR: u8 = vic2::to_sprite_pointer(SPRITE_ADDRESS);
    unsafe {
        *(SPRITE_ADDRESS as *mut [u8; 63]) = RUST_LOGO;
        poke!(c64::DEFAULT_SPRITE_PTR[0], SPRITE_PTR);
        (*c64::VIC).sprite_expand_x.write(Sprites::SPRITE0);
        (*c64::VIC).sprite_expand_y.write(Sprites::SPRITE0);
        (*c64::VIC).sprite_enable.write(Sprites::SPRITE0);
    }
    c64::vic2().set_sprite_color(0, GREEN);
    c64::hardware_raster_irq(20); // trigger at raster line 20
    loop {} // let's not return to dead BASIC
}

/// Sprite pattern
const RUST_LOGO: [u8; 63] = [
    0, 90, 0, 1, 255, 128, 7, 239, 224, 15, 24, 240, 28, 0, 56, 63, 255, 28, 127, 255, 158, 127,
    255, 222, 235, 195, 215, 115, 255, 142, 227, 255, 135, 99, 199, 142, 247, 195, 223, 127, 243,
    254, 127, 241, 254, 62, 0, 60, 29, 0, 184, 15, 129, 240, 7, 255, 224, 1, 255, 128, 0, 90, 0,
];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("panic!");
    loop {}
}
