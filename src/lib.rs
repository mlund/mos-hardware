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

//! This crate contains hardware register tables and support functions for
//! 8-bit retro computers like the Commodore 64, Commander X16, MEGA65 and others.
//! Please check the `examples/` directory to see how Rust can be
//! used generate basic graphics effects and interact with hardware.
//!
//! # Examples
//!
//! Read and write to labelled hardware registers:
//! ~~~
//! use mos_hardware::{c64,vic2};
//!
//! let old_border_color = (*c64::VIC).border_color.read();
//! (*c64::VIC).border_color.write(vic2::LIGHT_RED);
//! (*c64::SID).potentiometer_x.write(3); // error: read-only register
//! ~~~
//!
//! Use bitflags to control hardware behaviour, _e.g._ where the VIC-II chip accesses
//! screen memory and character sets:
//! ~~~
//! let bank = vic2::ScreenBank::AT_2C00.bits() | vic2::ScreenBank::AT_2000.bits();
//! (*c64::VIC).screen_and_charset_bank.write(bank);
//! ~~~
//!
//! Convenience functions to perform hardware-specific tasks, _e.g._ generate random numbers
//! using noise from the C64's SID chip:
//! ~~~
//! (*c64::SID).start_random_generator();
//! let value = (*c64::SID).random_byte();
//! ~~~

#![no_std]
#![feature(const_option)]
#![feature(core_ffi_c)]

extern crate static_assertions;

pub mod c64;
pub mod cia;
pub mod cx16;
pub mod mega65;
pub mod sid;
pub mod vera;
pub mod vic2;

use core::iter::Iterator;

/// Peek into memory (volatile read)
///
/// # Examples
/// ~~~
/// let value = peek!(0xC000 as *mut u8);
/// ~~~
#[macro_export]
macro_rules! peek {
    ($address:expr) => {{
        core::ptr::read_volatile($address)
    }};
}

/// Poke into memory (volatile write)
///
/// # Examples
/// ~~~
/// poke!(0xD020 as *mut u8, vic2::LIGHT_GREEN);
/// ~~~
#[macro_export]
macro_rules! poke {
    ($address:expr, $value:expr) => {{
        core::ptr::write_volatile($address, $value)
    }};
}

/// Add two integers using wrapping
#[macro_export]
macro_rules! add {
    ($value1:expr, $value2:expr) => {{
        $value1.wrapping_add($value2)
    }};
}

/// Subtract two integers using wrapping
#[macro_export]
macro_rules! sub {
    ($value1:expr, $value2:expr) => {{
        $value1.wrapping_sub($value2)
    }};
}

/// Get high byte from a 16-bit integer using pointer arithmetic
///
/// # Examples
/// ~~~
/// let high = highbyte(0xABCD);
/// let low = lowbyte(0xABCD);
/// assert_eq!(high, 0xAB);
/// assert_eq!(low, 0xCD);
/// ~~~
#[macro_export]
macro_rules! highbyte {
    ($word:expr) => {{
        ((&$word as *const u16) as *const u8)
            .offset(1)
            .read_volatile()
        // Can also be done using bit-shifting: ($word >> 8) as u8
    }};
}

/// Get low byte from a 16-bit integer using pointer arithmetic
///
/// # Examples
/// ~~~
/// let word = 0xABCD;
/// assert_eq!(highbyte!(word), 0xAB);
/// assert_eq!(lowbyte!(word), 0xCD);
/// ~~~
#[macro_export]
macro_rules! lowbyte {
    ($word:expr) => {{
        ((&$word as *const u16) as *const u8)
            .offset(0)
            .read_volatile()
        // Can also be done using bit-shifting: ($word & 0xff) as u8
    }};
}

/// Repeat each element n times
///
/// Convenience function currently used in the plasma examples.
/// See more [here](https://stackoverflow.com/questions/66482699/how-to-repeat-each-element-of-iterator-n-times).
pub fn repeat_element<T: Clone>(
    it: impl Iterator<Item = T>,
    cnt: usize,
) -> impl Iterator<Item = T> {
    it.flat_map(move |n| core::iter::repeat(n).take(cnt))
}

