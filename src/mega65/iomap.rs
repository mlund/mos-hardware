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

//! Automatically generated constants from `iomap.txt`
//!
//! The `iomap.txt` file is found in the
//! [`mega65-core`](https://github.com/MEGA65/mega65-core/blob/development/iomap.txt) repository.
//! The core is still under development so expect the contained values and
//! names to change over time.

pub mod audio {

    /// Audio Mixer register select [0xD6F4: MIXREGSEL]
    pub const AUDIO_MIXER_REGISTER_SELECT: *mut u8 = (0xD6F4) as *mut u8;

    /// Audio Mixer register read port [0xD6F5: MIXREGDATA]
    pub const AUDIO_MIXER_REGISTER_READ_PORT: *mut u8 = (0xD6F5) as *mut u8;

    /// Digital audio, left channel, LSB [0xD6F8: DIGILEFTLSB]
    pub const DIGITAL_AUDIO: *mut u8 = (0xD6F8) as *mut u8;

    /// 16-bit digital audio out (left LSB) [0xD6F8: DIGILLSB]
    pub const OUT_DIGITAL_AUDIO_16_BIT: *mut u8 = (0xD6F8) as *mut u8;

    /// audio read-back LSB (source selected by $D6F4) [0xD6FC: READBACKLSB]
    pub const AUDIO_READ_BACK_LSB: *mut u8 = (0xD6FC) as *mut u8;

    /// audio read-back MSB (source selected by $D6F4) [0xD6FD: READBACKMSB]
    pub const AUDIO_READ_BACK_MSB: *mut u8 = (0xD6FD) as *mut u8;

    /// PWM/PDM audio encoding select [0xD711: PWMPDM]
    pub const PWM_SLASH_PDM_AUDIO_ENCODING_SELECT_MASK: u8 = 0b00001000;
}

pub mod audiomix {

    /// Enable DC offset subtraction in audio mixer [0xD63C: DCTRKEN]
    pub const ENABLE_DC_OFFSET_SUBTRACTION_IN_AUDIO_MIXER_MASK: u8 = 0b00010000;

    /// Audio mixer DC-estimation time step. Lower values = faster updating of DC estimation, at the cost of making low-frequencies quieter. [0xD63D: DCTIME]
    pub const AUDIO_MIXER_DC_ESTIMATION_TIME_STEP: *mut u8 = (0xD63D) as *mut u8;

    /// Audio Mixer register write port [0xD6F5: REGWDATA]
    pub const AUDIO_MIXER_REGISTER_WRITE_PORT: *mut u8 = (0xD6F5) as *mut u8;
}

pub mod auxfpga {

    /// LSB of Auxilliary (MAX10) FPGA design date stamp (days since 1 Jan 2020) [0xD636: FWDATEL]
    pub const LSB_OF_AUXILLIARY: *mut u8 = (0xD636) as *mut u8;

    /// MSB of Auxilliary (MAX10) FPGA design date stamp (days since 1 Jan 2020) [0xD637: MFWDATEH]
    pub const MSB_OF_AUXILLIARY: *mut u8 = (0xD637) as *mut u8;

    /// 2nd byte of Auxilliary (MAX10) FPGA design git commit [0xD639: FWGIT0]
    pub const AUXILLIARY_BYTE_OF_2ND: *mut u8 = (0xD639) as *mut u8;

    /// 3rd byte of Auxilliary (MAX10) FPGA design git commit [0xD63A: FWGIT0]
    pub const AUXILLIARY_BYTE_OF_3RD: *mut u8 = (0xD63A) as *mut u8;
}

pub mod cia1 {

    /// Port A [0xDC00: PORTA]
    pub const PORT_A: *mut u8 = (0xDC00) as *mut u8;

    /// Port B [0xDC01: PORTB]
    pub const PORT_B: *mut u8 = (0xDC01) as *mut u8;

    /// Port A DDR [0xDC02: DDRA]
    pub const PORT_A_DDR: *mut u8 = (0xDC02) as *mut u8;

    /// Port B DDR [0xDC03: DDRB]
    pub const PORT_B_DDR: *mut u8 = (0xDC03) as *mut u8;

    /// Timer A counter (16 bit) [0xDC04: TIMERA]
    pub const TIMER_A_COUNTER: *mut u8 = (0xDC04) as *mut u8;

    /// Timer B counter (16 bit) [0xDC06: TIMERB]
    pub const TIMER_B_COUNTER: *mut u8 = (0xDC06) as *mut u8;

    /// TOD tenths of seconds [0xDC08: TODJIF]
    pub const TOD_TENTHS_OF_SECONDS_MASK: u8 = 0b00001111;

    /// TOD seconds [0xDC09: TODSEC]
    pub const TOD_SECONDS_MASK: u8 = 0b00111111;

    /// TOD minutes [0xDC0A: TODMIN]
    pub const TOD_MINUTES_MASK: u8 = 0b00111111;

    /// TOD hours [0xDC0B: TODHOUR]
    pub const TOD_HOURS_MASK: u8 = 0b00011111;

    /// TOD PM flag [0xDC0B: TOD]
    pub const TOD_PM_FLAG_MASK: u8 = 0b10000000;

    /// shift register data register(writing starts sending) [0xDC0C: SDR]
    pub const SHIFT_REGISTER_DATA_REGISTER: *mut u8 = (0xDC0C) as *mut u8;

    /// Timer A underflow [0xDC0D: TA]
    pub const TIMER_A_UNDERFLOW_MASK: u8 = 0b00000001;

    /// Timer B underflow [0xDC0D: TB]
    pub const TIMER_B_UNDERFLOW_MASK: u8 = 0b00000010;

    /// TOD alarm [0xDC0D: ALRM]
    pub const TOD_ALARM_MASK: u8 = 0b00000100;

    /// shift register full/empty [0xDC0D: SP]
    pub const SHIFT_REGISTER_FULL_SLASH_EMPTY_MASK: u8 = 0b00001000;

    /// FLAG edge detected [0xDC0D: FLG]
    pub const FLAG_EDGE_DETECTED_MASK: u8 = 0b00010000;

    /// Placeholder - Reading clears events [0xDC0D: ISRCLR]
    pub const PLACEHOLDER___READING_CLEARS_EVENTS_MASK: u8 = 0b00000011;

    /// Interrupt flag [0xDC0D: IR]
    pub const INTERRUPT_FLAG_MASK: u8 = 0b10000000;

    /// Timer A start [0xDC0E: STRTA]
    pub const TIMER_A_START_MASK: u8 = 0b00000001;

    /// Timer A PB6 out [0xDC0E: PBONA]
    pub const TIMER_A_PB6_OUT_MASK: u8 = 0b00000010;

    /// Timer A toggle or pulse [0xDC0E: OMODA]
    pub const TIMER_A_TOGGLE_OR_PULSE_MASK: u8 = 0b00000100;

    /// Timer A one-shot mode [0xDC0E: RMODA]
    pub const TIMER_A_ONE_SHOT_MODE_MASK: u8 = 0b00001000;

    /// Timer A Timer A tick source [0xDC0E: IMODA]
    pub const TIMER_A_TIMER_A_TICK_SOURCE_MASK: u8 = 0b00100000;

    /// Serial port direction [0xDC0E: SPMOD]
    pub const SERIAL_PORT_DIRECTION_MASK: u8 = 0b01000000;

    /// 50/60Hz select for TOD clock [0xDC0E: TOD50]
    pub const CLOCK_SELECT_FOR_TOD_50_SLASH_60HZ_MASK: u8 = 0b10000000;

    /// Timer B start [0xDC0F: STRTB]
    pub const TIMER_B_START_MASK: u8 = 0b00000001;

    /// Timer B PB7 out [0xDC0F: PBONB]
    pub const TIMER_B_PB7_OUT_MASK: u8 = 0b00000010;

    /// Timer B toggle or pulse [0xDC0F: OMODB]
    pub const TIMER_B_TOGGLE_OR_PULSE_MASK: u8 = 0b00000100;

    /// Timer B one-shot mode [0xDC0F: RMODB]
    pub const TIMER_B_ONE_SHOT_MODE_MASK: u8 = 0b00001000;

    /// Strobe input to force-load timers [0xDC0F: LOAD]
    pub const STROBE_INPUT_TO_FORCE_LOAD_TIMERS_MASK: u8 = 0b00010000;

    /// Timer B Timer A tick source [0xDC0F: IMODB]
    pub const TIMER_B_TIMER_A_TICK_SOURCE_MASK: u8 = 0b00000011;

    /// TOD alarm edit [0xDC0F: TODEDIT]
    pub const TOD_ALARM_EDIT_MASK: u8 = 0b10000000;

    /// Timer A latch value (16 bit) [0xDC10: TALATCH]
    pub const TIMER_A_LATCH_VALUE: *mut u8 = (0xDC10) as *mut u8;

    /// Timer B latch value (16 bit) [0xDC12: TALATCH]
    pub const TIMER_B_LATCH_VALUE: *mut u8 = (0xDC12) as *mut u8;

    /// Timer A current value (16 bit) [0xDC14: TALATCH]
    pub const TIMER_A_CURRENT_VALUE: *mut u8 = (0xDC14) as *mut u8;

    /// Timer B current value (16 bit) [0xDC16: TALATCH]
    pub const TIMER_B_CURRENT_VALUE: *mut u8 = (0xDC16) as *mut u8;

    /// TOD 10ths of seconds value [0xDC18: TOD]
    pub const TOD_10THS_OF_SECONDS_VALUE_MASK: u8 = 0b00001111;

    /// Interrupt mask for Timer B [0xDC18: IMTB]
    pub const INTERRUPT_MASK_FOR_TIMER_B_MASK: u8 = 0b00010000;

    /// Interrupt mask for TOD alarm [0xDC18: IM]
    pub const INTERRUPT_MASK_FOR_TOD_ALARM_MASK: u8 = 0b00100000;

    /// Interrupt mask for shift register (serial port) [0xDC18: IMSP]
    pub const INTERRUPT_MASK_FOR_SHIFT_REGISTER_MASK: u8 = 0b01000000;

    /// Interrupt mask for FLAG line [0xDC18: IMFLG]
    pub const INTERRUPT_MASK_FOR_FLAG_LINE_MASK: u8 = 0b10000000;

    /// TOD Alarm seconds value [0xDC19: TODSEC]
    pub const TOD_ALARM_SECONDS_VALUE: *mut u8 = (0xDC19) as *mut u8;

    /// TOD Alarm minutes value [0xDC1A: TODMIN]
    pub const TOD_ALARM_MINUTES_VALUE: *mut u8 = (0xDC1A) as *mut u8;

    /// TOD hours value [0xDC1B: TOD]
    pub const TOD_HOURS_VALUE_MASK: u8 = 0b01111111;

    /// TOD AM/PM flag [0xDC1B: TOD]
    pub const TOD_AM_SLASH_PM_FLAG_MASK: u8 = 0b10000000;

    /// TOD Alarm 10ths of seconds value (actually all 8 bits) [0xDC1C: ALRMJIF]
    pub const TOD_ALARM_10THS_OF_SECONDS_VALUE_MASK: u8 = 0b01111111;

    /// Enable delaying writes to $DD00 by 3 cycles to match real 6502 timing [0xDC1C: DD00]
    pub const ENABLE_DELAYING_WRITES_TO_0XDD00_BY_3_CYCLES_TO_MATCH_REAL_6502_TIMING_MASK: u8 =
        0b10000000;

    /// TOD Alarm hours value [0xDC1F: ALRM]
    pub const TOD_ALARM_HOURS_VALUE_MASK: u8 = 0b01111111;

    /// TOD Alarm AM/PM flag [0xDC1F: ALRM]
    pub const TOD_ALARM_AM_SLASH_PM_FLAG_MASK: u8 = 0b10000000;
}

pub mod cpu {

    /// 6510/45GS10 CPU port DDR [0x0000000: PORTDDR]
    pub const DDR_CPU_PORT_6510_SLASH_45GS10: *mut u8 = (0x0000000) as *mut u8;

    /// 6510/45GS10 CPU port data [0x0000001: PORT]
    pub const DATA_CPU_PORT_6510_SLASH_45GS10: *mut u8 = (0x0000001) as *mut u8;

    /// Writing triggers hypervisor trap \$XX [0xD640: HTRAP00]
    pub const WRITING_TRIGGERS_HYPERVISOR_TRAP_0XXX: *mut u8 = (0xD640) as *mut u8;

    /// @HTRAPXX [0xD641: HTRAP01]
    pub const HTRAPXX: *mut u8 = (0xD641) as *mut u8;

    /// 1=charge extra cycle(s) for branches taken [0xD710: BRCOST]
    pub const CHARGE_EXTRA_CYCLE_MASK: u8 = 0b00001000;

    /// Cost of badlines minus 40. ie. 00=40 cycles, 11 = 43 cycles. [0xD710: BADEXTRA]
    pub const COST_OF_BADLINES_MINUS_40_MASK: u8 = 0b00000011;

    /// IEC bus is active [0xD7F1: IECBUSACT]
    pub const IEC_BUS_IS_ACTIVE_MASK: u8 = 0b00000001;

    /// Count the number of PHI cycles per video frame (LSB) [0xD7F2: PHIPERFRAME]
    pub const COUNT_THE_NUMBER_OF_PHI_CYCLES_PER_VIDEO_FRAME: *mut u8 = (0xD7F2) as *mut u8;

    /// Count the number of usable (proceed=1) CPU cycles per video frame (LSB) [0xD7F6: CYCPERFRAME]
    pub const COUNT_THE_NUMBER_OF_USABLE: *mut u8 = (0xD7F6) as *mut u8;

    /// Count number of elapsed video frames [0xD7FA: FRAMECOUNT]
    pub const COUNT_NUMBER_OF_ELAPSED_VIDEO_FRAMES: *mut u8 = (0xD7FA) as *mut u8;

    /// 1= enable cartridges [0xD7FB: CARTEN]
    pub const ENABLE_CARTRIDGES_MASK: u8 = 0b00000010;

    /// Set to zero to power off computer on supported systems. WRITE ONLY. [0xD7FD: POWEREN]
    pub const SET_TO_ZERO_TO_POWER_OFF_COMPUTER_ON_SUPPORTED_SYSTEMS_MASK: u8 = 0b00000001;

    /// Override for /GAME : Must be 0 to enable /GAME signal [0xD7FD: NOGAME]
    pub const OVERRIDE_FOR__SLASH_GAME_MASK: u8 = 0b01000000;

    /// Override for /EXROM : Must be 0 to enable /EXROM signal [0xD7FD: NOEXROM]
    pub const OVERRIDE_FOR__SLASH_EXROM_MASK: u8 = 0b10000000;

    /// Enable expansion RAM pre-fetch logic [0xD7FE: PREFETCH]
    pub const ENABLE_EXPANSION_RAM_PRE_FETCH_LOGIC_MASK: u8 = 0b00000001;

    /// Enable Ocean Type A cartridge emulation [0xD7FE: OCEANA]
    pub const ENABLE_OCEAN_TYPE_A_CARTRIDGE_EMULATION_MASK: u8 = 0b00000010;
}

pub mod debug {

    /// Sprite/bitplane first X DEBUG WILL BE REMOVED [0xD067: SBPDEBUG]
    pub const SPRITE_SLASH_BITPLANE_FIRST_X_DEBUG_WILL_BE_REMOVED: *mut u8 = (0xD067) as *mut u8;

    /// VIC-IV debug value read-back (read only) [0xD07D: DEBUGOUT]
    pub const VIC_IV_DEBUG_VALUE_READ_BACK: *mut u8 = (0xD07D) as *mut u8;

    /// VIC-IV debug X position (LSB) (write only) [0xD07D: DEBUGX]
    pub const VIC_IV_DEBUG_X_POSITION: *mut u8 = (0xD07D) as *mut u8;

    /// VIC-IV debug Y position (LSB) [0xD07E: DEBUGY]
    pub const VIC_IV_DEBUG_Y_POSITION: *mut u8 = (0xD07E) as *mut u8;

    /// VIC-IV debug out-of-frame signal enable [0xD07F: DEBUGOOF]
    pub const VIC_IV_DEBUG_OUT_OF_FRAME_SIGNAL_ENABLE_MASK: u8 = 0b10000000;

    /// Count of cartridge port memory accesses (read only) [0xD613: CRTACSCNT]
    pub const COUNT_OF_CARTRIDGE_PORT_MEMORY_ACCESSES: *mut u8 = (0xD613) as *mut u8;

    /// 8-bit segment of combined keyboard matrix (READ) [0xD614: KEYMATRIXPEEK]
    pub const MATRIX_SEGMENT_OF_COMBINED_KEYBOARD_8_BIT: *mut u8 = (0xD614) as *mut u8;

    /// READ 1351/amiga mouse auto detection DEBUG [0xD61B: AMIMOUSDETECT]
    pub const READ_1351_SLASH_AMIGA_MOUSE_AUTO_DETECTION_DEBUG: *mut u8 = (0xD61B) as *mut u8;

    /// internal 1541 PC LSB [0xD61C: PCLSB_1541]
    pub const INTERNAL_1541_PC_LSB: *mut u8 = (0xD61C) as *mut u8;

    /// DUPLICATE Modifier key state (hardware accelerated keyboard scanner). [0xD61F: BUCKYCOPY]
    pub const DUPLICATE_MODIFIER_KEY_STATE: *mut u8 = (0xD61F) as *mut u8;

    /// READ ONLY flags for paddles. See c65uart.vhdl for more information. [0xD624: POTDEBUG]
    pub const READ_ONLY_FLAGS_FOR_PADDLES: *mut u8 = (0xD624) as *mut u8;

    /// Source of last CPU reset [0xD63C: RESETSRC]
    pub const SOURCE_OF_LAST_CPU_RESET_MASK: u8 = 0b00000111;

    /// Status of M65 R3 J21 pins [0xD69B: J21INL]
    pub const STATUS_OF_M65_R3_J21_PINS: *mut u8 = (0xD69B) as *mut u8;

    /// Status of M65 R3 DIP switches [0xD69D: DIPSW]
    pub const STATUS_OF_M65_R3_DIP_SWITCHES: *mut u8 = (0xD69D) as *mut u8;

