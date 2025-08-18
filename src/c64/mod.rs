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

mod cpu6510;
mod mos6526;
use crate::cia::*;
use crate::sid::*;
use crate::vic2::*;
use cpu6510::*;
use mos6526::*;

/// Pointer to CPU 6510
pub const CPU: *const Cpu6510 = (0x0000) as _;

/// Default video memory address (0x0400)
pub const DEFAULT_VIDEO_ADDR: u16 = 0x0400;

/// Pointer to beginning of default video memory (0x0400)
pub const DEFAULT_VIDEO_MEMORY: *mut u8 = DEFAULT_VIDEO_ADDR as _;

/// Pointer to the default video matrix area (0x0400)
///
/// The video matrix is where text screen characters are stored in RAM.
/// By default this corresponds to 25 lines, each with 40 columns.
pub const DEFAULT_VIDEO_MATRIX: *mut [u8; 25 * 40] = (0x0400) as _;

/// Default sprite shape pointers (0x0400)
///
/// This is the default location for sprite shape pointers, i.e.
/// relative to the default screen memory location 0x0400.
/// Individual sprite shape pointers can be calculated with
/// `vic2::to_sprite_pointer()`.
pub const DEFAULT_SPRITE_PTR: [*mut u8; 8] = [
    (0x0400 + 0x3F8) as _,
    (0x0400 + 0x3F8 + 1) as _,
    (0x0400 + 0x3F8 + 2) as _,
    (0x0400 + 0x3F8 + 3) as _,
    (0x0400 + 0x3F8 + 4) as _,
    (0x0400 + 0x3F8 + 5) as _,
    (0x0400 + 0x3F8 + 6) as _,
    (0x0400 + 0x3F8 + 7) as _,
];

/// Default upper case font in the CHARROM (0x1000)
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as _;

/// Default mixed case font in the CHARROM (0x1800)
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as _;

/// Pointer to BASIC ROM start (0xa000)
pub const BASIC_ROM: *mut u8 = (0xa000) as _;

/// Pointer to BASIC ROM area (0xa000 - 0xbfff)
pub const BASIC_ROM_AREA: *mut [u8; 0x1fff] = (0xa000) as _;

/// Pointer to the video interface controller (0xd000)
pub const VIC: *const MOSVideoInterfaceControllerII = (0xd000) as _;

/// Pointer to the sound interface device (0xd400)
pub const SID: *const MOSSoundInterfaceDevice = (0xd400) as _;

/// Pointer to default color RAM (0xd800)
pub const COLOR_RAM: *mut u8 = (0xd800) as _;

/// Pointer to first complex interface adapter (0xdc00)
pub const CIA1: *const MOSComplexInterfaceAdapter6526_1 = (0xdc00) as _;

/// Pointer to second complex interface adapter (0xdd00)
pub const CIA2: *const MOSComplexInterfaceAdapter6526_2 = (0xdd00) as _;

/// Pointer to the KERNAL ROM memory area (0xe000 - 0xffff)
pub const KERNAL_ROM: *mut [u8; 8192] = (0xe000) as _;

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

/// Get reference to CPU register
pub const fn cpu() -> &'static Cpu6510 {
    unsafe { &*CPU }
}

/// Get reference to VIC2 chip
pub const fn vic2() -> &'static MOSVideoInterfaceControllerII {
    unsafe { &*VIC }
}

/// Get reference to SID chip
pub const fn sid() -> &'static MOSSoundInterfaceDevice {
    unsafe { &*SID }
}

/// Get reference to CIA1 chip
pub const fn cia1() -> &'static MOSComplexInterfaceAdapter6526_1 {
    unsafe { &*CIA1 }
}

/// Get reference to CIA1 control block
pub const fn cia1_control() -> &'static Mos6526ControlBlock {
    unsafe { &(*CIA1).control }
}

/// Get reference to CIA2 chip
pub const fn cia2() -> &'static MOSComplexInterfaceAdapter6526_2 {
    unsafe { &*CIA2 }
}

/// Get reference to CIA2 control block
pub const fn cia2_control() -> &'static Mos6526ControlBlock {
    unsafe { &(*CIA2).control }
}

/// Clears screen, functional style (fill with SPACE character)
pub fn clear_screen() {
    unsafe {
        (*DEFAULT_VIDEO_MATRIX)
            .iter_mut()
            .for_each(|i| *i = Keyboard::Space as u8);
    }
}

/// Shift to lower case ROM charset
pub fn set_lower_case() {
    unsafe {
        vic2().screen_and_charset_bank.write(23);
    }
}

/// Shift to upper case ROM charset
pub fn set_upper_case() {
    unsafe {
        vic2().screen_and_charset_bank.write(21);
    }
}

/// Select one of four memory ranges that VIC II sees.
///
/// # Arguments
/// * `bank` - VIC bank to select:
///   - `CIA2PortA::VIC_BANK_0` for $0000-$3FFF
///   - `CIA2PortA::VIC_BANK_1` for $4000-$7FFF  
///   - `CIA2PortA::VIC_BANK_2` for $8000-$BFFF
///   - `CIA2PortA::VIC_BANK_3` for $C000-$FFFF
///
/// # Example
/// ```rust
/// // Switch to bank 1 ($4000-$7FFF)
/// set_vic_bank(CIA2PortA::VIC_BANK_1);
/// ```
pub fn set_vic_bank(bank: CIA2PortA) {
    let mut dir_a = cia2().data_direction_port_a.read();
    let mut port_a = cia2().port_a.read();
    unsafe {
        // Configure bits 0-1 as outputs for VIC bank control
        dir_a.set_raw(dir_a.raw() | 0b11);
        cia2().data_direction_port_a.write(dir_a);

        // Set the VIC bank using the provided constant
        port_a.set_vic_bank(bank.bits() & 0b11);
        cia2().port_a.write(port_a);
    }
}
