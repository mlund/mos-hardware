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
//
// originally from cc65 header file; modififed from original version.
//
//                                  cx16.h
//
//                      CX16 system-specific definitions
//                             For prerelease 39
//
//
// This software is provided "as-is", without any expressed or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated, but is not required.
// 2. Altered source versions must be plainly marked as such, and must not
//    be misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source
//    distribution.

//! Registers for the Versatile Embedded Retro Adapter (VERA) graphics chip.
//!
//! VERA consists of:
//! - Video generator featuring:
//!   - Multiple output formats (VGA, NTSC Composite, NTSC S-Video, RGB video) at a fixed resolution of 640x480 at 60Hz
//!   - Support for two layers, both supporting either tile or bitmap mode.
//!   - Support for up to 128 sprites.
//!   - Embedded video RAM of 128kB.
//!   - Palette with 256 colors selected from a total range of 4096 colors.
//! - 16-channel Programmable Sound Generator with multiple waveforms (Pulse, Sawtooth, Triangle, Noise)
//! - High quality PCM audio playback from 4 kB FIFO buffer with up to 48kHz 16-bit stereo sound.
//! - SPI controller for SecureDigital storage.
//! - [VERA Reference Guide](https://github.com/commanderx16/x16-docs/blob/master/VERA%20Programmer's%20Reference.md)

use bitflags::bitflags;
use core::mem::ManuallyDrop;
use volatile_register::{RW, WO};

pub const VIDEOMODE_80X60: u8 = 0;
pub const VIDEOMODE_80X30: u8 = 1;
pub const VIDEOMODE_40X60: u8 = 2;
pub const VIDEOMODE_40X30: u8 = 3;
pub const VIDEOMODE_40X15: u8 = 4;
pub const VIDEOMODE_20X30: u8 = 5;
pub const VIDEOMODE_20X15: u8 = 6;
pub const VIDEOMODE_80COL: u8 = 0;
pub const VIDEOMODE_40COL: u8 = 3;
pub const VIDEOMODE_320X240: u8 = 128;
pub const VIDEOMODE_SWAP: i32 = -1;

pub const IRQ_VSYNC: u8 = 1;
pub const IRQ_RASTER: u8 = 2;
pub const IRQ_SPR_COLL: u8 = 4;
pub const IRQ_AUDIO_LOW: u8 = 8;

pub const INC_0: u8 = convert_stride(0);
pub const INC_2: u8 = convert_stride(2);
pub const INC_4: u8 = convert_stride(4);
pub const INC_8: u8 = convert_stride(8);
pub const INC_16: u8 = convert_stride(16);
pub const INC_32: u8 = convert_stride(32);
pub const INC_64: u8 = convert_stride(64);
pub const INC_128: u8 = convert_stride(128);
pub const INC_256: u8 = convert_stride(256);
pub const INC_512: u8 = convert_stride(512);
pub const INC_40: u8 = convert_stride(40);
pub const INC_80: u8 = convert_stride(80);
pub const INC_160: u8 = convert_stride(160);
pub const INC_320: u8 = convert_stride(320);
pub const INC_640: u8 = convert_stride(640);

pub const DEC_0: u8 = convert_stride(0);
pub const DEC_2: u8 = convert_stride(-2);
pub const DEC_4: u8 = convert_stride(-4);
pub const DEC_8: u8 = convert_stride(-8);
pub const DEC_16: u8 = convert_stride(-16);
pub const DEC_32: u8 = convert_stride(-32);
pub const DEC_64: u8 = convert_stride(-64);
pub const DEC_128: u8 = convert_stride(-128);
pub const DEC_256: u8 = convert_stride(-256);
pub const DEC_512: u8 = convert_stride(-512);
pub const DEC_40: u8 = convert_stride(-40);
pub const DEC_80: u8 = convert_stride(-80);
pub const DEC_160: u8 = convert_stride(-160);
pub const DEC_320: u8 = convert_stride(-320);
pub const DEC_640: u8 = convert_stride(-640);

