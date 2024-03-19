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

use crate::petscii;
use crate::sid::*;
use crate::vic2::*;
use volatile_register::RW;

pub mod iomap;
pub mod libc;
pub mod math;
mod memory;
pub mod random;
pub mod vic4;
pub use memory::*;

/// Default system palette
///
/// The following table is extracted from the MEGA65 user guide.
///
/// Code |  Red  | Green | Blue  | Name             | HTML color
/// ---- | ----- | ----- | ----- | ---------------- | -----------
/// 0    |    0  |   0   |  0    | Black            | #000000
/// 1    |   15  |  15   | 15    | White            | #FFFFFF
/// 2    |   15  |   0   |  0    | Red              | #FF0000
/// 3    |    0  |  15   | 15    | Cyan             | #00FFFF
/// 4    |   15  |   0   | 15    | Purple           | #FF00FF
/// 5    |    0  |  15   |  0    | Green            | #00FF00
/// 6    |    0  |   0   | 15    | Blue             | #0000FF
/// 7    |   15  |  15   |  0    | Yellow           | #FFFF00
/// 8    |   15  |   6   |  0    | Orange           | #FF6F00
/// 9    |   10  |   4   |  0    | Brown            | #A04000
/// 10   |   15  |   7   |  7    | Light Red (Pink) | #FF7777
/// 11   |    5  |   5   |  5    | Dark Grey        | #050505
/// 12   |    8  |   8   |  8    | Medium Grey      | #080808
/// 13   |    9  |  15   |  9    | Light Green      | #09FF09
/// 14   |    9  |   9   | 15    | Light Blue       | #0909FF
/// 15   |   11  |  11   | 11    | Light Grey       | #0B0B0B
/// 16   |   14  |   0   |  0    | Guru Meditation  | #E00000
/// 17   |   15  |   5   |  0    | Rambutan         | #FF5000
/// 18   |   15  |  11   |  0    | Carrot           | #FF6F00
/// 19   |   14  |  14   |  0    | Lemon Tart       | #8E8E00
/// 20   |    7  |  15   |  0    | Pandan           | #07FF00
/// 21   |    6  |  14   |  6    | Seasick Green    | #06E606
/// 22   |    0  |  14   |  3    | Soylent Green    | #00E003
/// 23   |    0  |  15   |  9    | Slimer Green     | #00FF09
/// 24   |    0  |  13   | 13    | The Other Cyan   | #00DDDD
/// 25   |    0  |   9   | 15    | Sea Sky          | #009FFF
/// 26   |    0  |   3   | 15    | Smurf Blue       | #003FFF
/// 27   |    0  |   0   | 14    | Screen of Death  | #0000E0
/// 28   |    7  |   0   | 15    | Plum Sauce       | #0700FF
/// 29   |   12  |   0   | 15    | Sour Grape       | #0C00FF
/// 30   |   15  |   0   | 11    | Bubblegum        | #FF0B0B
/// 31   |   15  |   3   |  6    | Hot Tamales      | #FF0306
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DefaultPalette {
    /// Black color (<a style="color:#000000;">&#9679;</a>)
    Black = 0,
    /// White color (<a style="color:#FFFFFF;">&#9679;</a>)
    White = 1,
    /// Red color (<a style="color:#FF0000;">&#9679;</a>)
    Red = 2,
    /// Cyan color (<a style="color:#00FFFF;">&#9679;</a>)
    Cyan = 3,
    /// Purple color (<a style="color:#FF00FF;">&#9679;</a>)
    Purple = 4,
    /// Green color (<a style="color:#00FF00;">&#9679;</a>)
    Green = 5,
    /// Blue color (<a style="color:#0000FF;">&#9679;</a>)
    Blue = 6,
    /// Yellow color (<a style="color:#FFFF00;">&#9679;</a>)
    Yellow = 7,
    /// Orange color (<a style="color:#FF6F00;">&#9679;</a>)
    Orange = 8,
    /// Brown color (<a style="color:#A04000;">&#9679;</a>)
    Brown = 9,
    /// Light Red (Pink) color (<a style="color:#FF7777;">&#9679;</a>)
    LightRed = 10,
    /// Dark Grey color (<a style="color:#050505;">&#9679;</a>)
    DarkGrey = 11,
    /// Medium Grey color (<a style="color:#080808;">&#9679;</a>)
    MediumGrey = 12,
    /// Light Green color (<a style="color:#09FF09;">&#9679;</a>)
    LightGreen = 13,
    /// Light Blue color (<a style="color:#0909FF;">&#9679;</a>)
    LightBlue = 14,
    /// Light Grey color (<a style="color:#0B0B0B;">&#9679;</a>)
    LightGrey = 15,
    /// Guru Meditation color (<a style="color:#E00000;">&#9679;</a>)
    GuruMeditation = 16,
    /// Rambutan color (<a style="color:#FF5000;">&#9679;</a>)
    Rambutan = 17,
    /// Carrot color (<a style="color:#FF6F00;">&#9679;</a>)
    Carrot = 18,
    /// Lemon Tart color (<a style="color:#8E8E00;">&#9679;</a>)
    LemonTart = 19,
    /// Pandan color (<a style="color:#07FF00;">&#9679;</a>)
    Pandan = 20,
    /// Seasick Green color (<a style="color:#06E606;">&#9679;</a>)
    SeasickGreen = 21,
    /// Soylent Green color (<a style="color:#00E003;">&#9679;</a>)
    SoylentGreen = 22,
    /// Slimer Green color (<a style="color:#00FF09;">&#9679;</a>)
    SlimerGreen = 23,
    /// The Other Cyan color (<a style="color:#00DDDD;">&#9679;</a>)
    TheOtherCyan = 24,
    /// Sea Sky color (<a style="color:#009FFF;">&#9679;</a>)
    SeaSky = 25,
    /// Smurf Blue color (<a style="color:#003FFF;">&#9679;</a>)
    SmurfBlue = 26,
    /// Screen of Death color (<a style="color:#0000E0;">&#9679;</a>)
    ScreenOfDeath = 27,
    /// Plum Sauce color (<a style="color:#0700FF;">&#9679;</a>)
    PlumSauce = 28,
    /// Sour Grape color (<a style="color:#0C00FF;">&#9679;</a>)
    SourGrape = 29,
    /// Bubblegum color (<a style="color:#FF0B0B;">&#9679;</a>)
    Bubblegum = 30,
    /// Hot Tamales color (<a style="color:#FF0306;">&#9679;</a>)
    HotTamales = 31,
}

