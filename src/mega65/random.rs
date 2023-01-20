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

use super::libc;
use core::ops::Shl;
use rand_core::{Error, RngCore, SeedableRng};

/// Generate random byte using MEGA65 libc
pub fn rand8(max_value: u8) -> u8 {
    unsafe { libc::rand8(max_value) }
}

/// Non-deterministic random number generator using MEGA65 Libc
///
/// Implements the [`rand::RngCore`](https://docs.rs/rand/latest/rand/trait.RngCore.html)
/// trait and can thus be used with Rusts `rand` crate.
///
/// ## Examples
/// ~~~
/// use mos_hardware::mega65::random;
/// use rand::seq::SliceRandom;
/// let mut rng = LibcRng::default();
/// let value = [11, 23].choose(&mut rng).unwrap(); // 11 or 23
/// ~~~
#[derive(Default)]
pub struct LibcRng {}

impl LibcRng {
    /// New seeded generator
    pub fn new(seed: LibcSeed) -> Self {
        unsafe { libc::srand(u32::from_ne_bytes(seed.0)) };
        LibcRng::default()
    }
}

impl RngCore for LibcRng {
    fn next_u32(&mut self) -> u32 {
        unsafe { libc::rand32(u32::MAX) }
    }
    fn next_u64(&mut self) -> u64 {
        // https://stackoverflow.com/a/2769598
        u64::from(self.next_u32()).shl(32) | u64::from(self.next_u32())
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.iter_mut()
            .for_each(|byte| *byte = unsafe { libc::rand8(u8::MAX) });
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// 32-bit random number seed
#[derive(Default)]
pub struct LibcSeed(pub [u8; 4]);

impl AsMut<[u8]> for LibcSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl SeedableRng for LibcRng {
    type Seed = LibcSeed;
    fn from_seed(seed: Self::Seed) -> Self {
        LibcRng::new(seed)
    }
}
