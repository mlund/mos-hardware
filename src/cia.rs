// copyright 2022 mikael lund aka wombat
// copyright 2025 Sebastien Bechet
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

// === COMMON ==============================

/// A real-time clock is incorporated in the CIA, providing a timekeeping device more
/// conducive to human needs than the microsecond precision of the interval timers.
/// Time is kept in the American 12-hour AM/PM format.
/// The TOD clock consists of four read/write registers: hours (with bit 7 acting as
/// the AM/PM flag), minutes, seconds and tenths of a second. All registers read out in
/// [BCD format](https://en.wikipedia.org/wiki/Binary-coded_decimal), thus simplifying
/// the encoding/decoding process.
#[repr(C, packed)]
pub struct TimeOfDay {
    /// 0x08
    pub tenths: RW<u8>,
    /// 0x09
    pub seconds: RW<u8>,
    /// 0x0a
    pub minutes: RW<u8>,
    /// 0x0b
    pub hours: RW<u8>,
}

const_assert!(size_of::<TimeOfDay>() == 4);

impl TimeOfDay {
    /// Set time with BCD validation
    pub fn validate_bcd_bool(hours: u8, minutes: u8, seconds: u8, tenths: u8) -> bool {
        // Compact BCD validation
        let is_valid_bcd = |val: u8| (val >> 4) <= 9 && (val & 0x0F) <= 9;

        // Single expression validation
        is_valid_bcd(hours)
            && hours >= 0x01
            && hours <= 0x12
            && is_valid_bcd(minutes)
            && minutes <= 0x59
            && is_valid_bcd(seconds)
            && seconds <= 0x59
            && is_valid_bcd(tenths)
            && (tenths & 0x0F) <= 9
    }

    /// Validate that values are in valid BCD format
    pub fn validate_bcd(&self) -> bool {
        let tenths = self.tenths.read();
        let seconds = self.seconds.read();
        let minutes = self.minutes.read();
        let hours = self.get_hour_bcd() & 0b0111_1111; // remove pm

        Self::validate_bcd_bool(hours, minutes, seconds, tenths)
    }

    /// Set time with BCD validation
    pub fn set_time_bcd_bool(
        &mut self,
        hours: u8,
        minutes: u8,
        seconds: u8,
        tenths: u8,
        pm: bool,
    ) -> bool {
        if !Self::validate_bcd_bool(hours, minutes, seconds, tenths) {
            return false;
        }

        let hours = if pm { hours | 0x80 } else { hours };

        unsafe {
            self.tenths.write(tenths);
            self.seconds.write(seconds);
            self.minutes.write(minutes);
            self.hours.write(hours);
        }

        true
    }

    pub fn is_pm(&self) -> bool {
        (self.hours.read() & 0x80) != 0
    }

    /// set AM/PM
    pub fn set_pm(&mut self, pm: bool) {
        let mut hours = self.hours.read();
        if pm {
            hours |= 0x80;
        } else {
            hours &= 0x7F;
        }
        unsafe { self.hours.write(hours) };
    }

    /// get hours without PM flag (BCD format)
    pub fn get_hour_bcd(&self) -> u8 {
        self.hours.read() & 0x7F
    }
}

bitflags! {
    /// CIA Interrupt Control Register (ICR)
    ///
    /// Dual behavior:
    /// - Read: shows active interrupt sources
    /// - Write: configures which IRQ sources to enable/disable
    #[derive(Default)]
    pub struct InterruptControl: u8 {
        /// Bit 7: Set (1) or Clear (0) mode for writing
        const SET_CLEAR        = 0b1000_0000;
        /// Bit 4: FLAG pin interrupt (external input)
        const FLAG             = 0b0001_0000;
        /// Bit 3: Serial Port interrupt
        const SERIAL           = 0b0000_1000;
        /// Bit 2: Timer A underflow interrupt
        const TIMER_A          = 0b0000_0100;
        /// Bit 1: Timer B underflow interrupt
        const TIMER_B          = 0b0000_0010;
        /// Bit 0: IRQ occurred (read only)
        const IRQ              = 0b0000_0001;
        /// Disable all irq
        const DISABLE_ALL      = 0b0111_1111;
    }
}

