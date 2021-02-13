//! The AVR ATxmega32E5 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATxmega32E5-AU | QFP-QFN-32 | TQFP32 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega32E5-MU | QFP-QFN-32 | VQFN32 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega32E5-M4U | QFP-QFN-32 | UQFN32 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega32E5-AN | QFP-QFN-32 | TQFP32 | -40°C - 105°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega32E5-MN | QFP-QFN-32 | VQFN32 | -40°C - 105°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega32E5-M4N | QFP-QFN-32 | UQFN32 | -40°C - 105°C | 1.6V - 3.6V | 32 MHz |
//!

#![allow(non_upper_case_globals)]

/// Lock Bits.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLBA | 110000 |
/// | LB | 11 |
/// | BLBB | 11000000 |
/// | BLBAT | 1100 |
pub const LOCKBITS: *mut u8 = 0x0 as *mut u8;

/// RCOSC 8MHz Calibration Value.
pub const RCOSC8M: *mut u8 = 0x0 as *mut u8;

/// I/O Port Data Direction.
pub const DIR: *mut u8 = 0x0 as *mut u8;

/// Control Register.
pub const CTRL: *mut u8 = 0x0 as *mut u8;

/// Timeout Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TTOUTSIF | 10000 |
/// | TMEXTIF | 100 |
/// | TSEXTIF | 10 |
/// | TTOUTMIF | 1 |
pub const TOS: *mut u8 = 0x0 as *mut u8;

/// Multi-pin Configuration Mask.
pub const MPCMASK: *mut u8 = 0x0 as *mut u8;

/// Analog Comparator 0 Control.
pub const AC0CTRL: *mut u8 = 0x0 as *mut u8;

/// Address Register 0.
pub const ADDR0: *mut u8 = 0x0 as *mut u8;

/// OCD Register 0.
pub const OCDR0: *mut u8 = 0x0 as *mut u8;

/// General Power Reduction.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EDMA | 1 |
/// | XCL | 10000000 |
/// | RTC | 100 |
/// | EVSYS | 10 |
pub const PRGEN: *mut u8 = 0x0 as *mut u8;

/// Event Channel 0 Multiplexer.
pub const CH0MUX: *mut u8 = 0x0 as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x0 as *mut u8;

/// Device ID byte 0.
pub const DEVID0: *mut u8 = 0x0 as *mut u8;

/// Analog Comparator 1 Control.
pub const AC1CTRL: *mut u8 = 0x1 as *mut u8;

/// Event Channel 1 Multiplexer.
pub const CH1MUX: *mut u8 = 0x1 as *mut u8;

/// I/O Port Data Direction Set.
pub const DIRSET: *mut u8 = 0x1 as *mut u8;

/// Prescaler Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSBCDIV | 11 |
/// | PSADIV | 1111100 |
pub const PSCTRL: *mut u8 = 0x1 as *mut u8;

/// Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSIE | 10000 |
/// | TXCIE | 1000000 |
/// | RXCIE | 10000000 |
/// | DREIE | 100000 |
pub const INTCTRL: *mut u8 = 0x1 as *mut u8;

/// Timeout Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TTOUTMSEL | 111 |
/// | TTOUTSSEL | 11100000 |
/// | TMSEXTSEL | 11000 |
pub const TOCONF: *mut u8 = 0x1 as *mut u8;

/// Address Register 1.
pub const ADDR1: *mut u8 = 0x1 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x1 as *mut u8;

/// Interrupt Priority.
pub const INTPRI: *mut u8 = 0x1 as *mut u8;

/// Power Reduction Port A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC | 10 |
/// | AC | 1 |
/// | DAC | 100 |
pub const PRPA: *mut u8 = 0x1 as *mut u8;

/// Dead-time Concurrent Write to Both Sides Register.
pub const DTBOTH: *mut u8 = 0x1 as *mut u8;

/// OCD Register 1.
pub const OCDR1: *mut u8 = 0x1 as *mut u8;

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

/// Watchdog Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDWP | 11110000 |
/// | WDP | 1111 |
pub const FUSEBYTE1: *mut u8 = 0x1 as *mut u8;

/// IrDA Transmitter Pulse Length Control Register.
pub const TXPLCTRL: *mut u8 = 0x1 as *mut u8;

/// IrDA Receiver Pulse Length Control Register.
pub const RXPLCTRL: *mut u8 = 0x2 as *mut u8;

/// RCOSC 32.768 kHz Calibration Value.
pub const RCOSC32K: *mut u8 = 0x2 as *mut u8;

/// Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DREINTLVL | 11 |
/// | RXSIE | 10000000 |
/// | DRIE | 1000000 |
/// | TXCINTLVL | 1100 |
/// | RXCINTLVL | 110000 |
pub const CTRLA: *mut u8 = 0x2 as *mut u8;

/// Reset Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODPD | 11 |
/// | BOOTRST | 1000000 |
pub const FUSEBYTE2: *mut u8 = 0x2 as *mut u8;

/// External Oscillator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | X32KLPM | 100000 |
/// | FRQRANGE | 11000000 |
/// | XOSCPWR | 10000 |
/// | XOSCSEL | 11111 |
pub const XOSCCTRL: *mut u8 = 0x2 as *mut u8;

/// Calibration Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CALL | 1111111 |
pub const CALA: *mut u8 = 0x2 as *mut u8;

/// Device ID byte 2.
pub const DEVID2: *mut u8 = 0x2 as *mut u8;

/// Event Channel 2 Multiplexer.
pub const CH2MUX: *mut u8 = 0x2 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x2 as *mut u8;

/// Dead-time Low Side Register.
pub const DTLS: *mut u8 = 0x2 as *mut u8;

/// Analog Comparator 0 MUX Control.
pub const AC0MUXCTRL: *mut u8 = 0x2 as *mut u8;

/// Reference Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFSEL | 1110000 |
/// | TEMPREF | 1 |
/// | BANDGAP | 10 |
pub const REFCTRL: *mut u8 = 0x2 as *mut u8;

/// I/O Port Data Direction Clear.
pub const DIRCLR: *mut u8 = 0x2 as *mut u8;

/// Memory Address Control for Peripheral Ch., or Source Address Control for Standard Ch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RELOAD | 110000 |
pub const ADDRCTRL: *mut u8 = 0x2 as *mut u8;

/// Lock register.
pub const LOCK: *mut u8 = 0x2 as *mut u8;

/// Address Register 2.
pub const ADDR2: *mut u8 = 0x2 as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IF | 10000000 |
/// | WRCOL | 1000000 |
/// | BUFOVF | 1 |
/// | RXCIF | 10000000 |
/// | DREIF | 100000 |
/// | TXCIF | 1000000 |
/// | SSIF | 10000 |
pub const STATUS: *mut u8 = 0x2 as *mut u8;

/// General Purpose IO Register 3.
pub const GPIOR3: *mut u8 = 0x3 as *mut u8;

/// Power Reduction Port C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI | 1000 |
/// | TWI | 1000000 |
/// | TC4 | 1 |
/// | HIRES | 100 |
pub const PRPC: *mut u8 = 0x3 as *mut u8;

/// Data Input.
pub const DATAIN: *mut u8 = 0x3 as *mut u8;

/// Event Channel 3 Multiplexer.
pub const CH3MUX: *mut u8 = 0x3 as *mut u8;

/// Address Register.
pub const ADDR: *mut u8 = 0x3 as *mut u8;

/// Revision ID.
pub const REVID: *mut u8 = 0x3 as *mut u8;

/// RTC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTCEN | 1 |
/// | RTCSRC | 1110 |
pub const RTCCTRL: *mut u8 = 0x3 as *mut u8;

/// Data Register.
pub const DATA: *mut u8 = 0x3 as *mut u8;

/// Dead-time High Side Register.
pub const DTHS: *mut u8 = 0x3 as *mut u8;

/// Destination Address Control for Standard Channels Only.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DESTDIR | 111 |
/// | DESTRELOAD | 110000 |
pub const DESTADDRCTRL: *mut u8 = 0x3 as *mut u8;

/// Oscillator Failure Detection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLFDEN | 100 |
/// | XOSCFDIF | 10 |
/// | PLLFDIF | 1000 |
/// | XOSCFDEN | 1 |
pub const XOSCFAIL: *mut u8 = 0x3 as *mut u8;

/// Analog Comparator 1 MUX Control.
pub const AC1MUXCTRL: *mut u8 = 0x3 as *mut u8;

/// Calibration Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CALH | 111111 |
pub const CALB: *mut u8 = 0x3 as *mut u8;

/// I/O Port Data Direction Toggle.
pub const DIRTGL: *mut u8 = 0x3 as *mut u8;

/// Event Input Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVSPLIT | 1000 |
pub const EVCTRL: *mut u8 = 0x3 as *mut u8;

/// RCOSC 32 MHz Calibration Value B.
pub const RCOSC32M: *mut u8 = 0x3 as *mut u8;

/// Clock Out Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKEVPIN | 10000000 |
/// | CLKOUTSEL | 1100 |
/// | RTCOUT | 1100000 |
pub const CLKOUT: *mut u8 = 0x4 as *mut u8;

/// Control E Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLANKB | 10 |
/// | CAPTB | 100000 |
/// | FILTERB | 100 |
/// | QUALB | 1 |
pub const CTRLE: *mut u8 = 0x4 as *mut u8;

/// Start-up Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDLOCK | 10 |
/// | SUT | 1100 |
/// | RSTDISBL | 10000 |
pub const FUSEBYTE4: *mut u8 = 0x4 as *mut u8;

/// Channel Trigger Source.
pub const TRIGSRC: *mut u8 = 0x4 as *mut u8;

/// Channel Result low byte.
pub const RESL: *mut u8 = 0x4 as *mut u8;

/// Checksum byte 0.
pub const CHECKSUM0: *mut u8 = 0x4 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x4 as *mut u8;

/// Power Reduction Port D.
pub const PRPD: *mut u8 = 0x4 as *mut u8;

/// I/O Port Output.
pub const OUT: *mut u8 = 0x4 as *mut u8;

/// Baurd Rate Control Register.
pub const BAUD: *mut u8 = 0x4 as *mut u8;

/// Status Clear Register.
pub const STATUSCLR: *mut u8 = 0x4 as *mut u8;

/// Clock Prescaler.
pub const PRESCALER: *mut u8 = 0x4 as *mut u8;

/// Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CHSIZE | 111 |
/// | SBMODE | 1000 |
/// | PMODE | 110000 |
/// | CMODE | 11000000 |
pub const CTRLC: *mut u8 = 0x4 as *mut u8;

/// RCOSC 32 MHz Calibration Value A.
pub const RCOSC32MA: *mut u8 = 0x4 as *mut u8;

/// 32.768 kHz Internal Oscillator Calibration Register.
pub const RC32KCAL: *mut u8 = 0x4 as *mut u8;

