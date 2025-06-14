use bitflags::bitflags;
use volatile_register::RW;

bitflags! {
    /// VIC-III control flags for `0xD031`
    pub struct Vic3Control: u8 {
        /// Enable C64 640 horizontal pixels / 80 column mode
        const H640 = 0b1000_0000;
        /// Enable C65 FAST mode (∼3.5MHz)
        const FAST = 0b0100_0000;
        /// Enable extended attributes and 8 bit colour entries
        const ATTR = 0b0010_0000;
        /// Bit-Plane Mode
        const BPM = 0b0001_0000;
        /// Enable 400 vertical pixels
        const V400 = 0b0000_1000;
        /// Enable 1280 horizontal pixels (not implemented)
        const H1280 = 0b0000_0100;
        /// Enable VIC-III MONO composite video output (colour if disabled)
        const MONO = 0b0000_0010;
        /// Enable VIC-III interlaced mode
        const INT = 0b0000_0001;
    }
}

bitflags! {
    /// VIC-IV control flags for `0xD051`
    pub struct Vic4Control: u8 {
        /// Alpha compositor enable
        const ALPHEN = 0b1000_0000;
        /// C65GS FAST mode (48MHz)
        const VFAST = 0b0100_0000;
        /// Enable PAL CRT-like scan-line emulation
        const PALEMU = 0b0010_0000;
        /// Sprite H640 enable
        const SPRH640 = 0b0001_0000;
        /// Video output horizontal smoothing enable
        const SMTH = 0b0000_1000;
        /// Enable full-colour mode for character numbers >$FF
        const FCLRHI = 0b0000_0100;
        /// Enable full-colour mode for character numbers <=$FF
        const FCLRLO = 0b0000_0010;
        /// Alternate char ROM bank on alternate raster lines in V200
        const CHR16 = 0b0000_0001;
    }
}

