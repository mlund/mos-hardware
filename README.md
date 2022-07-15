# MOS-Hardware

This crate contains hardware register tables and support functions for
8-bit retro computers like the Commodore 64, MEGA65 and others.
Please check the `examples/` directory to see how Rust can be
used generate demo effects.

## Aims

- Excellent support for Rust programming on CBM (inspired) 8-bit computers
- Labelled registers for expressive hardware programming
- Intuitive bitflags with type checks where possible
- Minimum resource impact

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is [available](https://hub.docker.com/r/mrkits/rust-mos) if you
do not fancy compiling LLVM.

## Examples

Read and write to labelled hardware registers:

~~~ rust
use mos_hardware::{c64,vic2};

let old_border_color = (*c64::VIC).border_color.read();
(*c64::VIC).border_color.write(c64::LIGHT_RED);

(*c64::SID).potentiometer_x.write(3); // error: read-only register
~~~

Use bitflags to control hardware behaviour, _e.g._ where the VIC-II chip accesses
screen memory and character sets:

~~~ rust
let bank = vic2::ScreenBank::AT_2C00.bits() | vic2::CharsetBank::AT_2000.bits();
(*c64::VIC).screen_and_charset_bank.write(bank);
~~~

Convenience functions to perform hardware-specific tasks, _e.g._ generate random numbers
using noise from the C64's SID chip:

~~~ rust
(*c64::SID).start_random_generator();
let random_number : u8 = rand8!(c64::SID);
~~~

## Status

The hardware registers are currently incomplete and the library may
be subject to significant changes.

- [x] `sid`
- [x] `vic2` (partially)
- [x] `cia` (partially)
- [x] `c64` (particlly)
- [x] Plasma-effect example
- [ ] `mega65` (scaffold only)

