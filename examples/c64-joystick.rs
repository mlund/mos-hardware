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

//! Example showing how to read joystick position (Port 2)

#![no_std]
#![no_main]
extern crate mos_alloc;

use core::panic::PanicInfo;
use mos_hardware::c64;
use mos_hardware::cia::GameController;
use mos_hardware::cia::JoystickPosition;
use ufmt_stdio::*;

#[no_mangle]
extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const u8) -> core::ffi::c_int {
    println!("WIGGLE JOYSTICK IN PORT 2!");
    loop {
        let controller1: GameController = c64::cia1().port_a.read().into();

        let (position, fire) = controller1.read_joystick();
        while (position, fire) == controller1.read_joystick() {}

        let (position, fire) = controller1.read_joystick();
        let message = match position {
            JoystickPosition::Up => "NORTH",
            JoystickPosition::Down => "SOUTH",
            JoystickPosition::Left => "WEST",
            JoystickPosition::Right => "EAST",
            JoystickPosition::UpLeft => "NORTH-WEST",
            JoystickPosition::UpRight => "NORTH-EAST",
            JoystickPosition::DownLeft => "SOUTH-WEST",
            JoystickPosition::DownRight => "SOUTH-EAST",
            JoystickPosition::Middle => "",
        };
        if !message.is_empty() {
            println!("{} WITH FIRE = {}", message, fire as u8);
        } else if fire {
            println!("FIRE");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        println!("PANIC!");
    }
}
