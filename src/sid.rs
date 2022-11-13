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

//! Registers for the MOS Technology 6581/8580 SID (Sound Interface Device)
//!
//! SID is the built-in programmable sound generator chip of Commodore's CBM-II,
//! Commodore 64, Commodore 128 and Commodore MAX Machine home computers.
//! It was one of the first sound chips of its kind to be included in a home computer.

use crate::*;
use bitflags::bitflags;
use core::mem::size_of;
use static_assertions::const_assert;
use volatile_register::{RO, WO};

bitflags! {
    /// Control flags for the `Voice::control` register
    pub struct VoiceControlFlags: u8 {
        const GATE     = 0b0000_0001; // bit 0
        /// Synch fundamental frequency of oscillator with fundamental frequency of neighboring voice
        const SYNC     = 0b0000_0010; // bit 1
        /// Set to replace triangle waveform w. ring modulation from neighbor voices
        const RING_MODULATION = 0b0000_0100; // bit 2
        /// Set to disable oscillations
        const TEST     = 0b0000_1000; // bit 3
        const TRIANGLE = 0b0001_0000; // bit 4
        const SAWTOOTH = 0b0010_0000; // bit 5
        const PULSE    = 0b0100_0000; // bit 6
        const NOISE    = 0b1000_0000; // bit 7
    }
}

#[repr(C, packed)]
/// Registers for a single SID voice/channel
pub struct Voice {
    /// `FRELO`/`FRELO` Frequency control (0x00-0x01)
    pub frequency: WO<u16>,
    /// `PWLO`/`PWHI` Pulse waveform width (0x02-0x03)
    pub pulse_width: WO<u16>,
    /// `VCREG` Control register (0x04)
    pub control: WO<VoiceControlFlags>,
    /// `ATDCY` Attack/decay cycle duration (0x05)
    pub attack_decay: WO<u8>,
    /// `SUREL` Sustain/Release Control (0x06)
    pub sustain_release: WO<u8>,
}

/// Attack times for `ATDCY`, bits 4-7 (milliseconds, `Ms`)
pub enum AttackTime {
    Ms2 = 0,
    Ms8 = 1,
    Ms16 = 2,
    Ms24 = 3,
    Ms38 = 4,
    Ms56 = 5,
    Ms68 = 6,
    Ms80 = 7,
    Ms100 = 8,
    Ms250 = 9,
    Ms500 = 10,
    Ms800 = 11,
    Ms1000 = 12,
    Ms3000 = 13,
    Ms5000 = 14,
    Ms8000 = 15,
}

/// Sustain times for `ATDCY`, bits 0-3 (milliseconds, `Ms`)
pub enum DecayTime {
    Ms6 = 0,
    Ms24 = 1,
    Ms48 = 2,
    Ms72 = 3,
    Ms114 = 4,
    Ms168 = 5,
    Ms204 = 6,
    Ms240 = 7,
    Ms300 = 8,
    Ms750 = 9,
    Ms1500 = 10,
    Ms2400 = 11,
    Ms3000 = 12,
    Ms9000 = 13,
    Ms15000 = 14,
    Ms24000 = 15,
}

/// Combines attack and decay times for register `ATDCY`
///
/// ## Example:
/// ~~~
/// const TIME: u8 = combine_attack_decay(AttackTime::Ms38, DecayTime::Ms240);
/// ~~~
pub const fn combine_attack_decay(attack_time: AttackTime, decay_time: DecayTime) -> u8 {
    (attack_time as u8 * 16) + (decay_time as u8)
}

impl Voice {
    /// Sets the attack/decay cycle duration (`ATDCY`)
    ///
    /// See e.g. Mapping the C64, page 162.
    ///
    /// ## Example:
    /// ~~~
    /// (*c64::SID).channel1.set_attack_decay(AttackTime::Ms38, DecayTime::Ms240);
    /// ~~~
    pub fn set_attack_decay(&self, attack_time: AttackTime, decay_time: DecayTime) {
        let value = combine_attack_decay(attack_time, decay_time);
        unsafe {
            self.attack_decay.write(value);
        }
    }
}

const_assert!(size_of::<Voice>() == 7);

#[repr(C, packed)]
/// MOS Technology Sound Interface Device (SID)
pub struct MOSSoundInterfaceDevice {
    pub channel1: Voice,
    pub channel2: Voice,
    pub channel3: Voice,
    pub filter_cutoff: WO<u16>, // 0x15
    /// `RESON` Filter resonance control (0x17)
    pub resonance_and_filter_setup: WO<u8>,
    /// `SIGVOL` Volume and filter select (0x18)
    pub volume_filter_mode: WO<u8>,
    pub potentiometer_x: RO<u8>,     // 0x19
    pub potentiometer_y: RO<u8>,     // 0x1a
    pub channel3_oscillator: RO<u8>, // 0x1b
    pub channel3_envelope: RO<u8>,   // 0x1c
}

const_assert!(size_of::<MOSSoundInterfaceDevice>() == 0x1d);

impl MOSSoundInterfaceDevice {
    /// Start noise generation on SID channel 3.
    ///
    /// Example:
    /// ```
    /// (*c64::SID).start_random_generator();
    /// let random_byte = rand8!(*c64::SID);
    /// ```
    /// More information [here](https://www.atarimagazines.com/compute/issue72/random_numbers.php).
    pub fn start_random_generator(&self) {
        unsafe {
            self.channel3.frequency.write(u16::MAX);
            self.channel3.control.write(VoiceControlFlags::NOISE);
        }
    }

    /// Random byte in the interval (0, max_value)
    pub fn rand8(&self, max_value: u8) -> u8 {
        loop {
            let r = self.channel3_oscillator.read();
            if r <= max_value {
                return r;
            }
        }
    }

    /// Random byte in the interval (0, 255)
    ///
    /// /// Example:
    /// ~~~
    /// (*c64::SID).start_random_generator();
    /// let value = (*c64::SID).random_byte();
    /// ~~~
    /// More information [here](https://www.atarimagazines.com/compute/issue72/random_numbers.php).
    pub fn random_byte(&self) -> u8 {
        self.channel3_oscillator.read()
    }

    /// Random word in the interval (0, max_value)
    pub fn rand16(&self, max_value: u16) -> u16 {
        loop {
            let r = ((self.channel3_oscillator.read() as u16) << 8)
                | (self.channel3_oscillator.read() as u16);
            if r <= max_value {
                return r;
            }
        }
    }
}
