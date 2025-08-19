use core::mem::size_of;
use static_assertions::const_assert;

use crate::cia::*;

pub type MOSComplexInterfaceAdapter6526_1 =
    MOSComplexInterfaceAdapter6526<CIA1PortA, CIA1PortB, CIA1DirA, CIA1DirB>;
pub type MOSComplexInterfaceAdapter6526_2 =
    MOSComplexInterfaceAdapter6526<CIA2PortA, CIA2PortB, CIA2DirA, CIA2DirB>;

const_assert!(size_of::<MOSComplexInterfaceAdapter6526_1>() == 16);
const_assert!(size_of::<MOSComplexInterfaceAdapter6526_2>() == 16);

impl MOSComplexInterfaceAdapter6526_1 {
    /// reset CIA#1, prepare keyboard with STOP key
    pub fn reset(&mut self) {
        unsafe {
            // Disable Interrupt
            self.control.interrupt.write(InterruptControl::DISABLE_ALL);

            // Turn on STOP key (bit 7 low)
            self.port_a.write(Default::default());

            // Shut off timers
            self.control.control_a.write(TIMER_OFF);
            self.control.control_b.write(TIMER_OFF);

            // Configure ports
            // Keyboard inputs (CIA1 Port B = inputs)
            self.data_direction_port_b.write(Default::default()); // All inputs
                                                                  // Keyboard outputs (CIA1 Port A = outputs)
            self.data_direction_port_a.write(Default::default()); // Set to keyboard mode
        }
    }

    /// Enable IRQ and timer 1. Keep TOD
    pub fn enable_keyboard(&mut self) {
        unsafe {
            // Enable IRQ
            self.control
                .interrupt
                .write(InterruptControl::SET_CLEAR | InterruptControl::IRQ);

            // save only tod bit and enable timer 1
            self.control
                .control_a
                .modify(|v| (v & TimerControl::TODIN) | TimerControl::LOAD | TimerControl::START);
        }
    }
}

impl MOSComplexInterfaceAdapter6526_2 {
    /// Reset CIA#2 and reset VIC Bank to 0, prepare User Port Input
    pub fn reset(&mut self) {
        unsafe {
            // Disable Interrupt
            self.control.interrupt.write(InterruptControl::DISABLE_ALL);

            // Shut off timers
            self.control.control_a.write(TIMER_OFF);
            self.control.control_b.write(TIMER_OFF);

            // User port (CIA2 Port B = no RS-232)
            self.data_direction_port_b
                .write(CIA2DirB::default_as_user_port()); // All inputs

            // Activate RS-232 TXD output and select VIC Bank 0
            self.port_a.write(CIA2PortA::default());

            // Set serial in/out, VA14/15 out
            self.data_direction_port_a.write(CIA2DirA::default());
        }
    }

    /// set clock line low  (inverted)
    pub fn clkhi(&mut self) {
        unsafe {
            self.port_a.modify(|v| v & !CIA2PortA::CLOCK_OUT);
        }
    }

    /// set clock line high (inverted)
    pub fn clklo(&mut self) {
        unsafe {
            self.port_a.modify(|v| v | CIA2PortA::CLOCK_OUT);
        }
    }
}
