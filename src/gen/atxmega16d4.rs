//! The AVR ATxmega16D4 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATxmega16D4-AU | QFN-QFP-44 | TQFP44 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega16D4-MH | QFN-QFP-44 | VQFN44 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega16D4-CU | BGA-49 | VFBGA49 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega16D4-AN | QFN-QFP-44 | TQFP44 | -40°C - 105°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega16D4-M7 | BGA-49 | VFBGA49 | -40°C - 105°C | 1.6V - 3.6V | 32 MHz |
//!

#![allow(non_upper_case_globals)]

/// Device ID byte 0.
pub const DEVID0: *mut u8 = 0x0 as *mut u8;

/// General Power Reduction.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTC | 100 |
/// | EVSYS | 10 |
pub const PRGEN: *mut u8 = 0x0 as *mut u8;

/// Multi-pin Configuration Mask.
pub const MPCMASK: *mut u8 = 0x0 as *mut u8;

/// Address Register 0.
pub const ADDR0: *mut u8 = 0x0 as *mut u8;

/// Analog Comparator 0 Control.
pub const AC0CTRL: *mut u8 = 0x0 as *mut u8;

/// Event Channel 0 Multiplexer.
pub const CH0MUX: *mut u8 = 0x0 as *mut u8;

/// OCD Register 0.
pub const OCDR0: *mut u8 = 0x0 as *mut u8;

/// Lock Bits.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLBB | 11000000 |
/// | LB | 11 |
/// | BLBA | 110000 |
/// | BLBAT | 1100 |
pub const LOCKBITS: *mut u8 = 0x0 as *mut u8;

/// I/O Port Data Direction.
pub const DIR: *mut u8 = 0x0 as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x0 as *mut u8;

/// Control Register.
pub const CTRL: *mut u8 = 0x0 as *mut u8;

/// RCOSC 2MHz Calibration Value.
pub const RCOSC2M: *mut u8 = 0x0 as *mut u8;

/// Event Channel 1 Multiplexer.
pub const CH1MUX: *mut u8 = 0x1 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x1 as *mut u8;

/// Prescaler Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSBCDIV | 11 |
/// | PSADIV | 1111100 |
pub const PSCTRL: *mut u8 = 0x1 as *mut u8;

/// I/O Port Data Direction Set.
pub const DIRSET: *mut u8 = 0x1 as *mut u8;

/// MUX Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUXINT | 1111000 |
pub const MUXCTRL: *mut u8 = 0x1 as *mut u8;

/// Device ID byte 1.
pub const DEVID1: *mut u8 = 0x1 as *mut u8;

/// Analog Comparator 1 Control.
pub const AC1CTRL: *mut u8 = 0x1 as *mut u8;

/// Interrupt Priority.
pub const INTPRI: *mut u8 = 0x1 as *mut u8;

/// OCD Register 1.
pub const OCDR1: *mut u8 = 0x1 as *mut u8;

/// Watchdog Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 1111 |
/// | WDWP | 11110000 |
pub const FUSEBYTE1: *mut u8 = 0x1 as *mut u8;

/// Power Reduction Port A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC | 1 |
/// | ADC | 10 |
pub const PRPA: *mut u8 = 0x1 as *mut u8;

/// IrDA Transmitter Pulse Length Control Register.
pub const TXPLCTRL: *mut u8 = 0x1 as *mut u8;

/// Address Register 1.
pub const ADDR1: *mut u8 = 0x1 as *mut u8;

/// Interrupt Control Register.
pub const INTCTRL: *mut u8 = 0x1 as *mut u8;

/// Reference Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TEMPREF | 1 |
/// | REFSEL | 1110000 |
/// | BANDGAP | 10 |
pub const REFCTRL: *mut u8 = 0x2 as *mut u8;

/// Calibration Register A.
pub const CALA: *mut u8 = 0x2 as *mut u8;

/// IrDA Receiver Pulse Length Control Register.
pub const RXPLCTRL: *mut u8 = 0x2 as *mut u8;

/// Lock register.
pub const LOCK: *mut u8 = 0x2 as *mut u8;

/// Analog Comparator 0 MUX Control.
pub const AC0MUXCTRL: *mut u8 = 0x2 as *mut u8;

/// Address Register 2.
pub const ADDR2: *mut u8 = 0x2 as *mut u8;

/// I/O Port Data Direction Clear.
pub const DIRCLR: *mut u8 = 0x2 as *mut u8;

/// Event Channel 2 Multiplexer.
pub const CH2MUX: *mut u8 = 0x2 as *mut u8;

/// Device ID byte 2.
pub const DEVID2: *mut u8 = 0x2 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x2 as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IF | 10000000 |
/// | WRCOL | 1000000 |
pub const STATUS: *mut u8 = 0x2 as *mut u8;

/// Reset Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DVSDON | 10000000 |
/// | BOOTRST | 1000000 |
/// | TOSCSEL | 100000 |
/// | BODPD | 11 |
pub const FUSEBYTE2: *mut u8 = 0x2 as *mut u8;

/// Fault Detection Event Mask.
pub const FDEMASK: *mut u8 = 0x2 as *mut u8;

/// Virtual Port Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VP1MAP | 11110000 |
/// | VP0MAP | 1111 |
pub const VPCTRLA: *mut u8 = 0x2 as *mut u8;

/// External Oscillator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XOSCSEL | 1111 |
/// | FRQRANGE | 11000000 |
/// | X32KLPM | 100000 |
pub const XOSCCTRL: *mut u8 = 0x2 as *mut u8;

/// RCOSC 32kHz Calibration Value.
pub const RCOSC32K: *mut u8 = 0x2 as *mut u8;

/// RCOSC 32MHz Calibration Value.
pub const RCOSC32M: *mut u8 = 0x3 as *mut u8;

/// Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXCINTLVL | 110000 |
/// | DREINTLVL | 11 |
/// | TXCINTLVL | 1100 |
pub const CTRLA: *mut u8 = 0x3 as *mut u8;

/// Event Control.
pub const EVCTRL: *mut u8 = 0x3 as *mut u8;

/// Revision ID.
pub const REVID: *mut u8 = 0x3 as *mut u8;

/// Virtual Port Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VP2MAP | 1111 |
/// | VP3MAP | 11110000 |
pub const VPCTRLB: *mut u8 = 0x3 as *mut u8;

/// Fault Detection Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FDDBD | 10000 |
/// | FDMODE | 100 |
/// | FDACT | 11 |
pub const FDCTRL: *mut u8 = 0x3 as *mut u8;

/// Control Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVDLY | 10000 |
pub const CTRLD: *mut u8 = 0x3 as *mut u8;

/// General Purpose IO Register 3.
pub const GPIOR3: *mut u8 = 0x3 as *mut u8;

/// I/O Port Data Direction Toggle.
pub const DIRTGL: *mut u8 = 0x3 as *mut u8;

/// Address Register.
pub const ADDR: *mut u8 = 0x3 as *mut u8;

/// Event Channel 3 Multiplexer.
pub const CH3MUX: *mut u8 = 0x3 as *mut u8;

/// RTC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTCSRC | 1110 |
/// | RTCEN | 1 |
pub const RTCCTRL: *mut u8 = 0x3 as *mut u8;

/// Data Input.
pub const DATAIN: *mut u8 = 0x3 as *mut u8;

/// Analog Comparator 1 MUX Control.
pub const AC1MUXCTRL: *mut u8 = 0x3 as *mut u8;

/// Data Register.
pub const DATA: *mut u8 = 0x3 as *mut u8;

/// Calibration Register B.
pub const CALB: *mut u8 = 0x3 as *mut u8;

/// Power Reduction Port C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HIRES | 100 |
/// | TC1 | 10 |
pub const PRPC: *mut u8 = 0x3 as *mut u8;

/// External Oscillator Failure Detection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XOSCFDEN | 1 |
/// | XOSCFDIF | 10 |
pub const XOSCFAIL: *mut u8 = 0x3 as *mut u8;

/// Start-up Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDLOCK | 10 |
/// | RSTDISBL | 10000 |
/// | SUT | 1100 |
pub const FUSEBYTE4: *mut u8 = 0x4 as *mut u8;

/// Baurd Rate Control Register.
pub const BAUD: *mut u8 = 0x4 as *mut u8;

/// I/O Port Output.
pub const OUT: *mut u8 = 0x4 as *mut u8;

/// Checksum byte 0.
pub const CHECKSUM0: *mut u8 = 0x4 as *mut u8;

/// Data Register 0.
pub const DATA0: *mut u8 = 0x4 as *mut u8;

/// JTAG User ID.
pub const JTAGUID: *mut u8 = 0x4 as *mut u8;

/// Oscillator Compare Register 0.
pub const COMP0: *mut u8 = 0x4 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x4 as *mut u8;

/// Clock and Event Out Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOUT | 11 |
/// | EVOUT | 110000 |
pub const CLKEVOUT: *mut u8 = 0x4 as *mut u8;

/// Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXEN | 1000 |
/// | TXB8 | 1 |
/// | RXEN | 10000 |
/// | CLK2X | 100 |
/// | MPCM | 10 |
pub const CTRLB: *mut u8 = 0x4 as *mut u8;

/// Channel Result low byte.
pub const RESL: *mut u8 = 0x4 as *mut u8;

/// Clock Prescaler.
pub const PRESCALER: *mut u8 = 0x4 as *mut u8;

/// Channel Result.
pub const RES: *mut u16 = 0x4 as *mut u16;

/// Power Reduction Port D.
pub const PRPD: *mut u8 = 0x4 as *mut u8;

/// 32kHz Internal Oscillator Calibration Register.
pub const RC32KCAL: *mut u8 = 0x4 as *mut u8;

/// Control Register E.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BYTEM | 11 |
pub const CTRLE: *mut u8 = 0x4 as *mut u8;

/// Channel Result high byte.
pub const RESH: *mut u8 = 0x5 as *mut u8;

/// I/O Port Output Set.
pub const OUTSET: *mut u8 = 0x5 as *mut u8;

/// Checksum byte 1.
pub const CHECKSUM1: *mut u8 = 0x5 as *mut u8;

/// PLL Control REgister.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLFAC | 11111 |
/// | PLLSRC | 11000000 |
pub const PLLCTRL: *mut u8 = 0x5 as *mut u8;

/// Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMODE | 11000000 |
/// | CHSIZE | 111 |
/// | UDORD | 100 |
/// | UCPHA | 10 |
/// | PMODE | 110000 |
/// | SBMODE | 1000 |
pub const CTRLC: *mut u8 = 0x5 as *mut u8;

/// Data Register 1.
pub const DATA1: *mut u8 = 0x5 as *mut u8;

/// Oscillator Compare Register 1.
pub const COMP1: *mut u8 = 0x5 as *mut u8;

/// Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADDREN | 1 |
pub const ADDRMASK: *mut u8 = 0x5 as *mut u8;

/// Power Reduction Port E.
pub const PRPE: *mut u8 = 0x5 as *mut u8;

/// EESAVE and BOD Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLVL | 111 |
/// | BODACT | 110000 |
/// | EESAVE | 1000 |
pub const FUSEBYTE5: *mut u8 = 0x5 as *mut u8;