/// Event Channel 4 Multiplexer.
pub const CH4MUX: *mut u8 = 0x4 as *mut u8;

/// Data Register 0.
pub const DATA0: *mut u8 = 0x4 as *mut u8;

/// Oscillator Compare Register 0.
pub const COMP0: *mut u8 = 0x4 as *mut u8;

/// Channel Result.
pub const RES: *mut u16 = 0x4 as *mut u16;

/// Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSD | 100 |
/// | BUFMODE | 11000000 |
pub const CTRLB: *mut u8 = 0x4 as *mut u8;

/// EESAVE and BOD Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | BODACT | 110000 |
/// | BODLVL | 111 |
pub const FUSEBYTE5: *mut u8 = 0x5 as *mut u8;

/// Checksum byte 1.
pub const CHECKSUM1: *mut u8 = 0x5 as *mut u8;

/// Control Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DECTYPE | 110000 |
/// | LUTACT | 1100 |
/// | PECACT | 11 |
pub const CTRLD: *mut u8 = 0x5 as *mut u8;

/// Oscillator Compare Register 1.
pub const COMP1: *mut u8 = 0x5 as *mut u8;

/// I/O Port Output Set.
pub const OUTSET: *mut u8 = 0x5 as *mut u8;

/// PLL Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLFAC | 11111 |
/// | PLLSRC | 11000000 |
/// | PLLDIV | 100000 |
pub const PLLCTRL: *mut u8 = 0x5 as *mut u8;

/// Data Register 1.
pub const DATA1: *mut u8 = 0x5 as *mut u8;

/// Channel Result high byte.
pub const RESH: *mut u8 = 0x5 as *mut u8;

/// Control Register F.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HCCAMODE | 11 |
/// | HCCBMODE | 1100 |
pub const CTRLF: *mut u8 = 0x5 as *mut u8;

/// Status Set Register.
pub const STATUSSET: *mut u8 = 0x5 as *mut u8;

/// Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADDREN | 1 |
pub const ADDRMASK: *mut u8 = 0x5 as *mut u8;

/// Event Channel 5 Multiplexer.
pub const CH5MUX: *mut u8 = 0x5 as *mut u8;

/// Swap Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SWAP3 | 1000 |
/// | SWAP2 | 100 |
/// | SWAP0 | 1 |
/// | SWAP1 | 10 |
pub const SWAP: *mut u8 = 0x6 as *mut u8;

/// Checksum byte 2.
pub const CHECKSUM2: *mut u8 = 0x6 as *mut u8;

/// Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TRGINTLVL | 110000 |
/// | ERRINTLVL | 1100 |
/// | OVFINTLVL | 11 |
pub const INTCTRLA: *mut u8 = 0x6 as *mut u8;

/// Analog Comparator and Event Out Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVOUTSEL | 111 |
/// | EVOUT | 110000 |
/// | EVASYEN | 1000 |
/// | ACOUT | 11000000 |
pub const ACEVOUT: *mut u8 = 0x6 as *mut u8;

/// Input Channel Scan.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INPUTSCAN | 1111 |
/// | INPUTOFFSET | 11110000 |
pub const SCAN: *mut u8 = 0x6 as *mut u8;

/// Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ERROR | 1111111 |
/// | SIGN | 10000000 |
pub const CALIB: *mut u8 = 0x6 as *mut u8;

/// Data Register 2.
pub const DATA2: *mut u8 = 0x6 as *mut u8;

/// Fault State.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FDACT4 | 1000000 |
/// | FDACT5 | 10000000 |
/// | VALUE | 111111 |
pub const FUSEBYTE6: *mut u8 = 0x6 as *mut u8;

/// Oscillator Compare Register 2.
pub const COMP2: *mut u8 = 0x6 as *mut u8;

/// Control Register G.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVACTEN | 10000000 |
/// | EVACT1 | 1100000 |
/// | EVACT0 | 11000 |
/// | EVSRC | 111 |
pub const CTRLG: *mut u8 = 0x6 as *mut u8;

/// Baud Rate Control Register A.
pub const BAUDCTRLA: *mut u8 = 0x6 as *mut u8;

/// Control Register G Clear.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HALTACLR | 1000000 |
/// | STATEECLR | 100000 |
/// | FAULTB | 100 |
/// | HALTBCLR | 10000000 |
/// | FAULTA | 10 |
/// | FAULTE | 1 |
pub const CTRLGCLR: *mut u8 = 0x6 as *mut u8;

/// Channel Block Transfer Count for Peripheral Ch., or Channel Block Transfer Count Low for Standard Ch. low byte.
pub const TRFCNTL: *mut u8 = 0x6 as *mut u8;

/// Window Mode Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WINTLVL | 11 |
/// | WINTMODE | 1100 |
/// | WEN | 10000 |
pub const WINCTRL: *mut u8 = 0x6 as *mut u8;

/// Channel Block Transfer Count for Peripheral Ch., or Channel Block Transfer Count Low for Standard Ch.
pub const TRFCNT: *mut u16 = 0x6 as *mut u16;

/// DFLL Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RC32MCREF | 110 |
pub const DFLLCTRL: *mut u8 = 0x6 as *mut u8;

/// I/O Port Output Clear.
pub const OUTCLR: *mut u8 = 0x6 as *mut u8;

/// Event Channel 6 Multiplexer.
pub const CH6MUX: *mut u8 = 0x6 as *mut u8;

/// Analog Startup Delay.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STARTUPDLYA | 11 |
pub const ANAINIT: *mut u8 = 0x7 as *mut u8;

/// Control Register G set.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FAULTESW | 100000 |
/// | IDXCMD | 11000 |
/// | FAULTASW | 1000000 |
/// | FAULTBSW | 10000000 |
pub const CTRLGSET: *mut u8 = 0x7 as *mut u8;

/// Channel Block Transfer Count for Peripheral Ch., or Channel Block Transfer Count Low for Standard Ch. high byte.
pub const TRFCNTH: *mut u8 = 0x7 as *mut u8;

/// Slew Rate Limit Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRLENRD | 1000 |
/// | SRLENRA | 1 |
/// | SRLENRC | 100 |
/// | SRLENRR | 10000000 |
pub const SRLCTRL: *mut u8 = 0x7 as *mut u8;

/// Baud Rate Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BSCALE | 11110000 |
pub const BAUDCTRLB: *mut u8 = 0x7 as *mut u8;

/// Correction Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CORREN | 1 |
pub const CORRCTRL: *mut u8 = 0x7 as *mut u8;

/// I/O Port Output Toggle.
pub const OUTTGL: *mut u8 = 0x7 as *mut u8;

/// Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCBINTLVL | 1100 |
/// | LCCAINTLVL | 11 |
/// | LCCBINTLVL | 1100 |
/// | CCAINTLVL | 11 |
pub const INTCTRLB: *mut u8 = 0x7 as *mut u8;

/// Event Channel 7 Multiplexer.
pub const CH7MUX: *mut u8 = 0x7 as *mut u8;

/// Internal 8 MHz RC Oscillator Calibration Register.
pub const RC8MCAL: *mut u8 = 0x7 as *mut u8;

/// Checksum byte 3.
pub const CHECKSUM3: *mut u8 = 0x7 as *mut u8;

/// Pattern Generation Override Register.
pub const PGO: *mut u8 = 0x7 as *mut u8;

/// Offset Correction Register 0.
pub const OFFSETCORR0: *mut u8 = 0x8 as *mut u8;

/// Lot Number Byte 0, ASCII.
pub const LOTNUM0: *mut u8 = 0x8 as *mut u8;

/// Channel 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | QDEN | 1000 |
/// | ROTARY | 10000000 |
/// | QDIEN | 10000 |
/// | QDIRM | 1100000 |
pub const CH0CTRL: *mut u8 = 0x8 as *mut u8;

/// ADC Sampling Time Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAMPVAL | 111111 |
pub const SAMPCTRL: *mut u8 = 0x8 as *mut u8;

/// I/O port Input.
pub const IN: *mut u8 = 0x8 as *mut u8;

/// Event System Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVSYS0LOCK | 1 |
/// | EVSYS1LOCK | 10000 |
pub const EVSYSLOCK: *mut u8 = 0x8 as *mut u8;

/// Pattern Generation Value Register.
pub const PGV: *mut u8 = 0x8 as *mut u8;

/// Ramp D.
pub const RAMPD: *mut u8 = 0x8 as *mut u8;

/// Gain Calibration.
pub const CH0GAINCAL: *mut u8 = 0x8 as *mut u8;

/// Current Source Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CURRMODE | 1000000 |
/// | AC0CURR | 1 |
/// | AC1CURR | 10 |
/// | CURREN | 10000000 |
pub const CURRCTRL: *mut u8 = 0x8 as *mut u8;

/// Offset Calibration.
pub const CH0OFFSETCAL: *mut u8 = 0x9 as *mut u8;

/// Lot Number Byte 1, ASCII.
pub const LOTNUM1: *mut u8 = 0x9 as *mut u8;

/// WEX Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WEXCLOCK | 1 |
pub const WEXLOCK: *mut u8 = 0x9 as *mut u8;

/// Current Source Calibration Register.
pub const CURRCALIB: *mut u8 = 0x9 as *mut u8;

/// Offset Correction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OFFSETCORR | 1111 |
pub const OFFSETCORR1: *mut u8 = 0x9 as *mut u8;

/// Ramp X.
pub const RAMPX: *mut u8 = 0x9 as *mut u8;

/// Peripheral Lenght Control Register.
pub const PLC: *mut u8 = 0x9 as *mut u8;

/// Channel 1 Control Register.
pub const CH1CTRL: *mut u8 = 0x9 as *mut u8;

/// Command.
pub const CMD: *mut u8 = 0xA as *mut u8;

/// Lot Number Byte 2, ASCII.
pub const LOTNUM2: *mut u8 = 0xA as *mut u8;

/// Gain Correction Register 0.
pub const GAINCORR0: *mut u8 = 0xA as *mut u8;

/// FAULT Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FAULTC4LOCK | 1 |
/// | FAULTC5LOCK | 10 |
pub const FAULTLOCK: *mut u8 = 0xA as *mut u8;

/// Channel 2 Control Register.
pub const CH2CTRL: *mut u8 = 0xA as *mut u8;

/// Control Register H Clear.
pub const CTRLHCLR: *mut u8 = 0xA as *mut u8;

/// Gain Calibration.
pub const CH1GAINCAL: *mut u8 = 0xA as *mut u8;

/// Port Interrupt Mask.
pub const INTMASK: *mut u8 = 0xA as *mut u8;

/// Dead Time Low Side Buffer.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SWAP0BUF | 1 |
/// | SWAP2BUF | 100 |
/// | SWAP3BUF | 1000 |
/// | SWAP1BUF | 10 |
pub const SWAPBUF: *mut u8 = 0xA as *mut u8;

