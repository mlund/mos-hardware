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

//! Mathematical support functions

use super::MATH_STATUS;
use bitflags::bitflags;
use num_integer::Integer;
use volatile_register::{RO, WO};

bitflags! {
    /// Control flags for hardware multiplier/divider
    pub struct StatusFlags: u8 {
        const DIVBUSY = 0b1000_0000; // bit 7
        const MULBUSY = 0b0100_0000; // bit 6
    }
}

/// Registers for Math Acceleration
///
/// See the MEGA64 book, Section G-19.
/// The hardware registers use little endian storage.
#[repr(C, packed)]
pub struct MathAccelerator {
    /// 32-bit fractional part DIVOUT(0-3) of MULTINA divided by MULTINB
    pub divout_fraction: RO<u32>, // 0x00
    /// 32-bit whole part DIVOUT(4-7) of MULTINA divided by MULTINB
    pub divout_whole: RO<u32>, // 0x04
    /// 32-bit input A
    pub multin_a: WO<u32>, // 0x08
    /// 32-bit input B
    pub multin_b: WO<u32>, // 0x0c
    /// 64-bit product MULTOUT of MULTINA and MULTINB
    pub multout: RO<u64>, // 0x10
}

/// Marker trait to limit integer size to `u8`, `u16`, or `u32`
pub trait Max32BitInteger: Integer {}
impl Max32BitInteger for u8 {}
impl Max32BitInteger for u16 {}
impl Max32BitInteger for u32 {}

impl MathAccelerator {
    /// Hardware multiplication for `u8`, `u16`, or `u32`
    #[inline]
    pub fn multiply<T: Max32BitInteger>(&self, a: T, b: T) -> T {
        unsafe {
            (&self.multin_a as *const _ as *mut T).write_volatile(a);
            (&self.multin_b as *const _ as *mut T).write_volatile(b);
            (&self.multout as *const _ as *mut T).read_volatile()
        }
    }

    /// Hardware 32 bit multiplication and division
    ///
    /// Returns a tuple with:
    /// 1. 32-bit `a x b` (truncated) product
    /// 2. 32-bit whole part of `a / b`
    /// 3. 32-bit fractional part of `a / b`
    ///
    /// Cycles: less than 20
    pub fn multiply_divide(&self, a: u32, b: u32) -> (u32, u32, u32) {
        let product = self.multiply(a, b);
        while unsafe { &(*MATH_STATUS) }
            .read()
            .contains(StatusFlags::DIVBUSY)
        {}
        (
            product,
            self.divout_whole.read(),
            self.divout_fraction.read(),
        )
    }
}