/// Input Channel Scan.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COUNT | 1111 |
/// | OFFSET | 11110000 |
/// | SCANNUM | 1111 |
pub const SCAN: *mut u8 = 0x6 as *mut u8;

/// Baud Rate Control Register A.
pub const BAUDCTRLA: *mut u8 = 0x6 as *mut u8;

/// MCU Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTAGD | 1 |
pub const MCUCR: *mut u8 = 0x6 as *mut u8;

/// Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LUNFINTLVL | 11 |
/// | HUNFINTLVL | 1100 |
pub const INTCTRLA: *mut u8 = 0x6 as *mut u8;

/// DFLL Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RC2MCREF | 1 |
/// | RC32MCREF | 10 |
pub const DFLLCTRL: *mut u8 = 0x6 as *mut u8;

/// Oscillator Compare Register 2.
pub const COMP2: *mut u8 = 0x6 as *mut u8;

/// Window Mode Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WINTMODE | 1100 |
/// | WEN | 10000 |
/// | WINTLVL | 11 |
pub const WINCTRL: *mut u8 = 0x6 as *mut u8;

/// Checksum byte 2.
pub const CHECKSUM2: *mut u8 = 0x6 as *mut u8;

/// Dead Time Both Sides.
pub const DTBOTH: *mut u8 = 0x6 as *mut u8;

/// Power Reduction Port F.
pub const PRPF: *mut u8 = 0x6 as *mut u8;

/// I/O Port Output Clear.
pub const OUTCLR: *mut u8 = 0x6 as *mut u8;

/// Data Register 2.
pub const DATA2: *mut u8 = 0x6 as *mut u8;

/// Checksum byte 3.
pub const CHECKSUM3: *mut u8 = 0x7 as *mut u8;

/// Dead Time Both Sides Buffer.
pub const DTBOTHBUF: *mut u8 = 0x7 as *mut u8;

/// I/O Port Output Toggle.
pub const OUTTGL: *mut u8 = 0x7 as *mut u8;

/// Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCMPCINTLVL | 110000 |
/// | LCMPDINTLVL | 11000000 |
/// | LCMPAINTLVL | 11 |
/// | LCMPBINTLVL | 1100 |
pub const INTCTRLB: *mut u8 = 0x7 as *mut u8;

/// Baud Rate Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BSCALE | 11110000 |
pub const BAUDCTRLB: *mut u8 = 0x7 as *mut u8;

/// Lot Number Byte 0, ASCII.
pub const LOTNUM0: *mut u8 = 0x8 as *mut u8;

/// Event System Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVSYS0LOCK | 1 |
/// | EVSYS1LOCK | 10000 |
pub const EVSYSLOCK: *mut u8 = 0x8 as *mut u8;

/// Sampling Time Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAMPVAL | 111111 |
pub const SAMPCTRL: *mut u8 = 0x8 as *mut u8;

/// I/O port Input.
pub const IN: *mut u8 = 0x8 as *mut u8;

/// Control Register F Clear.
pub const CTRLFCLR: *mut u8 = 0x8 as *mut u8;

/// Channel 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | QDIEN | 10000 |
/// | QDEN | 1000 |
/// | QDIRM | 1100000 |
pub const CH0CTRL: *mut u8 = 0x8 as *mut u8;

/// Dead Time Low Side.
pub const DTLS: *mut u8 = 0x8 as *mut u8;

/// Ramp D.
pub const RAMPD: *mut u8 = 0x8 as *mut u8;

/// Ramp X.
pub const RAMPX: *mut u8 = 0x9 as *mut u8;

/// Lot Number Byte 1, ASCII.
pub const LOTNUM1: *mut u8 = 0x9 as *mut u8;

/// AWEX Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AWEXCLOCK | 1 |
/// | AWEXELOCK | 100 |
pub const AWEXLOCK: *mut u8 = 0x9 as *mut u8;

/// Control Register F Set.
pub const CTRLFSET: *mut u8 = 0x9 as *mut u8;

/// Channel 1 Control Register.
pub const CH1CTRL: *mut u8 = 0x9 as *mut u8;

/// Control Register F.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMDEN | 11 |
pub const CTRLF: *mut u8 = 0x9 as *mut u8;

/// Dead Time High Side.
pub const DTHS: *mut u8 = 0x9 as *mut u8;

/// Port Interrupt 0 Mask.
pub const INT0MASK: *mut u8 = 0xA as *mut u8;

/// Lot Number Byte 2, ASCII.
pub const LOTNUM2: *mut u8 = 0xA as *mut u8;

/// Dead Time Low Side Buffer.
pub const DTLSBUF: *mut u8 = 0xA as *mut u8;

/// Command.
pub const CMD: *mut u8 = 0xA as *mut u8;

/// Channel 2 Control Register.
pub const CH2CTRL: *mut u8 = 0xA as *mut u8;

/// Control Register G Clear.
pub const CTRLGCLR: *mut u8 = 0xA as *mut u8;

/// Ramp Y.
pub const RAMPY: *mut u8 = 0xA as *mut u8;

/// Control Register G Set.
pub const CTRLGSET: *mut u8 = 0xB as *mut u8;

/// Ramp Z.
pub const RAMPZ: *mut u8 = 0xB as *mut u8;

/// Port Interrupt 1 Mask.
pub const INT1MASK: *mut u8 = 0xB as *mut u8;

/// Lot Number Byte 3, ASCII.
pub const LOTNUM3: *mut u8 = 0xB as *mut u8;

/// Channel 3 Control Register.
pub const CH3CTRL: *mut u8 = 0xB as *mut u8;

/// Dead Time High Side Buffer.
pub const DTHSBUF: *mut u8 = 0xB as *mut u8;

/// Calibration Value low byte.
pub const CALL: *mut u8 = 0xC as *mut u8;

/// Compare Register low byte.
pub const COMPL: *mut u8 = 0xC as *mut u8;

/// Lot Number Byte 4, ASCII.
pub const LOTNUM4: *mut u8 = 0xC as *mut u8;

/// Extended Indirect Jump.
pub const EIND: *mut u8 = 0xC as *mut u8;

/// Calibration Value.
pub const CAL: *mut u16 = 0xC as *mut u16;

/// Compare Register.
pub const COMP: *mut u16 = 0xC as *mut u16;

/// Output Override Enable.
pub const OUTOVEN: *mut u8 = 0xC as *mut u8;

/// Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCMPBIF | 100000 |
/// | HUNFIF | 10 |
/// | LUNFIF | 1 |
/// | LCMPAIF | 10000 |
/// | LCMPDIF | 10000000 |
/// | LCMPCIF | 1000000 |
pub const INTFLAGS: *mut u8 = 0xC as *mut u8;

/// Stack Pointer Low.
pub const SPL: *mut u8 = 0xD as *mut u8;

/// Calibration Value high byte.
pub const CALH: *mut u8 = 0xD as *mut u8;

/// Lot Number Byte 5, ASCII.
pub const LOTNUM5: *mut u8 = 0xD as *mut u8;

/// Compare Register high byte.
pub const COMPH: *mut u8 = 0xD as *mut u8;

/// Pin Remap Register (available for PORTC to PORTF only).
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TC0A | 1 |
/// | TC0B | 10 |
/// | TC0C | 100 |
/// | TC0D | 1000 |
pub const REMAP: *mut u8 = 0xE as *mut u8;

/// Stack Pointer High.
pub const SPH: *mut u8 = 0xE as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | S | 10000 |
/// | I | 10000000 |
/// | V | 1000 |
/// | Z | 10 |
/// | N | 100 |
/// | C | 1 |
/// | T | 1000000 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0xF as *mut u8;

/// Temporary Register For 16-bit Access.
pub const TEMP: *mut u8 = 0xF as *mut u8;

/// Channel 0 Result low byte.
pub const CH0RESL: *mut u8 = 0x10 as *mut u8;

/// Wafer Number.
pub const WAFNUM: *mut u8 = 0x10 as *mut u8;

/// Event Strobe.
pub const STROBE: *mut u8 = 0x10 as *mut u8;

/// Pin 0 Control Register.
pub const PIN0CTRL: *mut u8 = 0x10 as *mut u8;

/// Channel 0 Result.
pub const CH0RES: *mut u16 = 0x10 as *mut u16;

/// Channel 0 Result high byte.
pub const CH0RESH: *mut u8 = 0x11 as *mut u8;

/// Pin 1 Control Register.
pub const PIN1CTRL: *mut u8 = 0x11 as *mut u8;

/// Pin 2 Control Register.
pub const PIN2CTRL: *mut u8 = 0x12 as *mut u8;

/// Channel 1 Result low byte.
pub const CH1RESL: *mut u8 = 0x12 as *mut u8;

/// Channel 1 Result.
pub const CH1RES: *mut u16 = 0x12 as *mut u16;

/// Wafer Coordinate X Byte 0.
pub const COORDX0: *mut u8 = 0x12 as *mut u8;

/// Pin 3 Control Register.
pub const PIN3CTRL: *mut u8 = 0x13 as *mut u8;

/// Wafer Coordinate X Byte 1.
pub const COORDX1: *mut u8 = 0x13 as *mut u8;

/// Channel 1 Result high byte.
pub const CH1RESH: *mut u8 = 0x13 as *mut u8;

/// Channel 2 Result.
pub const CH2RES: *mut u16 = 0x14 as *mut u16;

/// Channel 2 Result low byte.
pub const CH2RESL: *mut u8 = 0x14 as *mut u8;

/// Wafer Coordinate Y Byte 0.
pub const COORDY0: *mut u8 = 0x14 as *mut u8;

/// Pin 4 Control Register.
pub const PIN4CTRL: *mut u8 = 0x14 as *mut u8;

/// Pin 5 Control Register.
pub const PIN5CTRL: *mut u8 = 0x15 as *mut u8;

/// Wafer Coordinate Y Byte 1.
pub const COORDY1: *mut u8 = 0x15 as *mut u8;

/// Channel 2 Result high byte.
pub const CH2RESH: *mut u8 = 0x15 as *mut u8;

/// Channel 3 Result.
pub const CH3RES: *mut u16 = 0x16 as *mut u16;

/// Channel 3 Result low byte.
pub const CH3RESL: *mut u8 = 0x16 as *mut u8;

/// Pin 6 Control Register.
pub const PIN6CTRL: *mut u8 = 0x16 as *mut u8;

/// Channel 3 Result high byte.
pub const CH3RESH: *mut u8 = 0x17 as *mut u8;

/// Pin 7 Control Register.
pub const PIN7CTRL: *mut u8 = 0x17 as *mut u8;

/// Compare Value low byte.
pub const CMPL: *mut u8 = 0x18 as *mut u8;

/// Compare Value.
pub const CMP: *mut u16 = 0x18 as *mut u16;

/// Compare Value high byte.
pub const CMPH: *mut u8 = 0x19 as *mut u8;

/// Count low byte.
pub const CNTL: *mut u8 = 0x20 as *mut u8;

/// ADCA Calibration Byte 0.
pub const ADCACAL0: *mut u8 = 0x20 as *mut u8;

/// Low Byte Count.
pub const LCNT: *mut u8 = 0x20 as *mut u8;

/// Count.
pub const CNT: *mut u16 = 0x20 as *mut u16;

/// Count high byte.
pub const CNTH: *mut u8 = 0x21 as *mut u8;

