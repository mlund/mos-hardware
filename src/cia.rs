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

use bitflags::bitflags;
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
    pub port_a: RW<GameController>,    // 0x00
    pub port_b: RW<GameController>,    // 0x01
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

/// Enum for joystick positions
pub enum JoystickPosition {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
    Middle,
}

bitflags! {
    /// Bit mask for joystick controller (CIA1 port 1 or 2)
    /// 
    /// # Examples
    /// ~~~
    /// let val = c64::cia1().port_a.read().complement(); // bits *unset* if pressed
    /// if val.contains(GameController::FIRE | GameController::UP) {
    ///     // joystick 2 up and fire pressed...
    /// }
    /// ~~~
    pub struct GameController: u8 {
        const UP    = 0b0000_0001; // bit 0
        const DOWN  = 0b0000_0010; // bit 1
        const LEFT  = 0b0000_0100; // bit 2
        const RIGHT = 0b0000_1000; // bit 3
        const FIRE  = 0b0001_0000; // bit 4
        const UP_LEFT = Self::UP.bits | Self::LEFT.bits;
        const UP_RIGHT = Self::UP.bits | Self::RIGHT.bits;
        const DOWN_LEFT = Self::DOWN.bits | Self::LEFT.bits;
        const DOWN_RIGHT = Self::DOWN.bits | Self::RIGHT.bits;
    }
}

impl GameController {
    /// Read joystick position and fire button status
    /// 
    /// # Examples
    /// ~~~
    /// let port_a = c64::cia1().port_a.read();
    /// let (position, fire) = port_a.read_joystick();
    /// ~~~
    pub fn read_joystick(&self) -> (JoystickPosition, bool) {
        let complement = self.complement(); // bit is unset if activated
        let position = if complement.contains(GameController::UP_LEFT) {
            JoystickPosition::UpLeft
        } else if complement.contains(GameController::UP_RIGHT) {
            JoystickPosition::UpRight
        } else if complement.contains(GameController::DOWN_LEFT) {
            JoystickPosition::DownLeft
        } else if complement.contains(GameController::DOWN_RIGHT) {
            JoystickPosition::DownRight
        } else if complement.contains(GameController::UP) {
            JoystickPosition::Up
        } else if complement.contains(GameController::DOWN) {
            JoystickPosition::Down
        } else if complement.contains(GameController::LEFT) {
            JoystickPosition::Left
        } else if complement.contains(GameController::RIGHT) {
            JoystickPosition::Right
        } else {
            JoystickPosition::Middle
        };
        let fire = complement.contains(GameController::FIRE);
        (position, fire)
    }
}