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
//
// Loosely based on C headers from cc65, here modififed from the original version.
//
//                                  cx16.h
//
//                      CX16 system-specific definitions
//                             For prerelease 39
//
//
// This software is provided "as-is", without any expressed or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated, but is not required.
// 2. Altered source versions must be plainly marked as such, and must not
//    be misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source
//    distribution.

//! Commander X16 support.
//!
//! The Commander X16 (also known as the X16) is a modern hardware platform being development.
//! It is built from modern, mostly off-the-shelf parts, with KERNAL
//! compatibility with the Commodore family of computers. It was initiated by David "the 8 Bit Guy" Murray,
//! who wanted an accessible and affordable platform with late-80s sound and graphics.

use crate::vera::VersatileEmbeddedRetroAdapter;

pub const CH_COLOR_SWAP: u8 = 1;
pub const CH_UNDERLINE: u8 = 4;
pub const CH_WHITE: u8 = 5;
pub const CH_BOLD: u8 = 6;
pub const CH_BACKSPACE: u8 = 8;
pub const CH_ITALIC: u8 = 11;
pub const CH_OUTLINE: u8 = 12;
pub const CH_FONT_ISO: u8 = 15;
pub const CH_RED: u8 = 28;
pub const CH_GREEN: u8 = 30;
pub const CH_BLUE: u8 = 31;
pub const CH_ORANGE: u8 = 129;
pub const CH_FONT_PET: u8 = 143;
pub const CH_BLACK: u8 = 144;
pub const CH_ATTR_CLEAR: u8 = 146;
pub const CH_BROWN: u8 = 149;
pub const CH_PINK: u8 = 150;
pub const CH_LIGHTRED: u8 = 150;
pub const CH_GRAY1: u8 = 151;
pub const CH_GRAY2: u8 = 152;
pub const CH_LIGHTGREEN: u8 = 153;
pub const CH_LIGHTBLUE: u8 = 154;
pub const CH_GRAY3: u8 = 155;
pub const CH_PURPLE: u8 = 156;
pub const CH_YELLOW: u8 = 158;
pub const CH_CYAN: u8 = 159;
pub const CH_SHIFT_SPACE: u8 = 160;
pub const CH_SHIFT_TAB: u8 = 24;
pub const CH_HELP: u8 = 132;
pub const CH_F1: u8 = 133;
pub const CH_F2: u8 = 137;
pub const CH_F3: u8 = 134;
pub const CH_F4: u8 = 138;
pub const CH_F5: u8 = 135;
pub const CH_F6: u8 = 139;
pub const CH_F7: u8 = 136;
pub const CH_F8: u8 = 140;
pub const CH_F9: u8 = 16;
pub const CH_F10: u8 = 21;
pub const CH_F11: u8 = 22;
pub const CH_F12: u8 = 23;

pub const COLOR_BLACK: u8 = 0;
pub const COLOR_WHITE: u8 = 1;
pub const COLOR_RED: u8 = 2;
pub const COLOR_CYAN: u8 = 3;
pub const COLOR_PURPLE: u8 = 4;
pub const COLOR_GREEN: u8 = 5;
pub const COLOR_BLUE: u8 = 6;
pub const COLOR_YELLOW: u8 = 7;
pub const COLOR_ORANGE: u8 = 8;
pub const COLOR_BROWN: u8 = 9;
pub const COLOR_PINK: u8 = 10;
pub const COLOR_LIGHTRED: u8 = 10;
pub const COLOR_GRAY1: u8 = 11;
pub const COLOR_GRAY2: u8 = 12;
pub const COLOR_LIGHTGREEN: u8 = 13;
pub const COLOR_LIGHTBLUE: u8 = 14;
pub const COLOR_GRAY3: u8 = 15;

pub const JOY_BTN_1_MASK: u8 = 128;
pub const JOY_BTN_2_MASK: u8 = 64;
pub const JOY_BTN_3_MASK: u8 = 32;
pub const JOY_BTN_4_MASK: u8 = 16;
pub const JOY_UP_MASK: u8 = 8;
pub const JOY_DOWN_MASK: u8 = 4;
pub const JOY_LEFT_MASK: u8 = 2;
pub const JOY_RIGHT_MASK: u8 = 1;

pub const JOY_BTN_A_MASK: u8 = 128;
pub const JOY_BTN_B_MASK: u8 = 64;
pub const JOY_SELECT_MASK: u8 = 32;
pub const JOY_START_MASK: u8 = 16;
pub const JOY_FIRE2_MASK: u8 = 64;

pub const MOUSE_BTN_MIDDLE: u8 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VersatileInterfaceAdapter65C22 {
    /// Port B `prb`
    pub port_b: u8,
    /// Port A `pra`
    pub port_a: u8,
    /// Data direction B `ddrb`
    pub data_direction_b: u8,
    /// Data direction A `ddra`
    pub data_direction_a: u8,
    /// Timer 1 `t1`
    pub timer1: u16,
    /// Timer 1 latch `t1l`
    pub timer1_latch: u16,
    /// Timer 2 `t2`
    pub timer2: u16,
    /// Shift `sr`
    pub shift: u8,
    /// Auxiliary control `acr`
    pub auxiliary_control: u8,
    /// Peripheral control `pcr`
    pub peripheral_control: u8,
    /// Interrupt flag `ifr`
    pub irq_flag: u8,
    /// Interrupt enable `ier`
    pub irq_enable: u8,
    /// Port A w/o handshake `pra2`
    pub port_a_no_handshape: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Access to emulator specific features
pub struct Emulator {
    /// Boolean: debugging enabled
    pub debug: u8,
    /// Boolean: displaying VERA activity
    pub vera_action: u8,
    /// Boolean: displaying typed keys
    pub keyboard: u8,
    /// How to send Kernal output to host
    pub echo: u8,
    /// Boolean: save machine state on exit
    pub save_on_exit: u8,
    /// How GIF movie is being recorded
    pub gif_method: u8,
    /// Unused
    pub unused1: [u8; 2],
    /// Running total of CPU cycles (8 MHz.)
    pub cycle_count: u32,
    /// Unused
    pub unused2: [u8; 1],
    /// Keyboard layout number
    pub keymap: u8,
    /// "16" if running on x16emu
    pub detect: [i8; 2],
}

// Memory layout
pub const RAM_BANK: *mut u8 = (0x00) as *mut u8;
pub const ROM_BANK: *mut u8 = (0x01) as *mut u8;

/// Pointer to first Versatile Interface Adapter (VIA1)
pub const VIA1: *const VersatileInterfaceAdapter65C22 =
    (0x9f00) as *const VersatileInterfaceAdapter65C22;

/// Pointer to second Versatile Interface Adapter (VIA2)
pub const VIA2: *const VersatileInterfaceAdapter65C22 =
    (0x9f10) as *const VersatileInterfaceAdapter65C22;

/// Pointer to the Versatile Embedded Retro Adapter chip
pub const VERA: *const VersatileEmbeddedRetroAdapter =
    (0x9f20) as *const VersatileEmbeddedRetroAdapter;

/// Pointer to Yamaha 2151 sound chip
pub const YM2151: *mut u8 = (0x9f40) as *mut u8;

/// Access to emulator specific features
pub const EMULATOR: *const Emulator = (0x9fb0) as *const Emulator;

pub const BANK_RAM: *mut u8 = (0xa000) as *mut u8;
