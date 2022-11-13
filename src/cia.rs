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

//! Registers for the MOS Technology 6526/8520 Complex Interface Adapter (CIA)
//!
//! The CIA served as an I/O port controller for the 6502 family of microprocessors,
//! providing for parallel and serial I/O capabilities as well as timers and a
//! Time-of-Day (TOD) clock. The device's most prominent use was in the Commodore 64
//! and Commodore 128(D), each of which included two CIA chips.

use core::mem::size_of;
use static_assertions::const_assert;
use volatile_register::RW;

/// A real-time clock is incorporated in the CIA, providing a timekeeping device more
/// conducive to human needs than the microsecond precision of the interval timers.
/// Time is kept in the American 12-hour AM/PM format.
/// The TOD clock consists of four read/write registers: hours (with bit 7 acting as
/// the AM/PM flag), minutes, seconds and tenths of a second. All registers read out in
/// [BCD format](https://en.wikipedia.org/wiki/Binary-coded_decimal), thus simplifying
/// the encoding/decoding process.
#[repr(C, packed)]
pub struct TimeOfDay {
    pub deci_seconds: RW<u8>, // 0x08
    pub seconds: RW<u8>,      // 0x09
    pub minutes: RW<u8>,      // 0x0a
    pub hours: RW<u8>,        // 0x0b
}

const_assert!(size_of::<TimeOfDay>() == 4);

#[repr(C, packed)]
/// Registers for the MOS Tehnology Complex Interface Adapter 6526
///
/// The CIA served as an I/O port controller for the 6502 family of microprocessors,
/// providing for parallel and serial I/O capabilities as well as timers and a
/// Time-of-Day (TOD) clock. The device's most prominent use was in the Commodore 64
/// and Commodore 128(D), each of which included two CIA chips.
pub struct MOSComplexInterfaceAdapter6526 {
    pub port_a: RW<u8>,                // 0x00
    pub port_b: RW<u8>,                // 0x01
    pub data_direction_port_a: RW<u8>, // 0x02
    pub data_direction_port_b: RW<u8>, // 0x03
    pub timer_a: RW<u16>,              // 0x04
    pub timer_b: RW<u16>,              // 0x06
    pub time_of_day: TimeOfDay,        // 0x08
    pub serial_shift: RW<u8>,          // 0x0c
    pub interrupt: RW<u8>,             // 0x0d
    pub control_a: RW<u8>,             // 0x0e
    pub control_b: RW<u8>,             // 0x0f
}

const_assert!(size_of::<MOSComplexInterfaceAdapter6526>() == 16);