/// Ramp Y.
pub const RAMPY: *mut u8 = 0xA as *mut u8;

/// Counter Register Low.
pub const CNTL: *mut u8 = 0xA as *mut u8;

/// Ramp Z.
pub const RAMPZ: *mut u8 = 0xB as *mut u8;

/// Offset Calibration.
pub const CH1OFFSETCAL: *mut u8 = 0xB as *mut u8;

/// Counter Register High.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCNT20 | 1111 |
/// | PCNT21 | 11110000 |
pub const CNTH: *mut u8 = 0xB as *mut u8;

/// Channel 3 Control Register.
pub const CH3CTRL: *mut u8 = 0xB as *mut u8;

/// Lot Number Byte 3, ASCII.
pub const LOTNUM3: *mut u8 = 0xB as *mut u8;

/// Pattern Generation Overwrite Buffer Register.
pub const PGOBUF: *mut u8 = 0xB as *mut u8;

/// Gain Correction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GAINCORR | 1111 |
pub const GAINCORR1: *mut u8 = 0xB as *mut u8;

/// Control Register H Set.
pub const CTRLHSET: *mut u8 = 0xB as *mut u8;

/// Channel Destination Address for Standard Channels Only. low byte.
pub const DESTADDRL: *mut u8 = 0xC as *mut u8;

/// Channel Destination Address for Standard Channels Only.
pub const DESTADDR: *mut u16 = 0xC as *mut u16;

/// Lot Number Byte 4, ASCII.
pub const LOTNUM4: *mut u8 = 0xC as *mut u8;

/// Extended Indirect Jump.
pub const EIND: *mut u8 = 0xC as *mut u8;

/// Channel 4 Control Register.
pub const CH4CTRL: *mut u8 = 0xC as *mut u8;

/// Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TRGIF | 100 |
/// | OVFIF | 1 |
/// | CCBIF | 100000 |
/// | LCCBIF | 100000 |
/// | ERRIF | 10 |
/// | LCCAIF | 10000 |
/// | CCAIF | 10000 |
pub const INTFLAGS: *mut u8 = 0xC as *mut u8;

/// Pattern Generation Value Buffer Register.
pub const PGVBUF: *mut u8 = 0xC as *mut u8;

/// Compare Register low byte.
pub const COMPL: *mut u8 = 0xC as *mut u8;

/// Average Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RIGHTSHIFT | 1110000 |
/// | SAMPNUM | 1111 |
pub const AVGCTRL: *mut u8 = 0xC as *mut u8;

/// Compare Register Low.
pub const CMPL: *mut u8 = 0xC as *mut u8;

/// Compare Register.
pub const COMP: *mut u16 = 0xC as *mut u16;

/// Calibration Value.
pub const CAL: *mut u8 = 0xC as *mut u8;

/// Compare Register high byte.
pub const COMPH: *mut u8 = 0xD as *mut u8;

/// Stack Pointer Low.
pub const SPL: *mut u8 = 0xD as *mut u8;

/// Channel Destination Address for Standard Channels Only. high byte.
pub const DESTADDRH: *mut u8 = 0xD as *mut u8;

/// Lot Number Byte 5, ASCII.
pub const LOTNUM5: *mut u8 = 0xD as *mut u8;

/// Compare Register High.
pub const CMPH: *mut u8 = 0xD as *mut u8;

/// Channel 5 Control Register.
pub const CH5CTRL: *mut u8 = 0xD as *mut u8;

/// Stack Pointer High.
pub const SPH: *mut u8 = 0xE as *mut u8;

/// Period or Capture Register Low.
pub const PERCAPTL: *mut u8 = 0xE as *mut u8;

/// Channel 6 Control Register.
pub const CH6CTRL: *mut u8 = 0xE as *mut u8;

/// Pin Remap Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TC4D | 1000 |
/// | TC4C | 100 |
/// | TC4B | 10 |
/// | TC4A | 1 |
pub const REMAP: *mut u8 = 0xE as *mut u8;

/// Temporary Register For 16-bit Access.
pub const TEMP: *mut u8 = 0xF as *mut u8;

/// Channel 7 Control Register.
pub const CH7CTRL: *mut u8 = 0xF as *mut u8;

/// Output Override Disable Register.
pub const OUTOVDIS: *mut u8 = 0xF as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | H | 100000 |
/// | Z | 10 |
/// | I | 10000000 |
/// | T | 1000000 |
/// | V | 1000 |
/// | N | 100 |
/// | S | 10000 |
/// | C | 1 |
pub const SREG: *mut u8 = 0xF as *mut u8;

/// Period or Capture Register High.
pub const PERCAPTH: *mut u8 = 0xF as *mut u8;

/// Channel 0 Result.
pub const CH0RES: *mut u16 = 0x10 as *mut u16;

/// Channel 0 Result low byte.
pub const CH0RESL: *mut u8 = 0x10 as *mut u8;

/// Wafer Number.
pub const WAFNUM: *mut u8 = 0x10 as *mut u8;

/// Pin 0 Control Register.
pub const PIN0CTRL: *mut u8 = 0x10 as *mut u8;

/// Event Strobe.
pub const STROBE: *mut u8 = 0x10 as *mut u8;

/// Channel 0 Result high byte.
pub const CH0RESH: *mut u8 = 0x11 as *mut u8;

/// Pin 1 Control Register.
pub const PIN1CTRL: *mut u8 = 0x11 as *mut u8;

/// Digital Filter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FILTSEL | 1000 |
/// | PRESCFILT | 11110000 |
/// | PRESC | 111 |
pub const DFCTRL: *mut u8 = 0x12 as *mut u8;

/// Pin 2 Control Register.
pub const PIN2CTRL: *mut u8 = 0x12 as *mut u8;

/// Wafer Coordinate X Byte 0.
pub const COORDX0: *mut u8 = 0x12 as *mut u8;

/// Wafer Coordinate X Byte 1.
pub const COORDX1: *mut u8 = 0x13 as *mut u8;

/// Pin 3 Control Register.
pub const PIN3CTRL: *mut u8 = 0x13 as *mut u8;

/// Pin 4 Control Register.
pub const PIN4CTRL: *mut u8 = 0x14 as *mut u8;

/// Wafer Coordinate Y Byte 0.
pub const COORDY0: *mut u8 = 0x14 as *mut u8;

/// Wafer Coordinate Y Byte 1.
pub const COORDY1: *mut u8 = 0x15 as *mut u8;

/// Pin 5 Control Register.
pub const PIN5CTRL: *mut u8 = 0x15 as *mut u8;

/// Pin 6 Control Register.
pub const PIN6CTRL: *mut u8 = 0x16 as *mut u8;

/// Pin 7 Control Register.
pub const PIN7CTRL: *mut u8 = 0x17 as *mut u8;

/// Channel 0 Data low byte.
pub const CH0DATAL: *mut u8 = 0x18 as *mut u8;

/// Channel 0 Data.
pub const CH0DATA: *mut u16 = 0x18 as *mut u16;

/// Compare Value.
pub const CMP: *mut u16 = 0x18 as *mut u16;

/// Channel 0 Data high byte.
pub const CH0DATAH: *mut u8 = 0x19 as *mut u8;

/// Channel 1 Data.
pub const CH1DATA: *mut u16 = 0x1A as *mut u16;

/// Channel 1 Data low byte.
pub const CH1DATAL: *mut u8 = 0x1A as *mut u8;

/// Channel 1 Data high byte.
pub const CH1DATAH: *mut u8 = 0x1B as *mut u8;

/// Temperature corresponds to TEMPSENSE3/2.
pub const ROOMTEMP: *mut u8 = 0x1E as *mut u8;

/// Temperature corresponds to TEMPSENSE1/0.
pub const HOTTEMP: *mut u8 = 0x1F as *mut u8;

/// ADCA Calibration Byte 0.
pub const ADCACAL0: *mut u8 = 0x20 as *mut u8;

/// Count.
pub const CNT: *mut u16 = 0x20 as *mut u16;

/// ADCA Calibration Byte 1.
pub const ADCACAL1: *mut u8 = 0x21 as *mut u8;

/// Period.
pub const PER: *mut u16 = 0x26 as *mut u16;

/// Period low byte.
pub const PERL: *mut u8 = 0x26 as *mut u8;

/// Period high byte.
pub const PERH: *mut u8 = 0x27 as *mut u8;

/// Compare or Capture A low byte.
pub const CCAL: *mut u8 = 0x28 as *mut u8;

/// ACA Current Calibration Byte.
pub const ACACURRCAL: *mut u8 = 0x28 as *mut u8;

/// Compare or Capture A.
pub const CCA: *mut u16 = 0x28 as *mut u16;

/// Compare or Capture A high byte.
pub const CCAH: *mut u8 = 0x29 as *mut u8;

/// Compare or Capture B.
pub const CCB: *mut u16 = 0x2A as *mut u16;

/// Compare or Capture B low byte.
pub const CCBL: *mut u8 = 0x2A as *mut u8;

/// Compare or Capture B high byte.
pub const CCBH: *mut u8 = 0x2B as *mut u8;

/// Compare or Capture C low byte.
pub const CCCL: *mut u8 = 0x2C as *mut u8;

/// Temperature Sensor Calibration Byte 2.
pub const TEMPSENSE2: *mut u8 = 0x2C as *mut u8;

/// Compare or Capture C.
pub const CCC: *mut u16 = 0x2C as *mut u16;

/// Compare or Capture C high byte.
pub const CCCH: *mut u8 = 0x2D as *mut u8;

/// Temperature Sensor Calibration Byte 3.
pub const TEMPSENSE3: *mut u8 = 0x2D as *mut u8;

/// Compare or Capture D.
pub const CCD: *mut u16 = 0x2E as *mut u16;

/// Temperature Sensor Calibration Byte 0.
pub const TEMPSENSE0: *mut u8 = 0x2E as *mut u8;

/// Compare or Capture D low byte.
pub const CCDL: *mut u8 = 0x2E as *mut u8;

/// Temperature Sensor Calibration Byte 1.
pub const TEMPSENSE1: *mut u8 = 0x2F as *mut u8;

/// Compare or Capture D high byte.
pub const CCDH: *mut u8 = 0x2F as *mut u8;

/// DACA0 Calibration Byte 0.
pub const DACA0OFFCAL: *mut u8 = 0x30 as *mut u8;

/// DACA0 Calibration Byte 1.
pub const DACA0GAINCAL: *mut u8 = 0x31 as *mut u8;

/// DACA1 Calibration Byte 0.
pub const DACA1OFFCAL: *mut u8 = 0x34 as *mut u8;

/// DACA1 Calibration Byte 1.
pub const DACA1GAINCAL: *mut u8 = 0x35 as *mut u8;

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

