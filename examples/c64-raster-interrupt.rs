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

//! C64 Raster Interrupt example 1

#![no_std]
#![no_main]

extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::{c64, vic2::BLACK, vic2::LIGHT_GREEN};

// This function is called at every triggering event.
#[no_mangle]
pub extern "C" fn called_every_frame() {
    unsafe { c64::vic2().border_color.write(LIGHT_GREEN) };
    loop {
        if c64::vic2().raster_counter.read() > 120 {
            break;
        }
    }
    unsafe { c64::vic2().border_color.write(BLACK) };
}

#[no_mangle]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    const TRIGGER_LINE: u8 = 100;
    c64::hardware_raster_irq(TRIGGER_LINE);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