/// High Byte Count.
pub const HCNT: *mut u8 = 0x21 as *mut u8;

/// ADCA Calibration Byte 1.
pub const ADCACAL1: *mut u8 = 0x21 as *mut u8;

/// ADCB Calibration Byte 0.
pub const ADCBCAL0: *mut u8 = 0x24 as *mut u8;

/// ADCB Calibration Byte 1.
pub const ADCBCAL1: *mut u8 = 0x25 as *mut u8;

/// Period.
pub const PER: *mut u16 = 0x26 as *mut u16;

/// Period low byte.
pub const PERL: *mut u8 = 0x26 as *mut u8;

/// Low Byte Period.
pub const LPER: *mut u8 = 0x26 as *mut u8;

/// High Byte Period.
pub const HPER: *mut u8 = 0x27 as *mut u8;

/// Period high byte.
pub const PERH: *mut u8 = 0x27 as *mut u8;

/// Compare or Capture A.
pub const CCA: *mut u16 = 0x28 as *mut u16;

/// Low Byte Compare A.
pub const LCMPA: *mut u8 = 0x28 as *mut u8;

/// Compare or Capture A low byte.
pub const CCAL: *mut u8 = 0x28 as *mut u8;

/// High Byte Compare A.
pub const HCMPA: *mut u8 = 0x29 as *mut u8;

/// Compare or Capture A high byte.
pub const CCAH: *mut u8 = 0x29 as *mut u8;

/// Compare or Capture B.
pub const CCB: *mut u16 = 0x2A as *mut u16;

/// Compare or Capture B low byte.
pub const CCBL: *mut u8 = 0x2A as *mut u8;

/// Low Byte Compare B.
pub const LCMPB: *mut u8 = 0x2A as *mut u8;

/// Compare or Capture B high byte.
pub const CCBH: *mut u8 = 0x2B as *mut u8;

/// High Byte Compare B.
pub const HCMPB: *mut u8 = 0x2B as *mut u8;

/// Compare or Capture C.
pub const CCC: *mut u16 = 0x2C as *mut u16;

/// Compare or Capture C low byte.
pub const CCCL: *mut u8 = 0x2C as *mut u8;

/// Low Byte Compare C.
pub const LCMPC: *mut u8 = 0x2C as *mut u8;

/// High Byte Compare C.
pub const HCMPC: *mut u8 = 0x2D as *mut u8;

/// Compare or Capture C high byte.
pub const CCCH: *mut u8 = 0x2D as *mut u8;

/// Low Byte Compare D.
pub const LCMPD: *mut u8 = 0x2E as *mut u8;

/// Compare or Capture D low byte.
pub const CCDL: *mut u8 = 0x2E as *mut u8;

/// Temperature Sensor Calibration Byte 0.
pub const TEMPSENSE0: *mut u8 = 0x2E as *mut u8;

/// Compare or Capture D.
pub const CCD: *mut u16 = 0x2E as *mut u16;

/// High Byte Compare D.
pub const HCMPD: *mut u8 = 0x2F as *mut u8;

/// Compare or Capture D high byte.
pub const CCDH: *mut u8 = 0x2F as *mut u8;

/// Temperature Sensor Calibration Byte 0.
pub const TEMPSENSE1: *mut u8 = 0x2F as *mut u8;

/// Period Buffer low byte.
pub const PERBUFL: *mut u8 = 0x36 as *mut u8;

/// Period Buffer.
pub const PERBUF: *mut u16 = 0x36 as *mut u16;

/// Period Buffer high byte.
pub const PERBUFH: *mut u8 = 0x37 as *mut u8;

/// Compare Or Capture A Buffer.
pub const CCABUF: *mut u16 = 0x38 as *mut u16;

/// Compare Or Capture A Buffer low byte.
pub const CCABUFL: *mut u8 = 0x38 as *mut u8;

/// Compare Or Capture A Buffer high byte.
pub const CCABUFH: *mut u8 = 0x39 as *mut u8;

/// Compare Or Capture B Buffer low byte.
pub const CCBBUFL: *mut u8 = 0x3A as *mut u8;

/// Compare Or Capture B Buffer.
pub const CCBBUF: *mut u16 = 0x3A as *mut u16;

/// Compare Or Capture B Buffer high byte.
pub const CCBBUFH: *mut u8 = 0x3B as *mut u8;

/// Compare Or Capture C Buffer low byte.
pub const CCCBUFL: *mut u8 = 0x3C as *mut u8;

/// Compare Or Capture C Buffer.
pub const CCCBUF: *mut u16 = 0x3C as *mut u16;

/// Compare Or Capture C Buffer high byte.
pub const CCCBUFH: *mut u8 = 0x3D as *mut u8;

/// Compare Or Capture D Buffer low byte.
pub const CCDBUFL: *mut u8 = 0x3E as *mut u8;

/// Compare Or Capture D Buffer.
pub const CCDBUF: *mut u16 = 0x3E as *mut u16;

/// Compare Or Capture D Buffer high byte.
pub const CCDBUFH: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ADDRMASK`
pub const ADDREN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `AWEXLOCK`
pub const AWEXCLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `AWEXLOCK`
pub const AWEXELOCK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BAUDCTRLB`
pub const BSCALE: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const QDIEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const QDEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const QDIRM: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const CLKOUT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const EVOUT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLA`
pub const RXCINTLVL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLA`
pub const DREINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLA`
pub const TXCINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CTRLB`
pub const TXEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CTRLB`
pub const TXB8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLB`
pub const RXEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CTRLB`
pub const CLK2X: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLB`
pub const MPCM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLC`
pub const CMODE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `CTRLC`
pub const CHSIZE: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CTRLC`
pub const UDORD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLC`
pub const UCPHA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLC`
pub const PMODE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLC`
pub const SBMODE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CTRLD`
pub const EVDLY: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CTRLE`
pub const BYTEM: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLF`
pub const CMDEN: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DFLLCTRL`
pub const RC2MCREF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DFLLCTRL`
pub const RC32MCREF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EVSYSLOCK`
pub const EVSYS0LOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EVSYSLOCK`
pub const EVSYS1LOCK: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FDCTRL`
pub const FDDBD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FDCTRL`
pub const FDMODE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FDCTRL`
pub const FDACT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FUSEBYTE1`
pub const WDP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FUSEBYTE1`
pub const WDWP: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const DVSDON: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const BOOTRST: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const TOSCSEL: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const BODPD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const WDLOCK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const RSTDISBL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const SUT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const BODLVL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const BODACT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const LUNFINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const HUNFINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPCINTLVL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPDINTLVL: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPAINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPBINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPBIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const HUNFIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LUNFIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPAIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPCIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBB: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBA: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBAT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MCUCR`
pub const JTAGD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MUXCTRL`
pub const MUXINT: *mut u8 = 0x78 as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLFAC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLSRC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PRGEN`
pub const RTC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRGEN`
pub const EVSYS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRPA`
pub const AC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRPA`
pub const ADC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRPC`
pub const HIRES: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRPC`
pub const TC1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PSCTRL`
pub const PSBCDIV: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PSCTRL`
pub const PSADIV: *mut u8 = 0x7C as *mut u8;

/// Bitfield on register `REFCTRL`
pub const TEMPREF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const REFSEL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const BANDGAP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0B: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0C: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RTCCTRL`
pub const RTCSRC: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `RTCCTRL`
pub const RTCEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SAMPCTRL`
pub const SAMPVAL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SCAN`
pub const COUNT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SCAN`
pub const OFFSET: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SCAN`
pub const SCANNUM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `STATUS`
pub const IF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `STATUS`
pub const WRCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `VPCTRLA`
pub const VP1MAP: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `VPCTRLA`
pub const VP0MAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VPCTRLB`
pub const VP2MAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VPCTRLB`
pub const VP3MAP: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WINTMODE: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const XOSCSEL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const FRQRANGE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const X32KLPM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const XOSCFDEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const XOSCFDIF: *mut u8 = 0x2 as *mut u8;

/// Hysteresis mode selection
#[allow(non_upper_case_globals)]
pub mod ac_hysmode {
   /// No hysteresis.
   pub const NO: u32 = 0x0;
   /// Small hysteresis.
   pub const SMALL: u32 = 0x1;
   /// Large hysteresis.
   pub const LARGE: u32 = 0x2;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod ac_intlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Interrupt mode
#[allow(non_upper_case_globals)]
pub mod ac_intmode {
   /// Interrupt on both edges.
   pub const BOTHEDGES: u32 = 0x0;
   /// Interrupt on falling edge.
   pub const FALLING: u32 = 0x2;
   /// Interrupt on rising edge.
   pub const RISING: u32 = 0x3;
}

/// Negative input multiplexer selection
#[allow(non_upper_case_globals)]
pub mod ac_muxneg {
   /// Pin 0.
   pub const PIN0: u32 = 0x0;
   /// Pin 1.
   pub const PIN1: u32 = 0x1;
   /// Pin 3.
   pub const PIN3: u32 = 0x2;
   /// Pin 5.
   pub const PIN5: u32 = 0x3;
   /// Pin 7.
   pub const PIN7: u32 = 0x4;
   /// DAC output.
   pub const DAC: u32 = 0x5;
   /// Bandgap Reference.
   pub const BANDGAP: u32 = 0x6;
   /// Internal voltage scaler.
   pub const SCALER: u32 = 0x7;
}

/// Positive input multiplexer selection
#[allow(non_upper_case_globals)]
pub mod ac_muxpos {
   /// Pin 0.
   pub const PIN0: u32 = 0x0;
   /// Pin 1.
   pub const PIN1: u32 = 0x1;
   /// Pin 2.
   pub const PIN2: u32 = 0x2;
   /// Pin 3.
   pub const PIN3: u32 = 0x3;
   /// Pin 4.
   pub const PIN4: u32 = 0x4;
   /// Pin 5.
   pub const PIN5: u32 = 0x5;
   /// Pin 6.
   pub const PIN6: u32 = 0x6;
   /// DAC output.
   pub const DAC: u32 = 0x7;
}

/// Window interrupt level
#[allow(non_upper_case_globals)]
pub mod ac_wintlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low priority.
   pub const LO: u32 = 0x1;
   /// Medium priority.
   pub const MED: u32 = 0x2;
   /// High priority.
   pub const HI: u32 = 0x3;
}

/// Windows interrupt mode
#[allow(non_upper_case_globals)]
pub mod ac_wintmode {
   /// Interrupt on above window.
   pub const ABOVE: u32 = 0x0;
   /// Interrupt on inside window.
   pub const INSIDE: u32 = 0x1;
   /// Interrupt on below window.
   pub const BELOW: u32 = 0x2;
   /// Interrupt on outside window.
   pub const OUTSIDE: u32 = 0x3;
}

/// Window mode state
#[allow(non_upper_case_globals)]
pub mod ac_wstate {
   /// Signal above window.
   pub const ABOVE: u32 = 0x0;
   /// Signal inside window.
   pub const INSIDE: u32 = 0x1;
   /// Signal below window.
   pub const BELOW: u32 = 0x2;
}