    /// Status of switches 0 to 7 [0xD69E: SWSTATUS]
    pub const STATUS_OF_SWITCHES_0_TO_7: *mut u8 = (0xD69E) as *mut u8;

    /// Status of switches 8 to 15 [0xD69F: SWSTATUS]
    pub const STATUS_OF_SWITCHES_8_TO_15: *mut u8 = (0xD69F) as *mut u8;
}

pub mod dma {

    /// DMAgic DMA list address LSB, and trigger DMA (when written) [0xD700: ADDRLSBTRIG]
    pub const DMAGIC_DMA_LIST_ADDRESS_LSB: *mut u8 = (0xD700) as *mut u8;

    /// DMA list address high byte (address bits 8 -- 15). [0xD701: ADDRMSB]
    pub const DMA_LIST_ADDRESS_HIGH_BYTE: *mut u8 = (0xD701) as *mut u8;

    /// DMA list address bank (address bits 16 -- 22). Writing clears \$D704. [0xD702: ADDRBANK]
    pub const DMA_LIST_ADDRESS_BANK: *mut u8 = (0xD702) as *mut u8;

    /// DMA enable F018B mode (adds sub-command byte) [0xD703: EN018B]
    pub const DMA_ENABLE_F018B_MODE_MASK: u8 = 0b00000001;

    /// DMA list address mega-byte [0xD704: ADDRMB]
    pub const DMA_LIST_ADDRESS_MEGA_BYTE: *mut u8 = (0xD704) as *mut u8;

    /// Set low-order byte of DMA list address, and trigger Enhanced DMA job, with list address specified as 28-bit flat address (uses DMA option list) [0xD705: ETRIG]
    pub const SET_LOW_ORDER_BYTE_OF_DMA_LIST_ADDRESS: *mut u8 = (0xD705) as *mut u8;

    /// DMA list address low byte (address bits 0 -- 7) WITHOUT STARTING A DMA JOB (used by Hypervisor for unfreezing DMA-using tasks) [0xD70E: ADDRLSB]
    pub const DMA_LIST_ADDRESS_LOW_BYTE: *mut u8 = (0xD70E) as *mut u8;

    /// Audio DMA block timeout (read only) DEBUG [0xD711: AUD]
    pub const AUDIO_DMA_BLOCK_TIMEOUT_MASK: u8 = 0b00000111;

    /// Audio DMA bypasses audio mixer [0xD711: NOMIX]
    pub const AUDIO_DMA_BYPASSES_AUDIO_MIXER_MASK: u8 = 0b00010000;

    /// Audio DMA block writes (samples still get read) [0xD711: AUD]
    pub const AUDIO_DMA_BLOCK_WRITES_MASK: u8 = 0b00100000;

    /// Audio DMA blocked (read only) DEBUG [0xD711: BLKD]
    pub const AUDIO_DMA_BLOCKED_MASK: u8 = 0b01000000;

    /// Enable Audio DMA [0xD711: AUDEN]
    pub const ENABLE_AUDIO_DMA_MASK: u8 = 0b10000000;

    /// Audio DMA channel 0 right channel volume [0xD71C: CH0RVOL]
    pub const AUDIO_DMA_CHANNEL_0_RIGHT_CHANNEL_VOLUME: *mut u8 = (0xD71C) as *mut u8;

    /// Audio DMA channel 1 right channel volume [0xD71D: CH1RVOL]
    pub const AUDIO_DMA_CHANNEL_1_RIGHT_CHANNEL_VOLUME: *mut u8 = (0xD71D) as *mut u8;

    /// Audio DMA channel 2 left channel volume [0xD71E: CH2LVOL]
    pub const AUDIO_DMA_CHANNEL_2_LEFT_CHANNEL_VOLUME: *mut u8 = (0xD71E) as *mut u8;

    /// Audio DMA channel 3 left channel volume [0xD71F: CH3LVOL]
    pub const AUDIO_DMA_CHANNEL_3_LEFT_CHANNEL_VOLUME: *mut u8 = (0xD71F) as *mut u8;

    /// Audio DMA channel X sample bits (11=16, 10=8, 01=upper nybl, 00=lower nybl) [0xD720: CH0]
    pub const AUDIO_DMA_CHANNEL_X_SAMPLE_BITS_MASK: u8 = 0b00000011;

    /// Audio DMA channel X stop flag [0xD720: CH0]
    pub const AUDIO_DMA_CHANNEL_X_STOP_FLAG_MASK: u8 = 0b00001000;

    /// Audio DMA channel X play 32-sample sine wave instead of DMA data [0xD720: CH0]
    pub const AUDIO_DMA_CHANNEL_X_PLAY_32_SAMPLE_SINE_WAVE_INSTEAD_OF_DMA_DATA_MASK: u8 =
        0b00010000;

    /// Enable Audio DMA channel X signed samples [0xD720: CH0]
    pub const ENABLE_AUDIO_DMA_CHANNEL_X_SIGNED_SAMPLES_MASK: u8 = 0b00100000;

    /// Enable Audio DMA channel X looping [0xD720: CH0]
    pub const ENABLE_AUDIO_DMA_CHANNEL_X_LOOPING_MASK: u8 = 0b01000000;

    /// Enable Audio DMA channel X [0xD720: CH0]
    pub const ENABLE_AUDIO_DMA_CHANNEL_X_MASK: u8 = 0b10000000;

    /// Audio DMA channel X base address LSB [0xD721: CH0BADDRL]
    pub const AUDIO_DMA_CHANNEL_X_BASE_ADDRESS_LSB: *mut u8 = (0xD721) as *mut u8;

    /// Audio DMA channel X base address middle byte [0xD722: CH0BADDRC]
    pub const AUDIO_DMA_CHANNEL_X_BASE_ADDRESS_MIDDLE_BYTE: *mut u8 = (0xD722) as *mut u8;

    /// Audio DMA channel X base address MSB [0xD723: CH0BADDRM]
    pub const AUDIO_DMA_CHANNEL_X_BASE_ADDRESS_MSB: *mut u8 = (0xD723) as *mut u8;

    /// Audio DMA channel X frequency LSB [0xD724: CH0FREQL]
    pub const AUDIO_DMA_CHANNEL_X_FREQUENCY_LSB: *mut u8 = (0xD724) as *mut u8;

    /// Audio DMA channel X frequency middle byte [0xD725: CH0FREQC]
    pub const AUDIO_DMA_CHANNEL_X_FREQUENCY_MIDDLE_BYTE: *mut u8 = (0xD725) as *mut u8;

    /// Audio DMA channel X frequency MSB [0xD726: CH0FREQM]
    pub const AUDIO_DMA_CHANNEL_X_FREQUENCY_MSB: *mut u8 = (0xD726) as *mut u8;

    /// Audio DMA channel X top address LSB [0xD727: CH0TADDRL]
    pub const AUDIO_DMA_CHANNEL_X_TOP_ADDRESS_LSB: *mut u8 = (0xD727) as *mut u8;

    /// Audio DMA channel X top address MSB [0xD728: CH0TADDRM]
    pub const AUDIO_DMA_CHANNEL_X_TOP_ADDRESS_MSB: *mut u8 = (0xD728) as *mut u8;

    /// Audio DMA channel X playback volume [0xD729: CH0VOLUME]
    pub const AUDIO_DMA_CHANNEL_X_PLAYBACK_VOLUME: *mut u8 = (0xD729) as *mut u8;

    /// Audio DMA channel X current address LSB [0xD72A: CH0CURADDRL]
    pub const AUDIO_DMA_CHANNEL_X_CURRENT_ADDRESS_LSB: *mut u8 = (0xD72A) as *mut u8;

    /// Audio DMA channel X current address middle byte [0xD72B: CH0CURADDRC]
    pub const AUDIO_DMA_CHANNEL_X_CURRENT_ADDRESS_MIDDLE_BYTE: *mut u8 = (0xD72B) as *mut u8;

    /// Audio DMA channel X current address MSB [0xD72C: CH0CURADDRM]
    pub const AUDIO_DMA_CHANNEL_X_CURRENT_ADDRESS_MSB: *mut u8 = (0xD72C) as *mut u8;

    /// Audio DMA channel X timing counter LSB [0xD72D: CH0TMRADDRL]
    pub const AUDIO_DMA_CHANNEL_X_TIMING_COUNTER_LSB: *mut u8 = (0xD72D) as *mut u8;

    /// Audio DMA channel X timing counter middle byte [0xD72E: CH0TMRADDRC]
    pub const AUDIO_DMA_CHANNEL_X_TIMING_COUNTER_MIDDLE_BYTE: *mut u8 = (0xD72E) as *mut u8;

    /// Audio DMA channel X timing counter MSB [0xD72F: CH0TMRADDRM]
    pub const AUDIO_DMA_CHANNEL_X_TIMING_COUNTER_MSB: *mut u8 = (0xD72F) as *mut u8;

    /// @CHXSBITS [0xD730: CH1]
    pub const CHXSBITS_MASK: u8 = 0b00000011;

    /// @CHXSTP [0xD730: CH1]
    pub const CHXSTP_MASK: u8 = 0b00001000;

    /// @CHXSINE [0xD730: CH1]
    pub const CHXSINE_MASK: u8 = 0b00010000;

    /// @CHXSGN [0xD730: CH1]
    pub const CHXSGN_MASK: u8 = 0b00100000;

    /// @CHXLOOP [0xD730: CH1]
    pub const CHXLOOP_MASK: u8 = 0b01000000;

    /// @CHXEN [0xD730: CH1]
    pub const CHXEN_MASK: u8 = 0b10000000;

    /// @CHXBADDRL [0xD731: CH1BADDRL]
    pub const CHXBADDRL: *mut u8 = (0xD731) as *mut u8;

    /// @CHXBADDRC [0xD732: CH1BADDRC]
    pub const CHXBADDRC: *mut u8 = (0xD732) as *mut u8;

    /// @CHXBADDRM [0xD733: CH1BADDRM]
    pub const CHXBADDRM: *mut u8 = (0xD733) as *mut u8;

    /// @CHXFREQL [0xD734: CH1FREQL]
    pub const CHXFREQL: *mut u8 = (0xD734) as *mut u8;

    /// @CHXFREQC [0xD735: CH1FREQC]
    pub const CHXFREQC: *mut u8 = (0xD735) as *mut u8;

    /// @CHXFREQM [0xD736: CH1FREQM]
    pub const CHXFREQM: *mut u8 = (0xD736) as *mut u8;

    /// @CHXTADDRL [0xD737: CH1TADDRL]
    pub const CHXTADDRL: *mut u8 = (0xD737) as *mut u8;

    /// @CHXTADDRM [0xD738: CH1TADDRM]
    pub const CHXTADDRM: *mut u8 = (0xD738) as *mut u8;

    /// @CHXVOLUME [0xD739: CH1VOLUME]
    pub const CHXVOLUME: *mut u8 = (0xD739) as *mut u8;

    /// @CHXCURADDRL [0xD73A: CH1CURADDRL]
    pub const CHXCURADDRL: *mut u8 = (0xD73A) as *mut u8;

    /// @CHXCURADDRC [0xD73B: CH1CURADDRC]
    pub const CHXCURADDRC: *mut u8 = (0xD73B) as *mut u8;

    /// @CHXCURADDRM [0xD73C: CH1CURADDRM]
    pub const CHXCURADDRM: *mut u8 = (0xD73C) as *mut u8;

    /// @CHXTMRADDRL [0xD73D: CH1TMRADDRL]
    pub const CHXTMRADDRL: *mut u8 = (0xD73D) as *mut u8;

    /// @CHXTMRADDRC [0xD73E: CH1TMRADDRC]
    pub const CHXTMRADDRC: *mut u8 = (0xD73E) as *mut u8;

    /// @CHXTMRADDRM [0xD73F: CH1TMRADDRM]
    pub const CHXTMRADDRM: *mut u8 = (0xD73F) as *mut u8;
}

pub mod ethcommand {

    /// Immediately stop transmitting the current ethernet frame.  Will cause a partially sent frame to be received, most likely resulting in the loss of that frame. [0x00: STOPTX]
    pub const IMMEDIATELY_STOP_TRANSMITTING_THE_CURRENT_ETHERNET_FRAME: *mut u8 = (0x00) as *mut u8;

    /// Transmit packet [0x01: STARTTX]
    pub const TRANSMIT_PACKET: *mut u8 = (0x01) as *mut u8;

    /// Disable the effects of RXONLYONE [0xD0: RXNORMAL]
    pub const DISABLE_THE_EFFECTS_OF_RXONLYONE: *mut u8 = (0xD0) as *mut u8;

    /// Select VIC-IV debug stream via ethernet when \$D6E1.3 is set [0xD4: DEBUGVIC]
    pub const SELECT_VIC_IV_DEBUG_STREAM_VIA_ETHERNET_WHEN_0XD6E1: *mut u8 = (0xD4) as *mut u8;

    /// Select CPU debug stream via ethernet when \$D6E1.3 is set [0xDC: DEBUGCPU]
    pub const SELECT_CPU_DEBUG_STREAM_VIA_ETHERNET_WHEN_0XD6E1: *mut u8 = (0xDC) as *mut u8;

    /// Receive exactly one ethernet frame only, and keep all signals states (for debugging ethernet sub-system) [0xDE: RXONLYONE]
    pub const RECEIVE_EXACTLY_ONE_ETHERNET_FRAME_ONLY: *mut u8 = (0xDE) as *mut u8;

    /// Select ~1KiB frames for video/cpu debug stream frames (for receivers that do not support MTUs of greater than 2KiB) [0xF1: FRAME1K]
    pub const SELECT_1KIB_FRAMES_FOR_VIDEO_SLASH_CPU_DEBUG_STREAM_FRAMES: *mut u8 =
        (0xF1) as *mut u8;

    /// Select ~2KiB frames for video/cpu debug stream frames, for optimal performance. [0xF2: FRAME2K]
    pub const SELECT_2KIB_FRAMES_FOR_VIDEO_SLASH_CPU_DEBUG_STREAM_FRAMES: *mut u8 =
        (0xF2) as *mut u8;
}

pub mod ethernet {

    /// Write 0 to hold ethernet controller under reset [0xD6E0: RST]
    pub const WRITE_0_TO_HOLD_ETHERNET_CONTROLLER_UNDER_RESET_MASK: u8 = 0b00000001;

    /// Write 0 to hold ethernet controller transmit sub-system under reset [0xD6E0: TXRST]
    pub const WRITE_0_TO_HOLD_ETHERNET_CONTROLLER_TRANSMIT_SUB_SYSTEM_UNDER_RESET_MASK: u8 =
        0b00000010;

    /// Read ethernet RX bits currently on the wire [0xD6E0: DRXD]
    pub const READ_ETHERNET_RX_BITS_CURRENTLY_ON_THE_WIRE_MASK: u8 = 0b00000100;

    /// Read ethernet RX data valid (debug) [0xD6E0: DRXDV]
    pub const READ_ETHERNET_RX_DATA_VALID_MASK: u8 = 0b00001000;

    /// Allow remote keyboard input via magic ethernet frames [0xD6E0: KEYEN]
    pub const ALLOW_REMOTE_KEYBOARD_INPUT_VIA_MAGIC_ETHERNET_FRAMES_MASK: u8 = 0b00010000;

    /// Indicate if ethernet RX is blocked until RX buffers freed [0xD6E0: RXBLKD]
    pub const INDICATE_IF_ETHERNET_RX_IS_BLOCKED_UNTIL_RX_BUFFERS_FREED_MASK: u8 = 0b01000000;

    /// Ethernet transmit side is idle, i.e., a packet can be sent. [0xD6E0: TXIDLE]
    pub const ETHERNET_TRANSMIT_SIDE_IS_IDLE_MASK: u8 = 0b10000000;

    /// Number of free receive buffers [0xD6E1: RXBF]
    pub const NUMBER_OF_FREE_RECEIVE_BUFFERS_MASK: u8 = 0b00000011;

    /// Enable streaming of CPU instruction stream or VIC-IV display on ethernet [0xD6E1: STRM]
    pub const ENABLE_STREAMING_OF_CPU_INSTRUCTION_STREAM_OR_VIC_IV_DISPLAY_ON_ETHERNET_MASK: u8 =
        0b00001000;

    /// Ethernet TX IRQ status [0xD6E1: TXQ]
    pub const ETHERNET_TX_IRQ_STATUS_MASK: u8 = 0b00010000;

    /// Ethernet RX IRQ status [0xD6E1: RXQ]
    pub const ETHERNET_RX_IRQ_STATUS_MASK: u8 = 0b00100000;

    /// Enable ethernet TX IRQ [0xD6E1: TXQEN]
    pub const ENABLE_ETHERNET_TX_IRQ_MASK: u8 = 0b01000000;

    /// Enable ethernet RX IRQ [0xD6E1: RXQEN]
    pub const ENABLE_ETHERNET_RX_IRQ_MASK: u8 = 0b10000000;

    /// TX Packet size (low byte) [0xD6E2: TXSZLSB]
    pub const TX_PACKET_SIZE: *mut u8 = (0xD6E2) as *mut u8;

    /// Ethernet command register (write only) [0xD6E4: COMMAND]
    pub const ETHERNET_COMMAND_REGISTER: *mut u8 = (0xD6E4) as *mut u8;

    /// Ethernet disable promiscuous mode [0xD6E5: NOPROM]
    pub const ETHERNET_DISABLE_PROMISCUOUS_MODE_MASK: u8 = 0b00000001;

    /// Disable CRC check for received packets [0xD6E5: NOCRC]
    pub const DISABLE_CRC_CHECK_FOR_RECEIVED_PACKETS_MASK: u8 = 0b00000010;

    /// Ethernet RX clock phase adjust [0xD6E5: RXPH]
    pub const ETHERNET_RX_CLOCK_PHASE_ADJUST_MASK: u8 = 0b00000011;

    /// Ethernet TX clock phase adjust [0xD6E5: TXPH]
    pub const ETHERNET_TX_CLOCK_PHASE_ADJUST_MASK: u8 = 0b00000011;

    /// Accept broadcast frames [0xD6E5: BCST]
    pub const ACCEPT_BROADCAST_FRAMES_MASK: u8 = 0b00010000;

