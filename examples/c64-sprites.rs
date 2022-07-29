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

//! C64 Sprite Example

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mos_hardware::{c64, poke, vic2};
use ufmt_stdio::*;

/// Sprite pattern
///
/// Images can be converted using ImageMagick:
///
/// ```bash
/// convert image.png -alpha off -resize 24x21! -monochrome sprite.png
/// ```
///
/// and then converted into a byte array using Python:
///
/// ```python
/// import numpy as np
/// from PIL import Image
/// image = Image.open('sprite.png')
/// bits = (~np.asarray(image).reshape(int(24*21/8), 8))
/// for bits_in_byte in bits.astype(int):
///     print(int(''.join('01'[i] for i in bits_in_byte), 2), end=',')
/// ```
const RUST_LOGO: [u8; 63] = [
    0, 90, 0, 1, 255, 128, 7, 239, 224, 15, 24, 240, 28, 0, 56, 63, 255, 28, 127, 255, 158, 127,
    255, 222, 235, 195, 215, 115, 255, 142, 227, 255, 135, 99, 199, 142, 247, 195, 223, 127, 243,
    254, 127, 241, 254, 62, 0, 60, 29, 0, 184, 15, 129, 240, 7, 255, 224, 1, 255, 128, 0, 90, 0,
];

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        // Copy Rust logo to sprite address and set sprite pointers
        const SPRITE_ADDRESS: u16 = 0x2000;
        *(SPRITE_ADDRESS as *mut [u8; 63]) = RUST_LOGO;
        poke!(
            c64::DEFAULT_SPRITE0_PTR,
            vic2::to_sprite_pointer(SPRITE_ADDRESS)
        );
        poke!(
            c64::DEFAULT_SPRITE2_PTR,
            vic2::to_sprite_pointer(SPRITE_ADDRESS)
        );

        // Borrow VIC-II chip on C64 (0xD000)
        let vic = &*c64::VIC;

        // Sprite 0 properties
        vic.sprite0_xpos.write(180);
        vic.sprite0_ypos.write(128);
        vic.sprite_colors[0].write(vic2::GREEN);
        vic.sprite_expand_x.write(vic2::Sprites::SPRITE0);
        vic.sprite_expand_y.write(vic2::Sprites::SPRITE0);

        // Sprite 2 properties
        vic.sprite2_xpos.write(180);
        vic.sprite2_ypos.write(60);
        vic.sprite_colors[2].write(vic2::RED);
        vic.sprite_background_priority.write(vic2::Sprites::SPRITE2);

        // Show sprite 0 and 2
        vic.sprite_enable.write(vic2::Sprites::SPRITE0 | vic2::Sprites::SPRITE2);
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("panic!");
    loop {}
}