/// Gain factor
#[allow(non_upper_case_globals)]
pub mod adc_ch_gain {
   /// 1x gain.
   pub const _1X: u32 = 0x0;
   /// 2x gain.
   pub const _2X: u32 = 0x1;
   /// 4x gain.
   pub const _4X: u32 = 0x2;
   /// 8x gain.
   pub const _8X: u32 = 0x3;
   /// 16x gain.
   pub const _16X: u32 = 0x4;
   /// 32x gain.
   pub const _32X: u32 = 0x5;
   /// 64x gain.
   pub const _64X: u32 = 0x6;
   /// x/2 gain.
   pub const DIV2: u32 = 0x7;
}

/// Input mode
#[allow(non_upper_case_globals)]
pub mod adc_ch_inputmode {
   /// Internal inputs, no gain.
   pub const INTERNAL: u32 = 0x0;
   /// Single-ended input, no gain.
   pub const SINGLEENDED: u32 = 0x1;
   /// Differential input, no gain.
   pub const DIFF: u32 = 0x2;
   /// Differential input, with gain.
   pub const DIFFWGAIN: u32 = 0x3;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod adc_ch_intlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Interupt mode
#[allow(non_upper_case_globals)]
pub mod adc_ch_intmode {
   /// Interrupt on conversion complete.
   pub const COMPLETE: u32 = 0x0;
   /// Interrupt on result below compare value.
   pub const BELOW: u32 = 0x1;
   /// Interrupt on result above compare value.
   pub const ABOVE: u32 = 0x3;
}

/// Internal input multiplexer selections
#[allow(non_upper_case_globals)]
pub mod adc_ch_muxint {
   /// Temperature Reference.
   pub const TEMP: u32 = 0x0;
   /// Bandgap Reference.
   pub const BANDGAP: u32 = 0x1;
   /// 1/10 scaled VCC.
   pub const SCALEDVCC: u32 = 0x2;
   /// DAC output.
   pub const DAC: u32 = 0x3;
}

/// Negative input multiplexer selection
#[allow(non_upper_case_globals)]
pub mod adc_ch_muxneg {
   /// Input pin 0 (Input Mode = 2).
   pub const PIN0: u32 = 0x0;
   /// Input pin 1 (Input Mode = 2).
   pub const PIN1: u32 = 0x1;
   /// Input pin 2 (Input Mode = 2).
   pub const PIN2: u32 = 0x2;
   /// Input pin 3 (Input Mode = 2).
   pub const PIN3: u32 = 0x3;
   /// Input pin 4 (Input Mode = 3).
   pub const PIN4: u32 = 0x0;
   /// Input pin 5 (Input Mode = 3).
   pub const PIN5: u32 = 0x1;
   /// Input pin 6 (Input Mode = 3).
   pub const PIN6: u32 = 0x2;
   /// Input pin 7 (Input Mode = 3).
   pub const PIN7: u32 = 0x3;
   /// PAD Ground (Input Mode = 2).
   pub const GND_MODE3: u32 = 0x5;
   /// Internal Ground (Input Mode = 2).
   pub const INTGND_MODE3: u32 = 0x7;
   /// Internal Ground (Input Mode = 3).
   pub const INTGND_MODE4: u32 = 0x4;
   /// PAD Ground (Input Mode = 3).
   pub const GND_MODE4: u32 = 0x7;
}

/// Positive input multiplexer selection
#[allow(non_upper_case_globals)]
pub mod adc_ch_muxpos {
   /// Input pin 0.
   pub const PIN0: u32 = 0x0;
   /// Input pin 1.
   pub const PIN1: u32 = 0x1;
   /// Input pin 2.
   pub const PIN2: u32 = 0x2;
   /// Input pin 3.
   pub const PIN3: u32 = 0x3;
   /// Input pin 4.
   pub const PIN4: u32 = 0x4;
   /// Input pin 5.
   pub const PIN5: u32 = 0x5;
   /// Input pin 6.
   pub const PIN6: u32 = 0x6;
   /// Input pin 7.
   pub const PIN7: u32 = 0x7;
   /// Input pin 8.
   pub const PIN8: u32 = 0x8;
   /// Input pin 9.
   pub const PIN9: u32 = 0x9;
   /// Input pin 10.
   pub const PIN10: u32 = 0xA;
   /// Input pin 11.
   pub const PIN11: u32 = 0xB;
}

/// Current Limitation Mode
#[allow(non_upper_case_globals)]
pub mod adc_currlimit {
   /// No limit.
   pub const NO: u32 = 0x0;
   /// Low current limit, max. sampling rate 1.5MSPS.
   pub const LOW: u32 = 0x1;
   /// Medium current limit, max. sampling rate 1MSPS.
   pub const MED: u32 = 0x2;
   /// High current limit, max. sampling rate 0.5MSPS.
   pub const HIGH: u32 = 0x3;
}

/// DMA request selection
#[allow(non_upper_case_globals)]
pub mod adc_dmasel {
   /// Combined DMA request OFF.
   pub const OFF: u32 = 0x0;
   /// ADC Channel 0 or 1.
   pub const CH01: u32 = 0x1;
   /// ADC Channel 0 or 1 or 2.
   pub const CH012: u32 = 0x2;
   /// ADC Channel 0 or 1 or 2 or 3.
   pub const CH0123: u32 = 0x3;
}

/// Event action selection
#[allow(non_upper_case_globals)]
pub mod adc_evact {
   /// No event action.
   pub const NONE: u32 = 0x0;
   /// First event triggers channel 0.
   pub const CH0: u32 = 0x1;
   /// First two events trigger channel 0,1.
   pub const CH01: u32 = 0x2;
   /// First three events trigger channel 0,1,2.
   pub const CH012: u32 = 0x3;
   /// Events trigger channel 0,1,2,3.
   pub const CH0123: u32 = 0x4;
   /// First event triggers sweep.
   pub const SWEEP: u32 = 0x5;
   /// The ADC is flushed and restarted for accurate timing.
   pub const SYNCSWEEP: u32 = 0x6;
}

/// Event channel input selection
#[allow(non_upper_case_globals)]
pub mod adc_evsel {
   /// Event Channel 0,1,2,3.
   pub const _0123: u32 = 0x0;
   /// Event Channel 1,2,3,4.
   pub const _1234: u32 = 0x1;
   /// Event Channel 2,3,4,5.
   pub const _2345: u32 = 0x2;
   /// Event Channel 3,4,5,6.
   pub const _3456: u32 = 0x3;
   /// Event Channel 4,5,6,7.
   pub const _4567: u32 = 0x4;
   /// Event Channel 5,6,7.
   pub const _567: u32 = 0x5;
   /// Event Channel 6,7.
   pub const _67: u32 = 0x6;
   /// Event Channel 7.
   pub const _7: u32 = 0x7;
}

/// Clock prescaler
#[allow(non_upper_case_globals)]
pub mod adc_prescaler {
   /// Divide clock by 4.
   pub const DIV4: u32 = 0x0;
   /// Divide clock by 8.
   pub const DIV8: u32 = 0x1;
   /// Divide clock by 16.
   pub const DIV16: u32 = 0x2;
   /// Divide clock by 32.
   pub const DIV32: u32 = 0x3;
   /// Divide clock by 64.
   pub const DIV64: u32 = 0x4;
   /// Divide clock by 128.
   pub const DIV128: u32 = 0x5;
   /// Divide clock by 256.
   pub const DIV256: u32 = 0x6;
   /// Divide clock by 512.
   pub const DIV512: u32 = 0x7;
}

/// Voltage reference selection
#[allow(non_upper_case_globals)]
pub mod adc_refsel {
   /// Internal 1V.
   pub const INT1V: u32 = 0x0;
   /// Internal VCC / 1.6.
   pub const INTVCC: u32 = 0x1;
   /// External reference on PORT A.
   pub const AREFA: u32 = 0x2;
   /// External reference on PORT B.
   pub const AREFB: u32 = 0x3;
   /// Internal VCC / 2.
   pub const INTVCC2: u32 = 0x4;
}

/// Conversion result resolution
#[allow(non_upper_case_globals)]
pub mod adc_resolution {
   /// 12-bit right-adjusted result.
   pub const _12BIT: u32 = 0x0;
   /// 8-bit right-adjusted result.
   pub const _8BIT: u32 = 0x2;
   /// 12-bit left-adjusted result.
   pub const LEFT12BIT: u32 = 0x3;
}

/// Fault Detect Action
#[allow(non_upper_case_globals)]
pub mod awex_fdact {
   /// No Fault Protection.
   pub const NONE: u32 = 0x0;
   /// Clear Output Enable Bits.
   pub const CLEAROE: u32 = 0x1;
   /// Clear I/O Port Direction Bits.
   pub const CLEARDIR: u32 = 0x3;
}

/// BOD operation
#[allow(non_upper_case_globals)]
pub mod bod {
   /// BOD enabled in sampled mode.
   pub const INSAMPLEDMODE: u32 = 0x1;
   /// BOD enabled continuously.
   pub const CONTINOUSLY: u32 = 0x2;
   /// BOD Disabled.
   pub const DISABLED: u32 = 0x3;
}

/// Brownout Detection Voltage Level
#[allow(non_upper_case_globals)]
pub mod bodlvl {
   /// 1.6 V.
   pub const _1V6: u32 = 0x7;
   /// 1.8 V.
   pub const _1V8: u32 = 0x6;
   /// 2.0 V.
   pub const _2V0: u32 = 0x5;
   /// 2.2 V.
   pub const _2V2: u32 = 0x4;
   /// 2.4 V.
   pub const _2V4: u32 = 0x3;
   /// 2.6 V.
   pub const _2V6: u32 = 0x2;
   /// 2.8 V.
   pub const _2V8: u32 = 0x1;
   /// 3.0 V.
   pub const _3V0: u32 = 0x0;
}

/// Boot Loader Section Reset Vector
#[allow(non_upper_case_globals)]
pub mod bootrst {
   /// Boot Loader Reset.
   pub const BOOTLDR: u32 = 0x0;
   /// Application Reset.
   pub const APPLICATION: u32 = 0x1;
}

/// CCP signatures
#[allow(non_upper_case_globals)]
pub mod ccp {
   /// SPM Instruction Protection.
   pub const SPM: u32 = 0x9D;
   /// IO Register Protection.
   pub const IOREG: u32 = 0xD8;
}

/// Prescaler A Division Factor
#[allow(non_upper_case_globals)]
pub mod clk_psadiv {
   /// Divide by 1.
   pub const _1: u32 = 0x0;
   /// Divide by 2.
   pub const _2: u32 = 0x1;
   /// Divide by 4.
   pub const _4: u32 = 0x3;
   /// Divide by 8.
   pub const _8: u32 = 0x5;
   /// Divide by 16.
   pub const _16: u32 = 0x7;
   /// Divide by 32.
   pub const _32: u32 = 0x9;
   /// Divide by 64.
   pub const _64: u32 = 0xB;
   /// Divide by 128.
   pub const _128: u32 = 0xD;
   /// Divide by 256.
   pub const _256: u32 = 0xF;
   /// Divide by 512.
   pub const _512: u32 = 0x11;
}

/// Prescaler B and C Division Factor
#[allow(non_upper_case_globals)]
pub mod clk_psbcdiv {
   /// Divide B by 1 and C by 1.
   pub const _1_1: u32 = 0x0;
   /// Divide B by 1 and C by 2.
   pub const _1_2: u32 = 0x1;
   /// Divide B by 4 and C by 1.
   pub const _4_1: u32 = 0x2;
   /// Divide B by 2 and C by 2.
   pub const _2_2: u32 = 0x3;
}

/// RTC Clock Source
#[allow(non_upper_case_globals)]
pub mod clk_rtcsrc {
   /// 1 kHz from internal 32kHz ULP.
   pub const ULP: u32 = 0x0;
   /// 1.024 kHz from 32.768 kHz crystal oscillator on TOSC.
   pub const TOSC: u32 = 0x1;
   /// 1.024 kHz from 32.768 kHz internal oscillator.
   pub const RCOSC: u32 = 0x2;
   /// 32.768 kHz from 32.768 kHz crystal oscillator on TOSC.
   pub const TOSC32: u32 = 0x5;
   /// 32.768 kHz from 32.768 kHz internal oscillator.
   pub const RCOSC32: u32 = 0x6;
   /// External Clock from TOSC1.
   pub const EXTCLK: u32 = 0x7;
}

/// System Clock Selection
#[allow(non_upper_case_globals)]
pub mod clk_sclksel {
   /// Internal 2 MHz RC Oscillator.
   pub const RC2M: u32 = 0x0;
   /// Internal 32 MHz RC Oscillator.
   pub const RC32M: u32 = 0x1;
   /// Internal 32.768 kHz RC Oscillator.
   pub const RC32K: u32 = 0x2;
   /// External Crystal Oscillator or Clock.
   pub const XOSC: u32 = 0x3;
   /// Phase Locked Loop.
   pub const PLL: u32 = 0x4;
}

/// Reset
#[allow(non_upper_case_globals)]
pub mod crc_reset {
   /// No Reset.
   pub const NO: u32 = 0x0;
   /// Reset CRC with CHECKSUM to all zeros.
   pub const RESET0: u32 = 0x2;
   /// Reset CRC with CHECKSUM to all ones.
   pub const RESET1: u32 = 0x3;
}

/// Input Source
#[allow(non_upper_case_globals)]
pub mod crc_source {
   /// Disabled.
   pub const DISABLE: u32 = 0x0;
   /// I/O Interface.
   pub const IO: u32 = 0x1;
   /// Flash.
   pub const FLASH: u32 = 0x2;
}

/// Event Channel multiplexer input selection
#[allow(non_upper_case_globals)]
pub mod evsys_chmux {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// RTC Overflow.
   pub const RTC_OVF: u32 = 0x8;
   /// RTC Compare Match.
   pub const RTC_CMP: u32 = 0x9;
   /// Analog Comparator A Channel 0.
   pub const ACA_CH0: u32 = 0x10;
   /// Analog Comparator A Channel 1.
   pub const ACA_CH1: u32 = 0x11;
   /// Analog Comparator A Window.
   pub const ACA_WIN: u32 = 0x12;
   /// ADC A Channel 0.
   pub const ADCA_CH0: u32 = 0x20;
   /// Port A, Pin0.
   pub const PORTA_PIN0: u32 = 0x50;
   /// Port A, Pin1.
   pub const PORTA_PIN1: u32 = 0x51;
   /// Port A, Pin2.
   pub const PORTA_PIN2: u32 = 0x52;
   /// Port A, Pin3.
   pub const PORTA_PIN3: u32 = 0x53;
   /// Port A, Pin4.
   pub const PORTA_PIN4: u32 = 0x54;
   /// Port A, Pin5.
   pub const PORTA_PIN5: u32 = 0x55;
   /// Port A, Pin6.
   pub const PORTA_PIN6: u32 = 0x56;
   /// Port A, Pin7.
   pub const PORTA_PIN7: u32 = 0x57;
   /// Port B, Pin0.
   pub const PORTB_PIN0: u32 = 0x58;
   /// Port B, Pin1.
   pub const PORTB_PIN1: u32 = 0x59;
   /// Port B, Pin2.
   pub const PORTB_PIN2: u32 = 0x5A;
   /// Port B, Pin3.
   pub const PORTB_PIN3: u32 = 0x5B;
   /// Port B, Pin4.
   pub const PORTB_PIN4: u32 = 0x5C;
   /// Port B, Pin5.
   pub const PORTB_PIN5: u32 = 0x5D;
   /// Port B, Pin6.
   pub const PORTB_PIN6: u32 = 0x5E;
   /// Port B, Pin7.
   pub const PORTB_PIN7: u32 = 0x5F;
   /// Port C, Pin0.
   pub const PORTC_PIN0: u32 = 0x60;
   /// Port C, Pin1.
   pub const PORTC_PIN1: u32 = 0x61;
   /// Port C, Pin2.
   pub const PORTC_PIN2: u32 = 0x62;
   /// Port C, Pin3.
   pub const PORTC_PIN3: u32 = 0x63;
   /// Port C, Pin4.
   pub const PORTC_PIN4: u32 = 0x64;
   /// Port C, Pin5.
   pub const PORTC_PIN5: u32 = 0x65;
   /// Port C, Pin6.
   pub const PORTC_PIN6: u32 = 0x66;
   /// Port C, Pin7.
   pub const PORTC_PIN7: u32 = 0x67;
   /// Port D, Pin0.
   pub const PORTD_PIN0: u32 = 0x68;
   /// Port D, Pin1.
   pub const PORTD_PIN1: u32 = 0x69;
   /// Port D, Pin2.
   pub const PORTD_PIN2: u32 = 0x6A;
   /// Port D, Pin3.
   pub const PORTD_PIN3: u32 = 0x6B;
   /// Port D, Pin4.
   pub const PORTD_PIN4: u32 = 0x6C;
   /// Port D, Pin5.
   pub const PORTD_PIN5: u32 = 0x6D;
   /// Port D, Pin6.
   pub const PORTD_PIN6: u32 = 0x6E;
   /// Port D, Pin7.
   pub const PORTD_PIN7: u32 = 0x6F;
   /// Port E, Pin0.
   pub const PORTE_PIN0: u32 = 0x70;
   /// Port E, Pin1.
   pub const PORTE_PIN1: u32 = 0x71;
   /// Port E, Pin2.
   pub const PORTE_PIN2: u32 = 0x72;
   /// Port E, Pin3.
   pub const PORTE_PIN3: u32 = 0x73;
   /// Port E, Pin4.
   pub const PORTE_PIN4: u32 = 0x74;
   /// Port E, Pin5.
   pub const PORTE_PIN5: u32 = 0x75;
   /// Port E, Pin6.
   pub const PORTE_PIN6: u32 = 0x76;
   /// Port E, Pin7.
   pub const PORTE_PIN7: u32 = 0x77;
   /// Port F, Pin0.
   pub const PORTF_PIN0: u32 = 0x78;
   /// Port F, Pin1.
   pub const PORTF_PIN1: u32 = 0x79;
   /// Port F, Pin2.
   pub const PORTF_PIN2: u32 = 0x7A;
   /// Port F, Pin3.
   pub const PORTF_PIN3: u32 = 0x7B;
   /// Port F, Pin4.
   pub const PORTF_PIN4: u32 = 0x7C;
   /// Port F, Pin5.
   pub const PORTF_PIN5: u32 = 0x7D;
   /// Port F, Pin6.
   pub const PORTF_PIN6: u32 = 0x7E;
   /// Port F, Pin7.
   pub const PORTF_PIN7: u32 = 0x7F;
   /// Prescaler, divide by 1.
   pub const PRESCALER_1: u32 = 0x80;
   /// Prescaler, divide by 2.
   pub const PRESCALER_2: u32 = 0x81;
   /// Prescaler, divide by 4.
   pub const PRESCALER_4: u32 = 0x82;
   /// Prescaler, divide by 8.
   pub const PRESCALER_8: u32 = 0x83;
   /// Prescaler, divide by 16.
   pub const PRESCALER_16: u32 = 0x84;
   /// Prescaler, divide by 32.
   pub const PRESCALER_32: u32 = 0x85;
   /// Prescaler, divide by 64.
   pub const PRESCALER_64: u32 = 0x86;
   /// Prescaler, divide by 128.
   pub const PRESCALER_128: u32 = 0x87;
   /// Prescaler, divide by 256.
   pub const PRESCALER_256: u32 = 0x88;
   /// Prescaler, divide by 512.
   pub const PRESCALER_512: u32 = 0x89;
   /// Prescaler, divide by 1024.
   pub const PRESCALER_1024: u32 = 0x8A;
   /// Prescaler, divide by 2048.
   pub const PRESCALER_2048: u32 = 0x8B;
   /// Prescaler, divide by 4096.
   pub const PRESCALER_4096: u32 = 0x8C;
   /// Prescaler, divide by 8192.
   pub const PRESCALER_8192: u32 = 0x8D;
   /// Prescaler, divide by 16384.
   pub const PRESCALER_16384: u32 = 0x8E;
   /// Prescaler, divide by 32768.
   pub const PRESCALER_32768: u32 = 0x8F;
   /// Timer/Counter C0 Overflow.
   pub const TCC0_OVF: u32 = 0xC0;
   /// Timer/Counter C0 Error.
   pub const TCC0_ERR: u32 = 0xC1;
   /// Timer/Counter C0 Compare or Capture A.
   pub const TCC0_CCA: u32 = 0xC4;
   /// Timer/Counter C0 Compare or Capture B.
   pub const TCC0_CCB: u32 = 0xC5;
   /// Timer/Counter C0 Compare or Capture C.
   pub const TCC0_CCC: u32 = 0xC6;
   /// Timer/Counter C0 Compare or Capture D.
   pub const TCC0_CCD: u32 = 0xC7;
   /// Timer/Counter C1 Overflow.
   pub const TCC1_OVF: u32 = 0xC8;
   /// Timer/Counter C1 Error.
   pub const TCC1_ERR: u32 = 0xC9;
   /// Timer/Counter C1 Compare or Capture A.
   pub const TCC1_CCA: u32 = 0xCC;
   /// Timer/Counter C1 Compare or Capture B.
   pub const TCC1_CCB: u32 = 0xCD;
   /// Timer/Counter D0 Overflow.
   pub const TCD0_OVF: u32 = 0xD0;
   /// Timer/Counter D0 Error.
   pub const TCD0_ERR: u32 = 0xD1;
   /// Timer/Counter D0 Compare or Capture A.
   pub const TCD0_CCA: u32 = 0xD4;
   /// Timer/Counter D0 Compare or Capture B.
   pub const TCD0_CCB: u32 = 0xD5;
   /// Timer/Counter D0 Compare or Capture C.
   pub const TCD0_CCC: u32 = 0xD6;
   /// Timer/Counter D0 Compare or Capture D.
   pub const TCD0_CCD: u32 = 0xD7;
   /// Timer/Counter E0 Overflow.
   pub const TCE0_OVF: u32 = 0xE0;
   /// Timer/Counter E0 Error.
   pub const TCE0_ERR: u32 = 0xE1;
   /// Timer/Counter E0 Compare or Capture A.
   pub const TCE0_CCA: u32 = 0xE4;
   /// Timer/Counter E0 Compare or Capture B.
   pub const TCE0_CCB: u32 = 0xE5;
   /// Timer/Counter E0 Compare or Capture C.
   pub const TCE0_CCC: u32 = 0xE6;
   /// Timer/Counter E0 Compare or Capture D.
   pub const TCE0_CCD: u32 = 0xE7;
   /// Timer/Counter F0 Overflow.
   pub const TCF0_OVF: u32 = 0xF0;
   /// Timer/Counter F0 Error.
   pub const TCF0_ERR: u32 = 0xF1;
   /// Timer/Counter F0 Compare or Capture A.
   pub const TCF0_CCA: u32 = 0xF4;
   /// Timer/Counter F0 Compare or Capture B.
   pub const TCF0_CCB: u32 = 0xF5;
   /// Timer/Counter F0 Compare or Capture C.
   pub const TCF0_CCC: u32 = 0xF6;
   /// Timer/Counter F0 Compare or Capture D.
   pub const TCF0_CCD: u32 = 0xF7;
}

/// Digital filter coefficient
#[allow(non_upper_case_globals)]
pub mod evsys_digfilt {
   /// 1 SAMPLE.
   pub const _1SAMPLE: u32 = 0x0;
   /// 2 SAMPLES.
   pub const _2SAMPLES: u32 = 0x1;
   /// 3 SAMPLES.
   pub const _3SAMPLES: u32 = 0x2;
   /// 4 SAMPLES.
   pub const _4SAMPLES: u32 = 0x3;
   /// 5 SAMPLES.
   pub const _5SAMPLES: u32 = 0x4;
   /// 6 SAMPLES.
   pub const _6SAMPLES: u32 = 0x5;
   /// 7 SAMPLES.
   pub const _7SAMPLES: u32 = 0x6;
   /// 8 SAMPLES.
   pub const _8SAMPLES: u32 = 0x7;
}

/// Quadrature Decoder Index Recognition Mode
#[allow(non_upper_case_globals)]
pub mod evsys_qdirm {
   /// QDPH0 = 0, QDPH90 = 0.
   pub const _00: u32 = 0x0;
   /// QDPH0 = 0, QDPH90 = 1.
   pub const _01: u32 = 0x1;
   /// QDPH0 = 1, QDPH90 = 0.
   pub const _10: u32 = 0x2;
   /// QDPH0 = 1, QDPH90 = 1.
   pub const _11: u32 = 0x3;
}

/// High Resolution Enable
#[allow(non_upper_case_globals)]
pub mod hires_hren {
   /// No Fault Protection.
   pub const NONE: u32 = 0x0;
   /// Enable High Resolution on Timer/Counter 0.
   pub const TC0: u32 = 0x1;
   /// Enable High Resolution on Timer/Counter 1.
   pub const TC1: u32 = 0x2;
   /// Enable High Resolution both Timer/Counters.
   pub const BOTH: u32 = 0x3;
}

/// Event channel selection
#[allow(non_upper_case_globals)]
pub mod irda_evsel {
   /// No Event Source.
   pub const OFF: u32 = 0x0;
   /// Event Channel 0.
   pub const _0: u32 = 0x8;
   /// Event Channel 1.
   pub const _1: u32 = 0x9;
   /// Event Channel 2.
   pub const _2: u32 = 0xA;
   /// Event Channel 3.
   pub const _3: u32 = 0xB;
   /// Event Channel 4.
   pub const _4: u32 = 0xC;
   /// Event Channel 5.
   pub const _5: u32 = 0xD;
   /// Event Channel 6.
   pub const _6: u32 = 0xE;
   /// Event Channel 7.
   pub const _7: u32 = 0xF;
}

/// Boot lock bits - application section
#[allow(non_upper_case_globals)]
pub mod nvm_blba {
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
}

/// Boot lock bits - application table section
#[allow(non_upper_case_globals)]
pub mod nvm_blbat {
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
}

/// Boot lock bits - boot setcion
#[allow(non_upper_case_globals)]
pub mod nvm_blbb {
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
}

/// NVM Command
#[allow(non_upper_case_globals)]
pub mod nvm_cmd {
   /// Noop/Ordinary LPM.
   pub const NO_OPERATION: u32 = 0x0;
   /// Read calibration row.
   pub const READ_CALIB_ROW: u32 = 0x2;
   /// Read user signature row.
   pub const READ_USER_SIG_ROW: u32 = 0x1;
   /// Read EEPROM.
   pub const READ_EEPROM: u32 = 0x6;
   /// Read fuse byte.
   pub const READ_FUSES: u32 = 0x7;
   /// Write lock bits.
   pub const WRITE_LOCK_BITS: u32 = 0x8;
   /// Erase user signature row.
   pub const ERASE_USER_SIG_ROW: u32 = 0x18;
   /// Write user signature row.
   pub const WRITE_USER_SIG_ROW: u32 = 0x1A;
   /// Erase Application Section.
   pub const ERASE_APP: u32 = 0x20;
   /// Erase Application Section page.
   pub const ERASE_APP_PAGE: u32 = 0x22;
   /// Load Flash page buffer.
   pub const LOAD_FLASH_BUFFER: u32 = 0x23;
   /// Write Application Section page.
   pub const WRITE_APP_PAGE: u32 = 0x24;
   /// Erase-and-write Application Section page.
   pub const ERASE_WRITE_APP_PAGE: u32 = 0x25;
   /// Erase/flush Flash page buffer.
   pub const ERASE_FLASH_BUFFER: u32 = 0x26;
   /// Erase Boot Section page.
   pub const ERASE_BOOT_PAGE: u32 = 0x2A;
   /// Erase Flash page.
   pub const ERASE_FLASH_PAGE: u32 = 0x2B;
   /// Write Boot Section page.
   pub const WRITE_BOOT_PAGE: u32 = 0x2C;
   /// Erase-and-write Boot Section page.
   pub const ERASE_WRITE_BOOT_PAGE: u32 = 0x2D;
   /// Write Flash page.
   pub const WRITE_FLASH_PAGE: u32 = 0x2E;
   /// Erase and Write Flash page.
   pub const ERASE_WRITE_FLASH_PAGE: u32 = 0x2F;
   /// Erase EEPROM.
   pub const ERASE_EEPROM: u32 = 0x30;
   /// Erase EEPROM page.
   pub const ERASE_EEPROM_PAGE: u32 = 0x32;
   /// Load EEPROM page buffer.
   pub const LOAD_EEPROM_BUFFER: u32 = 0x33;
   /// Write EEPROM page.
   pub const WRITE_EEPROM_PAGE: u32 = 0x34;
   /// Erase-and-write EEPROM page.
   pub const ERASE_WRITE_EEPROM_PAGE: u32 = 0x35;
   /// Erase/flush EEPROM page buffer.
   pub const ERASE_EEPROM_BUFFER: u32 = 0x36;
   /// Generate Application section CRC.
   pub const APP_CRC: u32 = 0x38;
   /// Generate Boot Section CRC.
   pub const BOOT_CRC: u32 = 0x39;
   /// Generate Flash Range CRC.
   pub const FLASH_RANGE_CRC: u32 = 0x3A;
}

/// EEPROM ready interrupt level
#[allow(non_upper_case_globals)]
pub mod nvm_eelvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Lock bits
#[allow(non_upper_case_globals)]
pub mod nvm_lb {
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
}

/// SPM ready interrupt level
#[allow(non_upper_case_globals)]
pub mod nvm_spmlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Oscillator Frequency Range
#[allow(non_upper_case_globals)]
pub mod osc_frqrange {
   /// 0.4 - 2 MHz.
   pub const _04TO2: u32 = 0x0;
   /// 2 - 9 MHz.
   pub const _2TO9: u32 = 0x1;
   /// 9 - 12 MHz.
   pub const _9TO12: u32 = 0x2;
   /// 12 - 16 MHz.
   pub const _12TO16: u32 = 0x3;
}

/// PLL Clock Source
#[allow(non_upper_case_globals)]
pub mod osc_pllsrc {
   /// Internal 2MHz RC Oscillator.
   pub const RC2M: u32 = 0x0;
   /// Internal 32MHz RC Oscillator.
   pub const RC32M: u32 = 0x2;
   /// External Oscillator.
   pub const XOSC: u32 = 0x3;
}

/// External Oscillator Selection and Startup Time
#[allow(non_upper_case_globals)]
pub mod osc_xoscsel {
   /// External Clock - 6 CLK.
   pub const EXTCLK: u32 = 0x0;
   /// 32kHz TOSC - 32K CLK.
   pub const _32KHz: u32 = 0x2;
   /// 0.4-16MHz XTAL - 256 CLK.
   pub const XTAL_256CLK: u32 = 0x3;
   /// 0.4-16MHz XTAL - 1K CLK.
   pub const XTAL_1KCLK: u32 = 0x7;
   /// 0.4-16MHz XTAL - 16K CLK.
   pub const XTAL_16KCLK: u32 = 0xB;
}

/// Clock Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_clkout {
   /// Clock Output Disabled.
   pub const OFF: u32 = 0x0;
   /// Clock Output on Port C pin 7.
   pub const PC7: u32 = 0x1;
   /// Clock Output on Port D pin 7.
   pub const PD7: u32 = 0x2;
   /// Clock Output on Port E pin 7.
   pub const PE7: u32 = 0x3;
}

