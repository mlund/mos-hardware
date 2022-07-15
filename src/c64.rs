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
use crate::vic2::*;
use crate::sid::*;
use crate::cia::*;

pub const DEFAULT_SCREEN: *mut u8 = (0x0400) as *mut u8;
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as *mut u8;
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as *mut u8;

pub const VIC: *const MOSVideoInterfaceControllerII =
    (0xd000) as *const MOSVideoInterfaceControllerII;

pub const SID: *const MOSSoundInterfaceDevice = (0xd400) as *const MOSSoundInterfaceDevice;

pub const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;

pub const CIA1: *const MOSComplexInterfaceAdapter6526 = (0xdc00) as *const MOSComplexInterfaceAdapter6526;
pub const CIA2: *const MOSComplexInterfaceAdapter6526 = (0xdd00) as *const MOSComplexInterfaceAdapter6526;

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

