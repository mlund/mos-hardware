use bitflags::bitflags;
use core::mem::size_of;
use static_assertions::const_assert;
use volatile_register::RW;

pub struct Cpu6510 {
    direction: RW<CpuPortDdrFlags>,
    register: RW<CpuPortFlags>,
}

const_assert!(size_of::<Cpu6510>() == 2);

impl Cpu6510 {
    // Kernal like reset
    pub fn reset(&self) {
        unsafe {
            // Set up 6510 processor port
            // Motor off, HIRAM, LOWRAM, CHAREN high
            self.register.write(CpuPortFlags::DEFAULT);
            // Motor out, switch in, write out, control out
            self.direction.write(CpuPortDdrFlags::DEFAULT);
        }
    }
}

bitflags! {
    /// Data Direction Register flags for the CPU port `R6510` at 0x0000
    ///
    /// This register controls which bits of the CPU port (0x0001) are inputs or outputs.
    /// A bit set to 1 configures the corresponding bit in 0x0001 as an output,
    /// while a bit set to 0 configures it as an input.
    ///
    /// The C64 uses a 6-bit I/O port (bits 0-5): bits 0-2 for memory banking control
    /// and bits 3-5 for datasette (tape) control. Bits 6-7 are not connected/used.
    ///
    /// Default configuration sets bits 0-5 as outputs, bits 6-7 are unused.
    ///
    /// [More information](https://codebase64.org/doku.php?id=base:memory_management).
    ///
    /// # Examples
    ///
    /// Configure the standard C64 setup with 6-bit I/O port:
    /// ```
    /// (*CPU_PORT_DDR).write(CpuPortDdrFlags::DEFAULT);
    /// assert_eq!(CpuPortDdrFlags::DEFAULT.bits(), 0x2F);
    /// ```
    ///
    /// Configure only memory banking bits as outputs:
    /// ```
    /// (*CPU_PORT_DDR).write(CpuPortDdrFlags::MEMORY_BANKING_ONLY);
    /// assert_eq!(CpuPortDdrFlags::MEMORY_BANKING_ONLY.bits(), 0x07);
    /// ```
    pub struct CpuPortDdrFlags: u8 {
        /// Default configuration: memory banking and datasette motor as output, datasette button as input
        const DEFAULT                = 0b0010_1111;

        // Individual bit flags for output configuration (bits 0-5 only)
        /// Bit 0 output: LORAM control
        const LORAM_OUTPUT           = 0b0000_0001;

        /// Bit 1 output: HIRAM control
        const HIRAM_OUTPUT           = 0b0000_0010;

        /// Bit 2 output: CHAREN control
        const CHAREN_OUTPUT          = 0b0000_0100;

        /// Bit 3 output: Datasette signal
        const DATASETTE_SIGNAL_OUTPUT = 0b0000_1000;

        /// Bit 4 output: Datasette button sense
        const DATASETTE_BUTTON_OUTPUT = 0b0001_0000;

        /// Bit 5 output: Datasette motor control
        const DATASETTE_MOTOR_OUTPUT = 0b0010_0000;
    }
}

/// Pointer to the `D6510` Data Direction Register (0x0000)
pub const CPU_PORT_DDR: *mut RW<CpuPortDdrFlags> = (0x0000) as _;