/// Event Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_evout {
   /// Event Output Disabled.
   pub const OFF: u32 = 0x0;
   /// Event Channel 7 Output on Port C pin 7.
   pub const PC7: u32 = 0x1;
   /// Event Channel 7 Output on Port D pin 7.
   pub const PD7: u32 = 0x2;
   /// Event Channel 7 Output on Port E pin 7.
   pub const PE7: u32 = 0x3;
}

/// Virtual Port 0 Mapping
#[allow(non_upper_case_globals)]
pub mod portcfg_vp0map {
   /// Mapped To PORTA.
   pub const PORTA: u32 = 0x0;
   /// Mapped To PORTB.
   pub const PORTB: u32 = 0x1;
   /// Mapped To PORTC.
   pub const PORTC: u32 = 0x2;
   /// Mapped To PORTD.
   pub const PORTD: u32 = 0x3;
   /// Mapped To PORTE.
   pub const PORTE: u32 = 0x4;
   /// Mapped To PORTF.
   pub const PORTF: u32 = 0x5;
   /// Mapped To PORTG.
   pub const PORTG: u32 = 0x6;
   /// Mapped To PORTH.
   pub const PORTH: u32 = 0x7;
   /// Mapped To PORTJ.
   pub const PORTJ: u32 = 0x8;
   /// Mapped To PORTK.
   pub const PORTK: u32 = 0x9;
   /// Mapped To PORTL.
   pub const PORTL: u32 = 0xA;
   /// Mapped To PORTM.
   pub const PORTM: u32 = 0xB;
   /// Mapped To PORTN.
   pub const PORTN: u32 = 0xC;
   /// Mapped To PORTP.
   pub const PORTP: u32 = 0xD;
   /// Mapped To PORTQ.
   pub const PORTQ: u32 = 0xE;
   /// Mapped To PORTR.
   pub const PORTR: u32 = 0xF;
}