    /// Accept multicast frames [0xD6E5: MCST]
    pub const ACCEPT_MULTICAST_FRAMES_MASK: u8 = 0b00100000;

    /// Ethernet MIIM register number [0xD6E6: MIIMREG]
    pub const ETHERNET_MIIM_REGISTER_NUMBER_MASK: u8 = 0b00011111;

    /// Ethernet MIIM PHY number (use 0 for Nexys4, 1 for MEGA65 r1 PCBs) [0xD6E6: MIIMPHY]
    pub const ETHERNET_MIIM_PHY_NUMBER_MASK: u8 = 0b00000111;

    /// Ethernet MIIM register value (LSB) [0xD6E7: MIIMVLSB]
    pub const ETHERNET_MIIM_REGISTER_VALUE: *mut u8 = (0xD6E7) as *mut u8;

    /// Ethernet MAC address [0xD6E9: MACADDR1]
    pub const ETHERNET_MAC_ADDRESS: *mut u8 = (0xD6E9) as *mut u8;

    /// @MACADDRX [0xD6EA: MACADDR2]
    pub const MACADDRX: *mut u8 = (0xD6EA) as *mut u8;

    /// DEBUG show number of writes to eth RX buffer [0xD6EF: DBGRXWCOUNT]
    pub const DEBUG_SHOW_NUMBER_OF_WRITES_TO_ETH_RX_BUFFER: *mut u8 = (0xD6EF) as *mut u8;

    /// DEBUG show current ethernet TX state [0xD6EF: DBGTXSTAT]
    pub const DEBUG_SHOW_CURRENT_ETHERNET_TX_STATE: *mut u8 = (0xD6EF) as *mut u8;
}

pub mod f011 {

    /// Enable D65 ``MEGA Disk'' for F011 emulated drive 0 [0xD68B: MDISK0]
    pub const ENABLE_D65_MEGA_DISK_FOR_F011_EMULATED_DRIVE_0_MASK: u8 = 0b01000000;

    /// Enable D65 ``MEGA Disk'' for F011 emulated drive 1 [0xD68B: MDISK0]
    pub const ENABLE_D65_MEGA_DISK_FOR_F011_EMULATED_DRIVE_1_MASK: u8 = 0b10000000;

    /// Diskimage sector number (bits 0-7) [0xD68C: DISKADDR0]
    pub const DISKIMAGE_SECTOR_NUMBER: *mut u8 = (0xD68C) as *mut u8;

    /// Diskimage 2 sector number (bits 0-7) [0xD690: DISK2ADDR0]
    pub const DISKIMAGE_2_SECTOR_NUMBER: *mut u8 = (0xD690) as *mut u8;

    /// Enable automatic track seeking for sector reads and writes [0xD696: AUTOTUNE]
    pub const ENABLE_AUTOMATIC_TRACK_SEEKING_FOR_SECTOR_READS_AND_WRITES: *mut u8 =
        (0xD696) as *mut u8;

    /// Use real floppy drive instead of SD card for 1st floppy drive [0xD6A1: DRV0EN]
    pub const USE_REAL_FLOPPY_DRIVE_INSTEAD_OF_SD_CARD_FOR_1ST_FLOPPY_DRIVE_MASK: u8 = 0b00000001;

    /// Use real floppy drive instead of SD card for 2nd floppy drive [0xD6A1: DRV2EN]
    pub const USE_REAL_FLOPPY_DRIVE_INSTEAD_OF_SD_CARD_FOR_2ND_FLOPPY_DRIVE_MASK: u8 = 0b00000100;
}

pub mod fdc {

    /// Drive select (0 to 7). Internal drive is 0. Second floppy drive on internal cable is 1. Other values reserved for C1565 external drive interface. [0xD080: DS]
    pub const DRIVE_SELECT_MASK: u8 = 0b00000111;

    /// Directly controls the SIDE signal to the floppy drive, i.e., selecting which side of the media is active. [0xD080: SIDE]
    pub const DIRECTLY_CONTROLS_THE_SIDE_SIGNAL_TO_THE_FLOPPY_DRIVE_MASK: u8 = 0b00001000;

    /// Swap upper and lower halves of data buffer (i.e. invert bit 8 of the sector buffer) [0xD080: SWAP]
    pub const SWAP_UPPER_AND_LOWER_HALVES_OF_DATA_BUFFER_MASK: u8 = 0b00010000;

    /// Activates drive motor and LED (unless LED signal is also set, causing the drive LED to blink) [0xD080: MOTOR]
    pub const ACTIVATES_DRIVE_MOTOR_AND_LED_MASK: u8 = 0b00100000;

    /// Drive LED blinks when set [0xD080: LED]
    pub const DRIVE_LED_BLINKS_WHEN_SET_MASK: u8 = 0b01000000;

    /// Reset the sector buffer read/write pointers [0xD081: NOBUF]
    pub const RESET_THE_SECTOR_BUFFER_READ_SLASH_WRITE_POINTERS_MASK: u8 = 0b00000001;

    /// Selects alternate DPLL read recovery method (not implemented) [0xD081: ALT]
    pub const SELECTS_ALTERNATE_DPLL_READ_RECOVERY_METHOD_MASK: u8 = 0b00000010;

    /// Selects reading and writing algorithm (currently ignored). [0xD081: ALGO]
    pub const SELECTS_READING_AND_WRITING_ALGORITHM_MASK: u8 = 0b00000100;

    /// Sets the stepping direction (inward vs [0xD081: DIR]
    pub const SETS_THE_STEPPING_DIRECTION_MASK: u8 = 0b00001000;

    /// Writing 1 causes the head to step in the indicated direction [0xD081: STEP]
    pub const WRITING_1_CAUSES_THE_HEAD_TO_STEP_IN_THE_INDICATED_DIRECTION_MASK: u8 = 0b00010000;

    /// Command is a free-format (low level) operation [0xD081: FREE]
    pub const COMMAND_IS_A_FREE_FORMAT_MASK: u8 = 0b00100000;

    /// Command is a read operation if set [0xD081: RDCMD]
    pub const COMMAND_IS_A_READ_OPERATION_IF_SET_MASK: u8 = 0b01000000;

    /// Command is a write operation if set [0xD081: WRCMD]
    pub const COMMAND_IS_A_WRITE_OPERATION_IF_SET_MASK: u8 = 0b10000000;

    /// F011 FDC command register [0xD081: COMMAND]
    pub const F011_FDC_COMMAND_REGISTER: *mut u8 = (0xD081) as *mut u8;

    /// F011 Head is over track 0 flag (read only) [0xD082: TK0]
    pub const F011_HEAD_IS_OVER_TRACK_0_FLAG_MASK: u8 = 0b00000001;

    /// F011 Disk write protect flag (read only) [0xD082: PROT]
    pub const F011_DISK_WRITE_PROTECT_FLAG_MASK: u8 = 0b00000010;

    /// F011 LOST flag (data was lost during transfer, i.e., CPU did not read data fast enough) (read only) [0xD082: LOST]
    pub const F011_LOST_FLAG_MASK: u8 = 0b00000100;

    /// F011 FDC CRC check failure flag (read only) [0xD082: CRC]
    pub const F011_FDC_CRC_CHECK_FAILURE_FLAG_MASK: u8 = 0b00001000;

    /// F011 FDC Request Not Found (RNF), i.e., a sector read or write operation did not find the requested sector (read only) [0xD082: RNF]
    pub const F011_FDC_REQUEST_NOT_FOUND_MASK: u8 = 0b00010000;

    /// F011 FDC CPU and disk pointers to sector buffer are equal, indicating that the sector buffer is either full or empty. (read only) [0xD082: EQ]
    pub const F011_FDC_CPU_AND_DISK_POINTERS_TO_SECTOR_BUFFER_ARE_EQUAL_MASK: u8 = 0b00100000;

    /// F011 FDC DRQ flag (one or more bytes of data are ready) (read only) [0xD082: DRQ]
    pub const F011_FDC_DRQ_FLAG_MASK: u8 = 0b01000000;

    /// F011 FDC busy flag (command is being executed) (read only) [0xD082: BUSY]
    pub const F011_FDC_BUSY_FLAG_MASK: u8 = 0b10000000;

    /// F011 disk change sense (read only) [0xD083: DSKCHG]
    pub const F011_DISK_CHANGE_SENSE_MASK: u8 = 0b00000001;

    /// The floppy controller has generated an interrupt (read only). Note that interrupts are not currently implemented on the 45GS27. [0xD083: IRQ]
    pub const THE_FLOPPY_CONTROLLER_HAS_GENERATED_AN_INTERRUPT_MASK: u8 = 0b00000010;

    /// F011 Index hole sense (read only) [0xD083: INDEX]
    pub const F011_INDEX_HOLE_SENSE_MASK: u8 = 0b00000100;

    /// F011 Disk sense (read only) [0xD083: DISKIN]
    pub const F011_DISK_SENSE_MASK: u8 = 0b00001000;

    /// F011 write gate flag. Indicates that the drive is currently writing to media.  Bad things may happen if a write transaction is aborted (read only) [0xD083: WGATE]
    pub const F011_WRITE_GATE_FLAG_MASK: u8 = 0b00010000;

    /// F011 Successive match.  A synonym of RDREQ on the 45IO47 (read only) [0xD083: RUN]
    pub const F011_SUCCESSIVE_MATCH_MASK: u8 = 0b00100000;

    /// F011 Write Request flag, i.e., the requested sector was found during a write operation (read only) [0xD083: WTREQ]
    pub const F011_WRITE_REQUEST_FLAG_MASK: u8 = 0b01000000;

    /// F011 Read Request flag, i.e., the requested sector was found during a read operation (read only) [0xD083: RDREQ]
    pub const F011_READ_REQUEST_FLAG_MASK: u8 = 0b10000000;

    /// F011 FDC track selection register [0xD084: TRACK]
    pub const F011_FDC_TRACK_SELECTION_REGISTER: *mut u8 = (0xD084) as *mut u8;

    /// F011 FDC sector selection register [0xD085: SECTOR]
    pub const F011_FDC_SECTOR_SELECTION_REGISTER: *mut u8 = (0xD085) as *mut u8;

    /// F011 FDC side selection register [0xD086: SIDE]
    pub const F011_FDC_SIDE_SELECTION_REGISTER: *mut u8 = (0xD086) as *mut u8;

    /// F011 FDC data register (read/write) for accessing the floppy controller's 512 byte sector buffer [0xD087: DATA]
    pub const F011_FDC_DATA_REGISTER: *mut u8 = (0xD087) as *mut u8;

    /// Set or read the clock pattern to be used when writing address and data marks. Should normally be left $FF [0xD088: CLOCK]
    pub const SET_OR_READ_THE_CLOCK_PATTERN_TO_BE_USED_WHEN_WRITING_ADDRESS_AND_DATA_MARKS:
        *mut u8 = (0xD088) as *mut u8;

    /// Set or read the track stepping rate in 62.5 microsecond steps (normally 128, i.e., 8 milliseconds). [0xD089: STEP]
    pub const SET_OR_READ_THE_TRACK_STEPPING_RATE_IN_62: *mut u8 = (0xD089) as *mut u8;

    /// (Read only) returns the protection code of the most recently read sector. Was intended for rudimentary copy protection. Not implemented. [0xD08A: PCODE]
    pub const PROTECTION_CODE_OF_THE_MOST_RECENTLY_READ_SECTOR: *mut u8 = (0xD08A) as *mut u8;

    /// Control floppy drive SIDE1 line [0xD6A0: DBGWGATE]
    pub const CONTROL_FLOPPY_DRIVE_SIDE1_LINE_MASK: u8 = 0b00000001;

    /// Control floppy drive WGATE line [0xD6A0: DBGWGATE]
    pub const CONTROL_FLOPPY_DRIVE_WGATE_LINE_MASK: u8 = 0b00000010;

    /// Control floppy drive WDATA line [0xD6A0: DBGWDATA]
    pub const CONTROL_FLOPPY_DRIVE_WDATA_LINE_MASK: u8 = 0b00000100;

    /// Control floppy drive STEP line [0xD6A0: DBGDIR]
    pub const CONTROL_FLOPPY_DRIVE_STEP_LINE_MASK: u8 = 0b00001000;

    /// Control floppy drive STEPDIR line [0xD6A0: DBGDIR]
    pub const CONTROL_FLOPPY_DRIVE_STEPDIR_LINE_MASK: u8 = 0b00010000;

    /// Control floppy drive SELECT line [0xD6A0: DBGMOTORA]
    pub const CONTROL_FLOPPY_DRIVE_SELECT_LINE_MASK: u8 = 0b00100000;

    /// Control floppy drive MOTOR line [0xD6A0: DBGMOTORA]
    pub const CONTROL_FLOPPY_DRIVE_MOTOR_LINE_MASK: u8 = 0b01000000;

    /// Control floppy drive density select line [0xD6A0: DENSITY]
    pub const CONTROL_FLOPPY_DRIVE_DENSITY_SELECT_LINE_MASK: u8 = 0b10000000;

    /// Set number of bus cycles per floppy magnetic interval (decrease to increase data rate) [0xD6A2: DATARATE]
    pub const SET_NUMBER_OF_BUS_CYCLES_PER_FLOPPY_MAGNETIC_INTERVAL: *mut u8 = (0xD6A2) as *mut u8;
}

pub mod fpga {

    /// LSB of MEGA65 FPGA design date stamp (days since 1 Jan 2020) [0xD630: FWDATEL]
    pub const LSB_OF_MEGA65_FPGA_DESIGN_DATE_STAMP: *mut u8 = (0xD630) as *mut u8;

    /// MSB of MEGA65 FPGA design date stamp (days since 1 Jan 2020) [0xD631: FWDATEH]
    pub const MSB_OF_MEGA65_FPGA_DESIGN_DATE_STAMP: *mut u8 = (0xD631) as *mut u8;

    /// LSB of MEGA65 FPGA design git commit [0xD632: FWGIT0]
    pub const LSB_OF_MEGA65_FPGA_DESIGN_GIT_COMMIT: *mut u8 = (0xD632) as *mut u8;

    /// 2nd byte of MEGA65 FPGA design git commit [0xD633: FWGIT0]
    pub const COMMIT_BYTE_OF_MEGA65_FPGA_DESIGN_GIT_2ND: *mut u8 = (0xD633) as *mut u8;

    /// 3rd byte of MEGA65 FPGA design git commit [0xD634: FWGIT0]
    pub const COMMIT_BYTE_OF_MEGA65_FPGA_DESIGN_GIT_3RD: *mut u8 = (0xD634) as *mut u8;

    /// MSB of MEGA65 FPGA design git commit [0xD635: FWGIT0]
    pub const MSB_OF_MEGA65_FPGA_DESIGN_GIT_COMMIT: *mut u8 = (0xD635) as *mut u8;

    /// Select ICAPE2 FPGA configuration register for reading WRITE ONLY [0xD6C4: REGNUM]
    pub const SELECT_ICAPE2_FPGA_CONFIGURATION_REGISTER_FOR_READING_WRITE_ONLY: *mut u8 =
        (0xD6C4) as *mut u8;

    /// Value of selected ICAPE2 register (least significant byte) [0xD6C4: REGVAL]
    pub const VALUE_OF_SELECTED_ICAPE2_REGISTER: *mut u8 = (0xD6C4) as *mut u8;

    /// Address of bitstream in boot flash for reconfiguration (least significant byte) [0xD6C8: BOOTADDR0]
    pub const ADDRESS_OF_BITSTREAM_IN_BOOT_FLASH_FOR_RECONFIGURATION: *mut u8 = (0xD6C8) as *mut u8;

    /// Write $42 to Trigger FPGA reconfiguration to switch to alternate bitstream. [0xD6CF: RECONFTRIG]
    pub const WRITE_0X42_TO_TRIGGER_FPGA_RECONFIGURATION_TO_SWITCH_TO_ALTERNATE_BITSTREAM: *mut u8 =
        (0xD6CF) as *mut u8;

    /// FPGA die temperature sensor (lower nybl) [0xD6DE: FPGATEMPLSB]
    pub const FPGA_DIE_TEMPERATURE_SENSOR: *mut u8 = (0xD6DE) as *mut u8;
}

pub mod hcpu {

    /// Hypervisor A register storage [0xD640: REGA]
    pub const HYPERVISOR_A_REGISTER_STORAGE: *mut u8 = (0xD640) as *mut u8;

    /// Hypervisor X register storage [0xD641: REGX]
    pub const HYPERVISOR_X_REGISTER_STORAGE: *mut u8 = (0xD641) as *mut u8;

    /// Hypervisor Z register storage [0xD643: REGZ]
    pub const HYPERVISOR_Z_REGISTER_STORAGE: *mut u8 = (0xD643) as *mut u8;

    /// Hypervisor B register storage [0xD644: REGB]
    pub const HYPERVISOR_B_REGISTER_STORAGE: *mut u8 = (0xD644) as *mut u8;

    /// Hypervisor SPL register storage [0xD645: SPL]
    pub const HYPERVISOR_SPL_REGISTER_STORAGE: *mut u8 = (0xD645) as *mut u8;

    /// Hypervisor SPH register storage [0xD646: SPH]
    pub const HYPERVISOR_SPH_REGISTER_STORAGE: *mut u8 = (0xD646) as *mut u8;

    /// Hypervisor P register storage [0xD647: PFLAGS]
    pub const HYPERVISOR_P_REGISTER_STORAGE: *mut u8 = (0xD647) as *mut u8;

    /// Hypervisor PC-low register storage [0xD648: PCL]
    pub const HYPERVISOR_PC_LOW_REGISTER_STORAGE: *mut u8 = (0xD648) as *mut u8;

    /// Hypervisor PC-high register storage [0xD649: PCH]
    pub const HYPERVISOR_PC_HIGH_REGISTER_STORAGE: *mut u8 = (0xD649) as *mut u8;

    /// Hypervisor MAPLO register storage (high bits) [0xD64A: MAPLO]
    pub const HYPERVISOR_MAPLO_REGISTER_STORAGE: *mut u8 = (0xD64A) as *mut u8;

