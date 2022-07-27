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
//! 8-bit retro computers like the Commodore 64, MEGA65 and others.
//! Please check the `examples/` directory to see how Rust can be
//! used generate demo effects.
//!
//! # Examples
//! 
//! Read and write to labelled hardware registers:
//! 
//! ```
//! use mos_hardware::{c64,vic2};
//! 
//! let old_border_color = (*c64::VIC).border_color.read();
//! (*c64::VIC).border_color.write(vic2::LIGHT_RED);
//! 
//! (*c64::SID).potentiometer_x.write(3); // error: read-only register
//! ```
//! 
//! Use bitflags to control hardware behaviour, _e.g._ where the VIC-II chip accesses
//! screen memory and character sets:
//! 
//! ```
//! let bank = vic2::ScreenBank::AT_2C00.bits() | vic2::CharsetBank::AT_2000.bits();
//! (*c64::VIC).screen_and_charset_bank.write(bank);
//! ```
//! 
//! Convenience functions to perform hardware-specific tasks, _e.g._ generate random numbers
//! using noise from the C64's SID chip:
//! 
//! ```
//! (*c64::SID).start_random_generator();
//! let random_number : u8 = rand8!(c64::SID);
//! ```
//!

#![no_std]
#![feature(const_option)]

extern crate static_assertions;

pub mod cia;
pub mod sid;
pub mod vic2;
pub mod c64;
pub mod mega65;

use core::iter::Iterator;

/**
 * Peek into memory (read)
 *
 * Example:
 * ~~~
 * let value = peek!(0xC000 as *mut u8);
 * ~~~
 */
#[macro_export]
macro_rules! peek {
    ($address:expr) => {{
        core::ptr::read_volatile($address)
    }};
}

/**
 * Poke into memory (read)
 *
 * Example:
 * ~~~
 * poke!(0xD020 as *mut u8, vic2::LIGHT_GREEN);
 * ~~~
 */
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
/// Example:
/// ```
/// let high = highbyte(0xABCD);
/// let low = lowbyte(0xABCD);
/// assert_eq!(high, 0xAB);
/// assert_eq!(low, 0xCD);
/// ```
#[macro_export]
macro_rules! highbyte {
    ($word:expr) => {{
        ((&$word as *const u16) as *const u8).offset(1).read_volatile()
        // Can also be done using bit-shifting: ($word >> 8) as u8
    }};
}

/// Get low byte from a 16-bit integer using pointer arithmetic
///
/// Example:
/// ```
/// let word = 0xABCD;
/// assert_eq!(highbyte!(word), 0xAB);
/// assert_eq!(lowbyte!(word), 0xCD);
/// ```
#[macro_export]
macro_rules! lowbyte {
    ($word:expr) => {{
        ((&$word as *const u16) as *const u8).offset(0).read_volatile()
        // Can also be done using bit-shifting: ($word & 0xff) as u8
    }};
}

/**
 * Repeat each element n times
 *
 * See more
 * [here](https://stackoverflow.com/questions/66482699/how-to-repeat-each-element-of-iterator-n-times).
 */
pub fn repeat_element<T: Clone>(
    it: impl Iterator<Item = T>,
    cnt: usize,
) -> impl Iterator<Item = T> {
    it.flat_map(move |n| core::iter::repeat(n).take(cnt))
}
