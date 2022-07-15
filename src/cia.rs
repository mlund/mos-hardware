use core::mem::size_of;
use volatile_register::RW;

#[repr(C, packed)]
pub struct TimeOfDay {
    pub deci_seconds: RW<u8>, // 0x08
    pub seconds: RW<u8>,      // 0x09
    pub minutes: RW<u8>,      // 0x0a
    pub hours: RW<u8>,        // 0x0b
}

#[repr(C, packed)]
pub struct MOSComplexInterfaceAdapter6526 {
    pub port_a: RW<u8>,                // 0x00
    pub port_b: RW<u8>,                // 0x01
    pub data_direction_port_a: RW<u8>, // 0x02
    pub data_direction_port_b: RW<u8>, // 0x03
    pub timer_a: RW<u16>,              // 0x04
    pub timer_b: RW<u16>,              // 0x06
    pub time_of_day: TimeOfDay,        // 0x08
    pub serial_shift: RW<u8>,          // 0x0c
    pub interrupt: RW<u8>,             // 0x0d
    pub control_a: RW<u8>,             // 0x0e
    pub control_b: RW<u8>,             // 0x0f
}

const_assert!(size_of::<MOSComplexInterfaceAdapter6526>() == 16);