pub const DEFAULT_SCREEN: *mut u8 = (0x0800) as _;
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as _;
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as _;
pub const VICII: *const MOSVideoInterfaceControllerII = (0xd000) as _;

/// Pointer to first sound interface device
pub const SID0: *const MOSSoundInterfaceDevice = (0xd400) as _;
/// Pointer to second sound interface device
pub const SID1: *const MOSSoundInterfaceDevice = (0xd420) as _;
/// Pointer to third sound interface device
pub const SID2: *const MOSSoundInterfaceDevice = (0xd440) as _;
/// Pointer to fourth sound interface device
pub const SID3: *const MOSSoundInterfaceDevice = (0xd460) as _;

pub const COLOR_RAM: *mut u8 = (0xd800) as _;

/// Math multiplication-division status flags
pub const MATH_STATUS: *const volatile_register::RO<math::StatusFlags> = (0xd70f) as _;

/// Math Acceleration registers
pub const MATH_ACCELERATOR: *const math::MathAccelerator = (0xd768) as _;

pub enum VicBank {
    Region0000 = 0x11, // Bank 0
    Region4000 = 0x10, // Bank 1
    Region8000 = 0x01, // Bank 2
    RegionC000 = 0x00, // Bank 3
}

/// Get reference to VIC2 chip
pub const fn vic2() -> &'static MOSVideoInterfaceControllerII {
    unsafe { &*VICII }
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

/// Get reference to math accelerator
pub const fn math_accelerator() -> &'static math::MathAccelerator {
    unsafe { &*MATH_ACCELERATOR }
}