/// Virtual Port 1 Mapping
#[allow(non_upper_case_globals)]
pub mod portcfg_vp1map {
   /// Mapped To PORTA.
   pub const PORTA: u32 = 0x0;
   /// Mapped To PORTB.
   pub const PORTB: u32 = 0x1;
   /// Mapped To PORTC.
   pub const PORTC: u32 = 0x2;
   /// Mapped To PORTD.
   pub const PORTD: u32 = 0x3;
   /// Mapped To PORTE.
   pub const PORTE: u32 = 0x4;
   /// Mapped To PORTF.
   pub const PORTF: u32 = 0x5;
   /// Mapped To PORTG.
   pub const PORTG: u32 = 0x6;
   /// Mapped To PORTH.
   pub const PORTH: u32 = 0x7;
   /// Mapped To PORTJ.
   pub const PORTJ: u32 = 0x8;
   /// Mapped To PORTK.
   pub const PORTK: u32 = 0x9;
   /// Mapped To PORTL.
   pub const PORTL: u32 = 0xA;
   /// Mapped To PORTM.
   pub const PORTM: u32 = 0xB;
   /// Mapped To PORTN.
   pub const PORTN: u32 = 0xC;
   /// Mapped To PORTP.
   pub const PORTP: u32 = 0xD;
   /// Mapped To PORTQ.
   pub const PORTQ: u32 = 0xE;
   /// Mapped To PORTR.
   pub const PORTR: u32 = 0xF;
}

