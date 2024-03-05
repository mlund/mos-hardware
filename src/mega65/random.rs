// copyright 2023 mikael lund aka wombat
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

//! Support for pseudo random numbers

use core::ops::Shl;
use rand_core::{Error, RngCore};

extern "C" {
    pub fn mega65_hardware_rand_c() -> u8;
}

/// Read random byte from MEGA65 hardware random number generator
pub fn rand8() -> u8 {
    unsafe { mega65_hardware_rand_c() }
}

/// Non-deterministic pseudo random number generator using MEGA65 hardware
///
/// Implements the [`rand::RngCore`](https://docs.rs/rand/latest/rand/trait.RngCore.html)
/// trait and can thus be used with Rusts `rand` crate.
///
/// ## Examples
/// ~~~
/// use mos_hardware::mega65::random::HardwareRng;
/// use rand::seq::SliceRandom;
/// let mut rng = HardwareRng::default();
/// let value = [11, 23].choose(&mut rng).unwrap(); // 11 or 23
/// ~~~
#[derive(Default)]
pub struct HardwareRng {}

impl RngCore for HardwareRng {
    fn next_u32(&mut self) -> u32 {
        u32::from_ne_bytes([rand8(), rand8(), rand8(), rand8()])
    }
    fn next_u64(&mut self) -> u64 {
        // https://stackoverflow.com/a/2769598
        u64::from(self.next_u32()).shl(32) | u64::from(self.next_u32())
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.iter_mut().for_each(|byte| *byte = rand8());
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
