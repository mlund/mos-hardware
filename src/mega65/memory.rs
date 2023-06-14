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
/// ~~~
/// let mut alloc = Allocator::new(0xc0000);
/// let ptr: Ptr28 = alloc.push("some large string".as_bytes()); // DMA copy to
/// let s = String::From(ptr); // Get using DMA copy
/// assert_eq!(s, "some large string");
/// ~~~
///
/// Subsequent calls to `push()` advances the allocator address.
/// Several external strings can be handled with `Vec<Ptr28>`.
pub struct Allocator {
    /// Current 28-bit address
    pub address: u32,
}

impl Allocator {
    pub fn new(address: u32) -> Self {
        Self { address }
    }
    /// DMA copy bytes to next available 28-bit memory location
    pub fn push(&mut self, bytes: &[u8]) -> Ptr28 {
        let len = bytes.len() as u16;
        let ptr = Ptr28 {
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

/// Fat pointer to 28-bit address with an additional length
#[derive(Clone, Copy)]
pub struct Ptr28 {
    /// Address
    pub address: u32,
    /// Length in bytes
    pub len: u16,
}

impl From<Ptr28> for String {
    fn from(value: Ptr28) -> Self {
        unsafe { Self::from_utf8_unchecked(value.into()) }
    }
}

impl From<Ptr28> for Vec<u8> {
    fn from(value: Ptr28) -> Self {
        MemoryIterator::new(value.address).get_chunk(value.len)
    }
}

/// Never-ending iterator to lpeek into 28-bit memory
///
/// The address is automatically pushed forward with every byte read.
///
/// # Examples
/// ~~~
/// const ADDRESS: u32 = 0x8010000;
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
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    /// Peek `n` bytes using fast Direct Memory Access (DMA) copy
    ///
    /// # Todo
    ///
    /// - Check that the DMA copy works as expected
    #[allow(clippy::uninit_vec)]
    pub fn get_chunk(&mut self, n: u16) -> Vec<u8> {
        let mut dst = Vec::<u8>::with_capacity(n as usize);
        unsafe {
            dst.set_len(n as usize);
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
            lcopy(self.address, dst.as_ptr() as u32, N as u16);
        }
        self.address += N as u32;
        Ok(dst)
    }

    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        self.address += n as u32;
        Ok(())
    }
}
