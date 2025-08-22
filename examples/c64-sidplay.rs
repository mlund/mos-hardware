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

//! Simple C64 PSID play example

#![no_std]
#![no_main]

extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::sid::SidTune;
use mos_hardware::{c64, sid, vic2};

pub struct SidFile;
impl sid::SidTune for SidFile {
    const BYTES: &'static [u8] = core::include_bytes!("../assets/last_hero.sid");
}

#[no_mangle]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    let music = SidFile;
    unsafe {
        music.to_memory();
    }
    music.init(0);
    loop {
        let line = c64::vic2().raster_counter.read();
        if line == 60 {
            unsafe { c64::vic2().border_color.write(vic2::RED) }
            music.play();
            unsafe { c64::vic2().border_color.write(vic2::BLACK) }
            while c64::vic2().raster_counter.read() < 100 {}
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
