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

//! MEGA65 support.
//!
//! The MEGA65 is a 100% open-source implementation of the official (but never-released)
//! Commodore 65 computer. It is in development by associates of the Museum of Electronic
//! Games and Art e. V., a not-for-profit institution "dedicated to the preservation of
//! our digital heritage." As well as the original C65 design, the MEGA65 provides
//! additional hardware and software enhancements, including a choice of using BASIC 10 or
//! BASIC 65 (containing improvements that go beyond BASIC 10).
//! The MEGA65 has an 8-bit CPU with additional 32-bit instructions implemented in FPGA.
//! Like the original C65, it also has a Commodore 64 mode with a level of compatibility
//! similar to that of the Commodore 128 running in C64 mode

use crate::sid::*;
use crate::vic2::*;
use crate::{peek, poke};

pub mod iomap;
pub mod libc;

pub const DEFAULT_SCREEN: *mut u8 = (0x0800) as *mut u8;
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as *mut u8;
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as *mut u8;

pub const VICII: *const MOSVideoInterfaceControllerII =
    (0xd000) as *const MOSVideoInterfaceControllerII;

/// Pointer to first sound interface device
pub const SID0: *const MOSSoundInterfaceDevice = (0xd400) as *const MOSSoundInterfaceDevice;
/// Pointer to second sound interface device
pub const SID1: *const MOSSoundInterfaceDevice = (0xd420) as *const MOSSoundInterfaceDevice;
/// Pointer to third sound interface device
pub const SID2: *const MOSSoundInterfaceDevice = (0xd440) as *const MOSSoundInterfaceDevice;
/// Pointer to fourth sound interface device
pub const SID3: *const MOSSoundInterfaceDevice = (0xd460) as *const MOSSoundInterfaceDevice;

pub const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;

pub enum VicBank {
    Region0000 = 0x11, // Bank 0
    Region4000 = 0x10, // Bank 1
    Region8000 = 0x01, // Bank 2
    RegionC000 = 0x00, // Bank 3
}

/// Get reference to first SID chip
pub const fn sid0() -> &'static MOSSoundInterfaceDevice {
    unsafe { &*SID0 }
}

/// Get reference to second SID chip
pub const fn sid1() -> &'static MOSSoundInterfaceDevice {
    unsafe { &*SID1 }
}

/// Get reference to third SID chip
pub const fn sid2() -> &'static MOSSoundInterfaceDevice {
    unsafe { &*SID2 }
}

/// Get reference to fourth SID chip
pub const fn sid3() -> &'static MOSSoundInterfaceDevice {
    unsafe { &*SID3 }
}

/// Set CPU speed to 1 Mhz
pub fn speed_mode1() {
    unsafe {
        let mut val: u8 = peek!(0xd031 as *mut u8) & 0b1011_1111; // unset FAST bit
        poke!(0xd031 as *mut u8, val);
        val = peek!(0xd054 as *mut u8) & 0b1011_1111; // unset VFAST bit
        poke!(0xd054 as *mut u8, val);
    }
}

/// Set CPU speed to 3.5 Mhz
pub fn speed_mode3() {
    unsafe {
        let mut val: u8 = peek!(0xd031 as *mut u8) | 0b0100_0000; // set FAST bit
        poke!(0xd031 as *mut u8, val);
        val = peek!(0xd054 as *mut u8) & 0b1011_1111; // unset VFAST
        poke!(0xd054 as *mut u8, val);
    }
}

/// Set CPU speed to 40 Mhz
pub fn speed_mode40() {
    unsafe {
        let mut val: u8 = peek!(0xd031 as *mut u8) | 0b0100_0000; // set FAST bit
        poke!(0xd031 as *mut u8, val);
        val = peek!(0xd054 as *mut u8) | 0b0100_0000; // set VFAST bit
        poke!(0xd054 as *mut u8, val);
    }
}

/// Generate random byte
pub fn rand8(max_value: u8) -> u8 {
    unsafe {
        libc::rand8(max_value)
    }
}

/// Read into 24 bit memory
pub fn lpeek(address: u32) -> u8 {
    unsafe {
        libc::lpeek(address as i32)
    }
}

/// Write into 24 bit memory
pub unsafe fn lpoke(address: u32, value: u8) {
    libc::lpoke(address as i32, value)
}

/// Struct used to store widht-height resolutions
pub struct Resolution<T> {
    pub width: T,
    pub height: T,
}

/// Returns screen resolution (char width, char heigh)
pub fn get_screen_size() -> Resolution<u8> {
    let mut resolution = Resolution{width: 0, height: 0};
    unsafe {
        libc::getscreensize(&mut resolution.width, &mut resolution.height);
    }
    resolution
}

/// Initialize conio
pub fn conio_init() {
    unsafe {
        libc::conioinit();
    }
}

/// Shift to lower case ROM charset
pub fn set_lower_case() {
    unsafe {
        libc::setlowercase();
    }
}

/// Shift to upper case ROM charset
pub fn set_upper_case() {
    unsafe {
        libc::setuppercase();
    }
}

/// Clear all chars on screen
pub fn clear_screen() {
    unsafe {
        libc::clrscr();        
    }
}

/// Goto top left corner
pub fn go_home() {
    unsafe {
        libc::gohome();        
    }
}

/// Goto specific character position
pub fn goto_xy(x: u8, y: u8) {
    unsafe {
        libc::gotoxy(x, y);        
    }
}

/// Sets the current border color
pub fn set_border_color(color: u8) {
    unsafe {
        libc::bordercolor(color);
    }
}

/// Sets the current screen (background) color
pub fn set_background_color(color: u8) {
    unsafe {
        libc::bgcolor(color);
    }
}

/// Sets the current text color
pub fn set_text_color(color: u8) {
    unsafe {
        libc::textcolor(color);
    }
}

/// Read real time clock
pub fn get_real_time_clock() -> libc::m65_tm {
    let mut rtc = libc::m65_tm::default();
    unsafe {
        libc::getrtc(&mut rtc);
    }
    rtc
}

pub fn set_extended_attributes(value: u8) {
    unsafe {
        libc::setextendedattrib(value);
    }
}
