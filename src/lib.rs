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

#![no_std]

extern crate static_assertions;

pub mod cia;
pub mod sid;
pub mod vic2;
pub mod c64;
pub mod mega65;

use core::iter::Iterator;

/// Peek into memory (read)
#[macro_export]
macro_rules! peek {
    ($address:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            core::ptr::read_volatile($address)
        }
    }};
}

/// Poke memory address (write)
#[macro_export]
macro_rules! poke {
    ($address:expr, $value:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            core::ptr::write_volatile($address, $value);
        }
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

/** https://stackoverflow.com/questions/66482699/how-to-repeat-each-element-of-iterator-n-times*/
pub fn repeat_element<T: Clone>(
    it: impl Iterator<Item = T>,
    cnt: usize,
) -> impl Iterator<Item = T> {
    it.flat_map(move |n| core::iter::repeat(n).take(cnt))
}