/// Control CPU clock speed
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Default)]
pub enum CPUSpeed {
    /// 1 Mhz clock frequency
    Slow,
    /// 3.5 Mhz clock frequency
    Medium,
    /// 40 Mhz clock frequency (default)
    #[default]
    Fast,
}

impl CPUSpeed {
    /// Set CPU speed
    pub fn set(&self) {
        const VICIV_CTRLB: *mut RW<u8> = 0xd031 as _;
        const VICIV_CTRLC: *mut RW<u8> = 0xd054 as _;
        match self {
            Self::Slow => unsafe {
                (*VICIV_CTRLB).modify(|m| m & 0b1011_1111);
                (*VICIV_CTRLC).modify(|m| m & 0b1011_1111);
            },
            Self::Medium => unsafe {
                (*VICIV_CTRLB).modify(|m| m | 0b0100_0000);
                (*VICIV_CTRLC).modify(|m| m & 0b1011_1111);
            },
            Self::Fast => unsafe {
                (*VICIV_CTRLB).modify(|m| m | 0b0100_0000);
                (*VICIV_CTRLC).modify(|m| m | 0b0100_0000);
            },
        }
    }
}

/// Struct used to store widht-height resolutions
#[derive(Default)]
pub struct Resolution<T> {
    pub width: T,
    pub height: T,
}

/// Returns screen resolution (char width, char heigh)
pub fn get_screen_size() -> Resolution<u8> {
    let mut resolution = Resolution::default();
    unsafe {
        libc::getscreensize(&mut resolution.width, &mut resolution.height);
    }
    resolution
}

/// Initialises the conio internal state
///
/// This must be called before using any conio library function.
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

/// Output multiple screen codes at X,Y coordinates
///
/// Works with _null-terminated_ screen codes only.
///
/// # Examples
/// ~~~
/// use mos_hardware::{petscii, petscii_null}
/// mega65::cputs_xy(2, 3, [8, 5, 12, 12, 15, 0].as_slice());
/// mega65::cputs_xy(4, 6, petscii_null!("hello").as_slice());
/// ~~~
pub fn cputs_xy(x: u8, y: u8, screen_codes: &[u8]) {
    assert_eq!(*screen_codes.last().unwrap(), 0u8);
    unsafe {
        libc::cputsxy(x, y, screen_codes.as_ptr());
    }
}

/// Output screen codes at current position
///
/// Works with _null-terminated_ screen codes only.
///
/// # Examples
/// ~~~
/// use mos_hardware::{petscii, petscii_null}
/// mega65::cputs(petscii_null!("hello").as_slice());
/// ~~~
pub fn cputs(screen_codes: &[u8]) {
    assert_eq!(*screen_codes.last().unwrap(), 0u8);
    unsafe {
        libc::cputs(screen_codes.as_ptr());
    }
}

/// Flush keyboard buffer
pub fn flush_keyboard_buffer() {
    unsafe {
        libc::flushkeybuf();
    }
}

/// Waits until a character is in the keyboard buffer and returns as petscii
pub fn cgetc() -> petscii::Petscii {
    unsafe { libc::cgetc() }.into()
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
///
/// # Examples
/// ~~~
/// let rtc = mega65::get_real_time_clock();
/// println!("TIME = {}:{}:{}", rtc.tm_hour, rtc.tm_min, rtc.tm_sec);
/// ~~~
pub fn get_real_time_clock() -> libc::m65_tm {
    let mut rtc = libc::m65_tm::default();
    unsafe {
        libc::getrtc(&mut rtc);
    }
    rtc
}

/// Sets VIC-III extended attributes mode to enable blink, underline, bold, highlight
pub fn set_extended_attributes() {
    unsafe {
        libc::setextendedattrib(1);
    }
}

/// Clears VIC-III extended attributes mode to disable blink, underline, bold, highlight
pub fn unset_extended_attributes() {
    unsafe {
        libc::setextendedattrib(0);
    }
}

/// Set character set address using mega65-libc
pub fn set_charset_address(address: u16) {
    unsafe {
        libc::setcharsetaddr(address as i32);
    }
}
