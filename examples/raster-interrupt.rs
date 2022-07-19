//! C64 Raster Interrupt example 1

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use ufmt_stdio::*;
use mos_hardware::*;

// This function is called at every triggering event.
#[no_mangle]
pub unsafe extern fn called_every_frame() {
    (*c64::VIC).border_color.write(7);
    loop {
        if (*c64::VIC).raster_counter.read() > 120 {
            break;
        }
    }
    (*c64::VIC).border_color.write(0);
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