    /// Hypervisor MAPHI register storage (high bits) [0xD64C: MAPHI]
    pub const HYPERVISOR_MAPHI_REGISTER_STORAGE: *mut u8 = (0xD64C) as *mut u8;

    /// Hypervisor MAPLO mega-byte number register storage [0xD64E: MAPLOMB]
    pub const HYPERVISOR_MAPLO_MEGA_BYTE_NUMBER_REGISTER_STORAGE: *mut u8 = (0xD64E) as *mut u8;

    /// Hypervisor MAPHI mega-byte number register storage [0xD64F: MAPHIMB]
    pub const HYPERVISOR_MAPHI_MEGA_BYTE_NUMBER_REGISTER_STORAGE: *mut u8 = (0xD64F) as *mut u8;

    /// Hypervisor CPU port \$00 value [0xD650: PORT00]
    pub const HYPERVISOR_CPU_PORT_0X00_VALUE: *mut u8 = (0xD650) as *mut u8;

    /// Hypervisor CPU port \$01 value [0xD651: PORT01]
    pub const HYPERVISOR_CPU_PORT_0X01_VALUE: *mut u8 = (0xD651) as *mut u8;

    /// VIC-II/VIC-III/VIC-IV mode select [0xD652: VICMODE]
    pub const VIC_II_SLASH_VIC_III_SLASH_VIC_IV_MODE_SELECT_MASK: u8 = 0b00000011;

    /// 0=Use internal SIDs, 1=Use external(1) SIDs [0xD652: EXSID]
    pub const USE_INTERNAL_SIDS_MASK: u8 = 0b00000100;

    /// Hypervisor DMAgic source MB [0xD653: DMASRCMB]
    pub const HYPERVISOR_DMAGIC_SOURCE_MB: *mut u8 = (0xD653) as *mut u8;

    /// Hypervisor DMAgic destination MB [0xD654: DMADSTMB]
    pub const HYPERVISOR_DMAGIC_DESTINATION_MB: *mut u8 = (0xD654) as *mut u8;

    /// Hypervisor DMAGic list address bits 0-7 [0xD655: DMALADDR]
    pub const HYPERVISOR_DMAGIC_LIST_ADDRESS_BITS_0_7: *mut u8 = (0xD655) as *mut u8;

    /// Hypervisor DMAGic list address bits 15-8 [0xD656: DMALADDR]
    pub const HYPERVISOR_DMAGIC_LIST_ADDRESS_BITS_15_8: *mut u8 = (0xD656) as *mut u8;

    /// Hypervisor DMAGic list address bits 23-16 [0xD657: DMALADDR]
    pub const HYPERVISOR_DMAGIC_LIST_ADDRESS_BITS_23_16: *mut u8 = (0xD657) as *mut u8;

    /// Hypervisor DMAGic list address bits 27-24 [0xD658: DMALADDR]
    pub const HYPERVISOR_DMAGIC_LIST_ADDRESS_BITS_27_24: *mut u8 = (0xD658) as *mut u8;

    /// 1=Virtualise SD/Floppy0 access (usually for access via serial debugger interface) [0xD659: VFLOP]
    pub const VIRTUALISE_SD_SLASH_FLOPPY0_ACCESS_MASK: u8 = 0b00000001;

    /// 1=Virtualise SD/Floppy1 access (usually for access via serial debugger interface) [0xD659: VFLOP]
    pub const VIRTUALISE_SD_SLASH_FLOPPY1_ACCESS_MASK: u8 = 0b00000010;

    /// Hypervisor GeoRAM base address (x MB) [0xD670: GEORAMBASE]
    pub const HYPERVISOR_GEORAM_BASE_ADDRESS: *mut u8 = (0xD670) as *mut u8;

    /// Hypervisor GeoRAM address mask (applied to GeoRAM block register) [0xD671: GEORAMMASK]
    pub const HYPERVISOR_GEORAM_ADDRESS_MASK: *mut u8 = (0xD671) as *mut u8;

    /// Enable composited Matrix Mode, and disable UART access to serial monitor. [0xD672: MATRIXEN]
    pub const ENABLE_COMPOSITED_MATRIX_MODE_MASK: u8 = 0b01000000;

    /// (write) Hypervisor write serial output to UART monitor [0xD67C: UARTDATA]
    pub const _MASK: u8 = 0b11111111;

    /// Hypervisor enable 32-bit JMP/JSR etc [0xD67D: JMP32EN]
    pub const HYPERVISOR_ENABLE_32_BIT_JMP_SLASH_JSR_ETC_MASK: u8 = 0b00000010;

    /// Hypervisor write protect C65 ROM \$20000-\$3FFFF [0xD67D: ROMPROT]
    pub const HYPERVISOR_WRITE_PROTECT_C65_ROM_0X20000_0X3FFFF_MASK: u8 = 0b00000100;

    /// Hypervisor enable ASC/DIN CAPS LOCK key to enable/disable CPU slow-down in C64/C128/C65 modes [0xD67D: ASCFAST]
    pub const HYPERVISOR_ENABLE_ASC_SLASH_DIN_CAPS_LOCK_KEY_TO_ENABLE_SLASH_DISABLE_CPU_SLOW_DOWN_IN_C64_SLASH_C128_SLASH_C65_MODES_MASK: u8 = 0b00001000;

    /// Hypervisor force CPU to 48MHz for userland (userland can override via POKE0) [0xD67D: CPUFAST]
    pub const HYPERVISOR_FORCE_CPU_TO_48MHZ_FOR_USERLAND_MASK: u8 = 0b00010000;

    /// Hypervisor force CPU to 4502 personality, even in C64 IO mode. [0xD67D: F4502]
    pub const HYPERVISOR_FORCE_CPU_TO_4502_PERSONALITY_MASK: u8 = 0b00100000;

    /// Hypervisor flag to indicate if an IRQ is pending on exit from the hypervisor / set 1 to force IRQ/NMI deferal for 1,024 cycles on exit from hypervisor. [0xD67D: PIRQ]
    pub const HYPERVISOR_FLAG_TO_INDICATE_IF_AN_IRQ_IS_PENDING_ON_EXIT_FROM_THE_HYPERVISOR__SLASH__SET_1_TO_FORCE_IRQ_SLASH_NMI_DEFERAL_FOR_1_MASK: u8 = 0b01000000;

    /// Hypervisor flag to indicate if an NMI is pending on exit from the hypervisor. [0xD67D: PNMI]
    pub const HYPERVISOR_FLAG_TO_INDICATE_IF_AN_NMI_IS_PENDING_ON_EXIT_FROM_THE_HYPERVISOR_MASK:
        u8 = 0b10000000;

    /// Hypervisor watchdog register: writing any value clears the watch dog [0xD67D: WATCHDOG]
    pub const HYPERVISOR_WATCHDOG_REGISTER: *mut u8 = (0xD67D) as *mut u8;

    /// Hypervisor already-upgraded bit (writing sets permanently) [0xD67E: HICKED]
    pub const HYPERVISOR_ALREADY_UPGRADED_BIT: *mut u8 = (0xD67E) as *mut u8;

    /// Writing trigger return from hypervisor [0xD67F: ENTEREXIT]
    pub const WRITING_TRIGGER_RETURN_FROM_HYPERVISOR: *mut u8 = (0xD67F) as *mut u8;
}

pub mod kbd {

    /// LSB of keyboard firmware date stamp (days since 1 Jan 2020) [0xD62A: FWDATEL]
    pub const LSB_OF_KEYBOARD_FIRMWARE_DATE_STAMP: *mut u8 = (0xD62A) as *mut u8;

    /// MSB of keyboard firmware date stamp (days since 1 Jan 2020) [0xD62B: FWDATEH]
    pub const MSB_OF_KEYBOARD_FIRMWARE_DATE_STAMP: *mut u8 = (0xD62B) as *mut u8;

    /// LSB of keyboard firmware git commit [0xD62C: FWGIT0]
    pub const LSB_OF_KEYBOARD_FIRMWARE_GIT_COMMIT: *mut u8 = (0xD62C) as *mut u8;

    /// 2nd byte of keyboard firmware git commit [0xD62D: FWGIT0]
    pub const COMMIT_BYTE_OF_KEYBOARD_FIRMWARE_GIT_2ND: *mut u8 = (0xD62D) as *mut u8;

    /// 3rd byte of keyboard firmware git commit [0xD62E: FWGIT0]
    pub const COMMIT_BYTE_OF_KEYBOARD_FIRMWARE_GIT_3RD: *mut u8 = (0xD62E) as *mut u8;

    /// MSB of keyboard firmware git commit [0xD62F: FWGIT0]
    pub const MSB_OF_KEYBOARD_FIRMWARE_GIT_COMMIT: *mut u8 = (0xD62F) as *mut u8;
}

pub mod math {

    /// Set if hardware multiplier is busy [0xD70F: MULBUSY]
    pub const SET_IF_HARDWARE_MULTIPLIER_IS_BUSY_MASK: u8 = 0b01000000;

    /// Set if hardware divider is busy [0xD70F: DIVBUSY]
    pub const SET_IF_HARDWARE_DIVIDER_IS_BUSY_MASK: u8 = 0b10000000;

    /// 64-bit output of MULTINA $\div$ MULTINB [0xD768: DIVOUT]
    pub const MULTINB_OUTPUT_OF_MULTINA_0XDIV0X_64_BIT: *mut u8 = (0xD768) as *mut u8;

    /// Multiplier input A / Divider numerator (32 bit) [0xD770: MULTINA]
    pub const MULTIPLIER_INPUT_A__SLASH__DIVIDER_NUMERATOR: *mut u8 = (0xD770) as *mut u8;

    /// Multiplier input B / Divider denominator (32 bit) [0xD774: MULTINB]
    pub const MULTIPLIER_INPUT_B__SLASH__DIVIDER_DENOMINATOR: *mut u8 = (0xD774) as *mut u8;

    /// 64-bit output of MULTINA $\times$ MULTINB [0xD778: MULTOUT]
    pub const MULTINB_OUTPUT_OF_MULTINA_0XTIMES0X_64_BIT: *mut u8 = (0xD778) as *mut u8;

    /// Math unit 32-bit input X [0xD780: MATHIN0]
    pub const MATH_UNIT_32_BIT_INPUT_X: *mut u8 = (0xD780) as *mut u8;

    /// @MATHINX [0xD781: MATHIN0]
    pub const MATHINX: *mut u8 = (0xD781) as *mut u8;

    /// Select which of the 16 32-bit math registers is input A for Math Function Unit X. [0xD7C0: UNIT0INA]
    pub const SELECT_WHICH_OF_THE_16_32_BIT_MATH_REGISTERS_IS_INPUT_A_FOR_MATH_FUNCTION_UNIT_X_MASK: u8 = 0b00001111;

    /// Select which of the 16 32-bit math registers is input B for Math Function Unit X. [0xD7C0: UNIT0INB]
    pub const SELECT_WHICH_OF_THE_16_32_BIT_MATH_REGISTERS_IS_INPUT_B_FOR_MATH_FUNCTION_UNIT_X_MASK: u8 = 0b00001111;

    /// @UNITXINA [0xD7C1: UNIT1INA]
    pub const UNITXINA_MASK: u8 = 0b00001111;

    /// @UNITXINB [0xD7C1: UNIT1INB]
    pub const UNITXINB_MASK: u8 = 0b00001111;

    /// Select which of the 16 32-bit math registers receives the output of Math Function Unit X [0xD7D0: UNIT0OUT]
    pub const SELECT_WHICH_OF_THE_16_32_BIT_MATH_REGISTERS_RECEIVES_THE_OUTPUT_OF_MATH_FUNCTION_UNIT_X_MASK: u8 = 0b00001111;

    /// @UNITXOUT [0xD7D1: UNIT1OUT]
    pub const UNITXOUT_MASK: u8 = 0b00001111;

    /// Latch interval for latched outputs (in CPU cycles) [0xD7E0: LATCHINT]
    pub const LATCH_INTERVAL_FOR_LATCHED_OUTPUTS: *mut u8 = (0xD7E0) as *mut u8;

    /// Enable setting of math registers (must normally be set) [0xD7E1: WREN]
    pub const ENABLE_SETTING_OF_MATH_REGISTERS_MASK: u8 = 0b00000001;

    /// Enable committing of output values from math units back to math registers (clearing effectively pauses iterative formulae) [0xD7E1: CALCEN]
    pub const ENABLE_COMMITTING_OF_OUTPUT_VALUES_FROM_MATH_UNITS_BACK_TO_MATH_REGISTERS_MASK: u8 =
        0b00000010;

    /// Iteration Counter (32 bit) [0xD7E4: ITERCNT]
    pub const ITERATION_COUNTER: *mut u8 = (0xD7E4) as *mut u8;

    /// Math iteration counter comparator (32 bit) [0xD7E8: ITERCMP]
    pub const MATH_ITERATION_COUNTER_COMPARATOR: *mut u8 = (0xD7E8) as *mut u8;
}

pub mod misc {

    /// I2C bus select (bus 0 = temp sensor on Nexys4 boardS) [0xD6D0: I2CBUSSELECT]
    pub const I2C_BUS_SELECT: *mut u8 = (0xD6D0) as *mut u8;

    /// DEBUG SD card last error code LSB [0xD6DA: SDDEBUGERRLSB]
    pub const DEBUG_SD_CARD_LAST_ERROR_CODE_LSB: *mut u8 = (0xD6DA) as *mut u8;

    /// DEBUG SD card last error code MSB [0xD6DB: SDDEBUGERRMSB]
    pub const DEBUG_SD_CARD_LAST_ERROR_CODE_MSB: *mut u8 = (0xD6DB) as *mut u8;

    /// Read FPGA five-way buttons [0xD6F2: FPGABUTTONS]
    pub const READ_FPGA_FIVE_WAY_BUTTONS: *mut u8 = (0xD6F2) as *mut u8;

    /// Accelerometer bit-bash interface [0xD6F3: ACCELBITBASH]
    pub const ACCELEROMETER_BIT_BASH_INTERFACE: *mut u8 = (0xD6F3) as *mut u8;

    /// Keyboard scan code reader (lower byte) [0xD6F6: PS2KEYSCANLSB]
    pub const KEYBOARD_SCAN_CODE_READER: *mut u8 = (0xD6F6) as *mut u8;

    /// Select audio channel volume to be set by thumb wheel #3 [0xD6AC: WHEEL3TARGET]
    pub const SELECT_AUDIO_CHANNEL_VOLUME_TO_BE_SET_BY_THUMB_WHEEL_3_MASK: u8 = 0b00001111;

    /// Enable control of LCD panel brightness via thumb wheel [0xD6AC: WHEELBRIGHTEN]
    pub const ENABLE_CONTROL_OF_LCD_PANEL_BRIGHTNESS_VIA_THUMB_WHEEL_MASK: u8 = 0b10000000;

    /// Select audio channel volume to be set by thumb wheel #1 [0xD6AD: WHEEL1TARGET]
    pub const SELECT_AUDIO_CHANNEL_VOLUME_TO_BE_SET_BY_THUMB_WHEEL_1_MASK: u8 = 0b00001111;

    /// Select audio channel volume to be set by thumb wheel #2 [0xD6AD: WHEEL2TARGET]
    pub const SELECT_AUDIO_CHANNEL_VOLUME_TO_BE_SET_BY_THUMB_WHEEL_2_MASK: u8 = 0b00001111;

    /// Flip X axis of touch interface if set [0xD6B0: TCHFLX]
    pub const FLIP_X_AXIS_OF_TOUCH_INTERFACE_IF_SET_MASK: u8 = 0b01000000;

    /// Flip Y axis of touch interface if set [0xD6B0: TCHFLX]
    pub const FLIP_Y_AXIS_OF_TOUCH_INTERFACE_IF_SET_MASK: u8 = 0b10000000;

    /// Set X scale value for touch interface (LSB) [0xD6B1: TCHXSCALE]
    pub const SET_X_SCALE_VALUE_FOR_TOUCH_INTERFACE: *mut u8 = (0xD6B1) as *mut u8;

    /// Set Y scale value for touch interface (LSB) [0xD6B3: TCHYSCALE]
    pub const SET_Y_SCALE_VALUE_FOR_TOUCH_INTERFACE: *mut u8 = (0xD6B3) as *mut u8;

    /// Set X delta value for touch interface (LSB) [0xD6B5: TCHXDELTA]
    pub const SET_X_DELTA_VALUE_FOR_TOUCH_INTERFACE: *mut u8 = (0xD6B5) as *mut u8;

    /// Set Y delta value for touch interface (LSB) [0xD6B7: TCHYDELTA]
    pub const SET_Y_DELTA_VALUE_FOR_TOUCH_INTERFACE: *mut u8 = (0xD6B7) as *mut u8;

    /// Select byte number for touch panel communications instrumentation [0xD6BF: TCHBYTENUM]
    pub const SELECT_BYTE_NUMBER_FOR_TOUCH_PANEL_COMMUNICATIONS_INSTRUMENTATION_MASK: u8 =
        0b01111111;

    /// Enable/disable touch panel I2C communications [0xD6BF: TCHI2CEN]
    pub const ENABLE_SLASH_DISABLE_TOUCH_PANEL_I2C_COMMUNICATIONS_MASK: u8 = 0b10000000;

    /// Select I2C bus number (I2C busses vary between MEGA65 and MEGAphone variants) [0xD6D0: I2CBUSSEL]
    pub const SELECT_I2C_BUS_NUMBER: *mut u8 = (0xD6D0) as *mut u8;

    /// I2C reset [0xD6D1: I2CRST]
    pub const I2C_RESET_MASK: u8 = 0b00000001;

    /// I2C command latch write strobe (write 1 to trigger command) [0xD6D1: I2CL]
    pub const I2C_COMMAND_LATCH_WRITE_STROBE_MASK: u8 = 0b00000010;

    /// I2C Select read (1) or write (0) [0xD6D1: I2CRW]
    pub const I2C_SELECT_READ_MASK: u8 = 0b00000100;

    /// I2C bus 1 swap SDA/SCL pins [0xD6D1: I2CSW]
    pub const I2C_BUS_1_SWAP_SDA_SLASH_SCL_PINS_MASK: u8 = 0b00100000;

