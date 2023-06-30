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

use super::libc;
use crate::poke;
use alloc::{string::String, vec::Vec};
use core::{convert::From, mem::MaybeUninit};

/// Maximum value in 28-bit address space
pub const MAX_28_BIT_ADDRESS: u32 = 0xFFFFFFF;

/// Read into 28 bit memory
pub fn lpeek(address: u32) -> u8 {
    unsafe { libc::lpeek(address as i32) }
}

/// Write into 28 bit memory
///
/// # Safety
/// Unsafe as it writes directly to memory
pub unsafe fn lpoke(address: u32, value: u8) {
    libc::lpoke(address as i32, value)
}

/// DMA copy in 28 bit address space
///
/// # Safety
/// Unsafe as it writes directly to memory
pub unsafe fn lcopy(source: u32, destination: u32, length: usize) {
    if length > 0 {
        unsafe { libc::lcopy(source as i32, destination as i32, length as u16) };
    }
}

/// Allocator for 28-bit memory
///
/// This can be used to allocate memory in 28-bit address space.
///
/// # Examples
///
/// The allocated 28-bit region is described by a fat `Ptr28` pointer,
/// which implements the `From` trait for common
/// types such as `Vec<u8>` and `String`.
/// ~~~
/// let mut bank = Allocator::new(0x40000);
/// let a = Vec::<u8>::from([7, 9, 13]);
/// let ptr = bank.write(a.as_slice()); // dma write
/// let b = Vec::<u8>::from(ptr); // dma read
/// assert_eq!(a, b);
/// ~~~
///
/// `Vec<Ptr28>` can be traversed almost as if a vector of values:
/// ~~~
/// let cnt = Vec::from([bank.push(b"first"), bank.push(b"second")])
///      .iter()
///      .copied()
///      .map(String::from) // dma write
///      .filter(|s| s.starts_with('s'))
///      .count();
/// assert_eq!(cnt, 1);
/// ~~~
pub struct Allocator {
    /// Current 28-bit address
    pub address: u32,
}

impl Allocator {
    /// New allocator starting at `address`
    pub const fn new(address: u32) -> Self {
        Self { address }
    }
    /// DMA copy bytes to next free 28-bit memory location.
    ///
    /// Every call to `write` advances the address by `bytes.len()`.
    pub fn write(&mut self, bytes: &[u8]) -> Ptr28 {
        let len = bytes.len();
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

/// Fat pointer to region in 28-bit address space
#[derive(Clone, Copy)]
pub struct Ptr28 {
    /// Address
    pub address: u32,
    /// Length in bytes
    pub len: usize,
}

impl From<Ptr28> for String {
    fn from(value: Ptr28) -> Self {
        // naughty camouflage of unsafe code...
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
/// const ADDRESS: u32 = 0x40000;
/// let mut mem = MemoryIterator::new(ADDRESS);
/// let byte: u8 = mem.next().unwrap();
/// let v: Vec<u8> = mem.get_chunk(10);
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

    #[cfg(version("1.69"))]
    fn advance_by(&mut self, n: usize) -> Result<(), core::num::NonZeroUsize> {
        self.address += n as u32;
        Ok(())
    }
    #[cfg(not(version("1.69")))]
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        self.address += n as u32;
        Ok(())
    }
}

impl Default for libc::dmagic_dmalist {
    fn default() -> Self {
        Self {
            option_0b: 0x0b,
            option_80: 0x80,
            option_81: 0x81,
            option_85: 0x85,
            sub_cmd: 0x00,
            end_of_options: 0x00,
            dest_skip: 1,
            source_addr: 0,
            source_bank: 0,
            source_mb: 0,
            dest_addr: 0,
            dest_bank: 0,
            dest_mb: 0,
            count: 0,
            modulo: 0,
            command: Self::COPY,
        }
    }
}

impl libc::dmagic_dmalist {
    /// Copy command
    const COPY: u8 = 1;
    /// Perform the copy once data is in place
    pub fn do_dma(&self) {
        let self_ptr = core::ptr::addr_of!(self) as u16;
        unsafe {
            libc::mega65_io_enable();
            poke!(0xd702 as *mut u8, 0);
            poke!(0xd704 as *mut u8, 0); // List is in $00xxxxx
            poke!(0xd701 as *mut u8, (self_ptr >> 8) as u8);
            poke!(0xd701 as *mut u8, (self_ptr & 0xff) as u8); // triggers enhanced DMA
        }
    }
    /// Set source address
    fn set_source(&mut self, src: u32) {
        // User should provide 28-bit address for IO
        // (otherwise we can't DMA to/from RAM under IO)
        //  if (source_address>=0xd000 && source_address<0xe000)
        //    dmalist.source_bank|=0x80;
        self.source_mb = (src >> 20) as u8;
        self.source_addr = (src & 0xffff) as u16;
        self.source_bank = ((src >> 16) & 0x0f) as u8;
    }

    /// Set destination address
    fn set_destinaion(&mut self, dst: u32) {
        // User should provide 28-bit address for IO
        // (otherwise we can't DMA to/from RAM under IO)
        //  if (destination_address>=0xd000 && destination_address<0xe000)
        //    dmalist.dest_bank|=0x80;
        self.dest_mb = (dst >> 20) as u8;
        self.dest_addr = (dst & 0xffff) as u16;
        self.dest_bank = ((dst >> 16) & 0x0f) as u8;
    }
    fn init(&mut self) {
        self.option_0b = 0x0b;
        self.option_80 = 0x80;
        self.option_81 = 0x81;
        self.option_85 = 0x85;
        self.sub_cmd = 0x00;
        self.end_of_options = 0x00;
        self.dest_skip = 1;
    }

    /// Peek into memory using DMA copy
    pub fn lpeek(&mut self, src: u32) -> u8 {
        let dst: u8 = 0;
        self.copy(src, (dst as *mut u8) as u32, 1);
        dst
    }

    pub fn lpoke(&mut self, dst: u32, value: u8) {
        self.copy((value as *mut u8) as u32, dst, 1);
    }

    /// DMA copy `n` bytes from `src` address to `dst` address
    pub fn copy(&mut self, src: u32, dst: u32, n: usize) {
        self.command = Self::COPY;
        self.count = n as u16;
        self.init();
        self.set_source(src);
        self.set_destinaion(dst);
        self.do_dma();
    }
}