/// Compare Or Capture D Buffer.
pub const CCDBUF: *mut u16 = 0x3E as *mut u16;

/// Compare Or Capture D Buffer low byte.
pub const CCDBUFL: *mut u8 = 0x3E as *mut u8;

/// Compare Or Capture D Buffer high byte.
pub const CCDBUFH: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACEVOUT`
pub const EVOUTSEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ACEVOUT`
pub const EVOUT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `ACEVOUT`
pub const EVASYEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACEVOUT`
pub const ACOUT: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADDRCTRL`
pub const RELOAD: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `ADDRMASK`
pub const ADDREN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ANAINIT`
pub const STARTUPDLYA: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `AVGCTRL`
pub const RIGHTSHIFT: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `AVGCTRL`
pub const SAMPNUM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `BAUDCTRLB`
pub const BSCALE: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CALA`
pub const CALL: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `CALB`
pub const CALH: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `CALIB`
pub const ERROR: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `CALIB`
pub const SIGN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const QDEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const ROTARY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const QDIEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CH0CTRL`
pub const QDIRM: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `CLKOUT`
pub const CLKEVPIN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKOUT`
pub const CLKOUTSEL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CLKOUT`
pub const RTCOUT: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `CNTH`
pub const PCNT20: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CNTH`
pub const PCNT21: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CORRCTRL`
pub const CORREN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLA`
pub const DREINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLA`
pub const RXSIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CTRLA`
pub const DRIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CTRLA`
pub const TXCINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CTRLA`
pub const RXCINTLVL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLB`
pub const SSD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLB`
pub const BUFMODE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `CTRLC`
pub const CHSIZE: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CTRLC`
pub const SBMODE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CTRLC`
pub const PMODE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLC`
pub const CMODE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `CTRLD`
pub const DECTYPE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLD`
pub const LUTACT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CTRLD`
pub const PECACT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLE`
pub const BLANKB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLE`
pub const CAPTB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CTRLE`
pub const FILTERB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLE`
pub const QUALB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLF`
pub const HCCAMODE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLF`
pub const HCCBMODE: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CTRLG`
pub const EVACTEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CTRLG`
pub const EVACT1: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `CTRLG`
pub const EVACT0: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `CTRLG`
pub const EVSRC: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CTRLGCLR`
pub const HALTACLR: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CTRLGCLR`
pub const STATEECLR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CTRLGCLR`
pub const FAULTB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLGCLR`
pub const HALTBCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CTRLGCLR`
pub const FAULTA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLGCLR`
pub const FAULTE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLGSET`
pub const FAULTESW: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CTRLGSET`
pub const IDXCMD: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `CTRLGSET`
pub const FAULTASW: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CTRLGSET`
pub const FAULTBSW: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CURRCTRL`
pub const CURRMODE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CURRCTRL`
pub const AC0CURR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CURRCTRL`
pub const AC1CURR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CURRCTRL`
pub const CURREN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DESTADDRCTRL`
pub const DESTDIR: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `DESTADDRCTRL`
pub const DESTRELOAD: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DFCTRL`
pub const FILTSEL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DFCTRL`
pub const PRESCFILT: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `DFCTRL`
pub const PRESC: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `DFLLCTRL`
pub const RC32MCREF: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `EVCTRL`
pub const EVSPLIT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EVSYSLOCK`
pub const EVSYS0LOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EVSYSLOCK`
pub const EVSYS1LOCK: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FAULTLOCK`
pub const FAULTC4LOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FAULTLOCK`
pub const FAULTC5LOCK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FUSEBYTE1`
pub const WDWP: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FUSEBYTE1`
pub const WDP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const BODPD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const BOOTRST: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const WDLOCK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const SUT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const RSTDISBL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const BODACT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const BODLVL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `FUSEBYTE6`
pub const FDACT4: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FUSEBYTE6`
pub const FDACT5: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FUSEBYTE6`
pub const VALUE: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `GAINCORR1`
pub const GAINCORR: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `INTCTRL`
pub const SSIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const TXCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const RXCIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const DREIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const TRGINTLVL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const ERRINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const OVFINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const CCBINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCCAINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCCBINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const CCAINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const TRGIF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const OVFIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const CCBIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCCBIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const ERRIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCCAIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const CCAIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBA: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBB: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBAT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MUXCTRL`
pub const MUXINT: *mut u8 = 0x78 as *mut u8;

