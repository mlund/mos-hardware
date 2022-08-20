[![Crates.io](https://img.shields.io/crates/v/mos-hardware)](https://crates.io/crates/mos-hardware)
[![docs.rs](https://img.shields.io/docsrs/mos-hardware)](https://docs.rs/mos-hardware/latest/mos_hardware)

# MOS-Hardware

This crate contains hardware register tables and support functions for
8-bit retro computers like the Commodore 64, Commander X16, MEGA65 and others.
Please check the [`examples`](https://github.com/mlund/mos-hardware/tree/main/examples)
directory to see how Rust can be used to generate simple demo effects.

## Aims

- Excellent support for Rust programming on CBM (inspired) 8-bit computers
- Labelled registers for expressive hardware programming
- Intuitive bitflags with type checks where possible
- Minimum resource impact

## Examples

### Read and write to labelled hardware registers

~~~ rust
use mos_hardware::{c64,vic2};
let old_border_color = (*c64::VIC).border_color.read();
(*c64::VIC).border_color.write(vic2::LIGHT_RED);
(*c64::SID).potentiometer_x.write(3); // error: read-only register
~~~

### Use bitflags to safely control hardware

...for example where the VIC-II chip accesses screen memory and character sets:

~~~ rust
let bank = vic2::ScreenBank::AT_2C00.bits() | vic2::CharsetBank::AT_2000.bits();
(*c64::VIC).screen_and_charset_bank.write(bank);
~~~

### Convenience functions to perform hardware-specific tasks

...for example to generate random numbers using noise from the C64's SID chip:

~~~ rust
(*c64::SID).start_random_generator();
let random_number : u8 = rand8!(c64::SID);
~~~

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is [available](https://hub.docker.com/r/mrkits/rust-mos) if you
do not fancy compiling LLVM.

### Docker and Visual Studio Code

The easiest way is to use the provided `.devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with the _Remote - Containers_ extension
2. Open the project inside devcontainer when asked
3. In the vscode terminal do:
   ~~~ bash
   # build for the MEGA65:
   cargo build --target mos-mega65-none
   ~~~

## Status

The hardware registers are currently incomplete and the library may
be subject to significant changes.

- [Commodore 64](https://www.c64-wiki.com/wiki/C64):
  - [x] `sid`
  - [x] `vic2`
  - [x] `cia` (partially)
  - [x] `c64` memory map (particlly)
- [Commander X16](https://www.commanderx16.com)
  - [x] `vera`
  - [x] `via` (partially)
  - [x] `cx16` Memory map (partially)
  - [ ] Support functions
- [MEGA65](https://mega65.org):
  - [x] partial support for vic3, vic4 and other hardware registers.
  - [x] [mega65-libc](https://github.com/MEGA65/mega65-libc) bindings
- [Examples](https://github.com/mlund/mos-hardware/tree/main/examples):
  - [x] Plasma effect (c64, mega65)
  - [x] Raster IRQ (c64)
  - [x] Sprites (c64)