    /// I2C busy flag [0xD6D1: I2CBSY]
    pub const I2C_BUSY_FLAG_MASK: u8 = 0b01000000;

    /// I2C ack error [0xD6D1: I2CERR]
    pub const I2C_ACK_ERROR_MASK: u8 = 0b10000000;

    /// I2C address [0xD6D2: I2CADDR]
    pub const I2C_ADDRESS_MASK: u8 = 0b01111111;

    /// I2C data write register [0xD6D3: I2CWDATA]
    pub const I2C_DATA_WRITE_REGISTER: *mut u8 = (0xD6D3) as *mut u8;

    /// I2C data read register [0xD6D4: I2CRDATA]
    pub const I2C_DATA_READ_REGISTER: *mut u8 = (0xD6D4) as *mut u8;

    /// LCD panel brightness control [0xD6F0: LCDBRIGHT]
    pub const LCD_PANEL_BRIGHTNESS_CONTROL: *mut u8 = (0xD6F0) as *mut u8;

    /// Accelerometer bit-bashing port (debug only) [0xD6F3: ACCELBASH]
    pub const ACCELEROMETER_BIT_BASHING_PORT: *mut u8 = (0xD6F3) as *mut u8;
}

pub mod qspi {

    /// Data bits for QSPI flash interface (read/write) [0xD6CC: DB]
    pub const DATA_BITS_FOR_QSPI_FLASH_INTERFACE_MASK: u8 = 0b00001111;

    /// Clock output line for QSPI flash [0xD6CC: CLOCK]
    pub const CLOCK_OUTPUT_LINE_FOR_QSPI_FLASH_MASK: u8 = 0b00100000;

    /// Active-low chip-select for QSPI flash [0xD6CC: CSN]
    pub const ACTIVE_LOW_CHIP_SELECT_FOR_QSPI_FLASH_MASK: u8 = 0b01000000;

    /// Tristate DB0-3 [0xD6CC: TRI]
    pub const TRISTATE_DB0_3_MASK: u8 = 0b10000000;

    /// Set to cause QSPI clock to free run at CPU clock frequency. [0xD6CD: CLOCKRUN]
    pub const SET_TO_CAUSE_QSPI_CLOCK_TO_FREE_RUN_AT_CPU_CLOCK_FREQUENCY_MASK: u8 = 0b00000001;

    /// Alternate address for direct manipulation of QSPI CLOCK [0xD6CD: CLOCK]
    pub const ALTERNATE_ADDRESS_FOR_DIRECT_MANIPULATION_OF_QSPI_CLOCK_MASK: u8 = 0b00000010;
}

pub mod rtc {

    /// Real-time Clock seconds value (binary coded decimal) [0xFFD7110: RTCSEC]
    pub const REAL_TIME_CLOCK_SECONDS_VALUE: *mut u8 = (0xFFD7110u32) as *mut u8;

    /// Real-time Clock minutes value (binary coded decimal) [0xFFD7111: RTCMIN]
    pub const REAL_TIME_CLOCK_MINUTES_VALUE: *mut u8 = (0xFFD7111u32) as *mut u8;

    /// Real-time Clock hours value (binary coded decimal) [0xFFD7112: RTCHOUR]
    pub const REAL_TIME_CLOCK_HOURS_VALUE: *mut u8 = (0xFFD7112u32) as *mut u8;

    /// Real-time Clock day of month value (binary coded decimal) [0xFFD7113: RTCDAY]
    pub const REAL_TIME_CLOCK_DAY_OF_MONTH_VALUE: *mut u8 = (0xFFD7113u32) as *mut u8;

    /// Real-time Clock month value (binary coded decimal) [0xFFD7114: RTCMONTH]
    pub const REAL_TIME_CLOCK_MONTH_VALUE: *mut u8 = (0xFFD7114u32) as *mut u8;

    /// Real-time Clock year value (binary coded decimal) [0xFFD7115: RTCYEAR]
    pub const REAL_TIME_CLOCK_YEAR_VALUE: *mut u8 = (0xFFD7115u32) as *mut u8;

    /// External Real-time Clock seconds value (binary coded decimal) [0xFFD7400: EXTRTCSEC]
    pub const EXTERNAL_REAL_TIME_CLOCK_SECONDS_VALUE: *mut u8 = (0xFFD7400u32) as *mut u8;

    /// External Real-time Clock minutes value (binary coded decimal) [0xFFD7401: EXTRTCMIN]
    pub const EXTERNAL_REAL_TIME_CLOCK_MINUTES_VALUE: *mut u8 = (0xFFD7401u32) as *mut u8;

    /// External Real-time Clock hours value (binary coded decimal) [0xFFD7402: EXTRTCHOUR]
    pub const EXTERNAL_REAL_TIME_CLOCK_HOURS_VALUE: *mut u8 = (0xFFD7402u32) as *mut u8;

    /// External Real-time Clock day of week value (binary coded decimal) [0xFFD7403: EXTRTCDOW]
    pub const EXTERNAL_REAL_TIME_CLOCK_DAY_OF_WEEK_VALUE: *mut u8 = (0xFFD7403u32) as *mut u8;

    /// External Real-time Clock day of month value (binary coded decimal) [0xFFD7404: EXTRTCDAY]
    pub const EXTERNAL_REAL_TIME_CLOCK_DAY_OF_MONTH_VALUE: *mut u8 = (0xFFD7404u32) as *mut u8;

    /// External Real-time Clock month value (binary coded decimal) [0xFFD7405: EXTRTCMONTH]
    pub const EXTERNAL_REAL_TIME_CLOCK_MONTH_VALUE: *mut u8 = (0xFFD7405u32) as *mut u8;

    /// External Real-time Clock year value (binary coded decimal) [0xFFD7406: EXTRTCYEAR]
    pub const EXTERNAL_REAL_TIME_CLOCK_YEAR_VALUE: *mut u8 = (0xFFD7406u32) as *mut u8;

    /// External Real-time Clock alarm 1 seconds value (binary coded decimal) [0xFFD7407: EXTRTCA1SEC]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_1_SECONDS_VALUE: *mut u8 = (0xFFD7407u32) as *mut u8;

    /// External Real-time Clock alarm 1 minutes value (binary coded decimal) [0xFFD7408: EXTRTCA1MIN]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_1_MINUTES_VALUE: *mut u8 = (0xFFD7408u32) as *mut u8;

    /// External Real-time Clock alarm 1 hours value (binary coded decimal) [0xFFD7409: EXTRTCA1HOUR]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_1_HOURS_VALUE: *mut u8 = (0xFFD7409u32) as *mut u8;

    /// External Real-time Clock alarm 1 day of week / day of month value (binary coded decimal) [0xFFD740A: EXTRTCA1DAYDATE]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_1_DAY_OF_WEEK__SLASH__DAY_OF_MONTH_VALUE: *mut u8 =
        (0xFFD740Au32) as *mut u8;

    /// External Real-time Clock alarm 2 minutes value (binary coded decimal) [0xFFD740B: EXTRTCA2MIN]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_2_MINUTES_VALUE: *mut u8 = (0xFFD740Bu32) as *mut u8;

    /// External Real-time Clock alarm 2 hours value (binary coded decimal) [0xFFD740C: EXTRTCA2HOUR]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_2_HOURS_VALUE: *mut u8 = (0xFFD740Cu32) as *mut u8;

    /// External Real-time Clock alarm 2 day of week / day of month value (binary coded decimal) [0xFFD740D: EXTRTCA2DAYDATE]
    pub const EXTERNAL_REAL_TIME_CLOCK_ALARM_2_DAY_OF_WEEK__SLASH__DAY_OF_MONTH_VALUE: *mut u8 =
        (0xFFD740Du32) as *mut u8;

    /// External Real-time Clock control [0xFFD740E: EXTRTCCTRL]
    pub const EXTERNAL_REAL_TIME_CLOCK_CONTROL: *mut u8 = (0xFFD740Eu32) as *mut u8;

    /// External Real-time Clock control/status register [0xFFD740F: EXTRTCST]
    pub const EXTERNAL_REAL_TIME_CLOCK_CONTROL_SLASH_STATUS_REGISTER: *mut u8 =
        (0xFFD740Fu32) as *mut u8;

    /// External Real-time Clock aging offset (do not modify) [0xFFD7410: EXTRTCAGINGOFS]
    pub const EXTERNAL_REAL_TIME_CLOCK_AGING_OFFSET: *mut u8 = (0xFFD7410u32) as *mut u8;

    /// External Real-time Clock temperature (MSB) [0xFFD7411: EXTRTCTEMPMSB]
    pub const EXTERNAL_REAL_TIME_CLOCK_TEMPERATURE: *mut u8 = (0xFFD7411u32) as *mut u8;
}

pub mod sd {

    /// SD controller status/command [0xD680: CMDANDSTAT]
    pub const SD_CONTROLLER_STATUS_SLASH_COMMAND: *mut u8 = (0xD680) as *mut u8;

    /// WRITE ONLY set fill byte for use in fill mode, instead of SD buffer data [0xD686: FILLVAL]
    pub const WRITE_ONLY_SET_FILL_BYTE_FOR_USE_IN_FILL_MODE: *mut u8 = (0xD686) as *mut u8;

    /// Set/read SD card sd_handshake signal [0xD689: HNDSHK]
    pub const SET_SLASH_READ_SD_CARD_SD_HANDSHAKE_SIGNAL_MASK: u8 = 0b00000100;

    /// SD Card Data Ready indication [0xD689: DRDY]
    pub const SD_CARD_DATA_READY_INDICATION_MASK: u8 = 0b00001000;

    /// Set to swap floppy drive 0 (the internal drive) and drive 1 (the drive on the 2nd position on the internal floppy cable). [0xD689: FDCSWAP]
    pub const SET_TO_SWAP_FLOPPY_DRIVE_0_MASK: u8 = 0b00100000;

    /// Set to switch sector buffer to view SD card direct access, clear for access to the F011 FDC sector buffer. [0xD689: BUFFSEL]
    pub const SET_TO_SWITCH_SECTOR_BUFFER_TO_VIEW_SD_CARD_DIRECT_ACCESS_MASK: u8 = 0b10000000;

    /// Select floppy encoding (0=MFM, 1=RLL2,7, F=Raw encoding) [0xD6AE: FDC]
    pub const SELECT_FLOPPY_ENCODING_MASK: u8 = 0b00001111;

    /// Automatically select DD or HD decoder for last sector display [0xD6AE: AUTO]
    pub const AUTOMATICALLY_SELECT_DD_OR_HD_DECODER_FOR_LAST_SECTOR_DISPLAY_MASK: u8 = 0b00010000;

    /// Enable automatic variable speed selection for floppy controller using Track Information Blocks on MEGA65 HD floppies [0xD6AE: FDC]
    pub const ENABLE_AUTOMATIC_VARIABLE_SPEED_SELECTION_FOR_FLOPPY_CONTROLLER_USING_TRACK_INFORMATION_BLOCKS_ON_MEGA65_HD_FLOPPIES_MASK: u8 = 0b00100000;

    /// Select HD decoder for last sector display [0xD6AE: FDC]
    pub const SELECT_HD_DECODER_FOR_LAST_SECTOR_DISPLAY_MASK: u8 = 0b01000000;

    /// Enable use of Track Info Block settings [0xD6AE: FDC]
    pub const ENABLE_USE_OF_TRACK_INFO_BLOCK_SETTINGS_MASK: u8 = 0b10000000;

    /// Manually set f011_rsector_found signal (indented for virtual F011 mode only) [0xD6AF: VR]
    pub const MANUALLY_SET_F011_RSECTOR_FOUND_SIGNAL_MASK: u8 = 0b00000001;

    /// Manually set f011_wsector_found signal (indented for virtual F011 mode only) [0xD6AF: VW]
    pub const MANUALLY_SET_F011_WSECTOR_FOUND_SIGNAL_MASK: u8 = 0b00000010;

    /// Manually set f011_eq_inhibit signal (indented for virtual F011 mode only) [0xD6AF: VEQ]
    pub const MANUALLY_SET_F011_EQ_INHIBIT_SIGNAL_MASK: u8 = 0b00000100;

    /// Manually set f011_rnf signal (indented for virtual F011 mode only) [0xD6AF: VRNF]
    pub const MANUALLY_SET_F011_RNF_SIGNAL_MASK: u8 = 0b00001000;

    /// Manually set f011_drq signal (indented for virtual F011 mode only) [0xD6AF: VDRQ]
    pub const MANUALLY_SET_F011_DRQ_SIGNAL_MASK: u8 = 0b00010000;

    /// Manually set f011_lost signal (indented for virtual F011 mode only) [0xD6AF: VLOST]
    pub const MANUALLY_SET_F011_LOST_SIGNAL_MASK: u8 = 0b00100000;
}

pub mod sdfdc {

    /// F011 drive 0 disk image is D64 mega image if set (otherwise 800KiB 1581 or D65 image) [0xD68A: D0D64]
    pub const F011_DRIVE_0_DISK_IMAGE_IS_D64_MEGA_IMAGE_IF_SET_MASK: u8 = 0b01000000;

    /// F011 drive 1 disk image is D64 image if set (otherwise 800KiB 1581 or D65 image) [0xD68A: D1D64]
    pub const F011_DRIVE_1_DISK_IMAGE_IS_D64_IMAGE_IF_SET_MASK: u8 = 0b10000000;

    /// F011 drive 0 use disk image if set, otherwise use real floppy drive. [0xD68B: D0IMG]
    pub const F011_DRIVE_0_USE_DISK_IMAGE_IF_SET_MASK: u8 = 0b00000001;

    /// F011 drive 0 media present [0xD68B: D0P]
    pub const F011_DRIVE_0_MEDIA_PRESENT_MASK: u8 = 0b00000010;

    /// Write enable F011 drive 0 [0xD68B: D0WP]
    pub const WRITE_ENABLE_F011_DRIVE_0_MASK: u8 = 0b00000100;

    /// F011 drive 1 use disk image if set, otherwise use real floppy drive. [0xD68B: D1IMG]
    pub const F011_DRIVE_1_USE_DISK_IMAGE_IF_SET_MASK: u8 = 0b00001000;

    /// F011 drive 1 media present [0xD68B: D1P]
    pub const F011_DRIVE_1_MEDIA_PRESENT_MASK: u8 = 0b00010000;

    /// Write enable F011 drive 1 [0xD68B: D1WP]
    pub const WRITE_ENABLE_F011_DRIVE_1_MASK: u8 = 0b00100000;

    /// F011 drive 0 disk image is D65 image if set (otherwise 800KiB 1581 image) [0xD68B: D0MD]
    pub const F011_DRIVE_0_DISK_IMAGE_IS_D65_IMAGE_IF_SET_MASK: u8 = 0b01000000;

    /// F011 drive 1 disk image is D65 image if set (otherwise 800KiB 1581 image) [0xD68B: D1MD]
    pub const F011_DRIVE_1_DISK_IMAGE_IS_D65_IMAGE_IF_SET_MASK: u8 = 0b10000000;

    /// Use real floppy drive for drive 0 if set (read-only, except for from hypervisor) [0xD6A1: USEREAL0]
    pub const USE_REAL_FLOPPY_DRIVE_FOR_DRIVE_0_IF_SET_MASK: u8 = 0b00000001;

    /// Read next sector under head if set, ignoring the requested side, track and sector number. [0xD6A1: TARGANY]
    pub const READ_NEXT_SECTOR_UNDER_HEAD_IF_SET_MASK: u8 = 0b00000010;

    /// Use real floppy drive for drive 1 if set (read-only, except for from hypervisor) [0xD6A1: USEREAL1]
    pub const USE_REAL_FLOPPY_DRIVE_FOR_DRIVE_1_IF_SET_MASK: u8 = 0b00000100;

    /// Disable floppy spinning and tracking for SD card operations. [0xD6A1: SILENT]
    pub const DISABLE_FLOPPY_SPINNING_AND_TRACKING_FOR_SD_CARD_OPERATIONS_MASK: u8 = 0b00001000;
}

pub mod sid {

    /// Voice X Frequency Low [0xD400: VOICE1]
    pub const VOICE_X_FREQUENCY_LOW: *mut u8 = (0xD400) as *mut u8;

    /// Voice X Frequency High [0xD401: VOICE1]
    pub const VOICE_X_FREQUENCY_HIGH: *mut u8 = (0xD401) as *mut u8;

    /// Voice X Pulse Waveform Width Low [0xD402: VOICE1]
    pub const VOICE_X_PULSE_WAVEFORM_WIDTH_LOW: *mut u8 = (0xD402) as *mut u8;

    /// Voice X Pulse Waveform Width High [0xD403: VOICE1]
    pub const VOICE_X_PULSE_WAVEFORM_WIDTH_HIGH_MASK: u8 = 0b00001111;

    /// Unused [0xD403: VOICE1]
    pub const UNUSED_MASK: u8 = 0b00001111;

    /// Voice X Gate Bit (1 = Start, 0 = Release) [0xD404: VOICE1]
    pub const VOICE_X_GATE_BIT_MASK: u8 = 0b00000001;

    /// Voice 1 Synchronize Osc. 1 with Osc. 3 Frequency [0xD404: VOICE1]
    pub const VOICE_1_SYNCHRONIZE_OSC_MASK: u8 = 0b00000010;

    /// Voice 1 Ring Modulate Osc. 1 with Osc. 3 Output [0xD404: VOICE1]
    pub const VOICE_1_RING_MODULATE_OSC_MASK: u8 = 0b00000100;

    /// Voice X Test Bit - Disable Oscillator [0xD404: VOICE1]
    pub const VOICE_X_TEST_BIT___DISABLE_OSCILLATOR_MASK: u8 = 0b00001000;

    /// Voice X Triangle Waveform [0xD404: VOICE1]
    pub const VOICE_X_TRIANGLE_WAVEFORM_MASK: u8 = 0b00010000;

    /// Voice X Sawtooth Waveform [0xD404: VOICE1]
    pub const VOICE_X_SAWTOOTH_WAVEFORM_MASK: u8 = 0b00100000;

    /// Voice X Pulse Waveform [0xD404: VOICE1]
    pub const VOICE_X_PULSE_WAVEFORM_MASK: u8 = 0b01000000;

