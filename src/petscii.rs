//
// Copyright 2022 Mikael Lund aka Wombat
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

//! Utility functions for working with single PETSCII characters
//!
//! This is based on the following resources:
//! - PETSCII to unicode look-up table from <https://github.com/simmons/cbm>
//!   which in turn is from <https://sta.c64.org/cbm64pettoscr.html>.
//! - PETSCII to screen code conversion based on <https://sta.c64.org/cbm64pettoscr.html>.

use core::fmt;

/// The Unicode code point we use for untranslatable PETSCII characters.
pub const NONE: char = char::REPLACEMENT_CHARACTER;

/// From: http://style64.org/petscii/
#[rustfmt::skip]
const PETSCII_TO_CHAR_MAP: [char; 256] = [
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

/// Structure for working with single PETSCII characters
///
/// # Examples
/// ~~~
/// use mos_hardware::petscii::Petscii;
///
/// let a = Petscii::default();
/// assert_eq!(u8::from(a), 0);
///
/// let byte: u8 = Petscii::from(1).into(); // u8 -> petscii -> u8
/// assert_eq!(byte, 1);
///
/// let c: Petscii = 'c'.into();
/// assert_eq!(u8::from(c), 67);
/// assert_eq!(c.to_char(), 'c');
/// assert_eq!(char::from(c), 'c');
///
/// let unicode: char = c.into();
/// assert_eq!(unicode, 'c');
///
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Petscii(u8);

impl Petscii {
    /// Create from byte
    pub const fn from_byte(byte: u8) -> Petscii {
        Petscii(byte)
    }

    /// Create from unicode character
    pub const fn from_char(letter: char) -> Petscii {
        let mut petscii = 0;
        while petscii < PETSCII_TO_CHAR_MAP.len() {
            if letter == PETSCII_TO_CHAR_MAP[petscii] {
                return Petscii::from_byte(petscii as u8);
            }
            petscii += 1;
        }
        panic!("INVALID LETTER");
    }

    /// Convert PETSCII to screen code
    ///
    /// See <https://sta.c64.org/cbm64pettoscr.html>
    ///
    /// # Examples
    /// ~~~
    /// use mos_hardware::petscii::Petscii;
    /// let value = Petscii::from('c');
    /// assert_eq!(value.to_byte(), 67);
    /// assert_eq!(value.to_screen_code(), 3);
    /// ~~~
    pub const fn to_screen_code(&self) -> u8 {
        match self.0 {
            0..=31 => self.0 + 128,
            32..=63 => self.0,
            64..=95 => self.0 - 64,
            96..=127 => self.0 - 32,
            128..=159 => self.0 + 64,
            160..=191 => self.0 - 64,
            192..=254 => self.0 - 128,
            255 => 94,
        }
    }

    /// Convert to unicode
    pub const fn to_char(&self) -> char {
        PETSCII_TO_CHAR_MAP[self.0 as usize]
    }

    /// Convert to byte
    pub const fn to_byte(&self) -> u8 {
        self.0
    }
}

impl From<Petscii> for char {
    fn from(petscii: Petscii) -> Self {
        petscii.to_char()
    }
}

impl From<Petscii> for u8 {
    fn from(petscii: Petscii) -> Self {
        petscii.0
    }
}

impl From<u8> for Petscii {
    fn from(value: u8) -> Self {
        Petscii::from_byte(value)
    }
}

impl From<char> for Petscii {
    fn from(value: char) -> Self {
        Petscii::from_char(value)
    }
}

/// Display as char
impl fmt::Display for Petscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Convert string slice to array of screen codes at _compile time_
///
/// Examples
/// ~~~
/// use mos_hardware::screen_codes;
/// const SCREEN_CODES: [u8; 4] = screen_codes!("way!");
/// assert_eq!(SCREEN_CODES, [23, 1, 25, 33]);
/// ~~~
#[macro_export]
macro_rules! screen_codes {
    ($A:expr) => {{
        use $crate::petscii::*;
        const N: usize = const_str::to_char_array!($A).len();
        const CHARS: [char; N] = const_str::to_char_array!($A);
        let mut screen_codes = [0u8; N];
        let mut i = 0;
        while i < N {
            let petscii = Petscii::from_char(CHARS[i]);
            screen_codes[i] = petscii.to_screen_code();
            i += 1;
        }
        screen_codes
    }};
}

/// As `screen_codes!` but null-terminated
///
/// # Examples
/// ~~~
/// use mos_hardware::screen_codes_null;
/// const SCREEN_CODES: [u8; 5] = screen_codes_null!("way!");
/// assert_eq!(SCREEN_CODES, [23, 1, 25, 33, 0]);
/// ~~~
#[macro_export]
macro_rules! screen_codes_null {
    ($A:expr) => {{
        use $crate::screen_codes;
        *const_str::concat_bytes!(screen_codes!($A), 0u8)
    }};
}

/// Convert string slice to array of petscii bytes at _compile time_
///
/// Examples
/// ~~~
/// use mos_hardware::petscii_codes;
/// const PETSCII_BYTES: [u8; 4] = petscii_codes!("way!");
/// ~~~
#[macro_export]
macro_rules! petscii_codes {
    ($A:expr) => {{
        use $crate::petscii::*;
        const N: usize = const_str::to_char_array!($A).len();
        const CHARS: [char; N] = const_str::to_char_array!($A);
        let mut petscii_bytes = [0u8; N];
        let mut i = 0;
        while i < N {
            let petscii = Petscii::from_char(CHARS[i]);
            screen_codes[i] = petscii.to_byte();
            i += 1;
        }
        petscii_bytes
    }};
}

/// As `petscii_codes!` but null-terminated
#[macro_export]
macro_rules! petscii_codes_null {
    ($A:expr) => {{
        use $crate::petscii_codes;
        *const_str::concat_bytes!(petscii_codes!($A), 0u8)
    }};
}
