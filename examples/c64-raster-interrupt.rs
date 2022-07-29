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
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mos_hardware::*;
use ufmt_stdio::*;

// This function is called at every triggering event.
#[no_mangle]
pub extern "C" fn called_every_frame() {
    unsafe {
        (*c64::VIC).border_color.write(vic2::LIGHT_GREEN);
        loop {
            if (*c64::VIC).raster_counter.read() > 120 {
                break;
            }
        }
        (*c64::VIC).border_color.write(vic2::BLACK);
    }
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    c64::hardware_raster_irq(100); // trigger at raster line 100
    loop {} // let's not return to dead BASIC
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}
