[![Crates.io](https://img.shields.io/crates/v/mos-hardware)](https://crates.io/crates/mos-hardware)

# MOS-Hardware

This crate contains hardware register tables and support functions for
8-bit retro computers like the Commodore 64, MEGA65 and others.
Please check the `examples/` directory to see how Rust can be
used to generate demo effects.

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

### Use bitflags to safely control hardware behaviour

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

The easiest way is to use provided `devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with `Remote - Containers` extension
2. Open this project inside devcontainer
3. In vscode terminal do:
    ```
      # build for mos-atari8-none target
      cargo build --target mos-mega65-none
    ```

## Status

The hardware registers are currently incomplete and the library may
be subject to significant changes.

- [x] `sid`
- [x] `vic2` (partially)
- [x] `cia` (partially)
- [x] `c64` (particlly)
- [x] `mega65` (partially)
- [x] Plasma-effect example
- [x] Raster IRQ example
- [x] Sprite example