/// Convert stride to register value.
///
/// By setting the 'Address Increment' field in `ADDRx_H`, the address will be incremented after each access to the data register.
/// Setting the `DECR` bit, will decrement instead of increment.
/// More [information](https://github.com/commanderx16/x16-docs/blob/master/VERA%20Programmer's%20Reference.md#video-ram-access)
///
/// Example:
/// ~~~
/// const DEC_8: u8 = convert_stride(-8); // negative stride
/// const INC_127: u8 = convert_stride(127); // compile time error: invalid stride
/// ~~~
pub const fn convert_stride(stride: i16) -> u8 {
    let value = match stride.abs() {
        0 => 0,
        1 => 1,
        2 => 2,
        4 => 3,
        8 => 4,
        16 => 5,
        32 => 6,
        64 => 7,
        128 => 8,
        256 => 9,
        512 => 10,
        40 => 11,
        80 => 12,
        160 => 13,
        320 => 14,
        640 => 15,
        _ => panic!("invalid stride"),
    };
    let decrement = stride < 0;
    // first a single shift to make room for the `DECR` bit.
    // then shift by 3 to end up in bits 7-3.
    ((value << 1) | decrement as u8) << 3
}

/// Versatile Embedded Retro Adapter (VERA) graphics chip
#[repr(C)]
pub struct VersatileEmbeddedRetroAdapter {
    /// VRAM Address 0-16 (offset 0x00)
    pub address: RW<u16>,
    /// `ADDRx_H` - Address (offset 0x02)
    pub address_hi: RW<u8>,
    /// `DATA0` - VRAM Data port 0 (offset 0x03)
    pub data0: RW<u8>,
    /// `DATA1` - VRAM Data port 1 (offset 0x04)
    pub data1: RW<u8>,
    /// `CTRL` - Control, offset 0x05
    pub control: RW<ControlFlags>,
    /// `IEN` - Interrupt enable, offset 0x06
    pub irq_enable: RW<u8>,
    /// `ISR` - Interrupt flags, offset 0x07
    pub irq_flags: RW<u8>,

    /// `IRQLINE_L` - Interrupt raster, offset 0x08
    ///
    /// `IRQLINE` specifies at which line the `LINE` interrupt will be generated.
    /// Note that bit 8 of this value is present in the `IEN` register.
    /// For interlaced modes the interrupt will be generated each field and the bit 0 of `IRQ_LINE` is ignored.
    pub irq_raster: RW<u8>,
    /// `DC_VIDEO` - Display composer
    pub display_composer: DisplayComposer,
    pub layer0: Layer,
    pub layer1: Layer,
    /// Audio (offset 0x1b)
    pub audio: Audio,
    pub spi: SPIController,
}

bitflags! {
    /// Flags for the `VersatileEmbeddedRetroAdapter::control` (`CTRL`) register at offset 0x05
    pub struct ControlFlags: u8 {
        const ADDRSEL = 0b0000_0001;
        const DCSEL = 0b0000_0010;
        /// RESET flag
        /// When set, the FPGA will reconfigure itself:
        /// all registers will be reset; the palette RAM will be set to default values.
        const RESET = 0b1000_0000;
    }
}

#[repr(C)]
pub union DisplayComposer {
    /// Visible when Display Composer (DC) `SEL` flag = 0
    pub display0: ManuallyDrop<Display0>,
    /// Visible when Display Composer (DC) `SEL` flag = 1
    pub display1: ManuallyDrop<Display1>,
}

bitflags! {
    /// Flags for Display Composer (DC) VIDEO at offset 0x09
    ///
    /// Bits 0-1 define the OUTPUT modes.
    pub struct VideoFlags: u8 {
        const DISABLED = 0b0000_0000;
        const VGA = 0b0000_0001;
        const NTSC = 0b0000_0010;
        /// RGB interlaced, composite sync (via VGA connector)
        const RGB = 0b0000_0011;

        /// Disable chroma
        ///
        /// Setting `CHROMA_DISABLE` disables output of chroma in NTSC composite mode and will give a
        /// better picture on a monochrome display.
        /// (Setting this bit will also disable the chroma output on the S-video output.)
        const CHROMA_DISABLE = 0b0000_0100; // bit 2
        const LAYER0_ENABLE = 0b0001_0000; // bit 4
        const LAYER1_ENABLE = 0b0010_0000; // bit 5
        const SPRITES_ENABLE = 0b0100_0000; // bit 6

        /// Read-only bit which reflects the active interlaced field in composite and RGB modes
        ///
        /// 0: even, 1: odd
        const CURRENT_FIELD = 0b1000_0000; // bit 7
    }
}

