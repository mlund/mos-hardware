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
