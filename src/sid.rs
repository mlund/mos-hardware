use bitflags::bitflags;
use core::mem::size_of;
use volatile_register::{RO, WO};
use static_assertions::const_assert;

bitflags! {
    pub struct VoiceControlFlags: u8 {
        const GATE     = 0b0000_0001;
        const SYNC     = 0b0000_0010;
        const RING_MODULATION = 0b0000_0100;
        const TEST     = 0b0000_1000;
        const TRIANGLE = 0b0001_0000;
        const SAWTOOTH = 0b0010_0000;
        const PULSE    = 0b0100_0000;
        const NOISE    = 0b1000_0000;
    }
}

#[repr(C, packed)]
/// Registers for a single SID voice/channel
pub struct Voice {
    pub frequency: WO<u16>,             // 0x00
    pub pulse_width: WO<u16>,           // 0x02
    pub control: WO<VoiceControlFlags>, // 0x04
    pub attack_decay: WO<u8>,           // 0x05
    pub sustain_release: WO<u8>,        // 0x06
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
    /**
     * Start noise generation on channel 3. For more information, see
     * https://www.atarimagazines.com/compute/issue72/random_numbers.php
     */
    pub unsafe fn start_random_generator(&self) {
        self.channel3.frequency.write(0xffff);
        self.channel3.control.write(VoiceControlFlags::NOISE);
    }
}

/**
 * Use SID entropy to generate random numbers in the interval [0:255].
 * Requires an initial call to `start_random_generator()`.
 */
#[macro_export]
macro_rules! rand8 {
    ($sid_pointer:expr) => {{
        (*$sid_pointer).channel3_oscillator.read()
    }};
}