/// Active when Display Composer (DC) SEL=0
#[repr(C)]
pub struct Display0 {
    /// Flags to enable video layers
    pub video: RW<VideoFlags>,
    /// `HSCALE` - Active Display H-Scale, offset 0x0a
    ///
    /// `HSCALE` and `VSCALE` will set the fractional scaling factor of the active part of the display.
    /// Setting this value to 128 will output 1 output pixel for every input pixel.
    /// Setting this to 64 will output 2 output pixels for every input pixel.
    pub hscale: RW<u8>,
    /// `VSCALE` - Active Display V-Scale, offset 0x0b
    ///
    /// `HSCALE` and `VSCALE` will set the fractional scaling factor of the active part of the display.
    /// Setting this value to 128 will output 1 output pixel for every input pixel.
    /// Setting this to 64 will output 2 output pixels for every input pixel.
    pub vscale: RW<u8>,
    /// `DC_BORDER` - Border Color, offset 0x0c
    ///
    /// Determines the palette index which is used for the non-active area of the screen.
    pub border: RW<u8>,
}

/// Active when Display Composer (DC) `SEL=1`
///
/// `HSTART`/`HSTOP` and `VSTART`/`VSTOP` determines the active part of the screen.
/// The values here are specified in the native 640x480 display space.
/// `HSTART=0`, `HSTOP=640`, `VSTART=0`, `VSTOP=480` will set the active area to the full resolution.
/// Note that the lower 2 bits of `HSTART`/`HSTOP` and the lower 1 bit of `VSTART`/`VSTOP` isn't available.
/// This means that horizontally the start and stop values can be set at a multiple of 4 pixels,
/// vertically at a multiple of 2 pixels.
#[repr(C)]
pub struct Display1 {
    /// Horizontal start position
    pub hstart: RW<u8>,
    /// Horizontal stop position
    pub hstop: RW<u8>,
    /// Vertical start position
    pub vstart: RW<u8>,
    /// Vertical stop position
    pub vstop: RW<u8>,
}

/// Video layer registers
///
/// The features of the two VERA layers are the same.
/// Each layer supports a few different modes which are specified using T256C / 'Bitmap Mode' / 'Color Depth' in `Lx_CONFIG`.
/// The layer can either operate in tile mode or bitmap mode.
/// This is selected using the 'Bitmap Mode' bit; 0 selects tile mode, 1 selects bitmap mode.
/// The handling of 1 bpp tile mode is different from the other tile modes.
/// Depending on the T256C bit the tiles use either a 16-color foreground and background color or a 256-color foreground color.
/// Other modes ignore the T256C bit.
#[repr(C)]
pub struct Layer {
    /// `Lx_CONFIG`
    pub config: RW<u8>,
    /// `Lx_MAPBASE` - Map Base Address (16:9)
    pub mapbase: RW<u8>,
    /// `Lx_TILEBASE`
    pub tilebase: RW<u8>,
    /// `H-SCROLL` - Horizontal scroll
    pub hscroll: RW<u16>,
    /// `V-SCROLL` - Vertical scroll
    pub vscroll: RW<u16>,
}

/// VERA audio
///
/// The audio functionality consists of two independent systems:
/// 1. The PSG or Programmable Sound Generator.
/// 2. The PCM (or Pulse-Code Modulation) playback system.
#[repr(C)]
pub struct Audio {
    /// `AUDIO_CTRL`
    pub control: RW<u8>,
    /// `AUDIO_RATE` - PCM Sample Rate
    pub rate: RW<u8>,
    /// `AUDIO_DATA` - Audio FIFO data (write-only)
    pub data: WO<u8>,
}

/// SPI controller connected to the SD card connector
///
/// The speed of the clock output of the SPI controller can be controlled by the 'Slow Clock' bit.
/// When this bit is 0 the clock is 12.5MHz, when 1 the clock is about 390kHz.
/// The slow clock speed is to be used during the initialization phase of the SD card.
/// Some SD cards require a clock less than 400kHz during part of the initialization.
/// A transfer can be started by writing to `SPI_DATA`.
/// While the transfer is in progress the BUSY bit will be set.
/// After the transfer is done, the result can be read from the SPI_DATA register.
/// The chip select can be controlled by writing the `SELECT` bit.
/// Writing 1 will assert the chip-select (logic-0) and writing 0 will release the chip-select (logic-1).
#[repr(C)]
pub struct SPIController {
    pub data: RW<u8>,
    pub control: RW<u8>,
}

/// VRAM, 0x00000 - 0x1F9BF
pub const VIDEO_RAM: *mut u8 = (0x00000) as *mut u8;

/// PSG registers, 0x1F9C0 - 0x1F9FF
pub const PSG_REGISTERS: *mut u8 = (0x1f9c0u32) as *mut u8;

/// Palette, 0x1FA00 - 0x1FBFF
pub const PALETTE: *mut u8 = (0x1fa00u32) as *mut u8;

/// Sprite attributes, 0x1FC00 - 0x1FFFF
pub const SPRITE_ATTRIBUTES: *mut u8 = (0x1fc00u32) as *mut u8;
