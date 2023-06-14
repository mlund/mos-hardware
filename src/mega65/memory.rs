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

//! Memory related tools

use super::{lcopy, lpeek};
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::From;
use core::mem::MaybeUninit;

/// Allocator for 28-bit memory
///
/// This can be used to allocate memory in 28-bit address space.
///
/// # Examples
///
/// The allocated 28-bit region is described by a `Fat28` pointer,
/// here called `fat` which implements the `From` trait for common
/// types such as `Vec<u8>` and `String`.
/// ~~~
/// let mut mem = memory::Allocator::new(0x40000);
/// let a = Vec::<u8>::from([7, 9, 13]);
/// let fat = mem.write(a.as_slice()); // dma write
/// let b = Vec::<u8>::from(fat); // dma read
/// assert_eq!(a, b);
/// ~~~
///
/// `Vec<Fat28>` can be traversed almost as if a vector of values:
/// ~~~
/// let cnt = Vec::from([mem.push(b"first"), mem.push(b"second")])
///      .iter()
///      .copied()
///      .map(String::from)
///      .filter(|s| s.starts_with('s'))
///      .count();
/// assert_eq!(cnt, 1);
/// ~~~
pub struct Allocator {
    /// Current 28-bit address
    pub address: u32,
}

impl Allocator {
    pub fn new(address: u32) -> Self {
        Self { address }
    }
    /// DMA copy bytes to next available 28-bit memory location.
    ///
    /// Every call to `write` advances the address by `bytes.len()`.
    pub fn write(&mut self, bytes: &[u8]) -> Fat28 {
        let len = bytes.len();
        let ptr = Fat28 {
            address: self.address,
            len,
        };
        unsafe {
            lcopy(bytes.as_ptr() as u32, self.address, len);
        }
        self.address += len as u32;
        ptr
    }
}

/// Fat pointer to region in 28-bit address space
#[derive(Clone, Copy)]
pub struct Fat28 {
    /// Address
    pub address: u32,
    /// Length in bytes
    pub len: usize,
}

impl From<Fat28> for String {
    fn from(value: Fat28) -> Self {
        unsafe { Self::from_utf8_unchecked(value.into()) }
    }
}

impl From<Fat28> for Vec<u8> {
    fn from(value: Fat28) -> Self {
        MemoryIterator::new(value.address).get_chunk(value.len)
    }
}

/// Never-ending iterator to lpeek into 28-bit memory
///
/// The address is automatically pushed forward with every byte read.
///
/// # Examples
/// ~~~
/// const ADDRESS: u32 = 0x40000;
/// let mut mem = MemoryIterator::new(ADDRESS);
/// let single_byte: u8 = mem.next().unwrap();
/// let byte_vector: Vec<u8> = mem.get_chunk(10);
/// for byte in mem.take(4) {
///     println!("{}", byte);
/// }
/// assert_eq!(mem.address, ADDRESS + 1 + 10 + 4);
/// ~~~
///
/// # Todo
///
/// This should eventually be submitted to the `mos-hardware` crate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MemoryIterator {
    /// Next address
    pub address: u32,
}

impl MemoryIterator {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }

    /// Peek `n` bytes using fast Direct Memory Access (DMA) copy
    ///
    /// # Todo
    ///
    /// - Check that the DMA copy works as expected
    #[allow(clippy::uninit_vec)]
    pub fn get_chunk(&mut self, n: usize) -> Vec<u8> {
        let mut dst = Vec::<u8>::with_capacity(n);
        unsafe {
            dst.set_len(n);
            lcopy(self.address, dst.as_mut_slice().as_ptr() as u32, n);
        }
        self.address += n as u32;
        dst
    }
}

impl Iterator for MemoryIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let value = lpeek(self.address);
        self.address += 1;
        Some(value)
    }

    #[allow(clippy::uninit_assumed_init)]
    fn next_chunk<const N: usize>(
        &mut self,
    ) -> Result<[Self::Item; N], core::array::IntoIter<Self::Item, N>>
    where
        Self: Sized,
    {
        let dst: [Self::Item; N] = unsafe { MaybeUninit::uninit().assume_init() };
        unsafe {
            lcopy(self.address, dst.as_ptr() as u32, N);
        }
        self.address += N as u32;
        Ok(dst)
    }

    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        self.address += n as u32;
        Ok(())
    }
}