/// Returns constantly evaluated _scaled_ and _shifted_ sine table.
///
/// # Arguments
///
/// * `divide` - Number to divide with
/// * `add`    - Number to add
///
/// # Examples
/// ~~~
/// const SINE: [u8; 256] = make_sine(1, 0);
/// const SCALED_SINE: [u8; 256] = make_sine(4, 70);
/// ~~~
pub const fn make_sine(divide: u8, add: u8) -> [u8; SINETABLE.len()] {
    let mut array = SINETABLE;
    let mut i: usize = 0;
    while i < array.len() {
        array[i] = array[i] / divide + add;
        i += 1;
    }
    array
}

/// Tabulated, cyclic sine table
pub const SINETABLE: [u8; 256] = [
    0x80, 0x7d, 0x7a, 0x77, 0x74, 0x70, 0x6d, 0x6a, 0x67, 0x64, 0x61, 0x5e, 0x5b, 0x58, 0x55, 0x52,
    0x4f, 0x4d, 0x4a, 0x47, 0x44, 0x41, 0x3f, 0x3c, 0x39, 0x37, 0x34, 0x32, 0x2f, 0x2d, 0x2b, 0x28,
    0x26, 0x24, 0x22, 0x20, 0x1e, 0x1c, 0x1a, 0x18, 0x16, 0x15, 0x13, 0x11, 0x10, 0x0f, 0x0d, 0x0c,
    0x0b, 0x0a, 0x08, 0x07, 0x06, 0x06, 0x05, 0x04, 0x03, 0x03, 0x02, 0x02, 0x02, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x03, 0x03, 0x04, 0x05, 0x06, 0x06, 0x07, 0x08, 0x0a,
    0x0b, 0x0c, 0x0d, 0x0f, 0x10, 0x11, 0x13, 0x15, 0x16, 0x18, 0x1a, 0x1c, 0x1e, 0x20, 0x22, 0x24,
    0x26, 0x28, 0x2b, 0x2d, 0x2f, 0x32, 0x34, 0x37, 0x39, 0x3c, 0x3f, 0x41, 0x44, 0x47, 0x4a, 0x4d,
    0x4f, 0x52, 0x55, 0x58, 0x5b, 0x5e, 0x61, 0x64, 0x67, 0x6a, 0x6d, 0x70, 0x74, 0x77, 0x7a, 0x7d,
    0x80, 0x83, 0x86, 0x89, 0x8c, 0x90, 0x93, 0x96, 0x99, 0x9c, 0x9f, 0xa2, 0xa5, 0xa8, 0xab, 0xae,
    0xb1, 0xb3, 0xb6, 0xb9, 0xbc, 0xbf, 0xc1, 0xc4, 0xc7, 0xc9, 0xcc, 0xce, 0xd1, 0xd3, 0xd5, 0xd8,
    0xda, 0xdc, 0xde, 0xe0, 0xe2, 0xe4, 0xe6, 0xe8, 0xea, 0xeb, 0xed, 0xef, 0xf0, 0xf1, 0xf3, 0xf4,
    0xf5, 0xf6, 0xf8, 0xf9, 0xfa, 0xfa, 0xfb, 0xfc, 0xfd, 0xfd, 0xfe, 0xfe, 0xfe, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfc, 0xfb, 0xfa, 0xfa, 0xf9, 0xf8, 0xf6,
    0xf5, 0xf4, 0xf3, 0xf1, 0xf0, 0xef, 0xed, 0xeb, 0xea, 0xe8, 0xe6, 0xe4, 0xe2, 0xe0, 0xde, 0xdc,
    0xda, 0xd8, 0xd5, 0xd3, 0xd1, 0xce, 0xcc, 0xc9, 0xc7, 0xc4, 0xc1, 0xbf, 0xbc, 0xb9, 0xb6, 0xb3,
    0xb1, 0xae, 0xab, 0xa8, 0xa5, 0xa2, 0x9f, 0x9c, 0x99, 0x96, 0x93, 0x90, 0x8c, 0x89, 0x86, 0x83,
];