    /// Voice X Control Random Noise Waveform [0xD404: VOICE1]
    pub const VOICE_X_CONTROL_RANDOM_NOISE_WAVEFORM_MASK: u8 = 0b10000000;

    /// Envelope Generator X Decay Cycle Duration [0xD405: ENV1]
    pub const ENVELOPE_GENERATOR_X_DECAY_CYCLE_DURATION_MASK: u8 = 0b00001111;

    /// Envelope Generator X Attack Cycle Duration [0xD405: ENV1]
    pub const ENVELOPE_GENERATOR_X_ATTACK_CYCLE_DURATION_MASK: u8 = 0b00001111;

    /// Envelope Generator X Release Cycle Duration [0xD406: ENV1]
    pub const ENVELOPE_GENERATOR_X_RELEASE_CYCLE_DURATION_MASK: u8 = 0b00001111;

    /// Envelope Generator X Sustain Cycle Duration [0xD406: ENV1]
    pub const ENVELOPE_GENERATOR_X_SUSTAIN_CYCLE_DURATION_MASK: u8 = 0b00001111;

    /// @VOICEX!FRQLO [0xD407: VOICE2]
    pub const VOICEX_FRQLO: *mut u8 = (0xD407) as *mut u8;

    /// @VOICEX!FRQHI [0xD408: VOICE2]
    pub const VOICEX_FRQHI: *mut u8 = (0xD408) as *mut u8;

    /// @VOICEX!PWLO [0xD409: VOICE2]
    pub const VOICEX_PWLO: *mut u8 = (0xD409) as *mut u8;

    /// @VOICEX!PWHI [0xD40A: VOICE2]
    pub const VOICEX_PWHI_MASK: u8 = 0b00001111;

    /// @VOICEX!UNSD [0xD40A: VOICE2]
    pub const VOICEX_UNSD_MASK: u8 = 0b00001111;

    /// @VOICEX!CTRLGATE [0xD40B: VOICE2]
    pub const VOICEX_CTRLGATE_MASK: u8 = 0b00000001;

    /// Voice 2 Synchronize Osc. 2 with Osc. 1 Frequency [0xD40B: VOICE2]
    pub const VOICE_2_SYNCHRONIZE_OSC_MASK: u8 = 0b00000010;

    /// Voice 2 Ring Modulate Osc. 2 with Osc. 1 Output [0xD40B: VOICE2]
    pub const VOICE_2_RING_MODULATE_OSC_MASK: u8 = 0b00000100;

    /// @VOICEX!CTRLTST [0xD40B: VOICE2]
    pub const VOICEX_CTRLTST_MASK: u8 = 0b00001000;

    /// @VOICEX!CTRLTRI [0xD40B: VOICE2]
    pub const VOICEX_CTRLTRI_MASK: u8 = 0b00010000;

    /// @VOICEX!CTRLSAW [0xD40B: VOICE2]
    pub const VOICEX_CTRLSAW_MASK: u8 = 0b00100000;

    /// @VOICEX!CTRLPUL [0xD40B: VOICE2]
    pub const VOICEX_CTRLPUL_MASK: u8 = 0b01000000;

    /// @VOICEX!CTRLRNW [0xD40B: VOICE2]
    pub const VOICEX_CTRLRNW_MASK: u8 = 0b10000000;

    /// @ENVX!DECDUR [0xD40C: ENV2]
    pub const ENVX_DECDUR_MASK: u8 = 0b00001111;

    /// @ENVX!ATTDUR [0xD40C: ENV2]
    pub const ENVX_ATTDUR_MASK: u8 = 0b00001111;

    /// @ENVX!RELDUR [0xD40D: ENV2]
    pub const ENVX_RELDUR_MASK: u8 = 0b00001111;

    /// @ENVX!SUSDUR [0xD40D: ENV2]
    pub const ENVX_SUSDUR_MASK: u8 = 0b00001111;

    /// Voice 3 Synchronize Osc. 3 with Osc. 2 Frequency [0xD412: VOICE3]
    pub const VOICE_3_SYNCHRONIZE_OSC_MASK: u8 = 0b00000010;

    /// Voice 3 Ring Modulate Osc. 3 with Osc. 2 Output [0xD412: VOICE3]
    pub const VOICE_3_RING_MODULATE_OSC_MASK: u8 = 0b00000100;

    /// Filter Cutoff Frequency Low [0xD415: FLTR]
    pub const FILTER_CUTOFF_FREQUENCY_LOW: *mut u8 = (0xD415) as *mut u8;

    /// Filter Cutoff Frequency High [0xD416: FLTR]
    pub const FILTER_CUTOFF_FREQUENCY_HIGH: *mut u8 = (0xD416) as *mut u8;

    /// @FLTR!VXOUT [0xD417: FLTR]
    pub const FLTR_VXOUT_MASK: u8 = 0b00000001;

    /// Filter Voice X Output [0xD417: FLTR]
    pub const FILTER_VOICE_X_OUTPUT_MASK: u8 = 0b00000100;

    /// Filter External Input [0xD417: FLTR]
    pub const FILTER_EXTERNAL_INPUT_MASK: u8 = 0b00001000;

    /// Filter Resonance [0xD417: FLTR]
    pub const FILTER_RESONANCE_MASK: u8 = 0b00001111;

    /// Filter Output Volume [0xD418: FLTR]
    pub const FILTER_OUTPUT_VOLUME_MASK: u8 = 0b00001111;

    /// Filter Low-Pass Mode [0xD418: FLTR]
    pub const FILTER_LOW_PASS_MODE_MASK: u8 = 0b00010000;

    /// Filter Band-Pass Mode [0xD418: FLTR]
    pub const FILTER_BAND_PASS_MODE_MASK: u8 = 0b00100000;

    /// Filter High-Pass Mode [0xD418: FLTR]
    pub const FILTER_HIGH_PASS_MODE_MASK: u8 = 0b01000000;

    /// Filter Cut-Off Voice 3 Output (1 = off) [0xD418: FLTR]
    pub const FILTER_CUT_OFF_VOICE_3_OUTPUT_MASK: u8 = 0b10000000;

    /// Analog/Digital Converter: Game Paddle 1 (0-255) [0xD419: PADDLE1]
    pub const ANALOG_SLASH_DIGITAL_CONVERTER: *mut u8 = (0xD419) as *mut u8;

    /// Analog/Digital Converter Game Paddle 2 (0-255) [0xD41A: PADDLE2]
    pub const ANALOG_SLASH_DIGITAL_CONVERTER_GAME_PADDLE_2: *mut u8 = (0xD41A) as *mut u8;

    /// Oscillator 3 Random Number Generator [0xD41B: OSC3RNG]
    pub const OSCILLATOR_3_RANDOM_NUMBER_GENERATOR: *mut u8 = (0xD41B) as *mut u8;

    /// Envelope Generator 3 Output [0xD41C: ENV3OUT]
    pub const ENVELOPE_GENERATOR_3_OUTPUT: *mut u8 = (0xD41C) as *mut u8;

    /// Select SID mode: 0=6581, 1=8580 [0xD63C: SIDMODE]
    pub const SELECT_SID_MODE_MASK: u8 = 0b00001111;
}

pub mod sysctl {

    /// Mute digital video audio (MEGA65 R2 only) [0xD61A: AUDMUTE]
    pub const MUTE_DIGITAL_VIDEO_AUDIO_MASK: u8 = 0b00000001;

    /// Control digital video as DVI (disables audio) [0xD61A: DVI]
    pub const CONTROL_DIGITAL_VIDEO_AS_DVI_MASK: u8 = 0b00000010;

    /// Visualise audio samples (DEBUG) [0xD61A: AUDDBG]
    pub const VISUALISE_AUDIO_SAMPLES_MASK: u8 = 0b00000100;

    /// Select 48KHz or 44.1KHz digital video audio sample rate [0xD61A: AUD48K]
    pub const SELECT_48KHZ_OR_44_MASK: u8 = 0b00001000;

    /// Control LED next to U1 on mother board [0xD61A: LED]
    pub const CONTROL_LED_NEXT_TO_U1_ON_MOTHER_BOARD_MASK: u8 = 0b00010000;

    /// Invert digital video audio sample values [0xD61A: AUDINV]
    pub const INVERT_DIGITAL_VIDEO_AUDIO_SAMPLE_VALUES_MASK: u8 = 0b10000000;
}

pub mod touch {

    /// Touch event 1 is valid [0xD6B0: EV1]
    pub const TOUCH_EVENT_1_IS_VALID_MASK: u8 = 0b00000001;

    /// Touch event 2 is valid [0xD6B0: EV2]
    pub const TOUCH_EVENT_2_IS_VALID_MASK: u8 = 0b00000010;

    /// Touch event 1 up/down state [0xD6B0: UPDN1]
    pub const TOUCH_EVENT_1_UP_SLASH_DOWN_STATE_MASK: u8 = 0b00000011;

    /// Touch event 2 up/down state [0xD6B0: UPDN2]
    pub const TOUCH_EVENT_2_UP_SLASH_DOWN_STATE_MASK: u8 = 0b00000011;

    /// Invert horizontal axis [0xD6B0: XINV]
    pub const INVERT_HORIZONTAL_AXIS_MASK: u8 = 0b01000000;

    /// Invert vertical axis [0xD6B0: YINV]
    pub const INVERT_VERTICAL_AXIS_MASK: u8 = 0b10000000;

    /// Touch pad X scaling LSB [0xD6B1: CALXSCALELSB]
    pub const TOUCH_PAD_X_SCALING_LSB: *mut u8 = (0xD6B1) as *mut u8;

    /// Touch pad X scaling MSB [0xD6B2: CALXSCALEMSB]
    pub const TOUCH_PAD_X_SCALING_MSB: *mut u8 = (0xD6B2) as *mut u8;

    /// Touch pad Y scaling LSB [0xD6B3: CALYSCALELSB]
    pub const TOUCH_PAD_Y_SCALING_LSB: *mut u8 = (0xD6B3) as *mut u8;

    /// Touch pad Y scaling MSB [0xD6B4: CALYSCALEMSB]
    pub const TOUCH_PAD_Y_SCALING_MSB: *mut u8 = (0xD6B4) as *mut u8;

    /// Touch pad X delta LSB [0xD6B5: CALXDELTALSB]
    pub const TOUCH_PAD_X_DELTA_LSB: *mut u8 = (0xD6B5) as *mut u8;

    /// Touch pad Y delta LSB [0xD6B7: CALYDELTALSB]
    pub const TOUCH_PAD_Y_DELTA_LSB: *mut u8 = (0xD6B7) as *mut u8;

    /// Touch pad Y delta MSB [0xD6B8: CALYDELTAMSB]
    pub const TOUCH_PAD_Y_DELTA_MSB: *mut u8 = (0xD6B8) as *mut u8;

    /// Touch pad touch #1 X LSB [0xD6B9: TOUCH1XLSB]
    pub const TOUCH_PAD_TOUCH_1_X_LSB: *mut u8 = (0xD6B9) as *mut u8;

    /// Touch pad touch #1 Y LSB [0xD6BA: TOUCH1YLSB]
    pub const TOUCH_PAD_TOUCH_1_Y_LSB: *mut u8 = (0xD6BA) as *mut u8;

    /// Touch pad touch \#1 X MSBs [0xD6BB: TOUCH1XMSB]
    pub const TOUCH_PAD_TOUCH_1_X_MSBS_MASK: u8 = 0b00000011;

    /// Touch pad touch \#1 Y MSBs [0xD6BB: TOUCH1YMSB]
    pub const TOUCH_PAD_TOUCH_1_Y_MSBS_MASK: u8 = 0b00000011;

    /// Touch pad touch \#2 X LSB [0xD6BC: TOUCH2XLSB]
    pub const TOUCH_PAD_TOUCH_2_X_LSB: *mut u8 = (0xD6BC) as *mut u8;

    /// Touch pad touch \#2 Y LSB [0xD6BD: TOUCH2YLSB]
    pub const TOUCH_PAD_TOUCH_2_Y_LSB: *mut u8 = (0xD6BD) as *mut u8;

    /// Touch pad touch \#2 X MSBs [0xD6BE: TOUCH2XMSB]
    pub const TOUCH_PAD_TOUCH_2_X_MSBS_MASK: u8 = 0b00000011;

    /// Touch pad touch \#2 Y MSBs [0xD6BE: TOUCH2YMSB]
    pub const TOUCH_PAD_TOUCH_2_Y_MSBS_MASK: u8 = 0b00000011;

    /// Touch pad gesture directions (left,right,up,down) [0xD6C0: GESTUREDIR]
    pub const TOUCH_PAD_GESTURE_DIRECTIONS_MASK: u8 = 0b00001111;

    /// Touch pad gesture ID [0xD6C0: GESTUREID]
    pub const TOUCH_PAD_GESTURE_ID_MASK: u8 = 0b00001111;
}

pub mod tough {

    /// Touch pad X delta MSB [0xD6B6: CALXDELTAMSB]
    pub const TOUCH_PAD_X_DELTA_MSB: *mut u8 = (0xD6B6) as *mut u8;
}

pub mod uart {

    /// UART data register (read or write) [0xD600: DATA]
    pub const UART_DATA_REGISTER: *mut u8 = (0xD600) as *mut u8;

    /// UART RX byte ready flag (clear by reading \$D600) [0xD601: RXRDY]
    pub const UART_RX_BYTE_READY_FLAG_MASK: u8 = 0b00000001;

    /// UART RX overrun flag (clear by reading \$D600) [0xD601: RXOVRRUN]
    pub const UART_RX_OVERRUN_FLAG_MASK: u8 = 0b00000010;

    /// UART RX parity error flag (clear by reading \$D600) [0xD601: PTYERR]
    pub const UART_RX_PARITY_ERROR_FLAG_MASK: u8 = 0b00000100;

    /// UART RX framing error flag (clear by reading \$D600) [0xD601: FRMERR]
    pub const UART_RX_FRAMING_ERROR_FLAG_MASK: u8 = 0b00001000;

    /// UART Parity: 1=even, 0=odd [0xD602: PTYEVEN]
    pub const UART_PARITY_MASK: u8 = 0b00000001;

    /// UART Parity enable: 1=enabled [0xD602: PTYEN]
    pub const UART_PARITY_ENABLE_MASK: u8 = 0b00000010;

    /// UART character size: 00=8, 01=7, 10=6, 11=5 bits per byte [0xD602: CHARSZ]
    pub const UART_CHARACTER_SIZE_MASK: u8 = 0b00000011;

    /// UART synchronisation mode flags (00=RX \& TX both async, 01=RX sync, TX async, 1x=TX sync, RX async (unused on the MEGA65) [0xD602: SYNCMOD]
    pub const UART_SYNCHRONISATION_MODE_FLAGS_MASK: u8 = 0b00000011;

    /// UART enable receive [0xD602: RXEN]
    pub const UART_ENABLE_RECEIVE_MASK: u8 = 0b01000000;

    /// UART enable transmit [0xD602: TXEN]
    pub const UART_ENABLE_TRANSMIT_MASK: u8 = 0b10000000;

    /// UART baud rate divisor (16 bit). Baud rate = 7.09375MHz / DIVISOR, unless MEGA65 fast UART mode is enabled, in which case baud rate = 80MHz / DIVISOR [0xD603: DIVISOR]
    pub const UART_BAUD_RATE_DIVISOR: *mut u8 = (0xD603) as *mut u8;

    /// UART interrupt mask: NMI on RX (not yet implemented on the MEGA65) [0xD605: IMRXNMI]
    pub const UART_INTERRUPT_MASK_MASK: u8 = 0b00010000;

    /// UART interrupt flag: NMI on RX (not yet implemented on the MEGA65) [0xD606: IFRXNMI]
    pub const UART_INTERRUPT_FLAG_MASK: u8 = 0b00010000;

    /// C65 capslock key sense [0xD607: CAPLOCK]
    pub const C65_CAPSLOCK_KEY_SENSE_MASK: u8 = 0b00000001;

    /// C65 keyboard column 8 select [0xD607: KEYCOL8]
    pub const C65_KEYBOARD_COLUMN_8_SELECT_MASK: u8 = 0b00000010;

    /// C65 keyboard extra lines Data Direction Register (DDR) [0xD608: PORTEDDR]
    pub const C65_KEYBOARD_EXTRA_LINES_DATA_DIRECTION_REGISTER_MASK: u8 = 0b00000011;

    /// C65 UART BAUD clock source: 1 = 7.09375MHz, 0 = 80MHz (VIC-IV pixel clock) [0xD609: UFAST]
    pub const C65_UART_BAUD_CLOCK_SOURCE_MASK: u8 = 0b00000001;

    /// PMOD port A on FPGA board (data) (Nexys4 boards only) [0xD60B: PORTF]
    pub const PMOD_PORT_A_ON_FPGA_BOARD_MASK: u8 = 0b00111111;

    /// Display hardware zoom of region under first touch point always [0xD60B: OSKZON]
    pub const DISPLAY_HARDWARE_ZOOM_OF_REGION_UNDER_FIRST_TOUCH_POINT_ALWAYS_MASK: u8 = 0b01000000;

    /// Display hardware zoom of region under first touch point for on-screen keyboard [0xD60B: OSKZEN]
    pub const DISPLAY_HARDWARE_ZOOM_OF_REGION_UNDER_FIRST_TOUCH_POINT_FOR_ON_SCREEN_KEYBOARD_MASK: u8 = 0b10000000;

    /// On Screen Keyboard (OSK) Zoom Control Data Direction Register (DDR). Must be set to output to control these features. [0xD60C: PORTFDDR]
    pub const ON_SCREEN_KEYBOARD_MASK: u8 = 0b00000011;

    /// SD card MOSI/MISO [0xD60D: SDDATA]
    pub const SD_CARD_MOSI_SLASH_MISO_MASK: u8 = 0b00000100;

    /// SD card SCLK [0xD60D: SDCLK]
    pub const SD_CARD_SCLK_MASK: u8 = 0b00001000;

    /// SD card CS_BO [0xD60D: SDCS]
    pub const SD_CARD_CS_BO_MASK: u8 = 0b00010000;