bitflags! {
    /// Control register for Timer A and Timer B
    pub struct TimerControl: u8 {
        /// Bit 0: START - Start the timer (1) or stop it (0)
        const START            = 0b0000_0001;
        /// Bit 1: PBON - Port B ON - Controls output on Port B bit 6/7
        const PBON             = 0b0000_0010;
        /// Bit 2: OUTMODE - Port B output mode (0=pulse, 1=toggle)
        const OUTMODE          = 0b0000_0100;
        /// Bit 3: RUNMODE - Execution mode (0=continuous, 1=one-shot)
        const RUNMODE          = 0b0000_1000;
        /// Bit 4: LOAD - Force timer reload (strobe)
        const LOAD             = 0b0001_0000;
        /// Bit 5: INMODE - Clock source Timer A (0=PHI2, 1=CNT)
        ///        or for Timer B (0=PHI2, 1=Timer A underflow)
        const INMODE           = 0b0010_0000;
        /// Bit 6: SPMODE - Serial Port mode (Timer A only)
        const SPMODE           = 0b0100_0000;
        /// Bit 7: TODIN - 50/60Hz TOD input (Timer A only)
        const TODIN            = 0b1000_0000;
    }
}

pub const TIMER_OFF: TimerControl = TimerControl::empty();

// === CIA ================================

/// MOSComplexInterfaceAdapter6526 control part
#[repr(C, packed)]
pub struct Mos6526ControlBlock {
    /// 0x04 - Timer A
    pub timer_a: RW<u16>,
    /// 0x06 - Timer B
    pub timer_b: RW<u16>,
    /// 0x08-0x0B - TOD Clock                
    pub time_of_day: TimeOfDay,
    // 0x0C - Serial shift register
    pub serial_shift: RW<u8>,
    // 0x0D - Interrupt control and flags
    pub interrupt: RW<InterruptControl>,
    // 0x0E - Timer A control register
    pub control_a: RW<TimerControl>,
    // 0x0F - Timer B control register
    pub control_b: RW<TimerControl>,
}

const_assert!(size_of::<Mos6526ControlBlock>() == 12);

#[repr(C, packed)]
/// Registers for the MOS Technology Complex Interface Adapter 6526
///
/// The CIA served as an I/O port controller for the 6502 family of microprocessors,
/// providing for parallel and serial I/O capabilities as well as timers and a
/// Time-of-Day (TOD) clock. The device's most prominent use was in the Commodore 64
/// and Commodore 128(D), each of which included two CIA chips.
pub struct MOSComplexInterfaceAdapter6526<
    PortA: Copy,
    PortB: Copy,
    DirectionA: Copy,
    DirectionB: Copy,
> {
    /// 0x00 - Port A I/O
    pub port_a: RW<PortA>,
    /// 0x01 - Port B I/O
    pub port_b: RW<PortB>,
    /// 0x02 - Data Direction Register A
    pub data_direction_port_a: RW<DirectionA>,
    /// 0x03 - Data Direction Register B
    pub data_direction_port_b: RW<DirectionB>,
    /// 0x04... Control Block
    pub control: Mos6526ControlBlock,
}

// === CIA1 ================================

/// Keyboard or Joystick #2
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct CIA1PortA(u8);

impl const From<CIA1PortA> for KeyboardColumn {
    fn from(pa: CIA1PortA) -> Self {
        KeyboardColumn::from_bits_truncate(pa.into())
    }
}

impl const From<CIA1PortA> for GameController {
    fn from(pa: CIA1PortA) -> Self {
        GameController::from_bits_truncate(pa.into())
    }
}

impl const From<KeyboardColumn> for CIA1PortA {
    fn from(col: KeyboardColumn) -> Self {
        Self(col.bits())
    }
}