/// Virtual Port 2 Mapping
#[allow(non_upper_case_globals)]
pub mod portcfg_vp2map {
   /// Mapped To PORTA.
   pub const PORTA: u32 = 0x0;
   /// Mapped To PORTB.
   pub const PORTB: u32 = 0x1;
   /// Mapped To PORTC.
   pub const PORTC: u32 = 0x2;
   /// Mapped To PORTD.
   pub const PORTD: u32 = 0x3;
   /// Mapped To PORTE.
   pub const PORTE: u32 = 0x4;
   /// Mapped To PORTF.
   pub const PORTF: u32 = 0x5;
   /// Mapped To PORTG.
   pub const PORTG: u32 = 0x6;
   /// Mapped To PORTH.
   pub const PORTH: u32 = 0x7;
   /// Mapped To PORTJ.
   pub const PORTJ: u32 = 0x8;
   /// Mapped To PORTK.
   pub const PORTK: u32 = 0x9;
   /// Mapped To PORTL.
   pub const PORTL: u32 = 0xA;
   /// Mapped To PORTM.
   pub const PORTM: u32 = 0xB;
   /// Mapped To PORTN.
   pub const PORTN: u32 = 0xC;
   /// Mapped To PORTP.
   pub const PORTP: u32 = 0xD;
   /// Mapped To PORTQ.
   pub const PORTQ: u32 = 0xE;
   /// Mapped To PORTR.
   pub const PORTR: u32 = 0xF;
}

/// Virtual Port 3 Mapping
#[allow(non_upper_case_globals)]
pub mod portcfg_vp3map {
   /// Mapped To PORTA.
   pub const PORTA: u32 = 0x0;
   /// Mapped To PORTB.
   pub const PORTB: u32 = 0x1;
   /// Mapped To PORTC.
   pub const PORTC: u32 = 0x2;
   /// Mapped To PORTD.
   pub const PORTD: u32 = 0x3;
   /// Mapped To PORTE.
   pub const PORTE: u32 = 0x4;
   /// Mapped To PORTF.
   pub const PORTF: u32 = 0x5;
   /// Mapped To PORTG.
   pub const PORTG: u32 = 0x6;
   /// Mapped To PORTH.
   pub const PORTH: u32 = 0x7;
   /// Mapped To PORTJ.
   pub const PORTJ: u32 = 0x8;
   /// Mapped To PORTK.
   pub const PORTK: u32 = 0x9;
   /// Mapped To PORTL.
   pub const PORTL: u32 = 0xA;
   /// Mapped To PORTM.
   pub const PORTM: u32 = 0xB;
   /// Mapped To PORTN.
   pub const PORTN: u32 = 0xC;
   /// Mapped To PORTP.
   pub const PORTP: u32 = 0xD;
   /// Mapped To PORTQ.
   pub const PORTQ: u32 = 0xE;
   /// Mapped To PORTR.
   pub const PORTR: u32 = 0xF;
}

/// Port Interrupt 0 Level
#[allow(non_upper_case_globals)]
pub mod port_int0lvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Port Interrupt 1 Level
#[allow(non_upper_case_globals)]
pub mod port_int1lvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Input/Sense Configuration
#[allow(non_upper_case_globals)]
pub mod port_isc {
   /// Sense Both Edges.
   pub const BOTHEDGES: u32 = 0x0;
   /// Sense Rising Edge.
   pub const RISING: u32 = 0x1;
   /// Sense Falling Edge.
   pub const FALLING: u32 = 0x2;
   /// Sense Level (Transparent For Events).
   pub const LEVEL: u32 = 0x3;
   /// Disable Digital Input Buffer.
   pub const INPUT_DISABLE: u32 = 0x7;
}

/// Output/Pull Configuration
#[allow(non_upper_case_globals)]
pub mod port_opc {
   /// Totempole.
   pub const TOTEM: u32 = 0x0;
   /// Totempole w/ Bus keeper on Input and Output.
   pub const BUSKEEPER: u32 = 0x1;
   /// Totempole w/ Pull-down on Input.
   pub const PULLDOWN: u32 = 0x2;
   /// Totempole w/ Pull-up on Input.
   pub const PULLUP: u32 = 0x3;
   /// Wired OR.
   pub const WIREDOR: u32 = 0x4;
   /// Wired AND.
   pub const WIREDAND: u32 = 0x5;
   /// Wired OR w/ Pull-down.
   pub const WIREDORPULL: u32 = 0x6;
   /// Wired AND w/ Pull-up.
   pub const WIREDANDPULL: u32 = 0x7;
}

/// Compare Interrupt level
#[allow(non_upper_case_globals)]
pub mod rtc_compintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Overflow Interrupt level
#[allow(non_upper_case_globals)]
pub mod rtc_ovfintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Prescaler Factor
#[allow(non_upper_case_globals)]
pub mod rtc_prescaler {
   /// RTC Off.
   pub const OFF: u32 = 0x0;
   /// RTC Clock.
   pub const DIV1: u32 = 0x1;
   /// RTC Clock / 2.
   pub const DIV2: u32 = 0x2;
   /// RTC Clock / 8.
   pub const DIV8: u32 = 0x3;
   /// RTC Clock / 16.
   pub const DIV16: u32 = 0x4;
   /// RTC Clock / 64.
   pub const DIV64: u32 = 0x5;
   /// RTC Clock / 256.
   pub const DIV256: u32 = 0x6;
   /// RTC Clock / 1024.
   pub const DIV1024: u32 = 0x7;
}

/// SDA hold time
#[allow(non_upper_case_globals)]
pub mod sda_hold_time {
   /// SDA hold time off.
   pub const OFF: u32 = 0x0;
   /// Typical 50ns hold time.
   pub const _50NS: u32 = 0x1;
   /// Typical 300ns hold time.
   pub const _300NS: u32 = 0x2;
   /// Typical 400ns hold time.
   pub const _400NS: u32 = 0x3;
}