/// Bitfield on register `OFFSETCORR1`
pub const OFFSETCORR: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLFAC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLSRC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLDIV: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRGEN`
pub const EDMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRGEN`
pub const XCL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRGEN`
pub const RTC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRGEN`
pub const EVSYS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRPA`
pub const ADC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRPA`
pub const AC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRPA`
pub const DAC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRPC`
pub const SPI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRPC`
pub const TWI: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRPC`
pub const TC4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRPC`
pub const HIRES: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PSCTRL`
pub const PSBCDIV: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PSCTRL`
pub const PSADIV: *mut u8 = 0x7C as *mut u8;

/// Bitfield on register `REFCTRL`
pub const REFSEL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const TEMPREF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const BANDGAP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC4D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC4C: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC4B: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC4A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RTCCTRL`
pub const RTCEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RTCCTRL`
pub const RTCSRC: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SAMPCTRL`
pub const SAMPVAL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SCAN`
pub const INPUTSCAN: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SCAN`
pub const INPUTOFFSET: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SRLCTRL`
pub const SRLENRD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SRLCTRL`
pub const SRLENRA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SRLCTRL`
pub const SRLENRC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SRLCTRL`
pub const SRLENRR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `STATUS`
pub const IF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `STATUS`
pub const WRCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `STATUS`
pub const BUFOVF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `STATUS`
pub const RXCIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `STATUS`
pub const DREIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `STATUS`
pub const TXCIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `STATUS`
pub const SSIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SWAP`
pub const SWAP3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SWAP`
pub const SWAP2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SWAP`
pub const SWAP0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SWAP`
pub const SWAP1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SWAPBUF`
pub const SWAP0BUF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SWAPBUF`
pub const SWAP2BUF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SWAPBUF`
pub const SWAP3BUF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SWAPBUF`
pub const SWAP1BUF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TOCONF`
pub const TTOUTMSEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TOCONF`
pub const TTOUTSSEL: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `TOCONF`
pub const TMSEXTSEL: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `TOS`
pub const TTOUTSIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TOS`
pub const TMEXTIF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TOS`
pub const TSEXTIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TOS`
pub const TTOUTMIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `WEXLOCK`
pub const WEXCLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WINTMODE: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const X32KLPM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const FRQRANGE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const XOSCPWR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const XOSCSEL: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const PLLFDEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const XOSCFDIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const PLLFDIF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const XOSCFDEN: *mut u8 = 0x1 as *mut u8;

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
   /// Differential input, gain with 4 LSB pins selection.
   pub const DIFFWGAINL: u32 = 0x2;
   /// Differential input, gain with 4 MSB pins selection.
   pub const DIFFWGAINH: u32 = 0x3;
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
   /// 1/10 Scaled VCC.
   pub const SCALEDVCC: u32 = 0x2;
   /// DAC Output.
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
   /// Input pin 12.
   pub const PIN12: u32 = 0xC;
   /// Input pin 13.
   pub const PIN13: u32 = 0xD;
   /// Input pin 14.
   pub const PIN14: u32 = 0xE;
   /// Input pin 15.
   pub const PIN15: u32 = 0xF;
}

/// Current Limitation
#[allow(non_upper_case_globals)]
pub mod adc_currlimit {
   /// No current limit,     300ksps max sampling rate.
   pub const NO: u32 = 0x0;
   /// Low current limit,    250ksps max sampling rate.
   pub const LOW: u32 = 0x1;
   /// Medium current limit, 150ksps max sampling rate.
   pub const MED: u32 = 0x2;
   /// High current limit,   50ksps max sampling rate.
   pub const HIGH: u32 = 0x3;
}

/// Event action selection
#[allow(non_upper_case_globals)]
pub mod adc_evact {
   /// No event action.
   pub const NONE: u32 = 0x0;
   /// First event triggers channel conversion.
   pub const CH0: u32 = 0x1;
   /// The ADC is flushed and restarted for accurate timing.
   pub const SYNCSWEEP: u32 = 0x6;
}

/// Event channel input selection
#[allow(non_upper_case_globals)]
pub mod adc_evsel {
   /// Event Channel 0.
   pub const _0: u32 = 0x0;
   /// Event Channel 1.
   pub const _1: u32 = 0x1;
   /// Event Channel 2.
   pub const _2: u32 = 0x2;
   /// Event Channel 3.
   pub const _3: u32 = 0x3;
   /// Event Channel 4.
   pub const _4: u32 = 0x4;
   /// Event Channel 5.
   pub const _5: u32 = 0x5;
   /// Event Channel 6.
   pub const _6: u32 = 0x6;
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
   /// External reference on PORT D.
   pub const AREFD: u32 = 0x3;
   /// Internal VCC / 2.
   pub const INTVCC2: u32 = 0x4;
}

/// Conversion result resolution
#[allow(non_upper_case_globals)]
pub mod adc_resolution {
   /// 12-bit right-adjusted result.
   pub const _12BIT: u32 = 0x0;
   /// More than 12-bit (oversapling) right-adjusted result.
   pub const MT12BIT: u32 = 0x1;
   /// 8-bit right-adjusted result.
   pub const _8BIT: u32 = 0x2;
   /// 12-bit left-adjusted result.
   pub const LEFT12BIT: u32 = 0x3;
}

/// Averaged Number of Samples
#[allow(non_upper_case_globals)]
pub mod adc_sampnum {
   /// 1 Sample.
   pub const _1X: u32 = 0x0;
   /// 2 Samples.
   pub const _2X: u32 = 0x1;
   /// 4 Samples.
   pub const _4X: u32 = 0x2;
   /// 8 Samples.
   pub const _8X: u32 = 0x3;
   /// 16 Samples.
   pub const _16X: u32 = 0x4;
   /// 32 Samples.
   pub const _32X: u32 = 0x5;
   /// 64 Samples.
   pub const _64X: u32 = 0x6;
   /// 128 Samples.
   pub const _128X: u32 = 0x7;
   /// 256 Samples.
   pub const _256X: u32 = 0x8;
   /// 512 Samples.
   pub const _512X: u32 = 0x9;
   /// 1024 Samples.
   pub const _1024X: u32 = 0xA;
}

/// BOD operation
#[allow(non_upper_case_globals)]
pub mod bod {
   /// BOD enabled in sampled mode.
   pub const SAMPLED: u32 = 0x1;
   /// BOD enabled continuously.
   pub const CONTINUOUS: u32 = 0x2;
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
   /// Divide by 6.
   pub const _6: u32 = 0x13;
   /// Divide by 10.
   pub const _10: u32 = 0x15;
   /// Divide by 12.
   pub const _12: u32 = 0x17;
   /// Divide by 24.
   pub const _24: u32 = 0x19;
   /// Divide by 48.
   pub const _48: u32 = 0x1B;
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
   /// 1.024 kHz from internal 32kHz ULP.
   pub const ULP: u32 = 0x0;
   /// 1.024 kHz from 32.768 kHz crystal oscillator on TOSC.
   pub const TOSC: u32 = 0x1;
   /// 1.024 kHz from internal 32.768 kHz RC oscillator.
   pub const RCOSC: u32 = 0x2;
   /// 32.768 kHz from 32.768 kHz crystal oscillator on TOSC.
   pub const TOSC32: u32 = 0x5;
   /// 32.768 kHz from internal 32.768 kHz RC oscillator.
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
   /// Internal 8 MHz RC Oscillator.
   pub const RC8M: u32 = 0x5;
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
   /// DMAC Channel 0.
   pub const DMAC0: u32 = 0x4;
   /// DMAC Channel 1.
   pub const DMAC1: u32 = 0x5;
   /// DMAC Channel 2.
   pub const DMAC2: u32 = 0x6;
   /// DMAC Channel 3.
   pub const DMAC3: u32 = 0x7;
}

/// Output channel selection
#[allow(non_upper_case_globals)]
pub mod dac_chsel {
   /// Single channel operation (Channel 0 only).
   pub const SINGLE: u32 = 0x0;
   /// Single channel operation (Channel 1 only).
   pub const SINGLE1: u32 = 0x1;
   /// Dual channel operation (Channel 0 and channel 1).
   pub const DUAL: u32 = 0x2;
}

/// Event channel selection
#[allow(non_upper_case_globals)]
pub mod dac_evsel {
   /// Event Channel 0.
   pub const _0: u32 = 0x0;
   /// Event Channel 1.
   pub const _1: u32 = 0x1;
   /// Event Channel 2.
   pub const _2: u32 = 0x2;
   /// Event Channel 3.
   pub const _3: u32 = 0x3;
   /// Event Channel 4.
   pub const _4: u32 = 0x4;
   /// Event Channel 5.
   pub const _5: u32 = 0x5;
   /// Event Channel 6.
   pub const _6: u32 = 0x6;
   /// Event Channel 7.
   pub const _7: u32 = 0x7;
}

/// Reference voltage selection
#[allow(non_upper_case_globals)]
pub mod dac_refsel {
   /// Internal 1V.
   pub const INT1V: u32 = 0x0;
   /// Analog supply voltage.
   pub const AVCC: u32 = 0x1;
   /// External reference on AREF on PORTA.
   pub const AREFA: u32 = 0x2;
   /// External reference on AREF on PORTD.
   pub const AREFD: u32 = 0x3;
   /// Define for PortB kept for legacy reasons.
   pub const AREFB: u32 = 0x3;
}

/// Channel mode
#[allow(non_upper_case_globals)]
pub mod edma_chmode {
   /// Channels 0, 1, 2 and 3 in peripheal conf.
   pub const PER0123: u32 = 0x0;
   /// Channel 0 in standard conf.; channels 2 and 3 in peripheral conf.
   pub const STD0: u32 = 0x1;
   /// Channel 2 in standard conf.; channels 0 and 1 in peripheral conf.
   pub const STD2: u32 = 0x2;
   /// Channels 0 and 2 in standard conf.
   pub const STD02: u32 = 0x3;
}

/// Destination addressing mode
#[allow(non_upper_case_globals)]
pub mod edma_ch_destdir {
   /// Fixed.
   pub const FIXED: u32 = 0x0;
   /// Increment.
   pub const INC: u32 = 0x1;
   /// 1-byte 'mask-match' (data: ADDRL, mask: ADDRH).
   pub const MP1: u32 = 0x4;
   /// 1-byte 'OR-match' (data-1: ADDRL OR data-2: ADDRH).
   pub const MP2: u32 = 0x5;
   /// 2-byte 'match' (data1: ADDRL followed by data2: ADDRH).
   pub const MP3: u32 = 0x6;
}

/// Memory Address Mode for Peripheral Ch., or Source Address Mode for Standard Ch.
#[allow(non_upper_case_globals)]
pub mod edma_ch_dir {
   /// Fixed.
   pub const FIXED: u32 = 0x0;
   /// Increment.
   pub const INC: u32 = 0x1;
   /// If Peripheral Ch. (Per ==> Mem), 1-byte 'mask-match' (data: ADDRL, mask: ADDRH), else reserved conf.
   pub const MP1: u32 = 0x4;
   /// If Peripheral Ch. (Per ==> Mem), 1-byte 'OR-match' (data-1: ADDRL OR data-2: ADDRH), else reserved conf.
   pub const MP2: u32 = 0x5;
   /// If Peripheral Ch. (Per ==> Mem), 2-byte 'match' (data-1: ADDRL followed by data-2: ADDRH), else reserved conf.
   pub const MP3: u32 = 0x6;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod edma_ch_intlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Memory Address Reload for Peripheral Ch., or Source Address Reload for Standard Ch.
#[allow(non_upper_case_globals)]
pub mod edma_ch_reload {
   /// No reload.
   pub const NONE: u32 = 0x0;
   /// Reload at end of each block transfer.
   pub const BLOCK: u32 = 0x1;
   /// Reload at end of each burst transfer.
   pub const BURST: u32 = 0x2;
   /// Reload at end of each transaction.
   pub const TRANSACTION: u32 = 0x3;
}

/// Transfer trigger source
#[allow(non_upper_case_globals)]
pub mod edma_ch_trigsrc {
   /// Software triggers only.
   pub const OFF: u32 = 0x0;
   /// Event CH0 as trigger (Standard Channels Only).
   pub const EVSYS_CH0: u32 = 0x1;
   /// Event CH1 as trigger (Standard Channels Only).
   pub const EVSYS_CH1: u32 = 0x2;
   /// Event CH2 as trigger (Standard Channels Only).
   pub const EVSYS_CH2: u32 = 0x3;
   /// ADCA CH0 as trigger.
   pub const ADCA_CH0: u32 = 0x10;
   /// DACA CH0 as trigger.
   pub const DACA_CH0: u32 = 0x15;
   /// DACA CH1 as trigger.
   pub const DACA_CH1: u32 = 0x16;
   /// TCC4 overflow/underflow as trigger (Standard Channels Only).
   pub const TCC4_OVF: u32 = 0x40;
   /// TCC4 error as trigger (Standard Channels Only).
   pub const TCC4_ERR: u32 = 0x41;
   /// TCC4 compare or capture channel A as trigger (Standard Channels Only).
   pub const TCC4_CCA: u32 = 0x42;
   /// TCC4 compare or capture channel B as trigger (Standard Channels Only).
   pub const TCC4_CCB: u32 = 0x43;
   /// TCC4 compare or capture channel C as trigger (Standard Channels Only).
   pub const TCC4_CCC: u32 = 0x44;
   /// TCC4 compare or capture channel D as trigger (Standard Channels Only).
   pub const TCC4_CCD: u32 = 0x45;
   /// TCC5 overflow/underflow as trigger (Standard Channels Only).
   pub const TCC5_OVF: u32 = 0x46;
   /// TCC5 error as trigger (Standard Channels Only).
   pub const TCC5_ERR: u32 = 0x47;
   /// TCC5 compare or capture channel A as trigger (Standard Channels Only).
   pub const TCC5_CCA: u32 = 0x48;
   /// TCC5 compare or capture channel B as trigger (Standard Channels Only).
   pub const TCC5_CCB: u32 = 0x49;
   /// SPI C transfer complete (SPI Standard Mode) or SPI C receive complete as trigger (SPI Buffer Modes).
   pub const SPIC_RXC: u32 = 0x4A;
   /// SPI C transfer complete (SPI Standard Mode) or SPI C data register empty as trigger (SPI Buffer modes).
   pub const SPIC_DRE: u32 = 0x4B;
   /// USART C0 receive complete as trigger.
   pub const USARTC0_RXC: u32 = 0x4C;
   /// USART C0 data register empty as trigger.
   pub const USARTC0_DRE: u32 = 0x4D;
   /// TCD5 overflow/underflow as trigger (Standard Channels Only).
   pub const TCD5_OVF: u32 = 0x66;
   /// TCD5 error as trigger (Standard Channels Only).
   pub const TCD5_ERR: u32 = 0x67;
   /// TCD5 compare or capture channel A as trigger (Standard Channels Only).
   pub const TCD5_CCA: u32 = 0x68;
   /// TCD5 compare or capture channel B as trigger (Standard Channels Only).
   pub const TCD5_CCB: u32 = 0x69;
   /// USART D0 receive complete as trigger.
   pub const USARTD0_RXC: u32 = 0x6C;
   /// USART D0 data register empty as trigger.
   pub const USARTD0_DRE: u32 = 0x6D;
}

/// Double buffer mode
#[allow(non_upper_case_globals)]
pub mod edma_dbufmode {
   /// No double buffer enabled.
   pub const DISABLE: u32 = 0x0;
   /// Double buffer enabled on peripheral channels 0/1 (if exist).
   pub const BUF01: u32 = 0x1;
   /// Double buffer enabled on peripheral channels 2/3 (if exist).
   pub const BUF23: u32 = 0x2;
   /// Double buffer enabled on peripheral channels 0/1 and 2/3 or standard channels 0/2.
   pub const BUF0123: u32 = 0x3;
}

/// Priority mode
#[allow(non_upper_case_globals)]
pub mod edma_primode {
   /// Round robin on all channels.
   pub const RR0123: u32 = 0x0;
   /// Ch0 > round robin (Ch 1 ch2 Ch3).
   pub const RR123: u32 = 0x1;
   /// Ch0 > Ch 1 > round robin (Ch2 Ch3).
   pub const RR23: u32 = 0x2;
   /// Ch0 > Ch1 > Ch2 > Ch3.
   pub const CH0123: u32 = 0x3;
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
   /// XCL BTC0 underflow.
   pub const XCL_UNF0: u32 = 0xB0;
   /// XCL BTC1 underflow.
   pub const XCL_UNF1: u32 = 0xB1;
   /// XCL BTC0 capture or compare.
   pub const XCL_CC0: u32 = 0xB2;
   /// XCL BTC1 capture or compare.
   pub const XCL_CC1: u32 = 0xB3;
   /// XCL PEC0 restart.
   pub const XCL_PEC0: u32 = 0xB4;
   /// XCL PEC1 restart.
   pub const XCL_PEC1: u32 = 0xB5;
   /// XCL LUT0 output.
   pub const XCL_LUT0: u32 = 0xB6;
   /// XCL LUT1 output.
   pub const XCL_LUT1: u32 = 0xB7;
   /// Timer/Counter C4 Overflow.
   pub const TCC4_OVF: u32 = 0xC0;
   /// Timer/Counter C4 Error.
   pub const TCC4_ERR: u32 = 0xC1;
   /// Timer/Counter C4 Compare or Capture A.
   pub const TCC4_CCA: u32 = 0xC4;
   /// Timer/Counter C4 Compare or Capture B.
   pub const TCC4_CCB: u32 = 0xC5;
   /// Timer/Counter C4 Compare or Capture C.
   pub const TCC4_CCC: u32 = 0xC6;
   /// Timer/Counter C4 Compare or Capture D.
   pub const TCC4_CCD: u32 = 0xC7;
   /// Timer/Counter C5 Overflow.
   pub const TCC5_OVF: u32 = 0xC8;
   /// Timer/Counter C5 Error.
   pub const TCC5_ERR: u32 = 0xC9;
   /// Timer/Counter C5 Compare or Capture A.
   pub const TCC5_CCA: u32 = 0xCC;
   /// Timer/Counter C5 Compare or Capture B.
   pub const TCC5_CCB: u32 = 0xCD;
   /// Timer/Counter D5 Overflow.
   pub const TCD5_OVF: u32 = 0xD8;
   /// Timer/Counter D5 Error.
   pub const TCD5_ERR: u32 = 0xD9;
   /// Timer/Counter D5 Compare or Capture A.
   pub const TCD5_CCA: u32 = 0xDC;
   /// Timer/Counter D5 Compare or Capture B.
   pub const TCD5_CCB: u32 = 0xDD;
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

/// Prescaler
#[allow(non_upper_case_globals)]
pub mod evsys_prescaler {
   /// CLKPER, divide by 8.
   pub const CLKPER_8: u32 = 0x0;
   /// CLKPER, divide by 64.
   pub const CLKPER_64: u32 = 0x1;
   /// CLKPER, divide by 512.
   pub const CLKPER_512: u32 = 0x2;
   /// CLKPER, divide by 4096.
   pub const CLKPER_4096: u32 = 0x3;
   /// CLKPER, divide by 32768.
   pub const CLKPER_32768: u32 = 0x4;
}

/// Prescaler Filter
#[allow(non_upper_case_globals)]
pub mod evsys_prescfilt {
   /// Enable prescaler filter for either channel 0 or 4.
   pub const CH04: u32 = 0x1;
   /// Enable prescaler filter for either channel 1 or 5.
   pub const CH15: u32 = 0x2;
   /// Enable prescaler filter for either channel 2 or 6.
   pub const CH26: u32 = 0x4;
   /// Enable prescaler filter for either channel 3 or 7.
   pub const CH37: u32 = 0x8;
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

/// Fault A Halt Action Selection
#[allow(non_upper_case_globals)]
pub mod fault_halta {
   /// Halt Action Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Hardware Halt Action.
   pub const HW: u32 = 0x1;
   /// Software Halt Action.
   pub const SW: u32 = 0x2;
}

/// Fault B Halt Action Selection
#[allow(non_upper_case_globals)]
pub mod fault_haltb {
   /// Halt Action Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Hardware Halt Action.
   pub const HW: u32 = 0x1;
   /// Software Halt Action.
   pub const SW: u32 = 0x2;
}

/// Channel index Command
#[allow(non_upper_case_globals)]
pub mod fault_idxcmd {
   /// Command Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Force Cycle B in Next Cycle.
   pub const SET: u32 = 0x1;
   /// Force Cycle A in Next Cycle.
   pub const CLEAR: u32 = 0x2;
   /// Hold Current Cycle Index in Next Cycle.
   pub const HOLD: u32 = 0x3;
}

/// Ramp Mode Selection
#[allow(non_upper_case_globals)]
pub mod fault_ramp {
   /// Normal Mode.
   pub const RAMP1: u32 = 0x0;
   /// RAMP2 Mode.
   pub const RAMP2: u32 = 0x2;
}

/// Fault A Source Selection
#[allow(non_upper_case_globals)]
pub mod fault_srca {
   /// Fault A Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Event Channel n.
   pub const CHN: u32 = 0x1;
   /// Event Channel n+1.
   pub const CHN1: u32 = 0x2;
   /// Fault A linked to Fault B State from previous cycle.
   pub const LINK: u32 = 0x3;
}

/// Fault B Source Selection
#[allow(non_upper_case_globals)]
pub mod fault_srcb {
   /// Fault B disabled.
   pub const DISABLE: u32 = 0x0;
   /// Event Channel n.
   pub const CHN: u32 = 0x1;
   /// Event Channel n+1.
   pub const CHN1: u32 = 0x2;
   /// Fault B linked to Fault A State from previous cycle.
   pub const LINK: u32 = 0x3;
}

/// Fault E Input Source Selection
#[allow(non_upper_case_globals)]
pub mod fault_srce {
   /// Fault Protection Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Event Channel n.
   pub const CHN: u32 = 0x1;
   /// Event Channel n+1.
   pub const CHN1: u32 = 0x2;
   /// Event Channel n+2.
   pub const CHN2: u32 = 0x3;
}

/// Boot lock bits - application section
#[allow(non_upper_case_globals)]
pub mod fuse_blba {
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// Boot lock bits - application table section
#[allow(non_upper_case_globals)]
pub mod fuse_blbat {
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// Boot lock bits - boot setcion
#[allow(non_upper_case_globals)]
pub mod fuse_blbb {
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// Lock bits
#[allow(non_upper_case_globals)]
pub mod fuse_lb {
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// High Resolution Mode
#[allow(non_upper_case_globals)]
pub mod hires_hren {
   /// No Hi-Res.
   pub const NONE: u32 = 0x0;
   /// Hi-Res enabled on Timer 4.
   pub const HRP4: u32 = 0x1;
   /// Hi-Res enabled on Timer 5.
   pub const HRP5: u32 = 0x2;
   /// Hi-Res enabled on Timer 4 and 5.
   pub const BOTH: u32 = 0x3;
}

/// High Resolution Plus Mode
#[allow(non_upper_case_globals)]
pub mod hires_hrplus {
   /// No Hi-Res Plus.
   pub const NONE: u32 = 0x0;
   /// Hi-Res Plus enabled on Timer 4.
   pub const HRP4: u32 = 0x1;
   /// Hi-Res Plus enabled on Timer 5.
   pub const HRP5: u32 = 0x2;
   /// Hi-Res Plus enabled on Timer 4 and 5.
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
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// Boot lock bits - application table section
#[allow(non_upper_case_globals)]
pub mod nvm_blbat {
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// Boot lock bits - boot setcion
#[allow(non_upper_case_globals)]
pub mod nvm_blbb {
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Read not allowed.
   pub const RLOCK: u32 = 0x1;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
}

/// NVM Command
#[allow(non_upper_case_globals)]
pub mod nvm_cmd {
   /// Noop/Ordinary LPM.
   pub const NO_OPERATION: u32 = 0x0;
   /// Read user signature row.
   pub const READ_USER_SIG_ROW: u32 = 0x1;
   /// Read calibration row.
   pub const READ_CALIB_ROW: u32 = 0x2;
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
   /// Erase Flash Page.
   pub const ERASE_FLASH_PAGE: u32 = 0x2B;
   /// Write Boot Section page.
   pub const WRITE_BOOT_PAGE: u32 = 0x2C;
   /// Erase-and-write Boot Section page.
   pub const ERASE_WRITE_BOOT_PAGE: u32 = 0x2D;
   /// Write Flash Page.
   pub const WRITE_FLASH_PAGE: u32 = 0x2E;
   /// Erase-and-write Flash Page.
   pub const ERASE_WRITE_FLASH_PAGE: u32 = 0x2F;
   /// Erase EEPROM.
   pub const ERASE_EEPROM: u32 = 0x30;
   /// Erase EEPROM page.
   pub const ERASE_EEPROM_PAGE: u32 = 0x32;
   /// Write EEPROM page.
   pub const WRITE_EEPROM_PAGE: u32 = 0x34;
   /// Erase-and-write EEPROM page.
   pub const ERASE_WRITE_EEPROM_PAGE: u32 = 0x35;
   /// Erase/flush EEPROM page buffer.
   pub const ERASE_EEPROM_BUFFER: u32 = 0x36;
   /// Application section CRC.
   pub const APP_CRC: u32 = 0x38;
   /// Boot Section CRC.
   pub const BOOT_CRC: u32 = 0x39;
   /// Flash Range CRC.
   pub const FLASH_RANGE_CRC: u32 = 0x3A;
   /// Erase Chip.
   pub const CHIP_ERASE: u32 = 0x40;
   /// Read NVM.
   pub const READ_NVM: u32 = 0x43;
   /// Write Fuse byte.
   pub const WRITE_FUSE: u32 = 0x4C;
   /// Erase Boot Section.
   pub const ERASE_BOOT: u32 = 0x68;
   /// Flash CRC.
   pub const FLASH_CRC: u32 = 0x78;
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
   /// Read and write not allowed.
   pub const RWLOCK: u32 = 0x0;
   /// Write not allowed.
   pub const WLOCK: u32 = 0x2;
   /// No locks.
   pub const NOLOCK: u32 = 0x3;
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
   /// Internal 2 MHz RC Oscillator.
   pub const RC2M: u32 = 0x0;
   /// Internal 8 MHz RC Oscillator.
   pub const RC8M: u32 = 0x1;
   /// Internal 32 MHz RC Oscillator.
   pub const RC32M: u32 = 0x2;
   /// External Oscillator.
   pub const XOSC: u32 = 0x3;
}

/// 32 MHz DFLL Calibration Reference
#[allow(non_upper_case_globals)]
pub mod osc_rc32mcref {
   /// Internal 32.768 kHz RC Oscillator.
   pub const RC32K: u32 = 0x0;
   /// External 32.768 kHz Crystal Oscillator.
   pub const XOSC32K: u32 = 0x1;
}

/// External Oscillator Selection and Startup Time
#[allow(non_upper_case_globals)]
pub mod osc_xoscsel {
   /// External Clock on port R1 - 6 CLK.
   pub const EXTCLK: u32 = 0x0;
   /// 32.768 kHz TOSC - 32K CLK.
   pub const _32KHz: u32 = 0x2;
   /// 0.4-16 MHz XTAL - 256 CLK.
   pub const XTAL_256CLK: u32 = 0x3;
   /// 0.4-16 MHz XTAL - 1K CLK.
   pub const XTAL_1KCLK: u32 = 0x7;
   /// 0.4-16 MHz XTAL - 16K CLK.
   pub const XTAL_16KCLK: u32 = 0xB;
   /// External Clock on port C4 - 6 CLK.
   pub const EXTCLK_C4: u32 = 0x14;
}

/// Analog Comparator Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_acout {
   /// Analog Comparator Outputs on Port A, Pin 6-7.
   pub const PA: u32 = 0x0;
   /// Analog Comparator Outputs on Port C, Pin 6-7.
   pub const PC: u32 = 0x1;
   /// Analog Comparator Outputs on Port D, Pin 6-7.
   pub const PD: u32 = 0x2;
   /// Analog Comparator Outputs on Port R, Pin 0-1.
   pub const PR: u32 = 0x3;
}

/// Clock and Event Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_clkevpin {
   /// Clock and Event Ouput on PIN 7.
   pub const PIN7: u32 = 0x0;
   /// Clock and Event Ouput on PIN 4.
   pub const PIN4: u32 = 0x1;
}

/// System Clock Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_clkout {
   /// System Clock Output Disabled.
   pub const OFF: u32 = 0x0;
   /// System Clock Output on Port C pin 7.
   pub const PC7: u32 = 0x1;
   /// System Clock Output on Port D pin 7.
   pub const PD7: u32 = 0x2;
   /// System Clock Output on Port R pin 0.
   pub const PR0: u32 = 0x3;
}

/// Peripheral Clock Output Select
#[allow(non_upper_case_globals)]
pub mod portcfg_clkoutsel {
   /// 1x Peripheral Clock Output to pin.
   pub const CLK1X: u32 = 0x0;
   /// 2x Peripheral Clock Output to pin.
   pub const CLK2X: u32 = 0x1;
   /// 4x Peripheral Clock Output to pin.
   pub const CLK4X: u32 = 0x2;
}

/// Event Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_evout {
   /// Event Output Disabled.
   pub const OFF: u32 = 0x0;
   /// Event Channel n Output on Port C pin 7.
   pub const PC7: u32 = 0x1;
   /// Event Channel n Output on Port D pin 7.
   pub const PD7: u32 = 0x2;
   /// Event Channel n Output on Port R pin 0.
   pub const PR0: u32 = 0x3;
}

/// Event Output Select
#[allow(non_upper_case_globals)]
pub mod portcfg_evoutsel {
   /// Event Channel 0 output to pin.
   pub const _0: u32 = 0x0;
   /// Event Channel 1 output to pin.
   pub const _1: u32 = 0x1;
   /// Event Channel 2 output to pin.
   pub const _2: u32 = 0x2;
   /// Event Channel 3 output to pin.
   pub const _3: u32 = 0x3;
   /// Event Channel 4 output to pin.
   pub const _4: u32 = 0x4;
   /// Event Channel 5 output to pin.
   pub const _5: u32 = 0x5;
   /// Event Channel 6 output to pin.
   pub const _6: u32 = 0x6;
   /// Event Channel 7 output to pin.
   pub const _7: u32 = 0x7;
}

/// RTC Clock Output Port
#[allow(non_upper_case_globals)]
pub mod portcfg_rtcclkout {
   /// System Clock Output Disabled.
   pub const OFF: u32 = 0x0;
   /// System Clock Output on Port C pin 6.
   pub const PC6: u32 = 0x1;
   /// System Clock Output on Port D pin 6.
   pub const PD6: u32 = 0x2;
   /// System Clock Output on Port R pin 0.
   pub const PR0: u32 = 0x3;
}

/// Port Interrupt Level
#[allow(non_upper_case_globals)]
pub mod port_intlvl {
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
   /// Digital Input Buffer Forced Enable.
   pub const FORCE_ENABLE: u32 = 0x6;
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

/// Buffer Modes
#[allow(non_upper_case_globals)]
pub mod spi_bufmode {
   /// SPI Unbuffered Mode.
   pub const OFF: u32 = 0x0;
   /// Buffer Mode 1 (with dummy byte).
   pub const BUFMODE1: u32 = 0x2;
   /// Buffer Mode 2 (no dummy byte).
   pub const BUFMODE2: u32 = 0x3;
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
   /// SPI Mode 0, base clock at "0", sampling on leading edge (rising) & set-up on trailling edge (falling).
   pub const _0: u32 = 0x0;
   /// SPI Mode 1, base clock at "0", set-up on leading edge (rising) & sampling on trailling edge (falling).
   pub const _1: u32 = 0x1;
   /// SPI Mode 2, base clock at "1", sampling on leading edge (falling) & set-up on trailling edge (rising).
   pub const _2: u32 = 0x2;
   /// SPI Mode 3, base clock at "1", set-up on leading edge (falling) & sampling on trailling edge (rising).
   pub const _3: u32 = 0x3;
}

/// Prescaler setting
#[allow(non_upper_case_globals)]
pub mod spi_prescaler {
   /// If CLK2X=1 CLKper/2, else (CLK2X=0) CLKper/4.
   pub const DIV4: u32 = 0x0;
   /// If CLK2X=1 CLKper/8, else (CLK2X=0) CLKper/16.
   pub const DIV16: u32 = 0x1;
   /// If CLK2X=1 CLKper/32, else (CLK2X=0) CLKper/64.
   pub const DIV64: u32 = 0x2;
   /// If CLK2X=1 CLKper/64, else (CLK2X=0) CLKper/128.
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
pub mod tc45_bytem {
   /// 16-bit mode.
   pub const NORMAL: u32 = 0x0;
   /// Timer/Counter Operating in Byte Mode Only.
   pub const BYTEMODE: u32 = 0x1;
}

/// Compare or Capture Channel A Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_ccaintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Channel A Mode
#[allow(non_upper_case_globals)]
pub mod tc45_ccamode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Channel B Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_ccbintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Channel B Mode
#[allow(non_upper_case_globals)]
pub mod tc45_ccbmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Channel C Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_cccintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Channel C Mode
#[allow(non_upper_case_globals)]
pub mod tc45_cccmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Channel D Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_ccdintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Channel D Mode
#[allow(non_upper_case_globals)]
pub mod tc45_ccdmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Circular Enable Mode
#[allow(non_upper_case_globals)]
pub mod tc45_circen {
   /// Circular Buffer Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Circular Buffer Enabled on PER/PERBUF.
   pub const PER: u32 = 0x1;
   /// Circular Buffer Enabled on CCA/CCABUF.
   pub const CCA: u32 = 0x2;
   /// Circular Buffer Enabled on All Buffered Registers.
   pub const BOTH: u32 = 0x3;
}

/// Clock Selection
#[allow(non_upper_case_globals)]
pub mod tc45_clksel {
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
pub mod tc45_cmd {
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
pub mod tc45_errintlvl {
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
pub mod tc45_evact {
   /// No Event Action.
   pub const OFF: u32 = 0x0;
   /// Fault Mode 1 capture.
   pub const FMODE1: u32 = 0x1;
   /// Fault Mode 2 capture.
   pub const FMODE2: u32 = 0x2;
   /// Up/down count.
   pub const UPDOWN: u32 = 0x3;
   /// Quadrature decode.
   pub const QDEC: u32 = 0x4;
   /// Restart.
   pub const RESTART: u32 = 0x5;
   /// Pulse-width Capture.
   pub const PWF: u32 = 0x6;
}

/// Event Selection
#[allow(non_upper_case_globals)]
pub mod tc45_evsel {
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

/// Compare or Capture High Channel A Mode
#[allow(non_upper_case_globals)]
pub mod tc45_hccamode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture High Channel B Mode
#[allow(non_upper_case_globals)]
pub mod tc45_hccbmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture High Channel C Mode
#[allow(non_upper_case_globals)]
pub mod tc45_hcccmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture High Channel D Mode
#[allow(non_upper_case_globals)]
pub mod tc45_hccdmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Low Channel A Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_lccaintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Low Channel A Mode
#[allow(non_upper_case_globals)]
pub mod tc45_lccamode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Low Channel B Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_lccbintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Low Channel B Mode
#[allow(non_upper_case_globals)]
pub mod tc45_lccbmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Low Channel C Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_lcccintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Low Channel C Mode
#[allow(non_upper_case_globals)]
pub mod tc45_lcccmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Compare or Capture Low Channel D Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_lccdintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Compare or Capture Low Channel D Mode
#[allow(non_upper_case_globals)]
pub mod tc45_lccdmode {
   /// Channel Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Ouput Compare enabled.
   pub const COMP: u32 = 0x1;
   /// Input Capture enabled.
   pub const CAPT: u32 = 0x2;
   /// Both Compare and Capture enabled.
   pub const BOTHCC: u32 = 0x3;
}

/// Overflow Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_ovfintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

/// Timer Trigger Restart Interrupt Level
#[allow(non_upper_case_globals)]
pub mod tc45_trgintlvl {
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
pub mod tc45_wgmode {
   /// Normal Mode.
   pub const NORMAL: u32 = 0x0;
   /// Frequency Generation Mode.
   pub const FRQ: u32 = 0x1;
   /// Single Slope.
   pub const SINGLESLOPE: u32 = 0x3;
   /// Dual Slope, Update on TOP.
   pub const DSTOP: u32 = 0x5;
   /// Dual Slope, Both.
   pub const DSBOTH: u32 = 0x6;
   /// Dual Slope, Update on BOTTOM.
   pub const DSBOTTOM: u32 = 0x7;
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

/// Master/Slave Extend Timeout
#[allow(non_upper_case_globals)]
pub mod twi_master_tmsext {
   /// Tmext 10ms Tsext 25ms.
   pub const _10MS25MS: u32 = 0x0;
   /// Tmext 9ms  Tsext 24ms.
   pub const _9MS24MS: u32 = 0x1;
   /// Tmext 11ms Tsext 26ms.
   pub const _11MS26MS: u32 = 0x2;
   /// Tmext 12ms Tsext 27ms.
   pub const _12MS27MS: u32 = 0x3;
}

/// Master Timeout
#[allow(non_upper_case_globals)]
pub mod twi_master_ttimeout {
   /// 25 Milliseconds.
   pub const _25MS: u32 = 0x0;
   /// 24 Milliseconds.
   pub const _24MS: u32 = 0x1;
   /// 23 Milliseconds.
   pub const _23MS: u32 = 0x2;
   /// 22 Milliseconds.
   pub const _22MS: u32 = 0x3;
   /// 26 Milliseconds.
   pub const _26MS: u32 = 0x4;
   /// 27 Milliseconds.
   pub const _27MS: u32 = 0x5;
   /// 28 Milliseconds.
   pub const _28MS: u32 = 0x6;
   /// 29 Milliseconds.
   pub const _29MS: u32 = 0x7;
}

/// SDA Hold Time
#[allow(non_upper_case_globals)]
pub mod twi_sdahold {
   /// SDA Hold Time off.
   pub const OFF: u32 = 0x0;
   /// SDA Hold Time 50 ns.
   pub const _50NS: u32 = 0x1;
   /// SDA Hold Time 300 ns.
   pub const _300NS: u32 = 0x2;
   /// SDA Hold Time 400 ns.
   pub const _400NS: u32 = 0x3;
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

/// Slave Ttimeout
#[allow(non_upper_case_globals)]
pub mod twi_slave_ttimeout {
   /// 25 Milliseconds.
   pub const _25MS: u32 = 0x0;
   /// 24 Milliseconds.
   pub const _24MS: u32 = 0x1;
   /// 23 Milliseconds.
   pub const _23MS: u32 = 0x2;
   /// 22 Milliseconds.
   pub const _22MS: u32 = 0x3;
   /// 26 Milliseconds.
   pub const _26MS: u32 = 0x4;
   /// 27 Milliseconds.
   pub const _27MS: u32 = 0x5;
   /// 28 Milliseconds.
   pub const _28MS: u32 = 0x6;
   /// 29 Milliseconds.
   pub const _29MS: u32 = 0x7;
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

/// Encoding and Decoding Type
#[allow(non_upper_case_globals)]
pub mod usart_dectype {
   /// DATA Field Encoding.
   pub const DATA: u32 = 0x0;
   /// Start and Data Fields Encoding.
   pub const SDATA: u32 = 0x2;
   /// Start and Data Fields Encoding, with invertion in START field.
   pub const NOTSDATA: u32 = 0x3;
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

/// XCL LUT Action
#[allow(non_upper_case_globals)]
pub mod usart_lutact {
   /// Standard Frame Configuration.
   pub const OFF: u32 = 0x0;
   /// Receiver Decoding Enabled.
   pub const RX: u32 = 0x1;
   /// Transmitter Encoding Enabled.
   pub const TX: u32 = 0x2;
   /// Both Encoding and Decoding Enabled.
   pub const BOTH: u32 = 0x3;
}

/// XCL Peripheral Counter Action
#[allow(non_upper_case_globals)]
pub mod usart_pecact {
   /// Standard Mode.
   pub const OFF: u32 = 0x0;
   /// Variable Data Lenght in Reception.
   pub const PEC0: u32 = 0x1;
   /// Variable Data Lenght in Transmission.
   pub const PEC1: u32 = 0x2;
   /// Variable Data Lenght in both Reception and Transmission.
   pub const PERC01: u32 = 0x3;
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

/// Receive Start Interrupt level
#[allow(non_upper_case_globals)]
pub mod usart_rxsintlvl {
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

/// Output Matrix Mode
#[allow(non_upper_case_globals)]
pub mod wex_otmx {
   /// Default Ouput Matrix Mode.
   pub const DEFAULT: u32 = 0x0;
   /// First Output matrix Mode.
   pub const FIRST: u32 = 0x1;
   /// Second Output matrix Mode.
   pub const SECOND: u32 = 0x2;
   /// Third Output matrix Mode.
   pub const THIRD: u32 = 0x3;
   /// Fourth Output matrix Mode.
   pub const FOURTH: u32 = 0x4;
}

/// Compare/Capture Interrupt level
#[allow(non_upper_case_globals)]
pub mod xcl_cc_intlvl {
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
pub mod xcl_clksel {
   /// OFF.
   pub const OFF: u32 = 0x0;
   /// Prescaler clk.
   pub const DIV1: u32 = 0x1;
   /// Prescaler clk/2.
   pub const DIV2: u32 = 0x2;
   /// Prescaler clk/4.
   pub const DIV4: u32 = 0x3;
   /// Prescaler clk/8.
   pub const DIV8: u32 = 0x4;
   /// Prescaler clk/64.
   pub const DIV64: u32 = 0x5;
   /// Prescaler clk/256.
   pub const DIV256: u32 = 0x6;
   /// Prescaler clk/1024.
   pub const DIV1024: u32 = 0x7;
   /// Event channel 0.
   pub const EVCH0: u32 = 0x8;
   /// Event channel 1.
   pub const EVCH1: u32 = 0x9;
   /// Event channel 2.
   pub const EVCH2: u32 = 0xA;
   /// Event channel 3.
   pub const EVCH3: u32 = 0xB;
   /// Event channel 4.
   pub const EVCH4: u32 = 0xC;
   /// Event channel 5.
   pub const EVCH5: u32 = 0xD;
   /// Event channel 6.
   pub const EVCH6: u32 = 0xE;
   /// Event channel 7.
   pub const EVCH7: u32 = 0xF;
}

/// Command Enable
#[allow(non_upper_case_globals)]
pub mod xcl_cmden {
   /// Command Ignored.
   pub const DISABLE: u32 = 0x0;
   /// Command valid for timer/counter 0.
   pub const CMD0: u32 = 0x1;
   /// Command valid for timer/counter 1.
   pub const CMD1: u32 = 0x2;
   /// Command valid for both timer/counter 0 and 1.
   pub const CMD01: u32 = 0x3;
}

/// Timer/Counter Command Selection
#[allow(non_upper_case_globals)]
pub mod xcl_cmdsel {
   /// None.
   pub const NONE: u32 = 0x0;
   /// Force restart.
   pub const RESTART: u32 = 0x1;
}

/// Compare Output Value Timer
#[allow(non_upper_case_globals)]
pub mod xcl_cmpen {
   /// Clear WG Output.
   pub const CLEAR: u32 = 0x0;
   /// Set WG Output.
   pub const SET: u32 = 0x1;
}

/// Delay Configuration on LUT
#[allow(non_upper_case_globals)]
pub mod xcl_dlyconf {
   /// Delay element disabled.
   pub const DISABLE: u32 = 0x0;
   /// Delay enabled on LUT input.
   pub const IN: u32 = 0x1;
   /// Delay enabled on LUT output.
   pub const OUT: u32 = 0x2;
}

/// Delay Selection
#[allow(non_upper_case_globals)]
pub mod xcl_dlysel {
   /// One cycle delay for each LUT1 and LUT0.
   pub const DLY11: u32 = 0x0;
   /// One cycle delay for LUT1 and two cycles for LUT0.
   pub const DLY12: u32 = 0x1;
   /// Two cycles delay for LUT1 and one cycle for LUT0.
   pub const DLY21: u32 = 0x2;
   /// Two cycle delays for each LUT1 and LUT0.
   pub const DLY22: u32 = 0x3;
}

/// Timer/Counter Event Action Selection
#[allow(non_upper_case_globals)]
pub mod xcl_evact {
   /// Input Capture.
   pub const INPUT: u32 = 0x0;
   /// Frequency Capture.
   pub const FREQ: u32 = 0x1;
   /// Pulse Width Capture.
   pub const PW: u32 = 0x2;
   /// Restart timer/counter.
   pub const RESTART: u32 = 0x3;
}

/// Timer/Counter Event Source Selection
#[allow(non_upper_case_globals)]
pub mod xcl_evsrc {
   /// Event channel 0.
   pub const EVCH0: u32 = 0x0;
   /// Event channel 1.
   pub const EVCH1: u32 = 0x1;
   /// Event channel 2.
   pub const EVCH2: u32 = 0x2;
   /// Event channel 3.
   pub const EVCH3: u32 = 0x3;
   /// Event channel 4.
   pub const EVCH4: u32 = 0x4;
   /// Event channel 5.
   pub const EVCH5: u32 = 0x5;
   /// Event channel 6.
   pub const EVCH6: u32 = 0x6;
   /// Event channel 7.
   pub const EVCH7: u32 = 0x7;
}

/// Input Selection
#[allow(non_upper_case_globals)]
pub mod xcl_insel {
   /// Event system selected as source.
   pub const EVSYS: u32 = 0x0;
   /// XCL selected as source.
   pub const XCL: u32 = 0x1;
   /// LSB port pin selected as source.
   pub const PINL: u32 = 0x2;
   /// MSB port pin selected as source.
   pub const PINH: u32 = 0x3;
}

/// LUT0 Output Enable
#[allow(non_upper_case_globals)]
pub mod xcl_lut0outen {
   /// LUT0 output disabled.
   pub const DISABLE: u32 = 0x0;
   /// LUT0 Output to pin 0.
   pub const PIN0: u32 = 0x1;
   /// LUT0 Output to pin 4.
   pub const PIN4: u32 = 0x2;
}

/// LUT Configuration
#[allow(non_upper_case_globals)]
pub mod xcl_lutconf {
   /// 2-Input two LUT.
   pub const _2LUT2IN: u32 = 0x0;
   /// Two LUT with duplicated input.
   pub const _2LUT1IN: u32 = 0x1;
   /// Two LUT with one common input.
   pub const _2LUT3IN: u32 = 0x2;
   /// 3-Input LUT.
   pub const _1LUT3IN: u32 = 0x3;
   /// One LUT Mux.
   pub const MUX: u32 = 0x4;
   /// One D-Latch LUT.
   pub const DLATCH: u32 = 0x5;
   /// One RS-Latch LUT.
   pub const RSLATCH: u32 = 0x6;
   /// One DFF LUT.
   pub const DFF: u32 = 0x7;
}

/// Port Selection
#[allow(non_upper_case_globals)]
pub mod xcl_portsel {
   /// Port C for LUT or USARTC0 for PEC.
   pub const PC: u32 = 0x0;
   /// Port D for LUT or USARTD0 for PEC.
   pub const PD: u32 = 0x1;
}

/// Timer/Counter Mode
#[allow(non_upper_case_globals)]
pub mod xcl_tcmode {
   /// Normal mode with compare/period.
   pub const NORMAL: u32 = 0x0;
   /// Capture mode.
   pub const CAPT: u32 = 0x1;
   /// Single Slope PWM.
   pub const PWM: u32 = 0x2;
   /// One-shot PWM.
   pub const _1SHOT: u32 = 0x3;
}

/// Timer/Counter Selection
#[allow(non_upper_case_globals)]
pub mod xcl_tcsel {
   /// 16-bit timer/counter.
   pub const TC16: u32 = 0x0;
   /// One 8-bit timer/counter.
   pub const BTC0: u32 = 0x1;
   /// Two 8-bit timer/counters.
   pub const BTC01: u32 = 0x2;
   /// One 8-bit timer/counter and one 8-bit peripheral transmitter counter.
   pub const BTC0PEC1: u32 = 0x3;
   /// One 8-bit timer/counter and one 8-bit peripheral receiver counter.
   pub const PEC0BTC1: u32 = 0x4;
   /// Two 8-bit peripheral counters.
   pub const PEC01: u32 = 0x5;
   /// One 8-bit timer/counter and two 4-bit peripheral counters.
   pub const BTC0PEC2: u32 = 0x6;
}

/// Underflow Interrupt level
#[allow(non_upper_case_globals)]
pub mod xcl_unf_intlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
}