impl const From<GameController> for CIA1PortA {
    fn from(joy: GameController) -> Self {
        Self(joy.bits())
    }
}

impl const From<u8> for CIA1PortA {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl const From<CIA1PortA> for u8 {
    fn from(cia1: CIA1PortA) -> u8 {
        cia1.0
    }
}

/// Keyboard or Joystick #1
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct CIA1PortB(u8);

impl const From<CIA1PortB> for KeyboardRow {
    fn from(pa: CIA1PortB) -> Self {
        KeyboardRow::from_bits_truncate(pa.into())
    }
}

impl const From<CIA1PortB> for GameController {
    fn from(pa: CIA1PortB) -> Self {
        GameController::from_bits_truncate(pa.into())
    }
}

impl const From<KeyboardColumn> for CIA1PortB {
    fn from(row: KeyboardColumn) -> Self {
        Self(row.bits())
    }
}

impl const From<GameController> for CIA1PortB {
    fn from(joy: GameController) -> Self {
        Self(joy.bits())
    }
}

impl const From<u8> for CIA1PortB {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl const From<CIA1PortB> for u8 {
    fn from(cia1: CIA1PortB) -> u8 {
        cia1.0
    }
}

/// Keyboard or Joystick #2 configuration
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CIA1DirectionA(u8);

impl CIA1DirectionA {
    /// For Column input
    pub const KEYBOARD: Self = Self(0b0000_0000);
    /// For Joystick #2 input
    pub const JOYSTICK: Self = Self(0b0001_1111);
}

impl const Default for CIA1DirectionA {
    fn default() -> Self {
        Self::KEYBOARD
    }
}

impl const From<u8> for CIA1DirectionA {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl const From<CIA1DirectionA> for u8 {
    fn from(cia1: CIA1DirectionA) -> u8 {
        cia1.0
    }
}

/// Keyboard or Joystick #1 configuration
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CIA1DirectionB(u8);

impl CIA1DirectionB {
    /// For Row input
    pub const KEYBOARD: Self = Self(0b1111_1111);
    /// For Joystick #1 input
    pub const JOYSTICK: Self = Self(0b0001_1111);
}

impl const Default for CIA1DirectionB {
    fn default() -> Self {
        Self::KEYBOARD
    }
}

impl const From<u8> for CIA1DirectionB {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl const From<CIA1DirectionB> for u8 {
    fn from(cia1: CIA1DirectionB) -> u8 {
        cia1.0
    }
}

// --- Keyboard ----------------------------

bitflags::bitflags! {
    /// Column selector for CIA1 Port A (write)
    #[derive(Default)]
    pub struct KeyboardColumn: u8 {
        const COL0 = 0b1111_1110;
        const COL1 = 0b1111_1101;
        const COL2 = 0b1111_1011;
        const COL3 = 0b1111_0111;
        const COL4 = 0b1110_1111;
        const COL5 = 0b1101_1111;
        const COL6 = 0b1011_1111;
        const COL7 = 0b0111_1111;
    }
}

bitflags::bitflags! {
    /// Line reader for CIA1 Port B (read)
    #[derive(Default)]
    pub struct KeyboardRow: u8 {
        const ROW0 = 0b0000_0001;
        const ROW1 = 0b0000_0010;
        const ROW2 = 0b0000_0100;
        const ROW3 = 0b0000_1000;
        const ROW4 = 0b0001_0000;
        const ROW5 = 0b0010_0000;
        const ROW6 = 0b0100_0000;
        const ROW7 = 0b1000_0000;
    }
}

// --- Control Port ----------------------------

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

/// Enum for joystick position (complexity: N enum values)
///
/// This is a convenience enum with some overhead in that
/// the status is usually conditionally dermined twice:
///
/// 1. When creating the enum with `new()`
/// 2. When matching the resulting enum in calling code
///
/// It is faster but more low-level to directly probe `GameController`
/// (see example)
impl JoystickPosition {
    pub const fn new(value: GameController) -> JoystickPosition {
        let complement = value.complement(); // complement since bit is UNSET if active
        if complement.contains(GameController::UP_LEFT) {
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
        }
    }
}

bitflags! {
    /// Bit mask for joystick controller (CIA1 port a or b)
    ///
    /// Note that bits are _unset_ when joystick actions are
    /// triggered which is why we use `.complement()`.
    ///
    /// # Examples
    /// ~~~
    /// use cia::GameController::{UP, FIRE};
    /// let joystick = c64::cia1().port_a.read().complement();
    /// if joystick.contains(UP | FIRE) {
    ///     // UP and FIRE pressed simultaneously...
    /// }
    /// ~~~
    pub struct GameController: u8 {
        const UP    = 0b0000_0001; // bit 0
        const DOWN  = 0b0000_0010; // bit 1
        const LEFT  = 0b0000_0100; // bit 2
        const RIGHT = 0b0000_1000; // bit 3
        const FIRE  = 0b0001_0000; // bit 4

        const UP_FIRE    = Self::UP.bits | Self::FIRE.bits;
        const DOWN_FIRE  = Self::DOWN.bits | Self::FIRE.bits;
        const LEFT_FIRE  = Self::LEFT.bits | Self::FIRE.bits;
        const RIGHT_FIRE = Self::RIGHT.bits | Self::FIRE.bits;

        const UP_LEFT = Self::UP.bits | Self::LEFT.bits;
        const UP_RIGHT = Self::UP.bits | Self::RIGHT.bits;
        const DOWN_LEFT = Self::DOWN.bits | Self::LEFT.bits;
        const DOWN_RIGHT = Self::DOWN.bits | Self::RIGHT.bits;

        const UP_LEFT_FIRE = Self::UP.bits | Self::LEFT.bits | Self::FIRE.bits;
        const UP_RIGHT_FIRE = Self::UP.bits | Self::RIGHT.bits | Self::FIRE.bits;
        const DOWN_LEFT_FIRE = Self::DOWN.bits | Self::LEFT.bits | Self::FIRE.bits;
        const DOWN_RIGHT_FIRE = Self::DOWN.bits | Self::RIGHT.bits | Self::FIRE.bits;
    }
}

impl GameController {
    /// Read joystick position and fire button status
    ///
    /// # Examples
    ///
    /// ~~~
    /// let port_a = c64::cia1().port_a.read();
    /// let (position, fire) = port_a.read_joystick();
    /// ~~~
    ///
    /// # Note
    ///
    /// This is a convenience enum with some overhead in that
    /// the status is usually conditionally dermined twice:
    ///
    /// 1. When creating the enum with `JoystickPosition::new()`
    /// 2. When matching the resulting enum in calling code
    ///
    /// It is faster but more low-level to directly probe `GameController`
    /// using bitflags.
    pub const fn read_joystick(&self) -> (JoystickPosition, bool) {
        let position = JoystickPosition::new(*self);
        let fire = self.complement().contains(Self::FIRE);
        (position, fire)
    }
}

// === CIA2 ================================

bitflags! {
    /// CIA2 Port A - Serial Bus Access and VIC Bank Control Register
    ///
    /// This register controls the serial bus (IEC bus) and VIC-II bank switching.
    /// Each bit can be configured as input or output via the corresponding
    /// Data Direction Register (DDRA).
    ///
    /// Serial bus logic:
    /// - Output bits: 0=High/Inactive, 1=Low/Active
    /// - Input bits: 0=Low/Active, 1=High/Inactive
    ///
    /// VIC Bank switching:
    /// - Bits 0-1 control VIC-II memory bank (inverted logic)
    /// - 00 = Bank 3 ($C000-$FFFF), 01 = Bank 2 ($8000-$BFFF)
    /// - 10 = Bank 1 ($4000-$7FFF), 11 = Bank 0 ($0000-$3FFF)
    #[repr(transparent)]
    pub struct CIA2PortA: u8 {
        /// Bit 7: DATA IN - Serial bus data input line
        const DATA_IN   = 0b1000_0000;
        /// Bit 6: CLOCK IN - Serial bus clock input line
        const CLOCK_IN  = 0b0100_0000;
        /// Bit 5: DATA OUT - Serial bus data output line
        const DATA_OUT  = 0b0010_0000;
        /// Bit 4: CLOCK OUT - Serial bus clock output line
        const CLOCK_OUT = 0b0001_0000;
        /// Bit 3: ATN OUT - Attention output line
        const ATN_OUT   = 0b0000_1000;

        /// I/O bit for GPIO pin
        /// Bit 2: PA2 - User Port pin M, general-purpose I/O
        const PA2       = 0b0000_0100;
        // I/O bit for RS-232 pins
        /// Bit 2: TXD OUT - RS-232 transmit data output (pin M)
        /// UART transmit data line for RS-232 interface
        const TXD_OUT  = 0b0000_0100;

        /// Bit 1: VA15 - VIC-II address line 15 (inverted)
        const VA15      = 0b0000_0010;
        /// Bit 0: VA14 - VIC-II address line 14 (inverted)
        const VA14      = 0b0000_0001;

        /// VIC Bank helpers (note: inverted logic)
        /// $0000-$3FFF
        const VIC_BANK_0 = 0b0000_0011;
        /// $4000-$7FFF
        const VIC_BANK_1 = 0b0000_0010;
        /// $8000-$BFFF
        const VIC_BANK_2 = 0b0000_0001;
        /// $C000-$FFFF
        const VIC_BANK_3 = 0b0000_0000;
    }
}

impl CIA2PortA {
    /// WARN: Configure bits 0-1 as input for VIC bank control (see CIA2::data_direction_port_a)
    pub const fn get_vic_bank(self) -> u8 {
        self.bits() & 0b0000_0011
    }