    /// Enable SD card bitbash mode [0xD60D: SDBSH]
    pub const ENABLE_SD_CARD_BITBASH_MODE_MASK: u8 = 0b00100000;

    /// HDMI I2C control interface SDA data line [0xD60D: HDSDA]
    pub const HDMI_I2C_CONTROL_INTERFACE_SDA_DATA_LINE_MASK: u8 = 0b01000000;

    /// HDMI I2C control interface SCL clock [0xD60D: HDSCL]
    pub const HDMI_I2C_CONTROL_INTERFACE_SCL_CLOCK_MASK: u8 = 0b10000000;

    /// Data Direction Register (DDR) for \$D60D bit bashing port. [0xD60E: BASHDDR]
    pub const DATA_DIRECTION_REGISTER: *mut u8 = (0xD60E) as *mut u8;

    /// Directly read C65 Cursor left key [0xD60F: KEYLEFT]
    pub const DIRECTLY_READ_C65_CURSOR_LEFT_KEY_MASK: u8 = 0b00000001;

    /// Directly read C65 Cursor up key [0xD60F: KEYUP]
    pub const DIRECTLY_READ_C65_CURSOR_UP_KEY_MASK: u8 = 0b00000010;

    /// Set to 1 if the MEGA65 is running on real hardware, set to 0 if emulated (Xemu) or simulated (ghdl) [0xD60F: REALHW]
    pub const SET_TO_1_IF_THE_MEGA65_IS_RUNNING_ON_REAL_HARDWARE_MASK: u8 = 0b00100000;

    /// Light or heavy dimming of background material behind on-screen keyboard [0xD60F: OSKDIM]
    pub const LIGHT_OR_HEAVY_DIMMING_OF_BACKGROUND_MATERIAL_BEHIND_ON_SCREEN_KEYBOARD_MASK: u8 =
        0b01000000;

    /// Enable accessible keyboard input via joystick port 2 fire button [0xD60F: ACCESSKEY]
    pub const ENABLE_ACCESSIBLE_KEYBOARD_INPUT_VIA_JOYSTICK_PORT_2_FIRE_BUTTON_MASK: u8 =
        0b10000000;

    /// Last key press as ASCII (hardware accelerated keyboard scanner). Write to clear event ready for next. [0xD610: ASCIIKEY]
    pub const LAST_KEY_PRESS_AS_ASCII: *mut u8 = (0xD610) as *mut u8;

    /// Right shift key state (hardware accelerated keyboard scanner). [0xD611: MRSHFT]
    pub const RIGHT_SHIFT_KEY_STATE_MASK: u8 = 0b00000001;

    /// Left shift key state (hardware accelerated keyboard scanner). [0xD611: MLSHFT]
    pub const LEFT_SHIFT_KEY_STATE_MASK: u8 = 0b00000010;

    /// CTRL key state (hardware accelerated keyboard scanner). [0xD611: MCTRL]
    pub const CTRL_KEY_STATE_MASK: u8 = 0b00000100;

    /// MEGA/C= key state (hardware accelerated keyboard scanner). [0xD611: MMEGA]
    pub const MEGA_SLASH_C_MASK: u8 = 0b00001000;

    /// ALT key state (hardware accelerated keyboard scanner). [0xD611: MALT]
    pub const ALT_KEY_STATE_MASK: u8 = 0b00010000;

    /// NOSCRL key state (hardware accelerated keyboard scanner). [0xD611: MSCRL]
    pub const NOSCRL_KEY_STATE_MASK: u8 = 0b00100000;

    /// CAPS LOCK key state (hardware accelerated keyboard scanner). [0xD611: MCAPS]
    pub const CAPS_LOCK_KEY_STATE_MASK: u8 = 0b01000000;

    /// Enable widget board keyboard/joystick input [0xD612: WGTKEY]
    pub const ENABLE_WIDGET_BOARD_KEYBOARD_SLASH_JOYSTICK_INPUT_MASK: u8 = 0b00000001;

    /// Enable ps2 keyboard/joystick input [0xD612: PS2KEY]
    pub const ENABLE_PS2_KEYBOARD_SLASH_JOYSTICK_INPUT_MASK: u8 = 0b00000010;

    /// Enable physical keyboard input [0xD612: PHYKEY]
    pub const ENABLE_PHYSICAL_KEYBOARD_INPUT_MASK: u8 = 0b00000100;

    /// Enable virtual/snythetic keyboard input [0xD612: VRTKEY]
    pub const ENABLE_VIRTUAL_SLASH_SNYTHETIC_KEYBOARD_INPUT_MASK: u8 = 0b00001000;

    /// Debug OSK overlay (WRITE ONLY) [0xD612: OSKDEBUG]
    pub const DEBUG_OSK_OVERLAY_MASK: u8 = 0b00010000;

    /// Enable PS/2 / USB keyboard simulated joystick input [0xD612: PS2JOY]
    pub const ENABLE_PS_SLASH_2__SLASH__USB_KEYBOARD_SIMULATED_JOYSTICK_INPUT_MASK: u8 = 0b00010000;

    /// Exchange joystick ports 1 \& 2 [0xD612: JOYSWAP]
    pub const EXCHANGE_JOYSTICK_PORTS_1__AND__2_MASK: u8 = 0b00100000;

    /// Rotate inputs of joystick A by 180 degrees (for left handed use) [0xD612: LJOYA]
    pub const ROTATE_INPUTS_OF_JOYSTICK_A_BY_180_DEGREES_MASK: u8 = 0b01000000;

    /// Rotate inputs of joystick B by 180 degrees (for left handed use) [0xD612: LJOYB]
    pub const ROTATE_INPUTS_OF_JOYSTICK_B_BY_180_DEGREES_MASK: u8 = 0b10000000;

    /// Set to \$7F for no key down, else specify virtual key press. [0xD615: VIRTKEY1]
    pub const SET_TO_0X7F_FOR_NO_KEY_DOWN_MASK: u8 = 0b01111111;

    /// Enable display of on-screen keyboard composited overlay [0xD615: OSKEN]
    pub const ENABLE_DISPLAY_OF_ON_SCREEN_KEYBOARD_COMPOSITED_OVERLAY_MASK: u8 = 0b10000000;

    /// Display alternate on-screen keyboard layout (typically dial pad for MEGA65 telephone) [0xD616: OSKALT]
    pub const DISPLAY_ALTERNATE_ON_SCREEN_KEYBOARD_LAYOUT_MASK: u8 = 0b10000000;

    /// 1=Display on-screen keyboard at top, 0=Disply on-screen keyboard at bottom of screen. [0xD617: OSKTOP]
    pub const DISPLAY_ON_SCREEN_KEYBOARD_AT_TOP_MASK: u8 = 0b10000000;

    /// Physical keyboard scan rate (\$00=50MHz, \$FF=~200KHz) [0xD618: KSCNRATE]
    pub const PHYSICAL_KEYBOARD_SCAN_RATE: *mut u8 = (0xD618) as *mut u8;

    /// Last key press as PETSCII (hardware accelerated keyboard scanner). Write to clear event ready for next. [0xD619: PETSCIIKEY]
    pub const LAST_KEY_PRESS_AS_PETSCII: *mut u8 = (0xD619) as *mut u8;

    /// System control flags (target specific) [0xD61A: SYSCTL]
    pub const SYSTEM_CONTROL_FLAGS: *mut u8 = (0xD61A) as *mut u8;

    /// Keyboard LED register select (R,G,B channels x 4 = 0 to 11) [0xD61D: KEYLED]
    pub const KEYBOARD_LED_REGISTER_SELECT_MASK: u8 = 0b01111111;

    /// Keyboard LED control enable [0xD61D: KEYLED]
    pub const KEYBOARD_LED_CONTROL_ENABLE_MASK: u8 = 0b10000000;

    /// Keyboard LED register value (write only) [0xD61E: KEYLED]
    pub const KEYBOARD_LED_REGISTER_VALUE: *mut u8 = (0xD61E) as *mut u8;

    /// Read Port A paddle X, without having to fiddle with SID/CIA settings. [0xD620: POTAX]
    pub const READ_PORT_A_PADDLE_X: *mut u8 = (0xD620) as *mut u8;

    /// Read Port A paddle Y, without having to fiddle with SID/CIA settings. [0xD621: POTAY]
    pub const READ_PORT_A_PADDLE_Y: *mut u8 = (0xD621) as *mut u8;

    /// Read Port B paddle X, without having to fiddle with SID/CIA settings. [0xD622: POTBX]
    pub const READ_PORT_B_PADDLE_X: *mut u8 = (0xD622) as *mut u8;

    /// Read Port B paddle Y, without having to fiddle with SID/CIA settings. [0xD623: POTBY]
    pub const READ_PORT_B_PADDLE_Y: *mut u8 = (0xD623) as *mut u8;

    /// J21 pins 1 -- 6, 9 -- 10 input/output values [0xD625: J21L]
    pub const J21_PINS_1___6: *mut u8 = (0xD625) as *mut u8;

    /// J21 pins 11 -- 14 input/output values [0xD626: J21H]
    pub const J21_PINS_11___14_INPUT_SLASH_OUTPUT_VALUES: *mut u8 = (0xD626) as *mut u8;

    /// J21 pins 11 -- 14 data direction register [0xD628: J21HDDR]
    pub const J21_PINS_11___14_DATA_DIRECTION_REGISTER: *mut u8 = (0xD628) as *mut u8;

    /// MEGA65 model ID. Can be used to determine the model of MEGA65 a programme is running on, e.g., to enable touch controls on MEGAphone. [0xD629: M65MODEL]
    pub const MEGA65_MODEL_ID: *mut u8 = (0xD629) as *mut u8;
}

pub mod vic2 {

    /// sprite N horizontal position [0xD000: S0X]
    pub const SPRITE_N_HORIZONTAL_POSITION: *mut u8 = (0xD000) as *mut u8;

    /// sprite N vertical position [0xD001: S0Y]
    pub const SPRITE_N_VERTICAL_POSITION: *mut u8 = (0xD001) as *mut u8;

    /// @SNX [0xD002: S1X]
    pub const SNX: *mut u8 = (0xD002) as *mut u8;

    /// @SNY [0xD003: S1Y]
    pub const SNY: *mut u8 = (0xD003) as *mut u8;

    /// sprite horizontal position MSBs [0xD010: SXMSB]
    pub const SPRITE_HORIZONTAL_POSITION_MSBS: *mut u8 = (0xD010) as *mut u8;

    /// 24/25 vertical smooth scroll [0xD011: YSCL]
    pub const SCROLL_VERTICAL_SMOOTH_24_SLASH_25_MASK: u8 = 0b00000111;

    /// 24/25 row select [0xD011: RSEL]
    pub const SELECT_ROW_24_SLASH_25_MASK: u8 = 0b00001000;

    /// disable display [0xD011: BLNK]
    pub const DISABLE_DISPLAY_MASK: u8 = 0b00010000;

    /// bitmap mode [0xD011: BMM]
    pub const BITMAP_MODE_MASK: u8 = 0b00100000;

    /// extended background mode [0xD011: ECM]
    pub const EXTENDED_BACKGROUND_MODE_MASK: u8 = 0b01000000;

    /// raster compare bit 8 [0xD011: RC8]
    pub const RASTER_COMPARE_BIT_8_MASK: u8 = 0b10000000;

    /// raster compare bits 0 to 7 [0xD012: RC]
    pub const RASTER_COMPARE_BITS_0_TO_7: *mut u8 = (0xD012) as *mut u8;

    /// Coarse horizontal beam position (was lightpen X) [0xD013: LPX]
    pub const COARSE_HORIZONTAL_BEAM_POSITION: *mut u8 = (0xD013) as *mut u8;

    /// Coarse vertical beam position (was lightpen Y) [0xD014: LPY]
    pub const COARSE_VERTICAL_BEAM_POSITION: *mut u8 = (0xD014) as *mut u8;

    /// sprite enable bits [0xD015: SE]
    pub const SPRITE_ENABLE_BITS: *mut u8 = (0xD015) as *mut u8;

    /// horizontal smooth scroll [0xD016: XSCL]
    pub const HORIZONTAL_SMOOTH_SCROLL_MASK: u8 = 0b00000111;

    /// 38/40 column select [0xD016: CSEL]
    pub const SELECT_COLUMN_38_SLASH_40_MASK: u8 = 0b00001000;

    /// Multi-colour mode [0xD016: MCM]
    pub const MULTI_COLOUR_MODE_MASK: u8 = 0b00010000;

    /// Disables video output on MAX Machine(tm) VIC-II 6566.  Ignored on normal C64s and the MEGA65 [0xD016: RST]
    pub const DISABLES_VIDEO_OUTPUT_ON_MAX_MACHINE_MASK: u8 = 0b00100000;

    /// sprite vertical expansion enable bits [0xD017: SEXY]
    pub const SPRITE_VERTICAL_EXPANSION_ENABLE_BITS: *mut u8 = (0xD017) as *mut u8;

    /// character set address location ($\times$ 1KiB) [0xD018: CB]
    pub const CHARACTER_SET_ADDRESS_LOCATION_MASK: u8 = 0b00000111;

    /// screen address ($\times$ 1KiB) [0xD018: VS]
    pub const SCREEN_ADDRESS_MASK: u8 = 0b00001111;

    /// raster compare indicate or acknowledge [0xD019: RIRQ]
    pub const RASTER_COMPARE_INDICATE_OR_ACKNOWLEDGE_MASK: u8 = 0b00000001;

    /// sprite:bitmap collision indicate or acknowledge [0xD019: ISBC]
    pub const SPRITE_MASK: u8 = 0b00000010;

    /// light pen indicate or acknowledge [0xD019: ILP]
    pub const LIGHT_PEN_INDICATE_OR_ACKNOWLEDGE_MASK: u8 = 0b00001000;

    /// mask raster IRQ [0xD01A: MRIRQ]
    pub const MASK_RASTER_IRQ_MASK: u8 = 0b00000001;

    /// mask sprite:bitmap collision IRQ [0xD01A: MISBC]
    pub const MASK_SPRITE_MASK: u8 = 0b00000010;

    /// sprite background priority bits [0xD01B: BSP]
    pub const SPRITE_BACKGROUND_PRIORITY_BITS: *mut u8 = (0xD01B) as *mut u8;

    /// sprite multicolour enable bits [0xD01C: SCM]
    pub const SPRITE_MULTICOLOUR_ENABLE_BITS: *mut u8 = (0xD01C) as *mut u8;

    /// sprite horizontal expansion enable bits [0xD01D: SEXX]
    pub const SPRITE_HORIZONTAL_EXPANSION_ENABLE_BITS: *mut u8 = (0xD01D) as *mut u8;

    /// sprite/sprite collision indicate bits [0xD01E: SSC]
    pub const SPRITE_SLASH_SPRITE_COLLISION_INDICATE_BITS: *mut u8 = (0xD01E) as *mut u8;

    /// sprite/foreground collision indicate bits [0xD01F: SBC]
    pub const SPRITE_SLASH_FOREGROUND_COLLISION_INDICATE_BITS: *mut u8 = (0xD01F) as *mut u8;

    /// display border colour (16 colour) [0xD020: BORDERCOL]
    pub const DISPLAY_BORDER_COLOUR_MASK: u8 = 0b00001111;

    /// screen colour (16 colour) [0xD021: SCREENCOL]
    pub const SCREEN_COLOUR_MASK: u8 = 0b00001111;

    /// multi-colour 1 (16 colour) [0xD022: MC1]
    pub const MULTI_COLOUR_1_MASK: u8 = 0b00001111;

    /// multi-colour 2 (16 colour) [0xD023: MC2]
    pub const MULTI_COLOUR_2_MASK: u8 = 0b00001111;

    /// multi-colour 3 (16 colour) [0xD024: MC3]
    pub const MULTI_COLOUR_3_MASK: u8 = 0b00001111;

    /// sprite N colour / 16-colour sprite transparency colour (lower nybl) [0xD027: SPR0COL]
    pub const SPRITE_N_COLOUR__SLASH__16_COLOUR_SPRITE_TRANSPARENCY_COLOUR: *mut u8 =
        (0xD027) as *mut u8;

    /// @SPRNCOL [0xD028: SPR1COL]
    pub const SPRNCOL: *mut u8 = (0xD028) as *mut u8;

    /// 2MHz select (for C128 2MHz emulation) [0xD030: C128]
    pub const SELECT_2MHZ_MASK: u8 = 0b00000001;
}

pub mod vic3 {

    /// Sprite multi-colour 0 (8-bit for selection of any palette colour) [0xD025: SPRMC0]
    pub const SPRITE_MULTI_COLOUR_0: *mut u8 = (0xD025) as *mut u8;

    /// Sprite multi-colour 1 (8-bit for selection of any palette colour) [0xD026: SPRMC1]
    pub const SPRITE_MULTI_COLOUR_1: *mut u8 = (0xD026) as *mut u8;

    /// Write $A5 then $96 to enable C65/VIC-III IO registers [0xD02F: KEY]
    pub const WRITE_0XA5_THEN_0X96_TO_ENABLE_C65_SLASH_VIC_III_IO_REGISTERS: *mut u8 =
        (0xD02F) as *mut u8;

    /// Map 2nd KB of colour RAM @ $DC00-$DFFF [0xD030: CRAM2K]
    pub const MAP_2ND_KB_OF_COLOUR_RAM__0XDC00_0XDFFF_MASK: u8 = 0b00000001;

    /// Enable external video sync (genlock input) [0xD030: EXTSYNC]
    pub const ENABLE_EXTERNAL_VIDEO_SYNC_MASK: u8 = 0b00000010;

    /// Use PALETTE ROM (0) or RAM (1) entries for colours 0 - 15 [0xD030: PAL]
    pub const USE_PALETTE_ROM_MASK: u8 = 0b00000100;

    /// Map C65 ROM @ $8000 [0xD030: ROM8]
    pub const MAP_C65_ROM__0X8000_MASK: u8 = 0b00001000;

    /// Map C65 ROM @ $A000 [0xD030: ROMA]
    pub const MAP_C65_ROM__0XA000_MASK: u8 = 0b00010000;

    /// Map C65 ROM @ $C000 [0xD030: ROMC]
    pub const MAP_C65_ROM__0XC000_MASK: u8 = 0b00100000;

