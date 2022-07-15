use bitflags::bitflags;
use memoffset::offset_of;
use volatile_register::RW;
use static_assertions::const_assert;

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
    pub struct ScreenBanks: u8 {
        const CHARSET_0000 = 0b0000_0000;
        const CHARSET_0800 = 0b0000_0010;
        const CHARSET_1000 = 0b0000_0100;
        const CHARSET_1800 = 0b0000_0110;
        const CHARSET_2000 = 0b0000_1000;
        const CHARSET_2800 = 0b0000_1010;
        const CHARSET_3000 = 0b0000_1100;
        const CHARSET_3800 = 0b0000_1110;
        const SCREEN_0000 = 0b0000_0000;
        const SCREEN_0400 = 0b0001_0000;
        const SCREEN_0800 = 0b0010_0000;
        const SCREEN_0C00 = 0b0011_0000;
        const SCREEN_1000 = 0b0100_0000;
        const SCREEN_1400 = 0b0101_0000;
        const SCREEN_1800 = 0b0110_0000;
        const SCREEN_1C00 = 0b0111_0000;
        const SCREEN_2000 = 0b1000_0000;
        const SCREEN_2400 = 0b1001_0000;
        const SCREEN_2800 = 0b1010_0000;
        const SCREEN_2C00 = 0b1011_0000;
        const SCREEN_3000 = 0b1100_0000;
        const SCREEN_3400 = 0b1101_0000;
        const SCREEN_3800 = 0b1110_0000;
        const SCREEN_3C00 = 0b1111_0000;
        const DEFAULT = Self::SCREEN_0800.bits | Self::CHARSET_1000.bits;
    }
}

impl ScreenBanks {
    /// Get the screen address the bank is pointing to
    pub fn get_screen(&self) -> *mut u8 {
        (self.bits as u16 >> 4 << 10) as *mut u8
    }

    /// Get the charset address the bank is pointing to
    pub fn get_charset(&self) -> *mut u8 {
        ((self.bits as u16) << 12 >> 2) as *mut u8
    }

    /// Create a bank based in given screen and charset addresses. Will check if the combination
    /// is possible.
    pub unsafe fn from_addresses(screen : *mut u8, charset : *mut u8) -> ScreenBanks {
        let bank = ((screen as u16 >> 6) & 0xf0 | ((charset as u16 >> 10) & 0x0e)) as u8;
        Self::from_bits_unchecked(bank)
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
    pub screen_and_charset_bank: RW<ScreenBanks>, // 0x18
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

const_assert!(offset_of!(MOSVideoInterfaceControllerII, control1) == 0x16);
const_assert!(offset_of!(MOSVideoInterfaceControllerII, sprites_expand_y) == 0x17);
