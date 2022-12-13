//
// Copyright 2022 Mikael Lund aka Wombat
// Copyright 2018 David Simmons (https://github.com/simmons/cbm)
//
// Uses the petscii<->unicode look-up table from https://github.com/simmons/cbm
// and PETSCII to screen code conversion from https://sta.c64.org/cbm64pettoscr.html
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Utility functions for working with PETSCII characters

// The Unicode code point we use for untranslatable PETSCII characters.
pub const NONE: char = char::REPLACEMENT_CHARACTER;

// From: http://style64.org/petscii/
#[rustfmt::skip]
pub const PETSCII_TO_CHAR_MAP: [char; 256] = [
    // control codes
    NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
    NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
    // punctuation, numbers, a-z
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
    '@', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    // [, british pound, ], up arrow, left arrow, horizontal line
    '[', '\u{00A3}', ']', '\u{2191}', '\u{2190}', '\u{2501}',
    // A-Z
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    // box vert/horiz, left checkerboard, box vert, checkerboard-0, \-diag lines,
    '\u{254b}', NONE, '\u{2503}', '\u{2592}', NONE,
    // control codes
    NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
    NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
    // non-breaking space, left half block, lower half block, upper 1/8 block
    '\u{00a0}', '\u{258c}', '\u{2584}', '\u{2594}',
    // lower 1/8 block, left 1/4 block, checkerboard-1, right 1/4 block -> 1/8
    '\u{2581}', '\u{258e}', '\u{2592}', '\u{2595}',
    // lower half checkerboard, /-diag lines, right 1/4 block -> 1/8, box vert+right
    NONE, NONE, '\u{2595}', '\u{2523}',
    // quadrant lower right, box up+right, box down+left, lower 1/4 block
    '\u{2597}', '\u{2517}', '\u{2513}', '\u{2582}',
    // box down+right, box up+horiz, box down+horiz, box vertical+left
    '\u{250f}', '\u{253b}', '\u{2533}', '\u{252b}',
    // left 1/4 block, left 3/8 block, right 3/8 block -> 1/8, upper 1/4 block -> 1/8
    '\u{258e}', '\u{258d}', '\u{2595}', '\u{2594}',
    // upper 3/8 block -> 1/8, lower 3/8 block, check mark, quadrant lower left
    '\u{2594}', '\u{2583}', '\u{2713}', '\u{2596}',
    // quadrant upper right, box up+left, quadrant upper left, quadrant upper left and lower right
    '\u{259d}', '\u{2518}', '\u{2598}', '\u{259a}',
    // box horiz
    '\u{2501}',
    // A-Z
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    // box vert/horiz, left checkerboard, box vert, checkerboard-0, \-diag lines,
    '\u{254b}', NONE, '\u{2503}', '\u{2592}', NONE,
    // non-breaking space, left half block, lower half block, upper 1/8 block
    '\u{00a0}', '\u{258c}', '\u{2584}', '\u{2594}',
    // lower 1/8 block, left 1/4 block, checkerboard-1, right 1/4 block -> 1/8
    '\u{2581}', '\u{258e}', '\u{2592}', '\u{2595}',
    // lower half checkerboard, /-diag lines, right 1/4 block -> 1/8, box vert+right
    NONE, NONE, '\u{2595}', '\u{2523}',
    // quadrant lower right, box up+right, box down+left, lower 1/4 block
    '\u{2597}', '\u{2517}', '\u{2513}', '\u{2582}',
    // box down+right, box up+horiz, box down+horiz, box vertical+left
    '\u{250f}', '\u{253b}', '\u{2533}', '\u{252b}',
    // left 1/4 block, left 3/8 block, right 3/8 block -> 1/8, upper 1/4 block -> 1/8
    '\u{258e}', '\u{258d}', '\u{2595}', '\u{2594}',
    // upper 3/8 block -> 1/8, lower 3/8 block, check mark, quadrant lower left
    '\u{2594}', '\u{2583}', '\u{2713}', '\u{2596}',
    // quadrant upper right, box up+left, quadrant upper left, checkerboard-0
    '\u{259d}', '\u{2518}', '\u{2598}', '\u{2592}',
];

/// Convert string slice to array of screen codes at compile time
/// 
/// Examples:
/// ~~~
/// use mos-hardware::petscii;
/// const SCREEN_CODES: [u8; 4] = petscii!("way!");
/// ~~~
#[macro_export]
macro_rules! petscii {
    ($A:expr) => {{
        use $crate::petscii::PETSCII_TO_CHAR_MAP;

        /// Convert PETSCII to screen code
        /// https://sta.c64.org/cbm64pettoscr.html
        const fn petscii_to_screen_code(petscii: u8) -> u8 {
            match petscii {
                0..=31 => petscii + 128,
                32..=63 => petscii,
                64..=95 => petscii - 64,
                96..=127 => petscii - 32,
                128..=159 => petscii + 64,
                160..=191 => petscii - 64,
                192..=254 => petscii - 128,
                255 => 94,
            }
        }

        /// Convert unicode char to PETSCII byte
        const fn unicode_to_petscii(unicode: char) -> u8 {
            let mut petscii = 0;
            while petscii < PETSCII_TO_CHAR_MAP.len() {
                if unicode == PETSCII_TO_CHAR_MAP[petscii] {
                    return petscii as u8;
                }
                petscii += 1;
            }
            panic!("INVALID LETTER");
        }

        const N: usize = const_str::to_char_array!($A).len();
        const CHARS: [char; N] = const_str::to_char_array!($A);

        let mut screen_codes = [0u8; N];
        let mut i = 0;
        while i < N {
            let petscii = unicode_to_petscii(CHARS[i]);
            screen_codes[i] = petscii_to_screen_code(petscii);
            i += 1;
        }
        screen_codes
    }};
}