    /// WARN: Configure bits 0-1 as outputs for VIC bank control (see CIA2::data_direction_port_a)
    pub fn set_vic_bank(&mut self, bank: u8) {
        let masked_self = self.bits() & 0b1111_1100;
        let masked_bank = bank & 0b0000_0011;
        let new_bits = masked_self | masked_bank;
        *self = Self::from_bits_truncate(new_bits);
    }
}

impl const Default for CIA2PortA {
    fn default() -> Self {
        Self::DATA_OUT | Self::CLOCK_OUT | Self::ATN_OUT | Self::PA2 | Self::VIC_BANK_0
    }
}

impl const From<u8> for CIA2PortA {
    fn from(value: u8) -> Self {
        Self::from_bits(value).unwrap()
    }
}

impl const From<CIA2PortA> for u8 {
    fn from(cia2: CIA2PortA) -> u8 {
        cia2.bits()
    }
}

/// CIA2 Port B - User Port or RS-232
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct CIA2PortB(u8);

impl const From<CIA2PortB> for GPIOPins {
    fn from(pb: CIA2PortB) -> Self {
        GPIOPins::from_bits_truncate(pb.into())
    }
}

impl const From<CIA2PortB> for RS232Access {
    fn from(pb: CIA2PortB) -> Self {
        RS232Access::from_bits_truncate(pb.into())
    }
}

impl const From<GPIOPins> for CIA2PortB {
    fn from(gpio: GPIOPins) -> Self {
        Self(gpio.bits())
    }
}

impl const From<RS232Access> for CIA2PortB {
    fn from(rs232: RS232Access) -> Self {
        Self(rs232.bits())
    }
}

impl const From<u8> for CIA2PortB {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl const From<CIA2PortB> for u8 {
    fn from(cia2: CIA2PortB) -> u8 {
        cia2.0
    }
}

bitflags! {
    /// CIA2 Port B - User Port
    ///
    /// Bus logic:
    /// - Output bits: 0=High/Inactive, 1=Low/Active
    /// - Input bits: 0=Low/Active, 1=High/Inactive
    ///
    #[repr(transparent)]
    #[derive(Default)]
    pub struct GPIOPins: u8 {
        /// Bit 7: PB7 - User Port pin L, general-purpose I/O
        const PIN_L = 0b1000_0000;
        /// Bit 6: PB6 - User Port pin K, general-purpose I/O
        const PIN_K = 0b0100_0000;
        /// Bit 5: PB5 - User Port pin J, general-purpose I/O
        const PIN_J = 0b0010_0000;
        /// Bit 4: PB4 - User Port pin H, general-purpose I/O
        const PIN_H = 0b0001_0000;
        /// Bit 3: PB3 - User Port pin F, general-purpose I/O
        const PIN_F = 0b0000_1000;
        /// Bit 2: PB2 - User Port pin E, general-purpose I/O
        const PIN_E = 0b0000_0100;
        /// Bit 1: PB1 - User Port pin D, general-purpose I/O
        const PIN_D = 0b0000_0010;
        /// Bit 0: PB0 - User Port pin C, general-purpose I/O
        const PIN_C = 0b0000_0001;

    }
}

bitflags! {
    /// CIA2 Port B - RS-232 pins
    #[repr(transparent)]
    pub struct RS232Access: u8 {
        /// 0 RXD I
        const RXD  = 0b0000_0001;
        /// 1 RTS O
        const RTS  = 0b0000_0010;
        /// 2 DTR O
        const DTR  = 0b0000_0100;
        /// 3 RI IO
        const RI   = 0b0000_1000;
        /// 4 DCD IO
        const DCD  = 0b0001_0000;
        /// 5 User Port Pin J IO
        const UP_J = 0b0010_0000;
        /// 6 CTS R
        const CTS  = 0b0100_0000;
        /// 7 DSR R
        const DSR  = 0b1000_0000;
    }
}

bitflags::bitflags! {
    /// CIA2 Data Direction A - Serial Bus configuration
    ///
    /// Controls the direction of each bit in CIA2 Port A:
    /// - 0 = Input (read from external signal)
    /// - 1 = Output (write to external signal)
    ///
    /// This register configures which pins can be read from vs written to.
    /// Serial bus lines marked as "IN" should be inputs, "OUT" should be outputs.
    #[repr(transparent)]
    pub struct CIA2DirectionA: u8 {
        /// Bit 7: DATA IN direction (0=Input, 1=Output)
        /// Should be INPUT to read serial bus data
        const DATA_IN   = 0b1000_0000;
        /// Bit 6: CLOCK IN direction (0=Input, 1=Output)
        /// Should be INPUT to read serial bus clock
        const CLK_IN   = 0b0100_0000;
        /// Bit 5: DATA OUT direction (0=Input, 1=Output)
        /// Should be OUTPUT to control serial bus data line
        const DATA_OUT  = 0b0010_0000;
        /// Bit 4: CLOCK OUT direction (0=Input, 1=Output)
        /// Should be OUTPUT to control serial bus clock line
        const CLK_OUT  = 0b0001_0000;
        /// Bit 3: ATN OUT direction (0=Input, 1=Output)
        /// Should be OUTPUT to control attention line
        const ATN_OUT  = 0b0000_1000;
        /// Bit 2: PA2 - User Port pin M, general-purpose I/O
        const PA2_OUT  = 0b0000_0100;
        /// Bit 1: VA15 direction (0=Input, 1=Output)
        /// Should be OUTPUT to control VIC bank switching
        const VA15     = 0b0000_0010;
        /// Bit 0: VA14 direction (0=Input, 1=Output)
        /// Should be OUTPUT to control VIC bank switching
        const VA14     = 0b0000_0001;

        /// Standard configuration: Serial IN/OUT + VA14/15 OUT
        /// Matches Kernal initialization: %00111111
        /// - DATA IN/CLOCK IN as inputs (bits 7,6 = 0)
        /// - All output signals as outputs (bits 5,4,3,2,1,0 = 1)
        const SERIAL_INOUT_VA_OUT = 0b0011_1111;
        const DEFAULT = 0b0011_1111;
    }
}

impl const Default for CIA2DirectionA {
    /// Standard Kernal configuration: $3F
    /// DATA/CLOCK IN as inputs, everything else as outputs
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl const From<u8> for CIA2DirectionA {
    fn from(value: u8) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl const From<CIA2DirectionA> for u8 {
    fn from(cia2: CIA2DirectionA) -> u8 {
        cia2.bits()
    }
}

/// CIA2 Data Direction B - User Port ou RS-232 direction
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CIA2DirectionB(u8);

impl CIA2DirectionB {
    pub const fn default_as_user_port() -> Self {
        Self((GPIOPinsDir::DEFAULT).bits())
    }

    pub const fn default_as_rs232() -> Self {
        Self((RS232AccessDir::DEFAULT).bits())
    }
}

impl const From<GPIOPinsDir> for CIA2DirectionB {
    fn from(gpio: GPIOPinsDir) -> Self {
        Self(gpio.bits())
    }
}

impl const From<RS232AccessDir> for CIA2DirectionB {
    fn from(rs232: RS232AccessDir) -> Self {
        Self(rs232.bits())
    }
}

impl const From<u8> for CIA2DirectionB {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl const From<CIA2DirectionB> for u8 {
    fn from(cia2: CIA2DirectionB) -> u8 {
        cia2.0
    }
}

bitflags! {
    /// CIA2 Data Direction Register B (DDRB)
    ///
    /// Controls the direction of each bit in CIA2 Port B:
    /// - 0 = Input (read from external signal)
    /// - 1 = Output (write to external signal)
    ///
    /// This register configures which pins can be read from vs written to.
    /// Serial bus lines marked as "IN" should be inputs, "OUT" should be outputs.
    pub struct GPIOPinsDir: u8 {
        /// Bit 7: PB7 - User Port pin L, general-purpose I/O
        const PIN_L = 0b1000_0000;
        /// Bit 6: PB6 - User Port pin K, general-purpose I/O
        const PIN_K = 0b0100_0000;
        /// Bit 5: PB5 - User Port pin J, general-purpose I/O
        const PIN_J = 0b0010_0000;
        /// Bit 4: PB4 - User Port pin H, general-purpose I/O
        const PIN_H = 0b0001_0000;
        /// Bit 3: PB3 - User Port pin F, general-purpose I/O
        const PIN_F = 0b0000_1000;
        /// Bit 2: PB2 - User Port pin E, general-purpose I/O
        const PIN_E = 0b0000_0100;
        /// Bit 1: PB1 - User Port pin D, general-purpose I/O
        const PIN_D = 0b0000_0010;
        /// Bit 0: PB0 - User Port pin C, general-purpose I/O
        const PIN_C = 0b0000_0001;

        const DEFAULT   = 0b0000_0000;
    }
}

bitflags! {
    /// RS-232 pins
    pub struct RS232AccessDir: u8 {
        /// Bit 0: Receive Data (pin C)
        const RXD  = 0b0000_0001;
        /// Bit 1: Request To Send (pin D)
        const RTS  = 0b0000_0010;
        /// Bit 2: Data Terminal Ready (pin E)
        const DTR  = 0b0000_0100;
        /// Bit 3: Ring Indicator (pin F)
        const RI   = 0b0000_1000;
        /// Bit 4: Data Carrier Detect (pin H)
        const DCD  = 0b0001_0000;
        /// Bit 5: User Port (pin J)
        const UP_J = 0b0010_0000;
        /// Bit 6: Clear To Send (pin K)
        const CTS  = 0b0100_0000;
        /// Bit 7: Data Set Ready (pin L)
        const DSR  = 0b1000_0000;

        const DEFAULT = 0b0000_0110;
    }
}

impl const Default for RS232AccessDir {
    fn default() -> Self {
        Self::DEFAULT
    }
}
