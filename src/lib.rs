// Copyright (C) 2022 Mikael Lund
//
// This program is free software; you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation; either version 3 of the
// License, or (at your option) any later version. This program is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program;
// if not, see http://www.gnu.org/licenses/gpl-3.0

#![no_std]
#![feature(const_ptr_offset_from, const_refs_to_cell)]
#[macro_use]
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