/// VIC-IV specific registers
#[repr(C)]
pub struct Vic4 {
    /// Offset 0x20
    pub border_color: RW<u8>,
    /// Offset 0x21
    pub screen_color0: RW<u8>,
    /// Multi-colour 1 (256 colour) (`MC1`, offset 0x22)
    pub multicolor1: RW<u8>,
    /// Multi-colour 2 (256 colour) (`MC2`, offset 0x23)
    pub multicolor2: RW<u8>,
    /// Multi-colour 3 (256 colour) (`MC3`, offset 0x24)
    pub multicolor3: RW<u8>,
    /// Offset 0x25
    pub sprite_multicolor0: RW<u8>,
    /// Offset 0x26
    pub sprite_multicolor1: RW<u8>,
    /// Reserved (offset 0x27 to 0x2d)
    pub _reserved1: [u8; 7],
    /// Write `0x47` then `0x53` to enable C65GS/VIC-IV IO registers (offset 0x2f)
    pub key: RW<u8>,
    /// Reserved (offset 0x30 to 0x47)
    pub _reserved2: [u8; 24],
    /// TBDRPOS (offset 0x48)
    pub topborder_pos: RW<u8>,
    /// Sprite bitplane-modify-mode enables, `SPRBPMEN` (offset 0x49)
    pub sprite_bitplane_modify_enable: RW<u8>,
    /// BBDRPOS (offset 0x4a)
    pub bottomborder_pos: RW<u8>,
    /// SPRBPMEN (offset 0x4b)
    pub sprbpmen2: RW<u8>,
    /// Character generator horizontal position TEXTXPOS (offset 0x4c)
    pub textxpos: RW<u8>,
    /// SPRTILEN (offset 0x4d)
    pub sprtilen: RW<u8>,
    /// Character generator vertical position TEXTYPOS (offset 0x4e)
    pub textypos: RW<u8>,
    /// SPRTILEN (offset 0x4f)
    pub sprtilen2: RW<u8>,
    /// XPOSLSB (offset 0x50)
    pub xpos_lsb: RW<u8>,
    /// NORRDEL (offset 0x51)
    pub norrdel: RW<u8>,
    /// FNRASTERLSB (offset 0x52)
    pub fnraster_lsb: RW<u8>,
    /// VIC-IV Control register (offset 0x53)
    pub ctrl1: RW<u8>,
    /// VIC-IV Control register C (offset 0x54)
    pub ctrl_c: RW<u8>,
    /// Sprite extended height enable (one bit per sprite) (`SPRHGTEN`, offset 0x55)
    pub sprite_extended_height_enable: RW<u8>,
    /// Sprite extended height size (sprite pixels high) (`SPRHGHT`, offset 0x56)
    pub sprite_height: RW<u8>,
    /// Sprite extended width enables (offset 0x57)
    pub sprite_x64en: RW<u8>,
    /// Characters per logical text row (`LINESTEP`, offset 0x58-0x59)
    pub linestep: RW<u16>,
    /// Horizontal hardware scale of text mode (pixel 120ths per pixel) (offset 0x5A)
    pub chrxscl: RW<u8>,
    /// Vertical scaling of text mode (number of physical rasters per char text row) (offset 0x5B)
    pub chryscl: RW<u8>,
    /// SDBDRWDLSB (offset 0x5C)
    pub sdbdrwd_lsb: RW<u8>,
    /// HOTREG (offset 0x5D)
    pub hotreg: RW<u8>,
    /// Number of characters to display per row, LSB (offset: 0x5E)
    pub chrcount: RW<u8>,
    /// Sprite H640 X Super-MSBs (`SPRXSMSBS`, offset 0x5F)
    pub spr_xsmsbs: RW<u8>,
    /// Screen RAM precise base address (SCRNPTRLSB, offset 0x60)
    pub scrnptr_lsb: RW<u8>,
    /// Screen RAM precise base address (SCRNPTRMSB, offset 0x61)
    pub scrnptr_msb: RW<u8>,
    /// Screen RAM precise base address (SCRNPTRBNK, offset 0x62)
    pub scrnptr_bnk: RW<u8>,
    /// EXGLYPH (offset 0x63)
    pub exglyph: RW<u8>,
    /// Colour RAM base address, `COLPTR` (offset 0x64-0x65)
    pub colptr_lsb: RW<u16>,
    /// Reserved (offset 0x66 to 0x67)
    pub _reserved3: [u8; 2],
    /// CHARPTRLSB (offset 0x68)
    pub charptr_lsb: RW<u8>,
    /// CHARPTRMSB (offset 0x69)
    pub charptr_msb: RW<u8>,
    /// CHARPTRMSB (offset 0x6A)
    pub charptr_bnk: RW<u8>,
    /// SPR16EN (offset 0x6B)
    pub spr16en: RW<u8>,
    /// Sprite pointer address, `SPRPTRADR` (offset 0x6C - ox6D)
    pub sprite_ptr_address: RW<u16>,
    /// SPRPTR16EN (offset 0x6E)
    pub sprptr16en: RW<u8>,
    /// PALNTSC (offset 0x6F)
    pub palntsc: RW<u8>,
    /// VIC-IV control register 2 (offset 0x70)
    pub ctrl2: RW<u8>,
    /// VIC-IV 16-colour bitplane enable flags (offset 0x71)
    pub bp16ens: RW<u8>,
    /// Sprite Y position adjustment (offset 0x72)
    pub spr_yadj: RW<u8>,
    /// Alpha delay and raster height (offset 0x73)
    pub alphadelay: RW<u8>,
    /// Sprite alpha-blend enable (offset 0x74)
    pub spr_enalpha: RW<u8>,
    /// Sprite alpha-blend value (offset 0x75)
    pub spr_alphaval: RW<u8>,
    /// Sprite V400 enables (offset 0x76)
    pub spr_env400: RW<u8>,
    /// Sprite V400 Y position MSBs (offset 0x77)
    pub spr_ymsbs: RW<u8>,
    /// Sprite V400 Y position super MSBs (offset 0x78)
    pub spr_ysmsbs: RW<u8>,
    /// Raster compare value (offset 0x79)
    pub rstcmp: RW<u8>,
    /// VIC-IV control register 3 (offset 0x7A)
    pub ctrl3: RW<u8>,
    /// Number of text rows to display (offset 0x7B)
    pub disp_rows: RW<u8>,
    /// hsync/vsync polarity (offset 0x7C)
    pub debugc: RW<u8>,
}

/// __vic4__bindgen_ty_7__bindgen_ty_1
pub struct Vic4BindgenTy7BindgenTy1 {
    ///Display Address Translater (DAT) Bitplane 0 port (offset 0x40)
    pub b0pix: u8,
    ///Display Address Translater (DAT) Bitplane 1 port (offset 0x41)
    pub b1pix: u8,
    ///Display Address Translater (DAT) Bitplane 2 port (offset 0x42)
    pub b2pix: u8,
    ///Display Address Translater (DAT) Bitplane 3 port (offset 0x43)
    pub b3pix: u8,
    ///Display Address Translater (DAT) Bitplane 4 port (offset 0x44)
    pub b4pix: u8,
    ///Display Address Translater (DAT) Bitplane 5 port (offset 0x45)
    pub b5pix: u8,
    ///Display Address Translater (DAT) Bitplane 6 port (offset 0x46)
    pub b6pix: u8,
    ///Display Address Translater (DAT) Bitplane 7 port (offset 0x47)
    pub b7pix: u8,
}
