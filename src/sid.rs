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

use bitflags::bitflags;
use core::mem::size_of;
use rand_core::{Error, RngCore};
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
    /// Reset like Kernal: turn off SID
    pub fn reset(&self) {
        unsafe {
            self.volume_filter_mode.write(0);
        }
    }

    /// Start noise generation on SID channel 3.
    ///
    /// Example:
    /// ```
    /// c64::sid().start_random_generator();
    /// let random_byte = c64::sid().rand8(20);
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
    /// # Examples
    /// ~~~
    /// c64::sid().start_random_generator();
    /// let value = c64::sid().random_byte();
    /// ~~~
    /// More information [here](https://www.atarimagazines.com/compute/issue72/random_numbers.php).
    /// Currently there's no way to select the subsong as this requires that the
    /// accumulator is set. Possibly this can be done wrapping function pointers to raw
    /// assembler code.
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

/// Random number generator using the SID oscillator
///
/// Implements the [`rand::RngCore`](https://docs.rs/rand/latest/rand/trait.RngCore.html)
/// trait and can thus be used with Rusts `rand` crate.
/// For single random bytes, it is likely more efficient to use `random_byte()`
/// from the SID chip directly, as the smallest integer implemented in `RngCore` is `u32`,
/// i.e. four random bytes.
///
/// ## Examples
/// ~~~
/// use mos_hardware::{c64, sid};
/// use rand::seq::SliceRandom;
/// let mut rng = sid::SIDRng::new(c64::sid());
/// let value = [11, 23].choose(&mut rng).unwrap(); // 11 or 23
/// ~~~
#[derive(Clone)]
pub struct SIDRng {
    sid: &'static MOSSoundInterfaceDevice,
}

impl SIDRng {
    /// Initialize and start SID oscillator
    pub fn new(sid_address: &'static MOSSoundInterfaceDevice) -> Self {
        sid_address.start_random_generator();
        Self { sid: sid_address }
    }
}

impl RngCore for SIDRng {
    fn next_u32(&mut self) -> u32 {
        u32::from_ne_bytes([
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
        ])
    }

    fn next_u64(&mut self) -> u64 {
        u64::from_ne_bytes([
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
            self.sid.random_byte(),
        ])
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.iter_mut()
            .for_each(|byte| *byte = self.sid.random_byte());
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// Trait for loading and parsing a PSID file at compile time
///
/// The PSID file format is described
/// [here](https://gist.github.com/cbmeeks/2b107f0a8d36fc461ebb056e94b2f4d6).
/// Since arrays in rust cannot be larger than `isize`, songs larger than
/// 32 kb cannot be loaded.
///
/// # Examples
/// ~~~
/// use mos_hardware::sid::SidTune;
/// struct Music;
/// impl SidTune for Music {
///     const BYTES: &'static [u8] = core::include_bytes!("last_hero.sid");
/// }
/// let music = Music;
/// unsafe  {
///     music.to_memory(); // copy data to found load address (danger!)
/// }
/// music.init(0);     // call song initialisation routine
/// music.play();      // call this at every frame
/// ~~~
pub trait SidTune {
    /// Full SID file as const byte array. Typically you would set
    /// this with `core::include_bytes!`.
    const BYTES: &'static [u8];

    /// True if data has an optional 2-byte header stating the load address (C64 style)
    const HAS_BASIC_LOAD_ADDRESS: bool = matches!(
        u16::from_be_bytes([Self::BYTES[0x08], Self::BYTES[0x09]]),
        0
    );

    /// Offset where data begins, excluding any optional 2-byte load address
    const DATA_OFFSET: usize = match Self::HAS_BASIC_LOAD_ADDRESS {
        true => u16::from_be_bytes([Self::BYTES[0x06], Self::BYTES[0x07]]) as usize + 2,
        false => u16::from_be_bytes([Self::BYTES[0x06], Self::BYTES[0x07]]) as usize,
    };

    /// Length of data part (exludes the optional 2-byte load address)
    const DATA_LEN: usize = Self::BYTES.len() - Self::DATA_OFFSET;

    /// Address of init routine
    const INIT_ADDRESS: u16 = u16::from_be_bytes([Self::BYTES[0x0a], Self::BYTES[0x0b]]);

    /// Function pointer to init routine
    const INIT_PTR: *const unsafe extern "C" fn() -> () =
        &Self::INIT_ADDRESS as *const u16 as *const unsafe extern "C" fn() -> ();

    /// Address of play routine
    const PLAY_ADDRESS: u16 = u16::from_be_bytes([Self::BYTES[0x0c], Self::BYTES[0x0d]]);

    /// Function pointer to play routine
    const PLAY_PTR: *const unsafe extern "C" fn() -> () =
        &Self::PLAY_ADDRESS as *const u16 as *const unsafe extern "C" fn() -> ();

    /// Number of subsongs
    const NUM_SONGS: usize = u16::from_be_bytes([Self::BYTES[0x0e], Self::BYTES[0x0f]]) as usize;

    /// Load address found either in PSID header or in data part
    const LOAD_ADDRESS: u16 = match Self::HAS_BASIC_LOAD_ADDRESS {
        true => u16::from_le_bytes([
            Self::BYTES[Self::DATA_OFFSET - 2],
            Self::BYTES[Self::DATA_OFFSET - 1],
        ]),
        false => u16::from_be_bytes([Self::BYTES[0x08], Self::BYTES[0x09]]),
    };

    fn num_songs(&self) -> usize {
        Self::NUM_SONGS
    }

    /// Call song initialisation routine
    ///
    /// Before calling the init routine found in the the PSID file, the
    /// accumulator (A) is set to the `song` number. This is done by placing
    /// 6502 wrapper code at the end of the SID file.
    ///
    /// ## Todo
    ///
    /// It would be nice to let the compiler decide where to place the
    /// wrapper code (`address`), but so far no luck.
    fn init(&self, song: u8) {
        let [high, low] = Self::INIT_ADDRESS.to_be_bytes();
        let address = Self::LOAD_ADDRESS as usize + Self::DATA_LEN;
        let init_fn = &address as *const usize as *const unsafe extern "C" fn() -> ();
        unsafe {
            // 0xa9 = lda; 0x4c = jmp
            *(address as *mut [u8; 5]) = [0xa9, song, 0x4c, low, high];
            (*init_fn)();
        }
    }

    /// Call song play routine
    fn play(&self) {
        unsafe { (*(Self::PLAY_PTR))() }
    }

    /// Copies data into memory at load address specified in PSID file.
    ///
    /// # Safety
    /// Unsafe, as this will perform copy into hard-coded
    /// memory pool that may clash with stack or allocated heap memory.
    unsafe fn to_memory(&self)
    where
        [(); Self::DATA_LEN]:,
    {
        let dst = Self::LOAD_ADDRESS as *mut [u8; Self::DATA_LEN];
        *dst = Self::BYTES[Self::DATA_OFFSET..Self::BYTES.len()]
            .try_into()
            .unwrap();
    }
}
