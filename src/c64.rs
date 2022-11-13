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

//! Commodore 64 support.
//!
//! Future information may be incorporated using the
//! [Ultimate Commodore 64 Reference](https://github.com/mist64/c64ref)

use crate::cia::*;
use crate::sid::*;
use crate::vic2::*;
use bitflags::bitflags;
use volatile_register::RW;

bitflags! {
    /// Control flags for the CPU port `R6510` at 0x0001
    ///
    /// Three-word combination constants like `RAM_IO_KERNAL` refer to banking configurations
    /// of what is visible at addresses `$A000-BFFF`, `$D000-DFFF`, and `$E000-FFFF`.
    /// Regardless of `0x0001`, the VIC-II chip *always* sees the `CHARROM` at `$1000-1FFF` and `$9000-9FFF`,
    /// and RAM everywhere else.
    ///
    /// [More information](https://codebase64.org/doku.php?id=base:memory_management).
    ///
    /// # Examples
    ///
    /// Here's an example that makes the RAM available "under" both the BASIC and KERNAL
    /// ROMs located at 0xA000-0xBFFF and 0xE000-0xFFFF.
    /// The VIC, SID, and CIA I/O devices are left accessible at 0xD000-0xDFFF:
    /// ~~~
    /// (*CPU_PORT).write(CpuPortFlags::RAM_IO_RAM);
    /// assert_eq!(CpuPortFlags::RAM_IO_RAM.bits(), 0x35);
    /// assert_eq!(CpuPortFlags::RAM_IO_KERNAL.bitw(), 0x36);
    /// ~~~
    pub struct CpuPortFlags: u8 {
        const DEFAULT              = Self::BASIC_IO_KERNAL.bits;
        const BASIC_IO_KERNAL      = 0b00110111;
        const RAM_RAM_RAM          = 0b00110000;
        const RAM_CHAR_RAM         = 0b00110001;
        const RAM_CHAR_KERNAL      = 0b00110010;
        const BASIC_CHAR_KERNAL    = 0b00110011;
        const RAM_IO_RAM           = 0b00110101;
        const RAM_IO_KERNAL        = 0b00110110;
        const DATASETTE_SIGNAL     = 0b00001000; // bit 3
        const DATASETTE_BUTTON_OFF = 0b00010000; // bit 4
        const DATASETTE_MOTOR_OFF  = 0b00100000; // bit 5
    }
}

/// Pointer to the `R6510` register for 6510 I/O (0x0001)
pub const CPU_PORT: *mut RW<CpuPortFlags> = (0x0001) as *mut RW<CpuPortFlags>;

/// Pointer to beginning of default video memory (0x0400)
pub const DEFAULT_VIDEO_MEMORY: *mut u8 = (0x0400) as *mut u8;

/// Pointer to the default video matrix area (0x0400)
///
/// The video matrix is where text screen characters are stored in RAM.
/// By default this corresponds to 25 lines, each with 40 columns.
pub const DEFAULT_VIDEO_MATRIX: *mut [u8; 25 * 40] = (0x0400) as *mut [u8; 25 * 40];

/// Default sprite shape pointers (0x0400)
///
/// This is the default location for sprite shape pointers, i.e.
/// relative to the default screen memory location 0x0400.
/// Individual sprite shape pointers can be calculated with
/// `vic2::to_sprite_pointer()`.
pub const DEFAULT_SPRITE_PTR: [*mut u8; 8] = [
    (0x0400 + 0x3F8 + 0) as *mut u8,
    (0x0400 + 0x3F8 + 1) as *mut u8,
    (0x0400 + 0x3F8 + 2) as *mut u8,
    (0x0400 + 0x3F8 + 3) as *mut u8,
    (0x0400 + 0x3F8 + 4) as *mut u8,
    (0x0400 + 0x3F8 + 5) as *mut u8,
    (0x0400 + 0x3F8 + 6) as *mut u8,
    (0x0400 + 0x3F8 + 7) as *mut u8,
];

/// Default upper case font in the CHARROM (0x1000)
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as *mut u8;

/// Default mixed case font in the CHARROM (0x1800)
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as *mut u8;

/// Pointer to BASIC ROM start (0xa000)
pub const BASIC_ROM: *mut u8 = (0xa000) as *mut u8;