    /// Select between C64 and C65 charset. [0xD030: CROM9]
    pub const SELECT_BETWEEN_C64_AND_C65_CHARSET_MASK: u8 = 0b01000000;

    /// Map C65 ROM @ $E000 [0xD030: ROME]
    pub const MAP_C65_ROM__0XE000_MASK: u8 = 0b10000000;

    /// Enable VIC-III interlaced mode [0xD031: INT]
    pub const ENABLE_VIC_III_INTERLACED_MODE_MASK: u8 = 0b00000001;

    /// Enable VIC-III MONO video output (not implemented) [0xD031: MONO]
    pub const ENABLE_VIC_III_MONO_VIDEO_OUTPUT_MASK: u8 = 0b00000010;

    /// Enable 1280 horizontal pixels (not implemented) [0xD031: H1280]
    pub const ENABLE_1280_HORIZONTAL_PIXELS_MASK: u8 = 0b00000100;

    /// Enable 400 vertical pixels [0xD031: V400]
    pub const ENABLE_400_VERTICAL_PIXELS_MASK: u8 = 0b00001000;

    /// Bit-Plane Mode [0xD031: BPM]
    pub const BIT_PLANE_MODE_MASK: u8 = 0b00010000;

    /// Enable extended attributes and 8 bit colour entries [0xD031: ATTR]
    pub const ENABLE_EXTENDED_ATTRIBUTES_AND_8_BIT_COLOUR_ENTRIES_MASK: u8 = 0b00100000;

    /// Enable C65 FAST mode (~3.5MHz) [0xD031: FAST]
    pub const ENABLE_C65_FAST_MODE_MASK: u8 = 0b01000000;

    /// Enable C64 640 horizontal pixels / 80 column mode [0xD031: H640]
    pub const ENABLE_C64_640_HORIZONTAL_PIXELS__SLASH__80_COLUMN_MODE_MASK: u8 = 0b10000000;

    /// Bitplane X address, even lines [0xD033: B0ADEVN]
    pub const BITPLANE_X_ADDRESS_MASK: u8 = 0b00000111;

    /// @BXADEVN [0xD034: B1ADEVN]
    pub const BXADEVN_MASK: u8 = 0b00000111;

    /// @BXADODD [0xD034: B1ADODD]
    pub const BXADODD_MASK: u8 = 0b00000111;

    /// Complement bitplane flags [0xD03B: BPCOMP]
    pub const COMPLEMENT_BITPLANE_FLAGS: *mut u8 = (0xD03B) as *mut u8;

    /// Bitplane X Offset [0xD03E: HPOS]
    pub const BITPLANE_X_OFFSET: *mut u8 = (0xD03E) as *mut u8;

    /// Bitplane Y Offset [0xD03F: VPOS]
    pub const BITPLANE_Y_OFFSET: *mut u8 = (0xD03F) as *mut u8;

    /// Display Address Translater (DAT) Bitplane N port [0xD040: B0PIX]
    pub const DISPLAY_ADDRESS_TRANSLATER: *mut u8 = (0xD040) as *mut u8;

    /// @BNPIX [0xD041: B1PIX]
    pub const BNPIX: *mut u8 = (0xD041) as *mut u8;
}

pub mod vic4 {

    /// Write $45 then $54 to map 45E100 ethernet controller buffers to $D000-$DFFF [0xD02F: KEY]
    pub const WRITE_0X45_THEN_0X54_TO_MAP_45E100_ETHERNET_CONTROLLER_BUFFERS_TO_0XD000_0XDFFF:
        *mut u8 = (0xD02F) as *mut u8;

    /// Write $47 then $53 to enable C65GS/VIC-IV IO registers [0xD02F: KEY]
    pub const WRITE_0X47_THEN_0X53_TO_ENABLE_C65GS_SLASH_VIC_IV_IO_REGISTERS: *mut u8 =
        (0xD02F) as *mut u8;

    /// top border position [0xD048: TBDRPOS]
    pub const TOP_BORDER_POSITION: *mut u8 = (0xD048) as *mut u8;

    /// top border position MSB [0xD049: TBDRPOS]
    pub const TOP_BORDER_POSITION_MSB_MASK: u8 = 0b00001111;

    /// Sprite bitplane-modify-mode enables [0xD049: SPRBPMEN]
    pub const SPRITE_BITPLANE_MODIFY_MODE_ENABLES_MASK: u8 = 0b00001111;

    /// bottom border position [0xD04A: BBDRPOS]
    pub const BOTTOM_BORDER_POSITION: *mut u8 = (0xD04A) as *mut u8;

    /// character generator horizontal position [0xD04C: TEXTXPOS]
    pub const CHARACTER_GENERATOR_HORIZONTAL_POSITION: *mut u8 = (0xD04C) as *mut u8;

    /// Sprite horizontal tile enables. [0xD04D: SPRTILEN]
    pub const SPRITE_HORIZONTAL_TILE_ENABLES_MASK: u8 = 0b00001111;

    /// Character generator vertical position [0xD04E: TEXTYPOS]
    pub const CHARACTER_GENERATOR_VERTICAL_POSITION: *mut u8 = (0xD04E) as *mut u8;

    /// Sprite 7-4 horizontal tile enables [0xD04F: SPRTILEN]
    pub const SPRITE_7_4_HORIZONTAL_TILE_ENABLES_MASK: u8 = 0b00001111;

    /// Read horizontal raster scan position LSB [0xD050: XPOSLSB]
    pub const READ_HORIZONTAL_RASTER_SCAN_POSITION_LSB: *mut u8 = (0xD050) as *mut u8;

    /// Read horizontal raster scan position MSB [0xD051: XPOSMSB]
    pub const READ_HORIZONTAL_RASTER_SCAN_POSITION_MSB_MASK: u8 = 0b00111111;

    /// When set, the Raster Rewrite Buffer is only updated every 2nd raster line, limiting resolution to V200, but allowing more cycles for Raster-Rewrite actions. [0xD051: DBLRR]
    pub const WHEN_SET_MASK: u8 = 0b01000000;

    /// When clear, raster rewrite double buffering is used [0xD051: NORRDEL]
    pub const WHEN_CLEAR_MASK: u8 = 0b10000000;

    /// Read physical raster position [0xD052: FNRASTERLSB]
    pub const READ_PHYSICAL_RASTER_POSITION: *mut u8 = (0xD052) as *mut u8;

    /// Enable simulated shadow-mask (PALEMU must also be enabled) [0xD053: SHDEMU]
    pub const ENABLE_SIMULATED_SHADOW_MASK_MASK: u8 = 0b01000000;

    /// Raster compare source (0=VIC-IV fine raster, 1=VIC-II raster) [0xD053: FNRST]
    pub const RASTER_COMPARE_SOURCE_MASK: u8 = 0b10000000;

    /// enable 16-bit character numbers (two screen bytes per character) [0xD054: CHR16]
    pub const ENABLE_16_BIT_CHARACTER_NUMBERS_MASK: u8 = 0b00000001;

    /// enable full-colour mode for character numbers <=$FF [0xD054: FCLRLO]
    pub const ENABLE_FULL_COLOUR_MODE_FOR_CHARACTER_NUMBERS_LE0XFF_MASK: u8 = 0b00000010;

    /// enable full-colour mode for character numbers >$FF [0xD054: FCLRHI]
    pub const ENABLE_FULL_COLOUR_MODE_FOR_CHARACTER_NUMBERS_GT0XFF_MASK: u8 = 0b00000100;

    /// video output horizontal smoothing enable [0xD054: SMTH]
    pub const VIDEO_OUTPUT_HORIZONTAL_SMOOTHING_ENABLE_MASK: u8 = 0b00001000;

    /// Sprite H640 enable [0xD054: SPR]
    pub const SPRITE_H640_ENABLE_MASK: u8 = 0b00010000;

    /// Enable PAL CRT-like scan-line emulation [0xD054: PALEMU]
    pub const ENABLE_PAL_CRT_LIKE_SCAN_LINE_EMULATION_MASK: u8 = 0b00100000;

    /// C65GS FAST mode (48MHz) [0xD054: VFAST]
    pub const C65GS_FAST_MODE_MASK: u8 = 0b01000000;

    /// Alpha compositor enable [0xD054: ALPHEN]
    pub const ALPHA_COMPOSITOR_ENABLE_MASK: u8 = 0b10000000;

    /// sprite extended height enable (one bit per sprite) [0xD055: SPRHGTEN]
    pub const SPRITE_EXTENDED_HEIGHT_ENABLE: *mut u8 = (0xD055) as *mut u8;

    /// Sprite extended height size (sprite pixels high) [0xD056: SPRHGHT]
    pub const SPRITE_EXTENDED_HEIGHT_SIZE: *mut u8 = (0xD056) as *mut u8;

    /// Sprite extended width enables (8 bytes per sprite row = 64 pixels wide for normal sprites or 16 pixels wide for 16-colour sprite mode) [0xD057: SPRX64EN]
    pub const SPRITE_EXTENDED_WIDTH_ENABLES: *mut u8 = (0xD057) as *mut u8;

    /// number of bytes to advance between each text row (LSB) [0xD058: LINESTEPLSB]
    pub const NUMBER_OF_BYTES_TO_ADVANCE_BETWEEN_EACH_TEXT_ROW: *mut u8 = (0xD058) as *mut u8;

    /// Horizontal hardware scale of text mode (pixel 120ths per pixel) [0xD05A: CHRXSCL]
    pub const HORIZONTAL_HARDWARE_SCALE_OF_TEXT_MODE: *mut u8 = (0xD05A) as *mut u8;

    /// Vertical scaling of text mode (number of physical rasters per char text row) [0xD05B: CHRYSCL]
    pub const VERTICAL_SCALING_OF_TEXT_MODE: *mut u8 = (0xD05B) as *mut u8;

    /// Width of single side border (LSB) [0xD05C: SDBDRWD]
    pub const WIDTH_OF_SINGLE_SIDE_BORDER: *mut u8 = (0xD05C) as *mut u8;

    /// side border width (MSB) [0xD05D: SDBDRWD]
    pub const SIDE_BORDER_WIDTH_MASK: u8 = 0b00111111;

    /// Enable raster delay (delays raster counter and interrupts by one line to match output pipeline latency) [0xD05D: RST]
    pub const ENABLE_RASTER_DELAY_MASK: u8 = 0b01000000;

    /// Enable VIC-II hot registers. When enabled, touching many VIC-II registers causes the VIC-IV to recalculate display parameters, such as border positions and sizes [0xD05D: HOTREG]
    pub const ENABLE_VIC_II_HOT_REGISTERS_MASK: u8 = 0b10000000;

    /// Number of characters to display per row (LSB) [0xD05E: CHRCOUNT]
    pub const NUMBER_OF_CHARACTERS_TO_DISPLAY_PER_ROW: *mut u8 = (0xD05E) as *mut u8;

    /// Sprite H640 X Super-MSBs [0xD05F: SPRXSMSBS]
    pub const SPRITE_H640_X_SUPER_MSBS: *mut u8 = (0xD05F) as *mut u8;

    /// screen RAM precise base address (bits 0 - 7) [0xD060: SCRNPTRLSB]
    pub const SCREEN_RAM_PRECISE_BASE_ADDRESS: *mut u8 = (0xD060) as *mut u8;

    /// Number of characters to display per [0xD063: CHRCOUNT]
    pub const NUMBER_OF_CHARACTERS_TO_DISPLAY_PER_MASK: u8 = 0b00000011;

    /// source full-colour character data from expansion RAM [0xD063: EXGLYPH]
    pub const SOURCE_FULL_COLOUR_CHARACTER_DATA_FROM_EXPANSION_RAM_MASK: u8 = 0b10000000;

    /// colour RAM base address (bits 0 - 7) [0xD064: COLPTRLSB]
    pub const COLOUR_RAM_BASE_ADDRESS: *mut u8 = (0xD064) as *mut u8;

    /// Character set precise base address (bits 0 - 7) [0xD068: CHARPTRLSB]
    pub const CHARACTER_SET_PRECISE_BASE_ADDRESS: *mut u8 = (0xD068) as *mut u8;

    /// sprite 16-colour mode enables [0xD06B: SPR16EN]
    pub const SPRITE_16_COLOUR_MODE_ENABLES: *mut u8 = (0xD06B) as *mut u8;

    /// sprite pointer address (bits 7 - 0) [0xD06C: SPRPTRADRLSB]
    pub const SPRITE_POINTER_ADDRESS: *mut u8 = (0xD06C) as *mut u8;

    /// 16-bit sprite pointer mode (allows sprites to be located on any 64 byte boundary in chip RAM) [0xD06E: SPR]
    pub const MODE_SPRITE_POINTER_16_BIT_MASK: u8 = 0b10000000;

    /// first VIC-II raster line [0xD06F: RASLINE0]
    pub const FIRST_VIC_II_RASTER_LINE_MASK: u8 = 0b00111111;

    /// Select more VGA-compatible mode if set, instead of HDMI/HDTV VIC-II cycle-exact frame timing. May help to produce a functional display on older VGA monitors. [0xD06F: VGAHDTV]
    pub const SELECT_MORE_VGA_COMPATIBLE_MODE_IF_SET_MASK: u8 = 0b01000000;

    /// NTSC emulation mode (max raster = 262) [0xD06F: PALNTSC]
    pub const NTSC_EMULATION_MODE_MASK: u8 = 0b10000000;

    /// VIC-IV bitmap/text palette bank (alternate palette) [0xD070: ABTPALSEL]
    pub const VIC_IV_BITMAP_SLASH_TEXT_PALETTE_BANK_MASK: u8 = 0b00000011;

    /// sprite palette bank [0xD070: SPRPALSEL]
    pub const SPRITE_PALETTE_BANK_MASK: u8 = 0b00000011;

    /// bitmap/text palette bank [0xD070: BTPALSEL]
    pub const BITMAP_SLASH_TEXT_PALETTE_BANK_MASK: u8 = 0b00000011;

    /// palette bank mapped at $D100-$D3FF [0xD070: MAPEDPAL]
    pub const PALETTE_BANK_MAPPED_AT_0XD100_0XD3FF_MASK: u8 = 0b00000011;

    /// VIC-IV 16-colour bitplane enable flags [0xD071: BP16ENS]
    pub const VIC_IV_16_COLOUR_BITPLANE_ENABLE_FLAGS: *mut u8 = (0xD071) as *mut u8;

    /// Sprite Y position adjustment [0xD072: SPRYADJ]
    pub const SPRITE_Y_POSITION_ADJUSTMENT: *mut u8 = (0xD072) as *mut u8;

    /// Alpha delay for compositor [0xD073: ALPHADELAY]
    pub const ALPHA_DELAY_FOR_COMPOSITOR_MASK: u8 = 0b00001111;

    /// physical rasters per VIC-II raster (1 to 16) [0xD073: RASTERHEIGHT]
    pub const PHYSICAL_RASTERS_PER_VIC_II_RASTER_MASK: u8 = 0b00001111;

    /// Sprite alpha-blend enable [0xD074: SPRENALPHA]
    pub const SPRITE_ALPHA_BLEND_ENABLE: *mut u8 = (0xD074) as *mut u8;

    /// Sprite alpha-blend value [0xD075: SPRALPHAVAL]
    pub const SPRITE_ALPHA_BLEND_VALUE: *mut u8 = (0xD075) as *mut u8;

    /// Sprite V400 enables [0xD076: SPRENV400]
    pub const SPRITE_V400_ENABLES: *mut u8 = (0xD076) as *mut u8;

    /// Sprite V400 Y position MSBs [0xD077: SPRYMSBS]
    pub const SPRITE_V400_Y_POSITION_MSBS: *mut u8 = (0xD077) as *mut u8;

    /// Sprite V400 Y position super MSBs [0xD078: SPRYSMSBS]
    pub const SPRITE_V400_Y_POSITION_SUPER_MSBS: *mut u8 = (0xD078) as *mut u8;

    /// Raster compare value [0xD079: RASCMP]
    pub const RASTER_COMPARE_VALUE: *mut u8 = (0xD079) as *mut u8;

    /// Raster compare value MSB [0xD07A: RASCMP]
    pub const RASTER_COMPARE_VALUE_MSB_MASK: u8 = 0b00000111;

    /// Continuously monitor sprite pointer, to allow changing sprite data source while a sprite is being drawn [0xD07A: SPTR]
    pub const CONTINUOUSLY_MONITOR_SPRITE_POINTER_MASK: u8 = 0b00001000;

    /// Reserved. [0xD07A: RESV]
    pub const RESERVED_MASK: u8 = 0b00000011;

    /// Enable additional IRQ sources, e.g., raster X position. [0xD07A: EXTIRQS]
    pub const ENABLE_ADDITIONAL_IRQ_SOURCES_MASK: u8 = 0b01000000;

    /// Raster compare is in physical rasters if set, or VIC-II raster if clear [0xD07A: FNRST]
    pub const RASTER_COMPARE_IS_IN_PHYSICAL_RASTERS_IF_SET_MASK: u8 = 0b10000000;

    /// Number of text rows to display [0xD07B: DISP]
    pub const NUMBER_OF_TEXT_ROWS_TO_DISPLAY: *mut u8 = (0xD07B) as *mut u8;

    /// Set which 128KB bank bitplanes [0xD07C: BIT]
    pub const SET_WHICH_128KB_BANK_BITPLANES_MASK: u8 = 0b00000111;

    /// @RESV [0xD07C: RESV]
    pub const RESV_MASK: u8 = 0b00001000;

    /// hsync polarity [0xD07C: HSYNCP]
    pub const HSYNC_POLARITY_MASK: u8 = 0b00010000;

    /// vsync polarity [0xD07C: VSYNCP]
    pub const VSYNC_POLARITY_MASK: u8 = 0b00100000;

    /// VIC-IV debug pixel select red(01), green(10) or blue(11) channel visible in $D07D [0xD07C: DEBUGC]
    pub const VIC_IV_DEBUG_PIXEL_SELECT_RED_MASK: u8 = 0b00000011;

    /// palette bank selection [0xD070: VIC_IV]
    pub const PALETTE_BANK_SELECTION: *mut u8 = (0xD070) as *mut u8;
}
