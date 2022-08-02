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
use tock_registers::registers::ReadWrite;
use tock_registers::{register_bitfields, register_structs};

register_structs! {
    /// Memory map for the Commodore 64.
    /// More information [here](https://sta.c64.org/cbm64mem.html).
    /// Currently not in use.
    pub MemoryMap {
        (0x0000 => cpu_port_data_direction: ReadWrite<u8>),
        (0x0001 => cpu_port_data: ReadWrite<u8, CpuPortControl::Register>),
        (0x0002 => _reserved_after_cpu_port_data),
        (0x0400 => screen_memory: [ReadWrite<u8>; 1000]),
        (0x07e8 => _reserved_after_screen_memory),
        (0x07f8 => sprite_ptr: [ReadWrite<u8>; 8]),
        (0x0800 => _reserved_must_be_zero_for_basic),
        (0x0801 => default_basic_ram: [ReadWrite<u8>; 38911]),
        (0xa000 => basic_rom: [ReadWrite<u8>; 8192]),
        (0xc000 => upper_ram: [ReadWrite<u8>; 4096]),
        (0xd000 => vic2: MOSVideoInterfaceControllerII),
        (0xd400 => sid: MOSSoundInterfaceDevice),
        (0xd41d => _reserved_after_sid),
        (0xd420 => sid_images),
        (0xd800 => color_ram: [ReadWrite<u8>; 1000]),
        (0xdbe8 => _reserved_after_color_ram),
        (0xdc00 => cia1: MOSComplexInterfaceAdapter6526),
        (0xdc10 => cia1_image),
        (0xdd00 => cia2: MOSComplexInterfaceAdapter6526),
        (0xdd10 => cia2_image),
        (0xffff => @END),
    }
}

register_bitfields! [
    u8,
    /// Currently not in use
    CpuPortControl [
        MEMORY_VISIBILITY OFFSET(0) NUMBITS(3) [
            RamEverywhere = 0b000,
            RamAtA000AndE000 = 0b001,
            RamAtA000KernalAtE000 = 0b010,
            BasicAtA000KernalAtE000 = 0b011,
        ],
        DATASETTE_OUTPUT_SIGNAL OFFSET(3) NUMBITS(1) [],
        DATASETTE_BUTTON OFFSET(4) NUMBITS(1) [
            Pressed = 0,
            NotPressed = 1
        ],
        DATASETTE_MOTOR OFFSET(5) NUMBITS(1) [
            On = 0,
            Off = 1
        ],
    ],
];

pub const DEFAULT_SCREEN: *mut u8 = (0x0400) as *mut u8;
pub const DEFAULT_SCREEN_AREA: *mut [u8; 1000] = (0x0400) as *mut [u8; 1000];
//pub const DEFAULT_BASIC_AREA : *mut [u8; 38911] = (0x0801) as *mut [u8; 38911];
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as *mut u8;
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as *mut u8;

/// This is the default location for sprite pointers, i.e.
/// relative to the default screen memory location 0x0400. The
/// sprite pointers can be calculated with `vic2::to_sprite_pointer()`.
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

pub const VIC: *const MOSVideoInterfaceControllerII =
    (0xd000) as *const MOSVideoInterfaceControllerII;

pub const SID: *const MOSSoundInterfaceDevice = (0xd400) as *const MOSSoundInterfaceDevice;

pub const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;

pub const CIA1: *const MOSComplexInterfaceAdapter6526 =
    (0xdc00) as *const MOSComplexInterfaceAdapter6526;
pub const CIA2: *const MOSComplexInterfaceAdapter6526 =
    (0xdd00) as *const MOSComplexInterfaceAdapter6526;

pub const KERNAL_ROM: *mut [u8; 8192] = (0xe000) as *mut [u8; 8192];

bitflags! {
    pub struct CIA1ControlFlags: u8 {
        const START         = 0b00000001;
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
/// the llvm-mos `__interrupt__` attribute is currently not available from Rust.
///
/// Example:
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