/// Sleep Mode
#[allow(non_upper_case_globals)]
pub mod sleep_smode {
   /// Idle mode.
   pub const IDLE: u32 = 0x0;
   /// Power-down Mode.
   pub const PDOWN: u32 = 0x2;
   /// Power-save Mode.
   pub const PSAVE: u32 = 0x3;
   /// Standby Mode.
   pub const STDBY: u32 = 0x6;
   /// Extended Standby Mode.
   pub const ESTDBY: u32 = 0x7;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod spi_intlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// SPI Mode
#[allow(non_upper_case_globals)]
pub mod spi_mode {
   /// SPI Mode 0.
   pub const _0: u32 = 0x0;
   /// SPI Mode 1.
   pub const _1: u32 = 0x1;
   /// SPI Mode 2.
   pub const _2: u32 = 0x2;
   /// SPI Mode 3.
   pub const _3: u32 = 0x3;
}

/// Prescaler setting
#[allow(non_upper_case_globals)]
pub mod spi_prescaler {
   /// System Clock / 4.
   pub const DIV4: u32 = 0x0;
   /// System Clock / 16.
   pub const DIV16: u32 = 0x1;
   /// System Clock / 64.
   pub const DIV64: u32 = 0x2;
   /// System Clock / 128.
   pub const DIV128: u32 = 0x3;
}

/// Start-up Time
#[allow(non_upper_case_globals)]
pub mod sut {
   /// 0 ms.
   pub const _0MS: u32 = 0x3;
   /// 4 ms.
   pub const _4MS: u32 = 0x1;
   /// 64 ms.
   pub const _64MS: u32 = 0x0;
}

/// Byte Mode
#[allow(non_upper_case_globals)]
pub mod tc2_bytem {
   /// 16-bit mode.
   pub const NORMAL: u32 = 0x0;
   /// Timer/Counter operating in byte mode only (TC2).
   pub const BYTEMODE: u32 = 0x1;
   /// Timer/Counter split into two 8-bit Counters.
   pub const SPLITMODE: u32 = 0x2;
}

/// Clock Selection
#[allow(non_upper_case_globals)]
pub mod tc2_clksel {
   /// Timer Off.
   pub const OFF: u32 = 0x0;
   /// System Clock.
   pub const DIV1: u32 = 0x1;
   /// System Clock / 2.
   pub const DIV2: u32 = 0x2;
   /// System Clock / 4.
   pub const DIV4: u32 = 0x3;
   /// System Clock / 8.
   pub const DIV8: u32 = 0x4;
   /// System Clock / 64.
   pub const DIV64: u32 = 0x5;
   /// System Clock / 256.
   pub const DIV256: u32 = 0x6;
   /// System Clock / 1024.
   pub const DIV1024: u32 = 0x7;
   /// Event Channel 0.
   pub const EVCH0: u32 = 0x8;
   /// Event Channel 1.
   pub const EVCH1: u32 = 0x9;
   /// Event Channel 2.
   pub const EVCH2: u32 = 0xA;
   /// Event Channel 3.
   pub const EVCH3: u32 = 0xB;
}

/// Timer/Counter Command
#[allow(non_upper_case_globals)]
pub mod tc2_cmd {
   /// No Command.
   pub const NONE: u32 = 0x0;
   /// Force Restart.
   pub const RESTART: u32 = 0x2;
   /// Force Hard Reset.
   pub const RESET: u32 = 0x3;
}

/// Timer/Counter Command
#[allow(non_upper_case_globals)]
pub mod tc2_cmden {
   /// Low Byte Timer/Counter.
   pub const LOW: u32 = 0x1;
   /// High Byte Timer/Counter.
   pub const HIGH: u32 = 0x2;
   /// Both Low Byte and High Byte Timer/Counters.
   pub const BOTH: u32 = 0x3;
}

/// High Byte Underflow Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc2_hunfintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Low Byte Compare A Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc2_lcmpaintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Low Byte Compare B Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc2_lcmpbintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Low Byte Compare C Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc2_lcmpcintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Low Byte Compare D Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc2_lcmpdintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Low Byte Underflow Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc2_lunfintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture A Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc_ccaintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture B Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc_ccbintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture C Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc_cccintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture D Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc_ccdintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Clock Selection
#[allow(non_upper_case_globals)]
pub mod tc_clksel {
   /// Timer Off.
   pub const OFF: u32 = 0x0;
   /// System Clock.
   pub const DIV1: u32 = 0x1;
   /// System Clock / 2.
   pub const DIV2: u32 = 0x2;
   /// System Clock / 4.
   pub const DIV4: u32 = 0x3;
   /// System Clock / 8.
   pub const DIV8: u32 = 0x4;
   /// System Clock / 64.
   pub const DIV64: u32 = 0x5;
   /// System Clock / 256.
   pub const DIV256: u32 = 0x6;
   /// System Clock / 1024.
   pub const DIV1024: u32 = 0x7;
   /// Event Channel 0.
   pub const EVCH0: u32 = 0x8;
   /// Event Channel 1.
   pub const EVCH1: u32 = 0x9;
   /// Event Channel 2.
   pub const EVCH2: u32 = 0xA;
   /// Event Channel 3.
   pub const EVCH3: u32 = 0xB;
   /// Event Channel 4.
   pub const EVCH4: u32 = 0xC;
   /// Event Channel 5.
   pub const EVCH5: u32 = 0xD;
   /// Event Channel 6.
   pub const EVCH6: u32 = 0xE;
   /// Event Channel 7.
   pub const EVCH7: u32 = 0xF;
}

/// Timer/Counter Command
#[allow(non_upper_case_globals)]
pub mod tc_cmd {
   /// No Command.
   pub const NONE: u32 = 0x0;
   /// Force Update.
   pub const UPDATE: u32 = 0x1;
   /// Force Restart.
   pub const RESTART: u32 = 0x2;
   /// Force Hard Reset.
   pub const RESET: u32 = 0x3;
}

/// Error Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc_errintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Event Action
#[allow(non_upper_case_globals)]
pub mod tc_evact {
   /// No Event Action.
   pub const OFF: u32 = 0x0;
   /// Input Capture.
   pub const CAPT: u32 = 0x1;
   /// Externally Controlled Up/Down Count.
   pub const UPDOWN: u32 = 0x2;
   /// Quadrature Decode.
   pub const QDEC: u32 = 0x3;
   /// Restart.
   pub const RESTART: u32 = 0x4;
   /// Frequency Capture.
   pub const FRQ: u32 = 0x5;
   /// Pulse-width Capture.
   pub const PW: u32 = 0x6;
}

/// Event Selection
#[allow(non_upper_case_globals)]
pub mod tc_evsel {
   /// No Event Source.
   pub const OFF: u32 = 0x0;
   /// Event Channel 0.
   pub const CH0: u32 = 0x8;
   /// Event Channel 1.
   pub const CH1: u32 = 0x9;
   /// Event Channel 2.
   pub const CH2: u32 = 0xA;
   /// Event Channel 3.
   pub const CH3: u32 = 0xB;
   /// Event Channel 4.
   pub const CH4: u32 = 0xC;
   /// Event Channel 5.
   pub const CH5: u32 = 0xD;
   /// Event Channel 6.
   pub const CH6: u32 = 0xE;
   /// Event Channel 7.
   pub const CH7: u32 = 0xF;
}

/// Overflow Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc_ovfintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Waveform Generation Mode
#[allow(non_upper_case_globals)]
pub mod tc_wgmode {
   /// Normal Mode.
   pub const NORMAL: u32 = 0x0;
   /// Frequency Generation Mode.
   pub const FRQ: u32 = 0x1;
   /// Single Slope.
   pub const SS: u32 = 0x3;
   /// Dual Slope, Update on TOP.
   pub const DS_T: u32 = 0x5;
   /// Dual Slope, Update on TOP and BOTTOM.
   pub const DS_TB: u32 = 0x6;
   /// Dual Slope, Update on BOTTOM.
   pub const DS_B: u32 = 0x7;
}

/// 32.768kHz Timer Oscillator Pin Selection
#[allow(non_upper_case_globals)]
pub mod toscsel {
   /// TOSC1/2 on separate pins.
   pub const ALTERNATE: u32 = 0x0;
   /// TOSC1/2 shared with XTAL.
   pub const XTAL: u32 = 0x1;
}

/// Master Bus State
#[allow(non_upper_case_globals)]
pub mod twi_master_busstate {
   /// Unknown Bus State.
   pub const UNKNOWN: u32 = 0x0;
   /// Bus is Idle.
   pub const IDLE: u32 = 0x1;
   /// This Module Controls The Bus.
   pub const OWNER: u32 = 0x2;
   /// The Bus is Busy.
   pub const BUSY: u32 = 0x3;
}

/// Master Command
#[allow(non_upper_case_globals)]
pub mod twi_master_cmd {
   /// No Action.
   pub const NOACT: u32 = 0x0;
   /// Issue Repeated Start Condition.
   pub const REPSTART: u32 = 0x1;
   /// Receive or Transmit Data.
   pub const RECVTRANS: u32 = 0x2;
   /// Issue Stop Condition.
   pub const STOP: u32 = 0x3;
}

/// Master Interrupt Level
#[allow(non_upper_case_globals)]
pub mod twi_master_intlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Inactive Timeout
#[allow(non_upper_case_globals)]
pub mod twi_master_timeout {
   /// Bus Timeout Disabled.
   pub const DISABLED: u32 = 0x0;
   /// 50 Microseconds.
   pub const _50US: u32 = 0x1;
   /// 100 Microseconds.
   pub const _100US: u32 = 0x2;
   /// 200 Microseconds.
   pub const _200US: u32 = 0x3;
}

/// Slave Command
#[allow(non_upper_case_globals)]
pub mod twi_slave_cmd {
   /// No Action.
   pub const NOACT: u32 = 0x0;
   /// Used To Complete a Transaction.
   pub const COMPTRANS: u32 = 0x2;
   /// Used in Response to Address/Data Interrupt.
   pub const RESPONSE: u32 = 0x3;
}

/// Slave Interrupt Level
#[allow(non_upper_case_globals)]
pub mod twi_slave_intlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Character Size
#[allow(non_upper_case_globals)]
pub mod usart_chsize {
   /// Character size: 5 bit.
   pub const _5BIT: u32 = 0x0;
   /// Character size: 6 bit.
   pub const _6BIT: u32 = 0x1;
   /// Character size: 7 bit.
   pub const _7BIT: u32 = 0x2;
   /// Character size: 8 bit.
   pub const _8BIT: u32 = 0x3;
   /// Character size: 9 bit.
   pub const _9BIT: u32 = 0x7;
}

/// Communication Mode
#[allow(non_upper_case_globals)]
pub mod usart_cmode {
   /// Asynchronous Mode.
   pub const ASYNCHRONOUS: u32 = 0x0;
   /// Synchronous Mode.
   pub const SYNCHRONOUS: u32 = 0x1;
   /// IrDA Mode.
   pub const IRDA: u32 = 0x2;
   /// Master SPI Mode.
   pub const MSPI: u32 = 0x3;
}

/// Data Register Empty Interrupt level
#[allow(non_upper_case_globals)]
pub mod usart_dreintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Parity Mode
#[allow(non_upper_case_globals)]
pub mod usart_pmode {
   /// No Parity.
   pub const DISABLED: u32 = 0x0;
   /// Even Parity.
   pub const EVEN: u32 = 0x2;
   /// Odd Parity.
   pub const ODD: u32 = 0x3;
}

/// Receive Complete Interrupt level
#[allow(non_upper_case_globals)]
pub mod usart_rxcintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Transmit Complete Interrupt level
#[allow(non_upper_case_globals)]
pub mod usart_txcintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Watchdog (Window) Timeout Period
#[allow(non_upper_case_globals)]
pub mod wd {
   /// 8 cycles (8ms @ 3.3V).
   pub const _8CLK: u32 = 0x0;
   /// 16 cycles (16ms @ 3.3V).
   pub const _16CLK: u32 = 0x1;
   /// 32 cycles (32ms @ 3.3V).
   pub const _32CLK: u32 = 0x2;
   /// 64 cycles (64ms @ 3.3V).
   pub const _64CLK: u32 = 0x3;
   /// 128 cycles (0.125s @ 3.3V).
   pub const _128CLK: u32 = 0x4;
   /// 256 cycles (0.25s @ 3.3V).
   pub const _256CLK: u32 = 0x5;
   /// 512 cycles (0.5s @ 3.3V).
   pub const _512CLK: u32 = 0x6;
   /// 1K cycles (1s @ 3.3V).
   pub const _1KCLK: u32 = 0x7;
   /// 2K cycles (2s @ 3.3V).
   pub const _2KCLK: u32 = 0x8;
   /// 4K cycles (4s @ 3.3V).
   pub const _4KCLK: u32 = 0x9;
   /// 8K cycles (8s @ 3.3V).
   pub const _8KCLK: u32 = 0xA;
}

/// Period setting
#[allow(non_upper_case_globals)]
pub mod wdt_per {
   /// 8 cycles (8ms @ 3.3V).
   pub const _8CLK: u32 = 0x0;
   /// 16 cycles (16ms @ 3.3V).
   pub const _16CLK: u32 = 0x1;
   /// 32 cycles (32ms @ 3.3V).
   pub const _32CLK: u32 = 0x2;
   /// 64 cycles (64ms @ 3.3V).
   pub const _64CLK: u32 = 0x3;
   /// 128 cycles (0.128s @ 3.3V).
   pub const _128CLK: u32 = 0x4;
   /// 256 cycles (0.256s @ 3.3V).
   pub const _256CLK: u32 = 0x5;
   /// 512 cycles (0.512s @ 3.3V).
   pub const _512CLK: u32 = 0x6;
   /// 1K cycles (1s @ 3.3V).
   pub const _1KCLK: u32 = 0x7;
   /// 2K cycles (2s @ 3.3V).
   pub const _2KCLK: u32 = 0x8;
   /// 4K cycles (4s @ 3.3V).
   pub const _4KCLK: u32 = 0x9;
   /// 8K cycles (8s @ 3.3V).
   pub const _8KCLK: u32 = 0xA;
}

/// Closed window period
#[allow(non_upper_case_globals)]
pub mod wdt_wper {
   /// 8 cycles (8ms @ 3.3V).
   pub const _8CLK: u32 = 0x0;
   /// 16 cycles (16ms @ 3.3V).
   pub const _16CLK: u32 = 0x1;
   /// 32 cycles (32ms @ 3.3V).
   pub const _32CLK: u32 = 0x2;
   /// 64 cycles (64ms @ 3.3V).
   pub const _64CLK: u32 = 0x3;
   /// 128 cycles (0.128s @ 3.3V).
   pub const _128CLK: u32 = 0x4;
   /// 256 cycles (0.256s @ 3.3V).
   pub const _256CLK: u32 = 0x5;
   /// 512 cycles (0.512s @ 3.3V).
   pub const _512CLK: u32 = 0x6;
   /// 1K cycles (1s @ 3.3V).
   pub const _1KCLK: u32 = 0x7;
   /// 2K cycles (2s @ 3.3V).
   pub const _2KCLK: u32 = 0x8;
   /// 4K cycles (4s @ 3.3V).
   pub const _4KCLK: u32 = 0x9;
   /// 8K cycles (8s @ 3.3V).
   pub const _8KCLK: u32 = 0xA;
}

