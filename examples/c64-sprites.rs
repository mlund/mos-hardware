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
use vic2::*;

/// Sprite pattern
///
/// Images can be converted to 24x21 sprite data using ImageMagick:
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
    const SPRITE_ADDRESS: u16 = 0x2000;
    const SPRITE_PTR: u8 = to_sprite_pointer(SPRITE_ADDRESS);

    // VIC-II chip on C64 (0xD000)
    let vic = c64::vic2();

    unsafe {
        // Copy Rust logo to sprite address and set sprite shape pointers
        *(SPRITE_ADDRESS as *mut [u8; 63]) = RUST_LOGO;
        poke!(c64::DEFAULT_SPRITE_PTR[0], SPRITE_PTR);
        poke!(c64::DEFAULT_SPRITE_PTR[2], SPRITE_PTR);

        // Sprite 0 properties
        vic.set_sprite_pos(0, 180, 100);
        vic.set_sprite_color(0, GREEN);
        vic.sprite_expand_x.write(Sprites::SPRITE0);
        vic.sprite_expand_y.write(Sprites::SPRITE0);

        // Sprite 2 properties
        vic.set_sprite_pos(2, 180, 60);
        vic.set_sprite_color(2, RED);
        vic.sprite_background_priority.write(Sprites::SPRITE2);

        // Show sprite 0, 2, and 7
        vic.sprite_enable
            .write(Sprites::SPRITE0 | Sprites::SPRITE2 | Sprites::SPRITE7);

        // Ups, we didn't mean to show sprite 7, so let's disable it again:
        let mut enabled_sprites = vic.sprite_enable.read();
        enabled_sprites.remove(Sprites::SPRITE7);
        vic.sprite_enable.write(enabled_sprites);
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("panic!");
    loop {}
}