bitflags! {
    /// Control flags for the CPU port `R6510` at 0x0001
    ///
    /// The 6510 CPU has a 6-bit I/O port (bits 0-5). Bits 6-7 are not connected in the C64.
    /// Three-word combination constants like `RAM_IO_KERNAL` refer to banking configurations
    /// of what is visible at addresses `0xA000-0xBFFF`, `0xD000-0xDFFF`, and `0xE000-0xFFFF`.
    /// Regardless of `0x0001`, the VIC-II chip *always* sees the `CHARROM` at `0x1000-0x1FFF` and `0x9000-0x9FFF`,
    /// and RAM everywhere else.
    ///
    /// Memory banking is controlled by bits 0-2:
    /// - Bit 0: LORAM (0=BASIC ROM disabled, 1=BASIC ROM enabled)
    /// - Bit 1: HIRAM (0=KERNAL ROM disabled, 1=KERNAL ROM enabled)
    /// - Bit 2: CHAREN (0=Character ROM at 0xD000 if any ROM active, otherwise RAM; 1=I/O at 0xD000)
    ///
    /// **For 0xD000-0xDFFF, the logic is:**
    /// 1. If **CHAREN = 1** → **I/O** (always, regardless of LORAM/HIRAM)
    /// 2. If **CHAREN = 0** AND **at least one ROM active** → **Character ROM**
    /// 3. If **CHAREN = 0** AND **no ROM active** → **RAM**
    /// **The critical special case:** When CHAREN = 0 and LORAM = HIRAM = 0 (RAM_RAM_RAM configuration), you get RAM at 0xD000 instead of Character ROM, which breaks the simple logic "CHAREN = 0 → Character ROM".
    ///
    /// Datasette control uses bits 3-5:
    /// - Bit 3: Datasette signal (usually input)
    /// - Bit 4: Datasette button sense (0=button pressed)
    /// - Bit 5: Datasette motor (0=motor on, 1=motor off)
    ///
    /// [More information](https://codebase64.org/doku.php?id=base:memory_management).
    ///
    /// # Examples
    ///
    /// Here's an example that makes the RAM available "under" both the BASIC and KERNAL
    /// ROMs located at 0xA000-0xBFFF and 0xE000-0xFFFF.
    /// The VIC, SID, and CIA I/O devices are left accessible at 0xD000-0xDFFF:
    /// ```
    /// (*CPU_PORT).write(CpuPortFlags::RAM_IO_RAM);
    /// assert_eq!(CpuPortFlags::RAM_IO_RAM.bits(), 0x25);
    /// assert_eq!(CpuPortFlags::RAM_IO_KERNAL.bits(), 0x26);
    /// ```
    pub struct CpuPortFlags: u8 {
        /// Default C64 configuration: BASIC ROM, I/O, KERNAL ROM visible, datasette motor off
        const DEFAULT              = 0b1111_0111;

        // Memory banking configurations (bits 0-2)
        /// BASIC ROM, I/O area, KERNAL ROM all visible
        const BASIC_IO_KERNAL      = 0b0011_0111;

        /// RAM visible everywhere (no ROMs, no I/O)
        const RAM_RAM_RAM          = 0b0011_0000;

        /// RAM at $A000-BFFF, Character ROM at $D000-DFFF, RAM at $E000-FFFF
        const RAM_CHAR_RAM         = 0b0011_0001;

        /// RAM at $A000-BFFF, Character ROM at $D000-DFFF, KERNAL ROM at $E000-FFFF
        const RAM_CHAR_KERNAL      = 0b0011_0010;

        /// BASIC ROM at $A000-BFFF, Character ROM at $D000-DFFF, KERNAL ROM at $E000-FFFF
        const BASIC_CHAR_KERNAL    = 0b0011_0011;

        /// RAM at $A000-BFFF, I/O at $D000-DFFF, RAM at $E000-FFFF
        const RAM_IO_RAM           = 0b0011_0101;

        /// RAM at $A000-BFFF, I/O at $D000-DFFF, KERNAL ROM at $E000-FFFF
        const RAM_IO_KERNAL        = 0b0011_0110;

        // Individual memory banking bits
        /// Bit 0: LORAM 0xA000-0xBFFF - BASIC ROM control (1=enabled, 0=disabled)
        const LORAM                = 0b0000_0001;

        /// Bit 1: HIRAM 0xE000-0xFFFF - KERNAL ROM control (1=enabled, 0=disabled)
        const HIRAM                = 0b0000_0010;

        /// Bit 2: CHAREN 0xD000-0xDFFF - Character ROM/I/O control (1=I/O, 0=Character ROM if any ROM active, otherwise RAM)
        const CHAREN               = 0b0000_0100;

        // Datasette control bits (bits 3-5)
        /// Bit 3: Datasette signal input
        const DATASETTE_SIGNAL     = 0b0000_1000;

        /// Bit 4: Datasette button sense (0=pressed, 1=not pressed)
        const DATASETTE_BUTTON_OFF = 0b0001_0000;

        /// Bit 5: Datasette motor control (0=on, 1=off)
        const DATASETTE_MOTOR_OFF  = 0b0010_0000;
    }
}

/// Pointer to the `R6510` register for 6510 I/O (0x0001)
pub const CPU_PORT: *mut RW<CpuPortFlags> = (0x0001) as _;
