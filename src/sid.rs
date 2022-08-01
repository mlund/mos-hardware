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
//! Commodore 64,[1] Commodore 128 and Commodore MAX Machine home computers.
//! It was one of the first sound chips of its kind to be included in a home computer
//! prior to the digital sound revolution.

use crate::*;
use bitflags::bitflags;
use core::mem::size_of;
use static_assertions::const_assert;
use volatile_register::{RO, WO};

bitflags! {
    pub struct VoiceControl: u8 {
        const GATE     = 0b00000001;
        const SYNC     = 0b00000010;
        const RING_MODULATION = 0b00000100;
        const TEST     = 0b00001000;
        const TRIANGLE = 0b00010000;
        const SAWTOOTH = 0b00100000;
        const PULSE    = 0b01000000;
        const NOISE    = 0b10000000;
    }
}

#[repr(C, packed)]
/// Registers for a single SID voice/channel
pub struct Voice {
    pub frequency: WO<u16>,        // 0x00
    pub pulse_width: WO<u16>,      // 0x02
    pub control: WO<VoiceControl>, // 0x04
    pub attack_decay: WO<u8>,      // 0x05
    pub sustain_release: WO<u8>,   // 0x06
}

const_assert!(size_of::<Voice>() == 7);

#[repr(C, packed)]
/// MOS Technology Sound Interface Device (SID)
pub struct MOSSoundInterfaceDevice {
    pub channel1: Voice,
    pub channel2: Voice,
    pub channel3: Voice,
    pub filter_cutoff: WO<u16>,             // 0x15
    pub resonance_and_filter_setup: WO<u8>, // 0x17
    pub volume_filter_mode: WO<u8>,         // 0x18
    pub potentiometer_x: RO<u8>,            // 0x19
    pub potentiometer_y: RO<u8>,            // 0x1a
    pub channel3_oscillator: RO<u8>,        // 0x1b
    pub channel3_envelope: RO<u8>,          // 0x1c
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
            self.channel3.control.write(VoiceControl::NOISE);
        }
    }

    /// Random byte in the interval [0:max_value]
    pub fn rand8(&self, max_value: u8) -> u8 {
        loop {
            let r = self.channel3_oscillator.read();
            if r <= max_value {
                return r;
            }
        }
    }

    /// Random word in the interval [0:max_value]
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

/// Use SID entropy to generate a random byte in the interval.
///
/// Example:
/// ```
/// (*c64::SID).start_random_generator();
/// let random_byte = rand8!(*c64::SID);
/// ```
///
/// More information [here](https://www.atarimagazines.com/compute/issue72/random_numbers.php).
#[macro_export]
macro_rules! rand8 {
    ($sid_pointer:expr) => {{
        (*$sid_pointer).channel3_oscillator.read()
    }};
}
