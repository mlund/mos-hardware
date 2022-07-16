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

use bitflags::bitflags;
use volatile_register::RW;

pub const BLACK: u8 = 0;
pub const WHITE: u8 = 1;
pub const RED: u8 = 2;
pub const CYAN: u8 = 3;
pub const PURPLE: u8 = 4;
pub const GREEN: u8 = 5;
pub const BLUE: u8 = 6;
pub const YELLOW: u8 = 7;
pub const ORANGE: u8 = 8;
pub const BROWN: u8 = 9;
pub const LIGHT_RED: u8 = 10;
pub const GRAY1: u8 = 11;
pub const GRAY2: u8 = 12;
pub const LIGHT_GREEN: u8 = 13;
pub const LIGHT_BLUE: u8 = 14;
pub const GRAY3: u8 = 15;

bitflags! {
    pub struct Control1Flags: u8 {
        const XSCROLL0 = 0b00000000;
        const XSCROLL1 = 0b00000001;
        const XSCROLL3 = 0b00000010;
        const COLUMN_SELECT = 0b00000100; // off=38 chars, on=40 chars
        const MULTICOLOR = 0b00001000;
        const DEFAULT = Self::MULTICOLOR.bits;
    }
}
bitflags! {
    /**
     * All possible charset memory locations
     *
     * Example:
     * ```
     * let bank = vic2::ScreenBank::AT_2C00.bits() | vic2::CharsetBank::AT_2000.bits();
     * (*c64::VIC).screen_and_charset_bank.write(bank);
     * ```
     */
    pub struct CharsetBank: u8 {
        const AT_0000 = 0b0000_0000;
        const AT_0800 = 0b0000_0010;
        const AT_1000 = 0b0000_0100;
        const AT_1800 = 0b0000_0110;
        const AT_2000 = 0b0000_1000;
        const AT_2800 = 0b0000_1010;
        const AT_3000 = 0b0000_1100;
        const AT_3800 = 0b0000_1110;
        const DEFAULT = Self::AT_1000.bits;
    }
}

impl CharsetBank {
    /**
     * Generate bank from charset memory address. Will check if it is valid.
     *
     * Example:
     * ```
     * const SCREEN: u16 = 0x2800;
     * const CHARSET: u16 = 0x2000;
     * const BANK: u8 = vic2::ScreenBank::from(SCREEN).bits() | vic2::CharsetBank::from(CHARSET).bits();
     * ```
     */
    pub const fn from(charset : u16) -> CharsetBank {
        let bank = ((charset >> 10) & 0x0e) as u8;
        Self::from_bits(bank).unwrap()
    }
}

bitflags! {
    /**
     * All possible screen memory locations
     */
    pub struct ScreenBank: u8 {
        const AT_0000 = 0b0000_0000;
        const AT_0400 = 0b0001_0000;
        const AT_0800 = 0b0010_0000;
        const AT_0C00 = 0b0011_0000;
        const AT_1000 = 0b0100_0000;
        const AT_1400 = 0b0101_0000;
        const AT_1800 = 0b0110_0000;
        const AT_1C00 = 0b0111_0000;
        const AT_2000 = 0b1000_0000;
        const AT_2400 = 0b1001_0000;
        const AT_2800 = 0b1010_0000;
        const AT_2C00 = 0b1011_0000;
        const AT_3000 = 0b1100_0000;
        const AT_3400 = 0b1101_0000;
        const AT_3800 = 0b1110_0000;
        const AT_3C00 = 0b1111_0000;
        const DEFAULT = Self::AT_0800.bits;
    }
}

impl ScreenBank {
    /**
     * Generate bank from screen memory address. Will check if it is valid.
     *
     * Example:
     * ```
     * const SCREEN: u16 = 0x2800;
     * const CHARSET: u16 = 0x2000;
     * const BANK: u8 = vic2::ScreenBank::from(SCREEN).bits() | vic2::CharsetBank::from(CHARSET).bits();
     * ```
     */
    pub const fn from(screen : u16) -> ScreenBank {
        let bank = (screen >> 6) as u8;
        Self::from_bits(bank).unwrap()
    }
}

#[repr(C, packed)]
pub struct MOSVideoInterfaceControllerII {
    pub sprite_positions: [RW<u8>; 16],      // x0,y0,...
    pub msb_xcoord: RW<u8>,                  // 0x10
    pub y_scroll_mode: RW<u8>,               // 0x11
    pub raster_counter: RW<u8>,              // 0x12
    pub lightpen_x: RW<u8>,                  // 0x13
    pub lightpen_y: RW<u8>,                  // 0x14
    pub sprite_enable: RW<u8>,               // 0x15
    pub control1: RW<Control1Flags>,         // 0x16
    pub sprites_expand_y: RW<u8>,            // 0x17
    pub screen_and_charset_bank: RW<u8>,     // 0x18
    pub irq_status: RW<u8>,                  // 0x19
    pub irq_enable: RW<u8>,                  // 0x1a
    pub sprites_priority: RW<u8>,            // 0x1b
    pub sprites_multicolor: RW<u8>,          // 0x1c
    pub sprites_expand_x: RW<u8>,            // 0x1d
    pub sprite_sprite_collision: RW<u8>,     // 0x1e
    pub sprite_background_collision: RW<u8>, // 0x1f
    pub border_color: RW<u8>,                // 0x20
    pub background_color: RW<u8>,            // 0x21
}