/// Pointer to BASIC ROM area (0xa000 - 0xbfff)
pub const BASIC_ROM_AREA: *mut [u8; 0x1fff] = (0xa000) as *mut [u8; 0x1fff];

/// Pointer to the video interface controller (0xd000)
pub const VIC: *const MOSVideoInterfaceControllerII =
    (0xd000) as *const MOSVideoInterfaceControllerII;

/// Pointer to the sound interface device (0xd400)
pub const SID: *const MOSSoundInterfaceDevice = (0xd400) as *const MOSSoundInterfaceDevice;

/// Pointer to default color RAM (0xd800)
pub const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;

/// Pointer to first complex interface adapter (0xdc00)
pub const CIA1: *const MOSComplexInterfaceAdapter6526 =
    (0xdc00) as *const MOSComplexInterfaceAdapter6526;

/// Pointer to second complex interface adapter (0xdd00)
pub const CIA2: *const MOSComplexInterfaceAdapter6526 =
    (0xdd00) as *const MOSComplexInterfaceAdapter6526;

/// Pointer to the KERNAL ROM memory area (0xe000 - 0xffff)
pub const KERNAL_ROM: *mut [u8; 8192] = (0xe000) as *mut [u8; 8192];

bitflags! {
    /// Flags for the `CIA1::control_a` register (0xdc0e)
    pub struct CIA1ControlAFlags: u8 {
        /// Start (1) or stop (0) timer A
        const START         = 0b00000001; // bit 0
        const PBON          = 0b00000010;
        const OUTMODE       = 0b00000100;
        const RUNMODE       = 0b00001000;
        const FORCE_LOAD    = 0b00010000;
        const INMODE        = 0b00100000;
        const SERIAL_OUTPUT = 0b01000000;
        const FIFTY_HZ_RTC  = 0b10000000;
    }
}

pub enum VicBank {
    Region0000 = 0x11, // Bank 0
    Region4000 = 0x10, // Bank 1
    Region8000 = 0x01, // Bank 2
    RegionC000 = 0x00, // Bank 3
}

extern "C" {
    // defined in c to allow assembly and interrupt attribute
    fn hardware_raster_irq_c(triggering_raster_line: u8);
}

/// Setup hardware raster interrupt (0xfffe)
///
/// This registers a Rust function, `called_every_frame()` to be triggered
/// at a specific raster line. The BASIC and KERNAL roms are disabled so
/// suffix your main program with an endless loop.
/// `fn called_every_frame()` must be defined and *exported*
/// on the Rust side and will be called from C via a wrapper. This is because
/// the LLVM `__interrupt__` attribute is currently not available from Rust.
///
/// # Examples
/// ```
/// #[no_mangle]
/// pub unsafe extern fn called_every_frame() {
///    ...
/// }
///
/// #[start]
/// fn _main(_argc: isize, _argv: *const *const u8) -> isize {
///    c64::hardware_raster_irq(100); // trigger at raster line 100
///    loop {}                        // let's not return to dead BASIC
/// }
/// ```
pub fn hardware_raster_irq(triggering_raster_line: u8) {
    unsafe {
        hardware_raster_irq_c(triggering_raster_line);
    }
}

/// Special keyboard and PETSCII codes
pub enum Keyboard {
    Delete = 0x14,
    Stop = 0x03,
    Return = 0x0d,
    Home = 0x13,
    CursorDown = 0x11,
    CursorRight = 0x1d,
    Space = 0x20,
    ArrowLeft = 0x5f,
    Run = 0x83,
    F1 = 0x85,
    F2 = 0x86,
    F3 = 0x87,
    F4 = 0x88,
    F5 = 0x89,
    F6 = 0x8a,
    F7 = 0x8b,
    F8 = 0x8c,
    ShiftReturn = 0x8d,
    CursorUp = 0x91,
    Clear = 0x93,
    Insert = 0x94,
    CursorLeft = 0x9d,
}

/// Get reference to VIC2 chip
pub const fn vic2() -> &'static MOSVideoInterfaceControllerII {
    unsafe { &*VIC }
}

/// Clears screen, functional style (fill with SPACE character)
pub fn clear_screen() {
    unsafe {
        (*DEFAULT_VIDEO_MATRIX)
            .iter_mut()
            .for_each(|i| *i = Keyboard::Space as u8);
    }
}
