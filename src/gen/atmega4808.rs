//! The AVR ATmega4808 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega4808-AFR | QFP32 | TQFP32 | -40°C - 105°C | 1.8V - 5V | 20 MHz |
//! | ATmega4808-MFR | QFN32 | VQFN32 | -40°C - 105°C | 1.8V - 5V | 20 MHz |
//! | ATmega4808-XFR | SSOP28 | SSOP28 | -40°C - 105°C | 1.8V - 5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

/// Channel Strobe.
pub const STROBE: *mut u8 = 0x0 as *mut u8;

/// Device ID Byte 0.
pub const DEVICEID0: *mut u8 = 0x0 as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x0 as *mut u8;

/// Reset Flags.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | EXTRF | 100 |
/// | SWRF | 10000 |
/// | UPDIRF | 100000 |
/// | BORF | 10 |
pub const RSTFR: *mut u8 = 0x0 as *mut u8;

/// MCLK Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOUT | 10000000 |
pub const MCLKCTRLA: *mut u8 = 0x0 as *mut u8;

/// Receive Data Low Byte.
pub const RXDATAL: *mut u8 = 0x0 as *mut u8;

/// Lock Bits.
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// Data Direction.
pub const DIR: *mut u8 = 0x0 as *mut u8;

/// User Row Byte 0.
pub const USERROW0: *mut u8 = 0x0 as *mut u8;

/// Port Multiplexer EVSYS.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVOUT5 | 100000 |
/// | EVOUT0 | 1 |
/// | EVOUT4 | 10000 |
/// | EVOUT2 | 100 |
/// | EVOUT3 | 1000 |
/// | EVOUT1 | 10 |
pub const EVSYSROUTEA: *mut u8 = 0x0 as *mut u8;

/// Control A.
pub const CTRLA: *mut u8 = 0x0 as *mut u8;

/// Watchdog Configuration.
pub const WDTCFG: *mut u8 = 0x0 as *mut u8;

/// Revision ID.
pub const REVID: *mut u8 = 0x1 as *mut u8;

/// MCLK Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PDIV | 11110 |
/// | PEN | 1 |
pub const MCLKCTRLB: *mut u8 = 0x1 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x1 as *mut u8;

/// Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC0REFEN | 10 |
/// | AC0REFEN | 1 |
pub const CTRLB: *mut u8 = 0x1 as *mut u8;

/// Data Direction Set.
pub const DIRSET: *mut u8 = 0x1 as *mut u8;

/// Receive Data High Byte.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BUFOVF | 1000000 |
/// | PERR | 10 |
/// | RXCIF | 10000000 |
/// | FERR | 100 |
pub const RXDATAH: *mut u8 = 0x1 as *mut u8;

/// BOD Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAMPFREQ | 10000 |
/// | ACTIVE | 1100 |
/// | SLEEP | 11 |
/// | LVL | 11100000 |
pub const BODCFG: *mut u8 = 0x1 as *mut u8;

/// Sequential Control 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEQSEL0 | 111 |
pub const SEQCTRL0: *mut u8 = 0x1 as *mut u8;

/// Software Reset.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SWRE | 1 |
pub const SWRR: *mut u8 = 0x1 as *mut u8;

/// Device ID Byte 1.
pub const DEVICEID1: *mut u8 = 0x1 as *mut u8;

/// Dual Control.
pub const DUALCTRL: *mut u8 = 0x1 as *mut u8;

/// Output Value.
pub const OUT: *mut u8 = 0x1 as *mut u8;

/// User Row Byte 1.
pub const USERROW1: *mut u8 = 0x1 as *mut u8;

/// Port Multiplexer CCL.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LUT1 | 10 |
/// | LUT0 | 1 |
/// | LUT3 | 1000 |
/// | LUT2 | 100 |
pub const CCLROUTEA: *mut u8 = 0x1 as *mut u8;

/// Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYNCBUSY | 1 |
pub const STATUS: *mut u8 = 0x1 as *mut u8;

/// Port Multiplexer USART register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USART0 | 11 |
/// | USART3 | 11000000 |
/// | USART2 | 110000 |
/// | USART1 | 1100 |
pub const USARTROUTEA: *mut u8 = 0x2 as *mut u8;

/// Interrupt Level 0 Priority.
pub const LVL0PRI: *mut u8 = 0x2 as *mut u8;

/// MCLK Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LOCKEN | 1 |
pub const MCLKLOCK: *mut u8 = 0x2 as *mut u8;

/// Sequential Control 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEQSEL1 | 111 |
pub const SEQCTRL1: *mut u8 = 0x2 as *mut u8;

/// Device ID Byte 2.
pub const DEVICEID2: *mut u8 = 0x2 as *mut u8;

/// Oscillator Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FREQSEL | 11 |
/// | OSCLOCK | 10000000 |
pub const OSCCFG: *mut u8 = 0x2 as *mut u8;

/// Input Value.
pub const IN: *mut u8 = 0x2 as *mut u8;

/// User Row Byte 2.
pub const USERROW2: *mut u8 = 0x2 as *mut u8;

/// Data Direction Clear.
pub const DIRCLR: *mut u8 = 0x2 as *mut u8;

/// Mux Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUXNEG | 11 |
/// | INVERT | 10000000 |
pub const MUXCTRLA: *mut u8 = 0x2 as *mut u8;

/// Transmit Data Low Byte.
pub const TXDATAL: *mut u8 = 0x2 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x2 as *mut u8;

/// External Break.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENEXTBRK | 1 |
pub const EXTBRK: *mut u8 = 0x2 as *mut u8;

/// Data Direction Toggle.
pub const DIRTGL: *mut u8 = 0x3 as *mut u8;

/// Port Multiplexer TWI and SPI.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI0 | 11 |
/// | TWI0 | 110000 |
pub const TWISPIROUTEA: *mut u8 = 0x3 as *mut u8;

/// Interrupt Flags.
pub const INTFLAGS: *mut u8 = 0x3 as *mut u8;

/// Interrupt Level 1 Priority Vector.
pub const LVL1VEC: *mut u8 = 0x3 as *mut u8;

/// Master Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | QCEN | 10000 |
/// | RIEN | 10000000 |
/// | WIEN | 1000000 |
/// | TIMEOUT | 1100 |
pub const MCTRLA: *mut u8 = 0x3 as *mut u8;

/// MCLK Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTS | 10000000 |
/// | OSC20MS | 10000 |
/// | OSC32KS | 100000 |
/// | XOSC32KS | 1000000 |
/// | SOSC | 1 |
pub const MCLKSTATUS: *mut u8 = 0x3 as *mut u8;

/// User Row Byte 3.
pub const USERROW3: *mut u8 = 0x3 as *mut u8;

/// Transmit Data High Byte.
pub const TXDATAH: *mut u8 = 0x3 as *mut u8;

/// Serial Number Byte 0.
pub const SERNUM0: *mut u8 = 0x3 as *mut u8;

/// General Purpose IO Register 3.
pub const GPIOR3: *mut u8 = 0x3 as *mut u8;

/// Port Multiplexer TCA.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCA0 | 111 |
pub const TCAROUTEA: *mut u8 = 0x4 as *mut u8;

/// Control E.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WINCM | 111 |
pub const CTRLE: *mut u8 = 0x4 as *mut u8;

/// Data.
pub const DATA: *mut u8 = 0x4 as *mut u8;

/// User Row Byte 4.
pub const USERROW4: *mut u8 = 0x4 as *mut u8;

/// Control E Clear.
pub const CTRLECLR: *mut u8 = 0x4 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x4 as *mut u8;

/// Master Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FLUSH | 1000 |
/// | MCMD | 11 |
pub const MCTRLB: *mut u8 = 0x4 as *mut u8;

/// Serial Number Byte 1.
pub const SERNUM1: *mut u8 = 0x4 as *mut u8;

/// Referance scale control.
pub const DACREF: *mut u8 = 0x4 as *mut u8;

/// Port Multiplexer TCB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCB0 | 1 |
/// | TCB2 | 100 |
/// | TCB1 | 10 |
/// | TCB3 | 1000 |
pub const TCBROUTEA: *mut u8 = 0x5 as *mut u8;

/// Interrupt Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CAPT | 1 |
pub const INTCTRL: *mut u8 = 0x5 as *mut u8;

/// User Row Byte 5.
pub const USERROW5: *mut u8 = 0x5 as *mut u8;

/// Output Value Set.
pub const OUTSET: *mut u8 = 0x5 as *mut u8;

/// System Configuration 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1 |
/// | CRCSRC | 11000000 |
/// | RSTPINCFG | 1000 |
pub const SYSCFG0: *mut u8 = 0x5 as *mut u8;

/// Serial Number Byte 2.
pub const SERNUM2: *mut u8 = 0x5 as *mut u8;

/// Control E Set.
pub const CTRLESET: *mut u8 = 0x5 as *mut u8;

/// Sample Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAMPLEN | 11111 |
pub const SAMPCTRL: *mut u8 = 0x5 as *mut u8;

/// Master Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BUSSTATE | 11 |
/// | WIF | 1000000 |
/// | RIF | 10000000 |
/// | ARBLOST | 1000 |
pub const MSTATUS: *mut u8 = 0x5 as *mut u8;

/// Interrupt Control 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTMODE0 | 11 |
/// | INTMODE3 | 11000000 |
/// | INTMODE2 | 110000 |
/// | INTMODE1 | 1100 |
pub const INTCTRL0: *mut u8 = 0x5 as *mut u8;

/// Output Value Clear.
pub const OUTCLR: *mut u8 = 0x6 as *mut u8;

/// Master Baurd Rate Control.
pub const MBAUD: *mut u8 = 0x6 as *mut u8;

/// User Row Byte 6.
pub const USERROW6: *mut u8 = 0x6 as *mut u8;

/// Positive mux input.
pub const MUXPOS: *mut u8 = 0x6 as *mut u8;

/// Calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SIGN | 10000000 |
/// | ERROR | 1111111 |
pub const CALIB: *mut u8 = 0x6 as *mut u8;

/// Serial Number Byte 3.
pub const SERNUM3: *mut u8 = 0x6 as *mut u8;

/// System Configuration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SUT | 111 |
pub const SYSCFG1: *mut u8 = 0x6 as *mut u8;

/// Control F Clear.
pub const CTRLFCLR: *mut u8 = 0x6 as *mut u8;

/// Application Code Section End.
pub const APPEND: *mut u8 = 0x7 as *mut u8;

/// Control F Set.
pub const CTRLFSET: *mut u8 = 0x7 as *mut u8;

/// Clock Select.
pub const CLKSEL: *mut u8 = 0x7 as *mut u8;

/// Output Value Toggle.
pub const OUTTGL: *mut u8 = 0x7 as *mut u8;

/// Serial Number Byte 4.
pub const SERNUM4: *mut u8 = 0x7 as *mut u8;

/// Control C.
pub const CTRLC: *mut u8 = 0x7 as *mut u8;

/// User Row Byte 7.
pub const USERROW7: *mut u8 = 0x7 as *mut u8;

/// Master Address.
pub const MADDR: *mut u8 = 0x7 as *mut u8;

/// Command.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STCONV | 1 |
pub const COMMAND: *mut u8 = 0x8 as *mut u8;

/// Baud Rate low byte.
pub const BAUDL: *mut u8 = 0x8 as *mut u8;

/// Address.
pub const ADDR: *mut u16 = 0x8 as *mut u16;

/// Address low byte.
pub const ADDRL: *mut u8 = 0x8 as *mut u8;

/// Boot Section End.
pub const BOOTEND: *mut u8 = 0x8 as *mut u8;

/// Serial Number Byte 5.
pub const SERNUM5: *mut u8 = 0x8 as *mut u8;

/// Voltage level monitor Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VLMLVL | 11 |
pub const VLMCTRLA: *mut u8 = 0x8 as *mut u8;

/// Baud Rate.
pub const BAUD: *mut u16 = 0x8 as *mut u16;

/// User Row Byte 8.
pub const USERROW8: *mut u8 = 0x8 as *mut u8;

/// LUT Control 0 A.
pub const LUT0CTRLA: *mut u8 = 0x8 as *mut u8;

/// Master Data.
pub const MDATA: *mut u8 = 0x8 as *mut u8;

/// Baud Rate high byte.
pub const BAUDH: *mut u8 = 0x9 as *mut u8;

/// User Row Byte 9.
pub const USERROW9: *mut u8 = 0x9 as *mut u8;

/// Serial Number Byte 6.
pub const SERNUM6: *mut u8 = 0x9 as *mut u8;

/// Temporary Value.
pub const TEMP: *mut u8 = 0x9 as *mut u8;

/// Address high byte.
pub const ADDRH: *mut u8 = 0x9 as *mut u8;

/// Slave Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PMEN | 100 |
/// | APIEN | 1000000 |
/// | PIEN | 100000 |
/// | DIEN | 10000000 |
pub const SCTRLA: *mut u8 = 0x9 as *mut u8;

/// LUT Control 0 B.
pub const LUT0CTRLB: *mut u8 = 0x9 as *mut u8;

/// Serial Number Byte 7.
pub const SERNUM7: *mut u8 = 0xA as *mut u8;

/// User Row Byte 10.
pub const USERROW10: *mut u8 = 0xA as *mut u8;

/// LUT Control 0 C.
pub const LUT0CTRLC: *mut u8 = 0xA as *mut u8;

/// Count low byte.
pub const CNTL: *mut u8 = 0xA as *mut u8;

/// Count.
pub const CNT: *mut u16 = 0xA as *mut u16;

/// Slave Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCMD | 11 |
pub const SCTRLB: *mut u8 = 0xA as *mut u8;

/// Port Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRL | 1 |
pub const PORTCTRL: *mut u8 = 0xA as *mut u8;

/// Control D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ABW | 11000000 |
pub const CTRLD: *mut u8 = 0xA as *mut u8;

/// Serial Number Byte 8.
pub const SERNUM8: *mut u8 = 0xB as *mut u8;

/// Slave Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | APIF | 1000000 |
/// | AP | 1 |
/// | DIF | 10000000 |
/// | COLL | 1000 |
pub const SSTATUS: *mut u8 = 0xB as *mut u8;

/// User Row Byte 11.
pub const USERROW11: *mut u8 = 0xB as *mut u8;

/// Count high byte.
pub const CNTH: *mut u8 = 0xB as *mut u8;

/// Debug Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ABMBP | 10000000 |
pub const DBGCTRL: *mut u8 = 0xB as *mut u8;

/// Truth 0.
pub const TRUTH0: *mut u8 = 0xB as *mut u8;

/// Compare or Capture.
pub const CCMP: *mut u16 = 0xC as *mut u16;

/// LUT Control 1 A.
pub const LUT1CTRLA: *mut u8 = 0xC as *mut u8;

/// Compare or Capture low byte.
pub const CCMPL: *mut u8 = 0xC as *mut u8;

/// Compare low byte.
pub const CMPL: *mut u8 = 0xC as *mut u8;

/// Serial Number Byte 9.
pub const SERNUM9: *mut u8 = 0xC as *mut u8;

/// Slave Address.
pub const SADDR: *mut u8 = 0xC as *mut u8;

/// User Row Byte 12.
pub const USERROW12: *mut u8 = 0xC as *mut u8;

/// Compare.
pub const CMP: *mut u16 = 0xC as *mut u16;

/// Event Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IREI | 1 |
pub const EVCTRL: *mut u8 = 0xC as *mut u8;

/// User Row Byte 13.
pub const USERROW13: *mut u8 = 0xD as *mut u8;

/// Compare high byte.
pub const CMPH: *mut u8 = 0xD as *mut u8;

/// Stack Pointer Low.
pub const SPL: *mut u8 = 0xD as *mut u8;

/// IRCOM Transmitter Pulse Length Control.
pub const TXPLCTRL: *mut u8 = 0xD as *mut u8;

/// Slave Data.
pub const SDATA: *mut u8 = 0xD as *mut u8;

/// Compare or Capture high byte.
pub const CCMPH: *mut u8 = 0xD as *mut u8;

/// LUT Control 1 B.
pub const LUT1CTRLB: *mut u8 = 0xD as *mut u8;

/// User Row Byte 14.
pub const USERROW14: *mut u8 = 0xE as *mut u8;

/// LUT Control 1 C.
pub const LUT1CTRLC: *mut u8 = 0xE as *mut u8;

/// IRCOM Receiver Pulse Length Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXPL | 1111111 |
pub const RXPLCTRL: *mut u8 = 0xE as *mut u8;

/// Slave Address Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADDREN | 1 |
/// | ADDRMASK | 11111110 |
pub const SADDRMASK: *mut u8 = 0xE as *mut u8;

/// Stack Pointer High.
pub const SPH: *mut u8 = 0xE as *mut u8;

/// Truth 1.
pub const TRUTH1: *mut u8 = 0xF as *mut u8;

/// User Row Byte 15.
pub const USERROW15: *mut u8 = 0xF as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | I | 10000000 |
/// | H | 100000 |
/// | N | 100 |
/// | Z | 10 |
/// | C | 1 |
/// | S | 10000 |
/// | V | 1000 |
/// | T | 1000000 |
pub const SREG: *mut u8 = 0xF as *mut u8;

/// OSC20M Control A.
pub const OSC20MCTRLA: *mut u8 = 0x10 as *mut u8;

/// User Row Byte 16.
pub const USERROW16: *mut u8 = 0x10 as *mut u8;

/// ADC Accumulator Result low byte.
pub const RESL: *mut u8 = 0x10 as *mut u8;

/// ADC Accumulator Result.
pub const RES: *mut u16 = 0x10 as *mut u16;

/// PIT Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PITEN | 1 |
pub const PITCTRLA: *mut u8 = 0x10 as *mut u8;

/// Pin 0 Control.
pub const PIN0CTRL: *mut u8 = 0x10 as *mut u8;

/// LUT Control 2 A.
pub const LUT2CTRLA: *mut u8 = 0x10 as *mut u8;

/// Multiplexer Channel 0.
pub const CHANNEL0: *mut u8 = 0x10 as *mut u8;

/// Multiplexer Channel 1.
pub const CHANNEL1: *mut u8 = 0x11 as *mut u8;

/// OSC20M Calibration A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CAL20M | 1111111 |
pub const OSC20MCALIBA: *mut u8 = 0x11 as *mut u8;

/// PIT Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTRLBUSY | 1 |
pub const PITSTATUS: *mut u8 = 0x11 as *mut u8;

/// ADC Accumulator Result high byte.
pub const RESH: *mut u8 = 0x11 as *mut u8;

/// User Row Byte 17.
pub const USERROW17: *mut u8 = 0x11 as *mut u8;

/// Pin 1 Control.
pub const PIN1CTRL: *mut u8 = 0x11 as *mut u8;

/// LUT Control 2 B.
pub const LUT2CTRLB: *mut u8 = 0x11 as *mut u8;

/// OSC20M Calibration B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TEMPCAL20M | 1111 |
pub const OSC20MCALIBB: *mut u8 = 0x12 as *mut u8;

/// LUT Control 2 C.
pub const LUT2CTRLC: *mut u8 = 0x12 as *mut u8;

/// Window comparator low threshold.
pub const WINLT: *mut u16 = 0x12 as *mut u16;

/// Window comparator low threshold low byte.
pub const WINLTL: *mut u8 = 0x12 as *mut u8;

/// User Row Byte 18.
pub const USERROW18: *mut u8 = 0x12 as *mut u8;

/// Multiplexer Channel 2.
pub const CHANNEL2: *mut u8 = 0x12 as *mut u8;

/// PIT Interrupt Control.
pub const PITINTCTRL: *mut u8 = 0x12 as *mut u8;

/// Pin 2 Control.
pub const PIN2CTRL: *mut u8 = 0x12 as *mut u8;

/// Pin 3 Control.
pub const PIN3CTRL: *mut u8 = 0x13 as *mut u8;

/// User Row Byte 19.
pub const USERROW19: *mut u8 = 0x13 as *mut u8;

/// PIT Interrupt Flags.
pub const PITINTFLAGS: *mut u8 = 0x13 as *mut u8;

/// Multiplexer Channel 3.
pub const CHANNEL3: *mut u8 = 0x13 as *mut u8;

/// Truth 2.
pub const TRUTH2: *mut u8 = 0x13 as *mut u8;

/// Window comparator low threshold high byte.
pub const WINLTH: *mut u8 = 0x13 as *mut u8;

/// Window comparator high threshold low byte.
pub const WINHTL: *mut u8 = 0x14 as *mut u8;

/// Window comparator high threshold.
pub const WINHT: *mut u16 = 0x14 as *mut u16;

/// Pin 4 Control.
pub const PIN4CTRL: *mut u8 = 0x14 as *mut u8;

/// Oscillator Calibration for 32kHz ULP.
pub const OSCCAL32K: *mut u8 = 0x14 as *mut u8;

/// User Row Byte 20.
pub const USERROW20: *mut u8 = 0x14 as *mut u8;

/// Multiplexer Channel 4.
pub const CHANNEL4: *mut u8 = 0x14 as *mut u8;

/// LUT Control 3 A.
pub const LUT3CTRLA: *mut u8 = 0x14 as *mut u8;

/// Multiplexer Channel 5.
pub const CHANNEL5: *mut u8 = 0x15 as *mut u8;

/// Window comparator high threshold high byte.
pub const WINHTH: *mut u8 = 0x15 as *mut u8;

/// LUT Control 3 B.
pub const LUT3CTRLB: *mut u8 = 0x15 as *mut u8;

/// Pin 5 Control.
pub const PIN5CTRL: *mut u8 = 0x15 as *mut u8;

/// User Row Byte 21.
pub const USERROW21: *mut u8 = 0x15 as *mut u8;

/// PIT Debug control.
pub const PITDBGCTRL: *mut u8 = 0x15 as *mut u8;

/// Pin 6 Control.
pub const PIN6CTRL: *mut u8 = 0x16 as *mut u8;

/// LUT Control 3 C.
pub const LUT3CTRLC: *mut u8 = 0x16 as *mut u8;

/// User Row Byte 22.
pub const USERROW22: *mut u8 = 0x16 as *mut u8;

/// Truth 3.
pub const TRUTH3: *mut u8 = 0x17 as *mut u8;

/// User Row Byte 23.
pub const USERROW23: *mut u8 = 0x17 as *mut u8;

/// Pin 7 Control.
pub const PIN7CTRL: *mut u8 = 0x17 as *mut u8;

/// Oscillator Calibration 16 MHz Byte 0.
pub const OSCCAL16M0: *mut u8 = 0x18 as *mut u8;

/// User Row Byte 24.
pub const USERROW24: *mut u8 = 0x18 as *mut u8;

/// OCD Message Register.
pub const OCDM: *mut u8 = 0x18 as *mut u8;

/// OSC32K Control A.
pub const OSC32KCTRLA: *mut u8 = 0x18 as *mut u8;

/// Oscillator Calibration 16 MHz Byte 1.
pub const OSCCAL16M1: *mut u8 = 0x19 as *mut u8;

/// OCD Message Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCDMR | 1 |
pub const OCDMS: *mut u8 = 0x19 as *mut u8;

/// User Row Byte 25.
pub const USERROW25: *mut u8 = 0x19 as *mut u8;

/// User Row Byte 26.
pub const USERROW26: *mut u8 = 0x1A as *mut u8;

/// Oscillator Calibration 20 MHz Byte 0.
pub const OSCCAL20M0: *mut u8 = 0x1A as *mut u8;

/// Oscillator Calibration 20 MHz Byte 1.
pub const OSCCAL20M1: *mut u8 = 0x1B as *mut u8;

/// User Row Byte 27.
pub const USERROW27: *mut u8 = 0x1B as *mut u8;

/// User Row Byte 28.
pub const USERROW28: *mut u8 = 0x1C as *mut u8;

/// XOSC32K Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CSUT | 110000 |
/// | SEL | 100 |
pub const XOSC32KCTRLA: *mut u8 = 0x1C as *mut u8;

/// User Row Byte 29.
pub const USERROW29: *mut u8 = 0x1D as *mut u8;

/// User Row Byte 30.
pub const USERROW30: *mut u8 = 0x1E as *mut u8;

/// User Row Byte 31.
pub const USERROW31: *mut u8 = 0x1F as *mut u8;

/// Temperature Sensor Calibration Byte 0.
pub const TEMPSENSE0: *mut u8 = 0x20 as *mut u8;

/// User CCL LUT0 Event A.
pub const USERCCLLUT0A: *mut u8 = 0x20 as *mut u8;

/// Low Count.
pub const LCNT: *mut u8 = 0x20 as *mut u8;

/// User Row Byte 32.
pub const USERROW32: *mut u8 = 0x20 as *mut u8;

/// High Count.
pub const HCNT: *mut u8 = 0x21 as *mut u8;

/// User Row Byte 33.
pub const USERROW33: *mut u8 = 0x21 as *mut u8;

/// User CCL LUT0 Event B.
pub const USERCCLLUT0B: *mut u8 = 0x21 as *mut u8;

/// Temperature Sensor Calibration Byte 1.
pub const TEMPSENSE1: *mut u8 = 0x21 as *mut u8;

/// OSC16 error at 3V.
pub const OSC16ERR3V: *mut u8 = 0x22 as *mut u8;

/// User CCL LUT1 Event A.
pub const USERCCLLUT1A: *mut u8 = 0x22 as *mut u8;

/// User Row Byte 34.
pub const USERROW34: *mut u8 = 0x22 as *mut u8;

/// User CCL LUT1 Event B.
pub const USERCCLLUT1B: *mut u8 = 0x23 as *mut u8;

/// OSC16 error at 5V.
pub const OSC16ERR5V: *mut u8 = 0x23 as *mut u8;

/// User Row Byte 35.
pub const USERROW35: *mut u8 = 0x23 as *mut u8;

/// OSC20 error at 3V.
pub const OSC20ERR3V: *mut u8 = 0x24 as *mut u8;

/// User CCL LUT2 Event A.
pub const USERCCLLUT2A: *mut u8 = 0x24 as *mut u8;

/// User Row Byte 36.
pub const USERROW36: *mut u8 = 0x24 as *mut u8;

/// User Row Byte 37.
pub const USERROW37: *mut u8 = 0x25 as *mut u8;

/// User CCL LUT2 Event B.
pub const USERCCLLUT2B: *mut u8 = 0x25 as *mut u8;

/// OSC20 error at 5V.
pub const OSC20ERR5V: *mut u8 = 0x25 as *mut u8;

/// User CCL LUT3 Event A.
pub const USERCCLLUT3A: *mut u8 = 0x26 as *mut u8;

/// Low Period.
pub const LPER: *mut u8 = 0x26 as *mut u8;

/// Period low byte.
pub const PERL: *mut u8 = 0x26 as *mut u8;

/// Period.
pub const PER: *mut u16 = 0x26 as *mut u16;

/// User Row Byte 38.
pub const USERROW38: *mut u8 = 0x26 as *mut u8;

/// Period high byte.
pub const PERH: *mut u8 = 0x27 as *mut u8;

/// High Period.
pub const HPER: *mut u8 = 0x27 as *mut u8;

/// User CCL LUT3 Event B.
pub const USERCCLLUT3B: *mut u8 = 0x27 as *mut u8;

/// User Row Byte 39.
pub const USERROW39: *mut u8 = 0x27 as *mut u8;

/// Compare 0.
pub const CMP0: *mut u16 = 0x28 as *mut u16;

/// User ADC0.
pub const USERADC0: *mut u8 = 0x28 as *mut u8;

/// User Row Byte 40.
pub const USERROW40: *mut u8 = 0x28 as *mut u8;

/// Low Compare.
pub const LCMP0: *mut u8 = 0x28 as *mut u8;

/// Compare 0 low byte.
pub const CMP0L: *mut u8 = 0x28 as *mut u8;

/// User Row Byte 41.
pub const USERROW41: *mut u8 = 0x29 as *mut u8;

/// High Compare.
pub const HCMP0: *mut u8 = 0x29 as *mut u8;

/// Compare 0 high byte.
pub const CMP0H: *mut u8 = 0x29 as *mut u8;

/// User EVOUT Port A.
pub const USEREVOUTA: *mut u8 = 0x29 as *mut u8;

/// Compare 1 low byte.
pub const CMP1L: *mut u8 = 0x2A as *mut u8;

/// User EVOUT Port B.
pub const USEREVOUTB: *mut u8 = 0x2A as *mut u8;

/// Compare 1.
pub const CMP1: *mut u16 = 0x2A as *mut u16;

/// User Row Byte 42.
pub const USERROW42: *mut u8 = 0x2A as *mut u8;

/// Low Compare.
pub const LCMP1: *mut u8 = 0x2A as *mut u8;

/// User Row Byte 43.
pub const USERROW43: *mut u8 = 0x2B as *mut u8;

/// High Compare.
pub const HCMP1: *mut u8 = 0x2B as *mut u8;

/// Compare 1 high byte.
pub const CMP1H: *mut u8 = 0x2B as *mut u8;

/// User EVOUT Port C.
pub const USEREVOUTC: *mut u8 = 0x2B as *mut u8;

/// Compare 2.
pub const CMP2: *mut u16 = 0x2C as *mut u16;

/// User EVOUT Port D.
pub const USEREVOUTD: *mut u8 = 0x2C as *mut u8;

/// Low Compare.
pub const LCMP2: *mut u8 = 0x2C as *mut u8;

/// User Row Byte 44.
pub const USERROW44: *mut u8 = 0x2C as *mut u8;

/// Compare 2 low byte.
pub const CMP2L: *mut u8 = 0x2C as *mut u8;

/// User EVOUT Port E.
pub const USEREVOUTE: *mut u8 = 0x2D as *mut u8;

/// Compare 2 high byte.
pub const CMP2H: *mut u8 = 0x2D as *mut u8;

/// User Row Byte 45.
pub const USERROW45: *mut u8 = 0x2D as *mut u8;

/// High Compare.
pub const HCMP2: *mut u8 = 0x2D as *mut u8;

/// User Row Byte 46.
pub const USERROW46: *mut u8 = 0x2E as *mut u8;

/// User EVOUT Port F.
pub const USEREVOUTF: *mut u8 = 0x2E as *mut u8;

/// CRC Checksum Byte 1.
pub const CHECKSUM1: *mut u8 = 0x2F as *mut u8;

/// User USART0.
pub const USERUSART0: *mut u8 = 0x2F as *mut u8;

/// User Row Byte 47.
pub const USERROW47: *mut u8 = 0x2F as *mut u8;

/// User Row Byte 48.
pub const USERROW48: *mut u8 = 0x30 as *mut u8;

/// User USART1.
pub const USERUSART1: *mut u8 = 0x30 as *mut u8;

/// User USART2.
pub const USERUSART2: *mut u8 = 0x31 as *mut u8;

/// User Row Byte 49.
pub const USERROW49: *mut u8 = 0x31 as *mut u8;

/// User USART3.
pub const USERUSART3: *mut u8 = 0x32 as *mut u8;

/// User Row Byte 50.
pub const USERROW50: *mut u8 = 0x32 as *mut u8;

/// User TCA0.
pub const USERTCA0: *mut u8 = 0x33 as *mut u8;

/// User Row Byte 51.
pub const USERROW51: *mut u8 = 0x33 as *mut u8;

/// User Row Byte 52.
pub const USERROW52: *mut u8 = 0x34 as *mut u8;

/// User TCB0.
pub const USERTCB0: *mut u8 = 0x34 as *mut u8;

/// User TCB1.
pub const USERTCB1: *mut u8 = 0x35 as *mut u8;

/// User Row Byte 53.
pub const USERROW53: *mut u8 = 0x35 as *mut u8;

/// User TCB2.
pub const USERTCB2: *mut u8 = 0x36 as *mut u8;

/// Period Buffer.
pub const PERBUF: *mut u16 = 0x36 as *mut u16;

/// User Row Byte 54.
pub const USERROW54: *mut u8 = 0x36 as *mut u8;

/// Period Buffer low byte.
pub const PERBUFL: *mut u8 = 0x36 as *mut u8;

/// Period Buffer high byte.
pub const PERBUFH: *mut u8 = 0x37 as *mut u8;

/// User Row Byte 55.
pub const USERROW55: *mut u8 = 0x37 as *mut u8;

/// User TCB3.
pub const USERTCB3: *mut u8 = 0x37 as *mut u8;

/// User Row Byte 56.
pub const USERROW56: *mut u8 = 0x38 as *mut u8;

/// Compare 0 Buffer low byte.
pub const CMP0BUFL: *mut u8 = 0x38 as *mut u8;

/// Compare 0 Buffer.
pub const CMP0BUF: *mut u16 = 0x38 as *mut u16;

/// User Row Byte 57.
pub const USERROW57: *mut u8 = 0x39 as *mut u8;

/// Compare 0 Buffer high byte.
pub const CMP0BUFH: *mut u8 = 0x39 as *mut u8;

/// Compare 1 Buffer low byte.
pub const CMP1BUFL: *mut u8 = 0x3A as *mut u8;

/// Compare 1 Buffer.
pub const CMP1BUF: *mut u16 = 0x3A as *mut u16;

/// User Row Byte 58.
pub const USERROW58: *mut u8 = 0x3A as *mut u8;

/// Compare 1 Buffer high byte.
pub const CMP1BUFH: *mut u8 = 0x3B as *mut u8;

/// User Row Byte 59.
pub const USERROW59: *mut u8 = 0x3B as *mut u8;

/// Compare 2 Buffer.
pub const CMP2BUF: *mut u16 = 0x3C as *mut u16;

/// Compare 2 Buffer low byte.
pub const CMP2BUFL: *mut u8 = 0x3C as *mut u8;

/// User Row Byte 60.
pub const USERROW60: *mut u8 = 0x3C as *mut u8;

/// User Row Byte 61.
pub const USERROW61: *mut u8 = 0x3D as *mut u8;

/// Compare 2 Buffer high byte.
pub const CMP2BUFH: *mut u8 = 0x3D as *mut u8;

/// User Row Byte 62.
pub const USERROW62: *mut u8 = 0x3E as *mut u8;

/// User Row Byte 63.
pub const USERROW63: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `BODCFG`
pub const SAMPFREQ: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BODCFG`
pub const ACTIVE: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `BODCFG`
pub const SLEEP: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `BODCFG`
pub const LVL: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `CALIB`
pub const SIGN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CALIB`
pub const ERROR: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `CCLROUTEA`
pub const LUT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CCLROUTEA`
pub const LUT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CCLROUTEA`
pub const LUT3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CCLROUTEA`
pub const LUT2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `COMMAND`
pub const STCONV: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLB`
pub const ADC0REFEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLB`
pub const AC0REFEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLD`
pub const ABW: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `CTRLE`
pub const WINCM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `DBGCTRL`
pub const ABMBP: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EVCTRL`
pub const IREI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EVSYSROUTEA`
pub const EVOUT5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EVSYSROUTEA`
pub const EVOUT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EVSYSROUTEA`
pub const EVOUT4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EVSYSROUTEA`
pub const EVOUT2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EVSYSROUTEA`
pub const EVOUT3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EVSYSROUTEA`
pub const EVOUT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EXTBRK`
pub const ENEXTBRK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const CAPT: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `INTCTRL0`
pub const INTMODE0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRL0`
pub const INTMODE3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `INTCTRL0`
pub const INTMODE2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `INTCTRL0`
pub const INTMODE1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MCLKCTRLA`
pub const CLKOUT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCLKCTRLB`
pub const PDIV: *mut u8 = 0x1E as *mut u8;

/// Bitfield on register `MCLKCTRLB`
pub const PEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCLKLOCK`
pub const LOCKEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const EXTS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const OSC20MS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const OSC32KS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const XOSC32KS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const SOSC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const QCEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const RIEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const WIEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const TIMEOUT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MCTRLB`
pub const FLUSH: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCTRLB`
pub const MCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const BUSSTATE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const WIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const RIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const ARBLOST: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MUXCTRLA`
pub const MUXNEG: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MUXCTRLA`
pub const INVERT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `OCDMS`
pub const OCDMR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `OSC20MCALIBA`
pub const CAL20M: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `OSC20MCALIBB`
pub const TEMPCAL20M: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `OSCCFG`
pub const FREQSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `OSCCFG`
pub const OSCLOCK: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PITCTRLA`
pub const PITEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PITSTATUS`
pub const CTRLBUSY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PORTCTRL`
pub const SRL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSTFR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFR`
pub const EXTRF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSTFR`
pub const SWRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSTFR`
pub const UPDIRF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSTFR`
pub const BORF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const BUFOVF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const PERR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const RXCIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const FERR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RXPLCTRL`
pub const RXPL: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `SADDRMASK`
pub const ADDREN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SADDRMASK`
pub const ADDRMASK: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `SAMPCTRL`
pub const SAMPLEN: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SCTRLA`
pub const PMEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SCTRLA`
pub const APIEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SCTRLA`
pub const PIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SCTRLA`
pub const DIEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SCTRLB`
pub const SCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SEQCTRL0`
pub const SEQSEL0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SEQCTRL1`
pub const SEQSEL1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const APIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const AP: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const DIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const COLL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `STATUS`
pub const SYNCBUSY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SWRR`
pub const SWRE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SYSCFG0`
pub const EESAVE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SYSCFG0`
pub const CRCSRC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `SYSCFG0`
pub const RSTPINCFG: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SYSCFG1`
pub const SUT: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCAROUTEA`
pub const TCA0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCBROUTEA`
pub const TCB0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCBROUTEA`
pub const TCB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCBROUTEA`
pub const TCB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCBROUTEA`
pub const TCB3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWISPIROUTEA`
pub const SPI0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWISPIROUTEA`
pub const TWI0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `USARTROUTEA`
pub const USART0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `USARTROUTEA`
pub const USART3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `USARTROUTEA`
pub const USART2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `USARTROUTEA`
pub const USART1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `VLMCTRLA`
pub const VLMLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `XOSC32KCTRLA`
pub const CSUT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `XOSC32KCTRLA`
pub const SEL: *mut u8 = 0x4 as *mut u8;

/// Hysteresis Mode select
#[allow(non_upper_case_globals)]
pub mod ac_hysmode {
   /// No hysteresis.
   pub const OFF: u32 = 0x0;
   /// 10mV hysteresis.
   pub const _10mV: u32 = 0x1;
   /// 25mV hysteresis.
   pub const _25mV: u32 = 0x2;
   /// 50mV hysteresis.
   pub const _50mV: u32 = 0x3;
}

/// Interrupt Mode select
#[allow(non_upper_case_globals)]
pub mod ac_intmode {
   /// Any Edge.
   pub const BOTHEDGE: u32 = 0x0;
   /// Negative Edge.
   pub const NEGEDGE: u32 = 0x2;
   /// Positive Edge.
   pub const POSEDGE: u32 = 0x3;
}

/// Low Power Mode select
#[allow(non_upper_case_globals)]
pub mod ac_lpmode {
   /// Low power mode disabled.
   pub const DIS: u32 = 0x0;
   /// Low power mode enabled.
   pub const EN: u32 = 0x1;
}

/// Negative Input MUX Selection select
#[allow(non_upper_case_globals)]
pub mod ac_muxneg {
   /// Negative Pin 0.
   pub const PIN0: u32 = 0x0;
   /// Negative Pin 1.
   pub const PIN1: u32 = 0x1;
   /// Negative Pin 2.
   pub const PIN2: u32 = 0x2;
   /// DAC Voltage Reference.
   pub const DACREF: u32 = 0x3;
}

/// Positive Input MUX Selection select
#[allow(non_upper_case_globals)]
pub mod ac_muxpos {
   /// Positive Pin 0.
   pub const PIN0: u32 = 0x0;
   /// Positive Pin 1.
   pub const PIN1: u32 = 0x1;
   /// Positive Pin 2.
   pub const PIN2: u32 = 0x2;
   /// Positive Pin 3.
   pub const PIN3: u32 = 0x3;
}

/// Automatic Sampling Delay Variation select
#[allow(non_upper_case_globals)]
pub mod adc_asdv {
   /// The Automatic Sampling Delay Variation is disabled.
   pub const ASVOFF: u32 = 0x0;
   /// The Automatic Sampling Delay Variation is enabled.
   pub const ASVON: u32 = 0x1;
}

/// Duty Cycle select
#[allow(non_upper_case_globals)]
pub mod adc_dutycyc {
   /// 50% Duty cycle.
   pub const DUTY50: u32 = 0x0;
   /// 25% Duty cycle.
   pub const DUTY25: u32 = 0x1;
}

/// Initial Delay Selection select
#[allow(non_upper_case_globals)]
pub mod adc_initdly {
   /// Delay 0 CLK_ADC cycles.
   pub const DLY0: u32 = 0x0;
   /// Delay 16 CLK_ADC cycles.
   pub const DLY16: u32 = 0x1;
   /// Delay 32 CLK_ADC cycles.
   pub const DLY32: u32 = 0x2;
   /// Delay 64 CLK_ADC cycles.
   pub const DLY64: u32 = 0x3;
   /// Delay 128 CLK_ADC cycles.
   pub const DLY128: u32 = 0x4;
   /// Delay 256 CLK_ADC cycles.
   pub const DLY256: u32 = 0x5;
}

/// Analog Channel Selection Bits select
#[allow(non_upper_case_globals)]
pub mod adc_muxpos {
   /// ADC input pin 0.
   pub const AIN0: u32 = 0x0;
   /// ADC input pin 1.
   pub const AIN1: u32 = 0x1;
   /// ADC input pin 2.
   pub const AIN2: u32 = 0x2;
   /// ADC input pin 3.
   pub const AIN3: u32 = 0x3;
   /// ADC input pin 4.
   pub const AIN4: u32 = 0x4;
   /// ADC input pin 5.
   pub const AIN5: u32 = 0x5;
   /// ADC input pin 6.
   pub const AIN6: u32 = 0x6;
   /// ADC input pin 7.
   pub const AIN7: u32 = 0x7;
   /// ADC input pin 8.
   pub const AIN8: u32 = 0x8;
   /// ADC input pin 9.
   pub const AIN9: u32 = 0x9;
   /// ADC input pin 10.
   pub const AIN10: u32 = 0xA;
   /// ADC input pin 11.
   pub const AIN11: u32 = 0xB;
   /// ADC input pin 12.
   pub const AIN12: u32 = 0xC;
   /// ADC input pin 13.
   pub const AIN13: u32 = 0xD;
   /// ADC input pin 14.
   pub const AIN14: u32 = 0xE;
   /// ADC input pin 15.
   pub const AIN15: u32 = 0xF;
   /// AC DAC Reference.
   pub const DACREF: u32 = 0x1C;
   /// Temperature sensor.
   pub const TEMPSENSE: u32 = 0x1E;
   /// 0V (GND).
   pub const GND: u32 = 0x1F;
}

/// Clock Pre-scaler select
#[allow(non_upper_case_globals)]
pub mod adc_presc {
   /// CLK_PER divided by 2.
   pub const DIV2: u32 = 0x0;
   /// CLK_PER divided by 4.
   pub const DIV4: u32 = 0x1;
   /// CLK_PER divided by 8.
   pub const DIV8: u32 = 0x2;
   /// CLK_PER divided by 16.
   pub const DIV16: u32 = 0x3;
   /// CLK_PER divided by 32.
   pub const DIV32: u32 = 0x4;
   /// CLK_PER divided by 64.
   pub const DIV64: u32 = 0x5;
   /// CLK_PER divided by 128.
   pub const DIV128: u32 = 0x6;
   /// CLK_PER divided by 256.
   pub const DIV256: u32 = 0x7;
}

/// Reference Selection select
#[allow(non_upper_case_globals)]
pub mod adc_refsel {
   /// Internal reference.
   pub const INTREF: u32 = 0x0;
   /// VDD.
   pub const VDDREF: u32 = 0x1;
   /// External reference.
   pub const VREFA: u32 = 0x2;
}

/// ADC Resolution select
#[allow(non_upper_case_globals)]
pub mod adc_ressel {
   /// 10-bit mode.
   pub const _10BIT: u32 = 0x0;
   /// 8-bit mode.
   pub const _8BIT: u32 = 0x1;
}

/// Accumulation Samples select
#[allow(non_upper_case_globals)]
pub mod adc_sampnum {
   /// 1 ADC sample.
   pub const ACC1: u32 = 0x0;
   /// Accumulate 2 samples.
   pub const ACC2: u32 = 0x1;
   /// Accumulate 4 samples.
   pub const ACC4: u32 = 0x2;
   /// Accumulate 8 samples.
   pub const ACC8: u32 = 0x3;
   /// Accumulate 16 samples.
   pub const ACC16: u32 = 0x4;
   /// Accumulate 32 samples.
   pub const ACC32: u32 = 0x5;
   /// Accumulate 64 samples.
   pub const ACC64: u32 = 0x6;
}

/// Window Comparator Mode select
#[allow(non_upper_case_globals)]
pub mod adc_wincm {
   /// No Window Comparison.
   pub const NONE: u32 = 0x0;
   /// Below Window.
   pub const BELOW: u32 = 0x1;
   /// Above Window.
   pub const ABOVE: u32 = 0x2;
   /// Inside Window.
   pub const INSIDE: u32 = 0x3;
   /// Outside Window.
   pub const OUTSIDE: u32 = 0x4;
}

/// Operation in active mode select
#[allow(non_upper_case_globals)]
pub mod bod_active {
   /// Disabled.
   pub const DIS: u32 = 0x0;
   /// Enabled.
   pub const ENABLED: u32 = 0x1;
   /// Sampled.
   pub const SAMPLED: u32 = 0x2;
   /// Enabled with wake-up halted until BOD is ready.
   pub const ENWAKE: u32 = 0x3;
}

/// Bod level select
#[allow(non_upper_case_globals)]
pub mod bod_lvl {
   /// 1.8 V.
   pub const BODLEVEL0: u32 = 0x0;
   /// 2.6 V.
   pub const BODLEVEL2: u32 = 0x2;
   /// 4.2 V.
   pub const BODLEVEL7: u32 = 0x7;
}

/// Sample frequency select
#[allow(non_upper_case_globals)]
pub mod bod_sampfreq {
   /// 1kHz sampling frequency.
   pub const _1KHZ: u32 = 0x0;
   /// 125Hz sampling frequency.
   pub const _125HZ: u32 = 0x1;
}

/// Operation in sleep mode select
#[allow(non_upper_case_globals)]
pub mod bod_sleep {
   /// Disabled.
   pub const DIS: u32 = 0x0;
   /// Enabled.
   pub const ENABLED: u32 = 0x1;
   /// Sampled.
   pub const SAMPLED: u32 = 0x2;
}

/// Configuration select
#[allow(non_upper_case_globals)]
pub mod bod_vlmcfg {
   /// Interrupt when supply goes below VLM level.
   pub const BELOW: u32 = 0x0;
   /// Interrupt when supply goes above VLM level.
   pub const ABOVE: u32 = 0x1;
   /// Interrupt when supply crosses VLM level.
   pub const CROSS: u32 = 0x2;
}

/// voltage level monitor level select
#[allow(non_upper_case_globals)]
pub mod bod_vlmlvl {
   /// VLM threshold 5% above BOD level.
   pub const _5ABOVE: u32 = 0x0;
   /// VLM threshold 15% above BOD level.
   pub const _15ABOVE: u32 = 0x1;
   /// VLM threshold 25% above BOD level.
   pub const _25ABOVE: u32 = 0x2;
}

/// Clock Source Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_clksrc {
   /// CLK_PER is clocking the LUT.
   pub const CLKPER: u32 = 0x0;
   /// IN\[2\] is clocking the LUT.
   pub const IN2: u32 = 0x1;
   /// 20MHz oscillator before prescaler is clocking the LUT.
   pub const OSC20M: u32 = 0x4;
   /// 32kHz oscillator is clocking the LUT.
   pub const OSCULP32K: u32 = 0x5;
   /// 32kHz oscillator after DIV32 is clocking the LUT.
   pub const OSCULP1K: u32 = 0x6;
}

/// Edge Detection Enable select
#[allow(non_upper_case_globals)]
pub mod ccl_edgedet {
   /// Edge detector is disabled.
   pub const DIS: u32 = 0x0;
   /// Edge detector is enabled.
   pub const EN: u32 = 0x1;
}

/// Filter Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_filtsel {
   /// Filter disabled.
   pub const DISABLE: u32 = 0x0;
   /// Synchronizer enabled.
   pub const SYNCH: u32 = 0x1;
   /// Filter enabled.
   pub const FILTER: u32 = 0x2;
}

/// LUT Input 0 Source Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_insel0 {
   /// Masked input.
   pub const MASK: u32 = 0x0;
   /// Feedback input source.
   pub const FEEDBACK: u32 = 0x1;
   /// Linked LUT input source.
   pub const LINK: u32 = 0x2;
   /// Event input source A.
   pub const EVENTA: u32 = 0x3;
   /// Event input source B.
   pub const EVENTB: u32 = 0x4;
   /// IO pin LUTn-IN0 input source.
   pub const IO: u32 = 0x5;
   /// AC0 OUT input source.
   pub const AC0: u32 = 0x6;
   /// USART0 TXD input source.
   pub const USART0: u32 = 0x8;
   /// SPI0 MOSI input source.
   pub const SPI0: u32 = 0x9;
   /// TCA0 WO0 input source.
   pub const TCA0: u32 = 0xA;
   /// TCB0 WO input source.
   pub const TCB0: u32 = 0xC;
}

/// LUT Input 1 Source Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_insel1 {
   /// Masked input.
   pub const MASK: u32 = 0x0;
   /// Feedback input source.
   pub const FEEDBACK: u32 = 0x1;
   /// Linked LUT input source.
   pub const LINK: u32 = 0x2;
   /// Event input source A.
   pub const EVENTA: u32 = 0x3;
   /// Event input source B.
   pub const EVENTB: u32 = 0x4;
   /// IO pin LUTn-N1 input source.
   pub const IO: u32 = 0x5;
   /// AC0 OUT input source.
   pub const AC0: u32 = 0x6;
   /// USART1 TXD input source.
   pub const USART1: u32 = 0x8;
   /// SPI0 MOSI input source.
   pub const SPI0: u32 = 0x9;
   /// TCA0 WO1 input source.
   pub const TCA0: u32 = 0xA;
   /// TCB1 WO input source.
   pub const TCB1: u32 = 0xC;
}

/// LUT Input 2 Source Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_insel2 {
   /// Masked input.
   pub const MASK: u32 = 0x0;
   /// Feedback input source.
   pub const FEEDBACK: u32 = 0x1;
   /// Linked LUT input source.
   pub const LINK: u32 = 0x2;
   /// Event input source A.
   pub const EVENTA: u32 = 0x3;
   /// Event input source B.
   pub const EVENTB: u32 = 0x4;
   /// IO pin LUTn-IN2 input source.
   pub const IO: u32 = 0x5;
   /// AC0 OUT input source.
   pub const AC0: u32 = 0x6;
   /// USART2 TXD input source.
   pub const USART2: u32 = 0x8;
   /// SPI0 SCK input source.
   pub const SPI0: u32 = 0x9;
   /// TCA0 WO2 input source.
   pub const TCA0: u32 = 0xA;
   /// TCB2 WO input source.
   pub const TCB2: u32 = 0xC;
}

/// Interrupt Mode for LUT0 select
#[allow(non_upper_case_globals)]
pub mod ccl_intmode0 {
   /// Interrupt disabled.
   pub const INTDISABLE: u32 = 0x0;
   /// Sense rising edge.
   pub const RISING: u32 = 0x1;
   /// Sense falling edge.
   pub const FALLING: u32 = 0x2;
   /// Sense both edges.
   pub const BOTH: u32 = 0x3;
}

/// Interrupt Mode for LUT1 select
#[allow(non_upper_case_globals)]
pub mod ccl_intmode1 {
   /// Interrupt disabled.
   pub const INTDISABLE: u32 = 0x0;
   /// Sense rising edge.
   pub const RISING: u32 = 0x1;
   /// Sense falling edge.
   pub const FALLING: u32 = 0x2;
   /// Sense both edges.
   pub const BOTH: u32 = 0x3;
}

/// Interrupt Mode for LUT2 select
#[allow(non_upper_case_globals)]
pub mod ccl_intmode2 {
   /// Interrupt disabled.
   pub const INTDISABLE: u32 = 0x0;
   /// Sense rising edge.
   pub const RISING: u32 = 0x1;
   /// Sense falling edge.
   pub const FALLING: u32 = 0x2;
   /// Sense both edges.
   pub const BOTH: u32 = 0x3;
}

/// Interrupt Mode for LUT3 select
#[allow(non_upper_case_globals)]
pub mod ccl_intmode3 {
   /// Interrupt disabled.
   pub const INTDISABLE: u32 = 0x0;
   /// Sense rising edge.
   pub const RISING: u32 = 0x1;
   /// Sense falling edge.
   pub const FALLING: u32 = 0x2;
   /// Sense both edges.
   pub const BOTH: u32 = 0x3;
}

/// Sequential Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_seqsel0 {
   /// Sequential logic disabled.
   pub const DISABLE: u32 = 0x0;
   /// D FlipFlop.
   pub const DFF: u32 = 0x1;
   /// JK FlipFlop.
   pub const JK: u32 = 0x2;
   /// D Latch.
   pub const LATCH: u32 = 0x3;
   /// RS Latch.
   pub const RS: u32 = 0x4;
}

/// Sequential Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_seqsel1 {
   /// Sequential logic disabled.
   pub const DISABLE: u32 = 0x0;
   /// D FlipFlop.
   pub const DFF: u32 = 0x1;
   /// JK FlipFlop.
   pub const JK: u32 = 0x2;
   /// D Latch.
   pub const LATCH: u32 = 0x3;
   /// RS Latch.
   pub const RS: u32 = 0x4;
}

/// clock select select
#[allow(non_upper_case_globals)]
pub mod clkctrl_clksel {
   /// 20MHz oscillator.
   pub const OSC20M: u32 = 0x0;
   /// 32KHz oscillator.
   pub const OSCULP32K: u32 = 0x1;
   /// 32.768kHz crystal oscillator.
   pub const XOSC32K: u32 = 0x2;
   /// External clock.
   pub const EXTCLK: u32 = 0x3;
}

/// Crystal startup time select
#[allow(non_upper_case_globals)]
pub mod clkctrl_csut {
   /// 1k cycles.
   pub const _1K: u32 = 0x0;
   /// 16k cycles.
   pub const _16K: u32 = 0x1;
   /// 32k cycles.
   pub const _32K: u32 = 0x2;
   /// 64k cycles.
   pub const _64K: u32 = 0x3;
}

/// Prescaler division select
#[allow(non_upper_case_globals)]
pub mod clkctrl_pdiv {
   /// 2X.
   pub const _2X: u32 = 0x0;
   /// 4X.
   pub const _4X: u32 = 0x1;
   /// 8X.
   pub const _8X: u32 = 0x2;
   /// 16X.
   pub const _16X: u32 = 0x3;
   /// 32X.
   pub const _32X: u32 = 0x4;
   /// 64X.
   pub const _64X: u32 = 0x5;
   /// 6X.
   pub const _6X: u32 = 0x8;
   /// 10X.
   pub const _10X: u32 = 0x9;
   /// 12X.
   pub const _12X: u32 = 0xA;
   /// 24X.
   pub const _24X: u32 = 0xB;
   /// 48X.
   pub const _48X: u32 = 0xC;
}

/// CCP signature select
#[allow(non_upper_case_globals)]
pub mod cpu_ccp {
   /// SPM Instruction Protection.
   pub const SPM: u32 = 0x9D;
   /// IO Register Protection.
   pub const IOREG: u32 = 0xD8;
}

/// CRC Source select
#[allow(non_upper_case_globals)]
pub mod crcscan_src {
   /// CRC on entire flash.
   pub const FLASH: u32 = 0x0;
   /// CRC on boot and appl section of flash.
   pub const APPLICATION: u32 = 0x1;
   /// CRC on boot section of flash.
   pub const BOOT: u32 = 0x2;
}

/// Channel selector select
#[allow(non_upper_case_globals)]
pub mod evsys_channel {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Connect user to event channel 0.
   pub const CHANNEL0: u32 = 0x1;
   /// Connect user to event channel 1.
   pub const CHANNEL1: u32 = 0x2;
   /// Connect user to event channel 2.
   pub const CHANNEL2: u32 = 0x3;
   /// Connect user to event channel 3.
   pub const CHANNEL3: u32 = 0x4;
   /// Connect user to event channel 4.
   pub const CHANNEL4: u32 = 0x5;
   /// Connect user to event channel 5.
   pub const CHANNEL5: u32 = 0x6;
}

/// Generator selector select
#[allow(non_upper_case_globals)]
pub mod evsys_generator {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Unified Program and Debug Interface.
   pub const UPDI: u32 = 0x1;
   /// Real Time Counter overflow.
   pub const RTC_OVF: u32 = 0x6;
   /// Real Time Counter compare.
   pub const RTC_CMP: u32 = 0x7;
   /// Periodic Interrupt Timer output 0.
   pub const RTC_PIT0: u32 = 0x8;
   /// Periodic Interrupt Timer output 1.
   pub const RTC_PIT1: u32 = 0x9;
   /// Periodic Interrupt Timer output 2.
   pub const RTC_PIT2: u32 = 0xA;
   /// Periodic Interrupt Timer output 3.
   pub const RTC_PIT3: u32 = 0xB;
   /// Configurable Custom Logic LUT0.
   pub const CCL_LUT0: u32 = 0x10;
   /// Configurable Custom Logic LUT1.
   pub const CCL_LUT1: u32 = 0x11;
   /// Configurable Custom Logic LUT2.
   pub const CCL_LUT2: u32 = 0x12;
   /// Configurable Custom Logic LUT3.
   pub const CCL_LUT3: u32 = 0x13;
   /// Analog Comparator 0 out.
   pub const AC0_OUT: u32 = 0x20;
   /// ADC 0 Result Ready Event.
   pub const ADC0_RESRDY: u32 = 0x24;
   /// Port 0 Pin 0.
   pub const PORT0_PIN0: u32 = 0x40;
   /// Port 0 Pin 1.
   pub const PORT0_PIN1: u32 = 0x41;
   /// Port 0 Pin 2.
   pub const PORT0_PIN2: u32 = 0x42;
   /// Port 0 Pin 3.
   pub const PORT0_PIN3: u32 = 0x43;
   /// Port 0 Pin 4.
   pub const PORT0_PIN4: u32 = 0x44;
   /// Port 0 Pin 5.
   pub const PORT0_PIN5: u32 = 0x45;
   /// Port 0 Pin 6.
   pub const PORT0_PIN6: u32 = 0x46;
   /// Port 0 Pin 7.
   pub const PORT0_PIN7: u32 = 0x47;
   /// Port 1 Pin 0.
   pub const PORT1_PIN0: u32 = 0x48;
   /// Port 1 Pin 1.
   pub const PORT1_PIN1: u32 = 0x49;
   /// Port 1 Pin 2.
   pub const PORT1_PIN2: u32 = 0x4A;
   /// Port 1 Pin 3.
   pub const PORT1_PIN3: u32 = 0x4B;
   /// Port 1 Pin 4.
   pub const PORT1_PIN4: u32 = 0x4C;
   /// Port 1 Pin 5.
   pub const PORT1_PIN5: u32 = 0x4D;
   /// Port 1 Pin 6.
   pub const PORT1_PIN6: u32 = 0x4E;
   /// Port 1 Pin 7.
   pub const PORT1_PIN7: u32 = 0x4F;
   /// USART 0 Xclock.
   pub const USART0_XCK: u32 = 0x60;
   /// USART 1 Xclock.
   pub const USART1_XCK: u32 = 0x61;
   /// USART 2 Xclock.
   pub const USART2_XCK: u32 = 0x62;
   /// USART 3 Xclock.
   pub const USART3_XCK: u32 = 0x63;
   /// SPI 0 Sclock.
   pub const SPI0_SCK: u32 = 0x68;
   /// Timer/Counter A0 overflow / low byte underflow.
   pub const TCA0_OVF_LUNF: u32 = 0x80;
   /// Timer/Counter A0 high byte underflow (split mode).
   pub const TCA0_HUNF: u32 = 0x81;
   /// Timer/Counter A0 compare 0.
   pub const TCA0_CMP0: u32 = 0x84;
   /// Timer/Counter A0 compare 1.
   pub const TCA0_CMP1: u32 = 0x85;
   /// Timer/Counter A0 compare 2.
   pub const TCA0_CMP2: u32 = 0x86;
   /// Timer/Counter B0 capture.
   pub const TCB0_CAPT: u32 = 0xA0;
   /// Timer/Counter B1 capture.
   pub const TCB1_CAPT: u32 = 0xA2;
   /// Timer/Counter B2 capture.
   pub const TCB2_CAPT: u32 = 0xA4;
   /// Timer/Counter B3 capture.
   pub const TCB3_CAPT: u32 = 0xA6;
}

/// Software event on channels select
#[allow(non_upper_case_globals)]
pub mod evsys_strobe0 {
}

/// BOD Operation in Active Mode select
#[allow(non_upper_case_globals)]
pub mod fuse_active {
   /// Disabled.
   pub const DIS: u32 = 0x0;
   /// Enabled.
   pub const ENABLED: u32 = 0x1;
   /// Sampled.
   pub const SAMPLED: u32 = 0x2;
   /// Enabled with wake-up halted until BOD is ready.
   pub const ENWAKE: u32 = 0x3;
}

/// CRC Source select
#[allow(non_upper_case_globals)]
pub mod fuse_crcsrc {
   /// The CRC is performed on the entire Flash (boot, application code and application data section).
   pub const FLASH: u32 = 0x0;
   /// The CRC is performed on the boot section of Flash.
   pub const BOOT: u32 = 0x1;
   /// The CRC is performed on the boot and application code section of Flash.
   pub const BOOTAPP: u32 = 0x2;
   /// Disable CRC.
   pub const NOCRC: u32 = 0x3;
}

/// Frequency Select select
#[allow(non_upper_case_globals)]
pub mod fuse_freqsel {
   /// 16 MHz.
   pub const _16MHZ: u32 = 0x1;
   /// 20 MHz.
   pub const _20MHZ: u32 = 0x2;
}

/// BOD Level select
#[allow(non_upper_case_globals)]
pub mod fuse_lvl {
   /// 1.8 V.
   pub const BODLEVEL0: u32 = 0x0;
   /// 2.6 V.
   pub const BODLEVEL2: u32 = 0x2;
   /// 4.2 V.
   pub const BODLEVEL7: u32 = 0x7;
}

/// Watchdog Timeout Period select
#[allow(non_upper_case_globals)]
pub mod fuse_period {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// 8 cycles (8ms).
   pub const _8CLK: u32 = 0x1;
   /// 16 cycles (16ms).
   pub const _16CLK: u32 = 0x2;
   /// 32 cycles (32ms).
   pub const _32CLK: u32 = 0x3;
   /// 64 cycles (64ms).
   pub const _64CLK: u32 = 0x4;
   /// 128 cycles (0.128s).
   pub const _128CLK: u32 = 0x5;
   /// 256 cycles (0.256s).
   pub const _256CLK: u32 = 0x6;
   /// 512 cycles (0.512s).
   pub const _512CLK: u32 = 0x7;
   /// 1K cycles (1.0s).
   pub const _1KCLK: u32 = 0x8;
   /// 2K cycles (2.0s).
   pub const _2KCLK: u32 = 0x9;
   /// 4K cycles (4.1s).
   pub const _4KCLK: u32 = 0xA;
   /// 8K cycles (8.2s).
   pub const _8KCLK: u32 = 0xB;
}

/// Reset Pin Configuration select
#[allow(non_upper_case_globals)]
pub mod fuse_rstpincfg {
   /// GPIO mode.
   pub const GPIO: u32 = 0x0;
   /// Reset mode.
   pub const RST: u32 = 0x1;
}

/// BOD Sample Frequency select
#[allow(non_upper_case_globals)]
pub mod fuse_sampfreq {
   /// 1kHz sampling frequency.
   pub const _1KHZ: u32 = 0x0;
   /// 125Hz sampling frequency.
   pub const _125HZ: u32 = 0x1;
}

/// BOD Operation in Sleep Mode select
#[allow(non_upper_case_globals)]
pub mod fuse_sleep {
   /// Disabled.
   pub const DIS: u32 = 0x0;
   /// Enabled.
   pub const ENABLED: u32 = 0x1;
   /// Sampled.
   pub const SAMPLED: u32 = 0x2;
}

/// Startup Time select
#[allow(non_upper_case_globals)]
pub mod fuse_sut {
   /// 0 ms.
   pub const _0MS: u32 = 0x0;
   /// 1 ms.
   pub const _1MS: u32 = 0x1;
   /// 2 ms.
   pub const _2MS: u32 = 0x2;
   /// 4 ms.
   pub const _4MS: u32 = 0x3;
   /// 8 ms.
   pub const _8MS: u32 = 0x4;
   /// 16 ms.
   pub const _16MS: u32 = 0x5;
   /// 32 ms.
   pub const _32MS: u32 = 0x6;
   /// 64 ms.
   pub const _64MS: u32 = 0x7;
}

/// Watchdog Window Timeout Period select
#[allow(non_upper_case_globals)]
pub mod fuse_window {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// 8 cycles (8ms).
   pub const _8CLK: u32 = 0x1;
   /// 16 cycles (16ms).
   pub const _16CLK: u32 = 0x2;
   /// 32 cycles (32ms).
   pub const _32CLK: u32 = 0x3;
   /// 64 cycles (64ms).
   pub const _64CLK: u32 = 0x4;
   /// 128 cycles (0.128s).
   pub const _128CLK: u32 = 0x5;
   /// 256 cycles (0.256s).
   pub const _256CLK: u32 = 0x6;
   /// 512 cycles (0.512s).
   pub const _512CLK: u32 = 0x7;
   /// 1K cycles (1.0s).
   pub const _1KCLK: u32 = 0x8;
   /// 2K cycles (2.0s).
   pub const _2KCLK: u32 = 0x9;
   /// 4K cycles (4.1s).
   pub const _4KCLK: u32 = 0xA;
   /// 8K cycles (8.2s).
   pub const _8KCLK: u32 = 0xB;
}

/// Lock Bits select
#[allow(non_upper_case_globals)]
pub mod lockbit_lb {
   /// Read and write lock.
   pub const RWLOCK: u32 = 0x3A;
   /// No locks.
   pub const NOLOCK: u32 = 0xC5;
}

/// Command select
#[allow(non_upper_case_globals)]
pub mod nvmctrl_cmd {
   /// No Command.
   pub const NONE: u32 = 0x0;
   /// Write page.
   pub const PAGEWRITE: u32 = 0x1;
   /// Erase page.
   pub const PAGEERASE: u32 = 0x2;
   /// Erase and write page.
   pub const PAGEERASEWRITE: u32 = 0x3;
   /// Page buffer clear.
   pub const PAGEBUFCLR: u32 = 0x4;
   /// Chip erase.
   pub const CHIPERASE: u32 = 0x5;
   /// EEPROM erase.
   pub const EEERASE: u32 = 0x6;
   /// Write fuse (PDI only).
   pub const FUSEWRITE: u32 = 0x7;
}

/// Port Multiplexer SPI0 select
#[allow(non_upper_case_globals)]
pub mod portmux_spi0 {
   /// SPI0 on PA\[7:4\].
   pub const DEFAULT: u32 = 0x0;
   /// SPI0 on PC\[3:0\].
   pub const ALT1: u32 = 0x1;
   /// SPI0 on PE\[3:0\].
   pub const ALT2: u32 = 0x2;
   /// Not connected to any pins.
   pub const NONE: u32 = 0x3;
}

/// Port Multiplexer TCA0 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca0 {
   /// TCA0 pins on PA\[5:0\].
   pub const PORTA: u32 = 0x0;
   /// TCA0 pins on PB\[5:0\].
   pub const PORTB: u32 = 0x1;
   /// TCA0 pins on PC\[5:0\].
   pub const PORTC: u32 = 0x2;
   /// TCA0 pins on PD\[5:0\].
   pub const PORTD: u32 = 0x3;
   /// TCA0 pins on PE\[5:0\].
   pub const PORTE: u32 = 0x4;
   /// TCA0 pins on PF\[5:0\].
   pub const PORTF: u32 = 0x5;
}

/// Port Multiplexer TWI0 select
#[allow(non_upper_case_globals)]
pub mod portmux_twi0 {
   /// SCL/SDA on PA\[3:2\], Slave mode on PC\[3:2\] in dual TWI mode.
   pub const DEFAULT: u32 = 0x0;
   /// SCL/SDA on PA\[3:2\], Slave mode on PF\[3:2\] in dual TWI mode.
   pub const ALT1: u32 = 0x1;
   /// SCL/SDA on PC\[3:2\], Slave mode on PF\[3:2\] in dual TWI mode.
   pub const ALT2: u32 = 0x2;
   /// Not connected to any pins.
   pub const NONE: u32 = 0x3;
}

/// Port Multiplexer USART0 select
#[allow(non_upper_case_globals)]
pub mod portmux_usart0 {
   /// USART0 on PA\[3:0\].
   pub const DEFAULT: u32 = 0x0;
   /// USART0 on PA\[7:4\].
   pub const ALT1: u32 = 0x1;
   /// Not connected to any pins.
   pub const NONE: u32 = 0x3;
}

/// Port Multiplexer USART1 select
#[allow(non_upper_case_globals)]
pub mod portmux_usart1 {
   /// USART1 on PC\[3:0\].
   pub const DEFAULT: u32 = 0x0;
   /// USART1 on PC\[7:4\].
   pub const ALT1: u32 = 0x1;
   /// Not connected to any pins.
   pub const NONE: u32 = 0x3;
}

/// Port Multiplexer USART2 select
#[allow(non_upper_case_globals)]
pub mod portmux_usart2 {
   /// USART2 on PF\[3:0\].
   pub const DEFAULT: u32 = 0x0;
   /// USART2 on PF\[6:4\].
   pub const ALT1: u32 = 0x1;
   /// Not connected to any pins.
   pub const NONE: u32 = 0x3;
}

/// Port Multiplexer USART3 select
#[allow(non_upper_case_globals)]
pub mod portmux_usart3 {
   /// USART3 on PB\[3:0\].
   pub const DEFAULT: u32 = 0x0;
   /// USART3 on PB\[5:4\].
   pub const ALT1: u32 = 0x1;
   /// Not connected to any pins.
   pub const NONE: u32 = 0x3;
}

/// Input/Sense Configuration select
#[allow(non_upper_case_globals)]
pub mod port_isc {
   /// Interrupt disabled but input buffer enabled.
   pub const INTDISABLE: u32 = 0x0;
   /// Sense Both Edges.
   pub const BOTHEDGES: u32 = 0x1;
   /// Sense Rising Edge.
   pub const RISING: u32 = 0x2;
   /// Sense Falling Edge.
   pub const FALLING: u32 = 0x3;
   /// Digital Input Buffer disabled.
   pub const INPUT_DISABLE: u32 = 0x4;
   /// Sense low Level.
   pub const LEVEL: u32 = 0x5;
}

/// Clock Select select
#[allow(non_upper_case_globals)]
pub mod rtc_clksel {
   /// Internal 32kHz OSC.
   pub const INT32K: u32 = 0x0;
   /// Internal 1kHz OSC.
   pub const INT1K: u32 = 0x1;
   /// 32KHz Crystal OSC.
   pub const TOSC32K: u32 = 0x2;
   /// External Clock.
   pub const EXTCLK: u32 = 0x3;
}

/// Period select
#[allow(non_upper_case_globals)]
pub mod rtc_period {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// RTC Clock Cycles 4.
   pub const CYC4: u32 = 0x1;
   /// RTC Clock Cycles 8.
   pub const CYC8: u32 = 0x2;
   /// RTC Clock Cycles 16.
   pub const CYC16: u32 = 0x3;
   /// RTC Clock Cycles 32.
   pub const CYC32: u32 = 0x4;
   /// RTC Clock Cycles 64.
   pub const CYC64: u32 = 0x5;
   /// RTC Clock Cycles 128.
   pub const CYC128: u32 = 0x6;
   /// RTC Clock Cycles 256.
   pub const CYC256: u32 = 0x7;
   /// RTC Clock Cycles 512.
   pub const CYC512: u32 = 0x8;
   /// RTC Clock Cycles 1024.
   pub const CYC1024: u32 = 0x9;
   /// RTC Clock Cycles 2048.
   pub const CYC2048: u32 = 0xA;
   /// RTC Clock Cycles 4096.
   pub const CYC4096: u32 = 0xB;
   /// RTC Clock Cycles 8192.
   pub const CYC8192: u32 = 0xC;
   /// RTC Clock Cycles 16384.
   pub const CYC16384: u32 = 0xD;
   /// RTC Clock Cycles 32768.
   pub const CYC32768: u32 = 0xE;
}

/// Prescaling Factor select
#[allow(non_upper_case_globals)]
pub mod rtc_prescaler {
   /// RTC Clock / 1.
   pub const DIV1: u32 = 0x0;
   /// RTC Clock / 2.
   pub const DIV2: u32 = 0x1;
   /// RTC Clock / 4.
   pub const DIV4: u32 = 0x2;
   /// RTC Clock / 8.
   pub const DIV8: u32 = 0x3;
   /// RTC Clock / 16.
   pub const DIV16: u32 = 0x4;
   /// RTC Clock / 32.
   pub const DIV32: u32 = 0x5;
   /// RTC Clock / 64.
   pub const DIV64: u32 = 0x6;
   /// RTC Clock / 128.
   pub const DIV128: u32 = 0x7;
   /// RTC Clock / 256.
   pub const DIV256: u32 = 0x8;
   /// RTC Clock / 512.
   pub const DIV512: u32 = 0x9;
   /// RTC Clock / 1024.
   pub const DIV1024: u32 = 0xA;
   /// RTC Clock / 2048.
   pub const DIV2048: u32 = 0xB;
   /// RTC Clock / 4096.
   pub const DIV4096: u32 = 0xC;
   /// RTC Clock / 8192.
   pub const DIV8192: u32 = 0xD;
   /// RTC Clock / 16384.
   pub const DIV16384: u32 = 0xE;
   /// RTC Clock / 32768.
   pub const DIV32768: u32 = 0xF;
}

/// Sleep mode select
#[allow(non_upper_case_globals)]
pub mod slpctrl_smode {
   /// Idle mode.
   pub const IDLE: u32 = 0x0;
   /// Standby Mode.
   pub const STDBY: u32 = 0x1;
   /// Power-down Mode.
   pub const PDOWN: u32 = 0x2;
}

/// SPI Mode select
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

/// Prescaler select
#[allow(non_upper_case_globals)]
pub mod spi_presc {
   /// System Clock / 4.
   pub const DIV4: u32 = 0x0;
   /// System Clock / 16.
   pub const DIV16: u32 = 0x1;
   /// System Clock / 64.
   pub const DIV64: u32 = 0x2;
   /// System Clock / 128.
   pub const DIV128: u32 = 0x3;
}

/// Clock Selection select
#[allow(non_upper_case_globals)]
pub mod tca_single_clksel {
   /// System Clock.
   pub const DIV1: u32 = 0x0;
   /// System Clock / 2.
   pub const DIV2: u32 = 0x1;
   /// System Clock / 4.
   pub const DIV4: u32 = 0x2;
   /// System Clock / 8.
   pub const DIV8: u32 = 0x3;
   /// System Clock / 16.
   pub const DIV16: u32 = 0x4;
   /// System Clock / 64.
   pub const DIV64: u32 = 0x5;
   /// System Clock / 256.
   pub const DIV256: u32 = 0x6;
   /// System Clock / 1024.
   pub const DIV1024: u32 = 0x7;
}

/// Command select
#[allow(non_upper_case_globals)]
pub mod tca_single_cmd {
   /// No Command.
   pub const NONE: u32 = 0x0;
   /// Force Update.
   pub const UPDATE: u32 = 0x1;
   /// Force Restart.
   pub const RESTART: u32 = 0x2;
   /// Force Hard Reset.
   pub const RESET: u32 = 0x3;
}

/// Direction select
#[allow(non_upper_case_globals)]
pub mod tca_single_dir {
   /// Count up.
   pub const UP: u32 = 0x0;
   /// Count down.
   pub const DOWN: u32 = 0x1;
}

/// Event Action select
#[allow(non_upper_case_globals)]
pub mod tca_single_evact {
   /// Count on positive edge event.
   pub const POSEDGE: u32 = 0x0;
   /// Count on any edge event.
   pub const ANYEDGE: u32 = 0x1;
   /// Count on prescaled clock while event line is 1.
   pub const HIGHLVL: u32 = 0x2;
   /// Count on prescaled clock. Event controls count direction. Up-count when event line is 0, down-count when event line is 1.
   pub const UPDOWN: u32 = 0x3;
}

/// Waveform generation mode select
#[allow(non_upper_case_globals)]
pub mod tca_single_wgmode {
   /// Normal Mode.
   pub const NORMAL: u32 = 0x0;
   /// Frequency Generation Mode.
   pub const FRQ: u32 = 0x1;
   /// Single Slope PWM.
   pub const SINGLESLOPE: u32 = 0x3;
   /// Dual Slope PWM, overflow on TOP.
   pub const DSTOP: u32 = 0x5;
   /// Dual Slope PWM, overflow on TOP and BOTTOM.
   pub const DSBOTH: u32 = 0x6;
   /// Dual Slope PWM, overflow on BOTTOM.
   pub const DSBOTTOM: u32 = 0x7;
}

/// Clock Selection select
#[allow(non_upper_case_globals)]
pub mod tca_split_clksel {
   /// System Clock.
   pub const DIV1: u32 = 0x0;
   /// System Clock / 2.
   pub const DIV2: u32 = 0x1;
   /// System Clock / 4.
   pub const DIV4: u32 = 0x2;
   /// System Clock / 8.
   pub const DIV8: u32 = 0x3;
   /// System Clock / 16.
   pub const DIV16: u32 = 0x4;
   /// System Clock / 64.
   pub const DIV64: u32 = 0x5;
   /// System Clock / 256.
   pub const DIV256: u32 = 0x6;
   /// System Clock / 1024.
   pub const DIV1024: u32 = 0x7;
}

/// Command select
#[allow(non_upper_case_globals)]
pub mod tca_split_cmd {
   /// No Command.
   pub const NONE: u32 = 0x0;
   /// Force Update.
   pub const UPDATE: u32 = 0x1;
   /// Force Restart.
   pub const RESTART: u32 = 0x2;
   /// Force Hard Reset.
   pub const RESET: u32 = 0x3;
}

/// Clock Select select
#[allow(non_upper_case_globals)]
pub mod tcb_clksel {
   /// CLK_PER (No Prescaling).
   pub const CLKDIV1: u32 = 0x0;
   /// CLK_PER/2 (From Prescaler).
   pub const CLKDIV2: u32 = 0x1;
   /// Use Clock from TCA.
   pub const CLKTCA: u32 = 0x2;
}

/// Timer Mode select
#[allow(non_upper_case_globals)]
pub mod tcb_cntmode {
   /// Periodic Interrupt.
   pub const INT: u32 = 0x0;
   /// Periodic Timeout.
   pub const TIMEOUT: u32 = 0x1;
   /// Input Capture Event.
   pub const CAPT: u32 = 0x2;
   /// Input Capture Frequency measurement.
   pub const FRQ: u32 = 0x3;
   /// Input Capture Pulse-Width measurement.
   pub const PW: u32 = 0x4;
   /// Input Capture Frequency and Pulse-Width measurement.
   pub const FRQPW: u32 = 0x5;
   /// Single Shot.
   pub const SINGLE: u32 = 0x6;
   /// 8-bit PWM.
   pub const PWM8: u32 = 0x7;
}

/// Acknowledge Action select
#[allow(non_upper_case_globals)]
pub mod twi_ackact {
   /// Send ACK.
   pub const ACK: u32 = 0x0;
   /// Send NACK.
   pub const NACK: u32 = 0x1;
}

/// Slave Address or Stop select
#[allow(non_upper_case_globals)]
pub mod twi_ap {
   /// Stop condition generated APIF.
   pub const STOP: u32 = 0x0;
   /// Address detection generated APIF.
   pub const ADR: u32 = 0x1;
}

/// Bus State select
#[allow(non_upper_case_globals)]
pub mod twi_busstate {
   /// Unknown Bus State.
   pub const UNKNOWN: u32 = 0x0;
   /// Bus is Idle.
   pub const IDLE: u32 = 0x1;
   /// This Module Controls The Bus.
   pub const OWNER: u32 = 0x2;
   /// The Bus is Busy.
   pub const BUSY: u32 = 0x3;
}

/// SDA Hold Time select
#[allow(non_upper_case_globals)]
pub mod twi_default_sdahold {
   /// SDA hold time off.
   pub const OFF: u32 = 0x0;
   /// Typical 50ns hold time.
   pub const _50NS: u32 = 0x1;
   /// Typical 300ns hold time.
   pub const _300NS: u32 = 0x2;
   /// Typical 500ns hold time.
   pub const _500NS: u32 = 0x3;
}

/// SDA Setup Time select
#[allow(non_upper_case_globals)]
pub mod twi_default_sdasetup {
   /// SDA setup time is 4 clock cycles.
   pub const _4CYC: u32 = 0x0;
   /// SDA setup time is 8 clock cycles.
   pub const _8CYC: u32 = 0x1;
}

/// Command select
#[allow(non_upper_case_globals)]
pub mod twi_mcmd {
   /// No Action.
   pub const NOACT: u32 = 0x0;
   /// Issue Repeated Start Condition.
   pub const REPSTART: u32 = 0x1;
   /// Receive or Transmit Data, depending on DIR.
   pub const RECVTRANS: u32 = 0x2;
   /// Issue Stop Condition.
   pub const STOP: u32 = 0x3;
}

/// Command select
#[allow(non_upper_case_globals)]
pub mod twi_scmd {
   /// No Action.
   pub const NOACT: u32 = 0x0;
   /// Used To Complete a Transaction.
   pub const COMPTRANS: u32 = 0x2;
   /// Used in Response to Address/Data Interrupt.
   pub const RESPONSE: u32 = 0x3;
}

/// Inactive Bus Timeout select
#[allow(non_upper_case_globals)]
pub mod twi_timeout {
   /// Bus Timeout Disabled.
   pub const DISABLED: u32 = 0x0;
   /// 50 Microseconds.
   pub const _50US: u32 = 0x1;
   /// 100 Microseconds.
   pub const _100US: u32 = 0x2;
   /// 200 Microseconds.
   pub const _200US: u32 = 0x3;
}

/// Auto Baud Window select
#[allow(non_upper_case_globals)]
pub mod usart_abw {
   /// 18% tolerance.
   pub const WDW0: u32 = 0x0;
   /// 15% tolerance.
   pub const WDW1: u32 = 0x1;
   /// 21% tolerance.
   pub const WDW2: u32 = 0x2;
   /// 25% tolerance.
   pub const WDW3: u32 = 0x3;
}

/// Communication Mode select
#[allow(non_upper_case_globals)]
pub mod usart_mspi_cmode {
   /// Asynchronous Mode.
   pub const ASYNCHRONOUS: u32 = 0x0;
   /// Synchronous Mode.
   pub const SYNCHRONOUS: u32 = 0x1;
   /// Infrared Communication.
   pub const IRCOM: u32 = 0x2;
   /// Master SPI Mode.
   pub const MSPI: u32 = 0x3;
}

/// Character Size select
#[allow(non_upper_case_globals)]
pub mod usart_normal_chsize {
   /// Character size: 5 bit.
   pub const _5BIT: u32 = 0x0;
   /// Character size: 6 bit.
   pub const _6BIT: u32 = 0x1;
   /// Character size: 7 bit.
   pub const _7BIT: u32 = 0x2;
   /// Character size: 8 bit.
   pub const _8BIT: u32 = 0x3;
   /// Character size: 9 bit read low byte first.
   pub const _9BITL: u32 = 0x6;
   /// Character size: 9 bit read high byte first.
   pub const _9BITH: u32 = 0x7;
}

/// Communication Mode select
#[allow(non_upper_case_globals)]
pub mod usart_normal_cmode {
   /// Asynchronous Mode.
   pub const ASYNCHRONOUS: u32 = 0x0;
   /// Synchronous Mode.
   pub const SYNCHRONOUS: u32 = 0x1;
   /// Infrared Communication.
   pub const IRCOM: u32 = 0x2;
   /// Master SPI Mode.
   pub const MSPI: u32 = 0x3;
}

/// Parity Mode select
#[allow(non_upper_case_globals)]
pub mod usart_normal_pmode {
   /// No Parity.
   pub const DISABLED: u32 = 0x0;
   /// Even Parity.
   pub const EVEN: u32 = 0x2;
   /// Odd Parity.
   pub const ODD: u32 = 0x3;
}

/// Stop Bit Mode select
#[allow(non_upper_case_globals)]
pub mod usart_normal_sbmode {
   /// 1 stop bit.
   pub const _1BIT: u32 = 0x0;
   /// 2 stop bits.
   pub const _2BIT: u32 = 0x1;
}

/// RS485 Mode internal transmitter select
#[allow(non_upper_case_globals)]
pub mod usart_rs485 {
   /// RS485 Mode disabled.
   pub const OFF: u32 = 0x0;
   /// RS485 Mode External drive.
   pub const EXT: u32 = 0x1;
   /// RS485 Mode Internal drive.
   pub const INT: u32 = 0x2;
}

/// Receiver Mode select
#[allow(non_upper_case_globals)]
pub mod usart_rxmode {
   /// Normal mode.
   pub const NORMAL: u32 = 0x0;
   /// CLK2x mode.
   pub const CLK2X: u32 = 0x1;
   /// Generic autobaud mode.
   pub const GENAUTO: u32 = 0x2;
   /// LIN constrained autobaud mode.
   pub const LINAUTO: u32 = 0x3;
}

/// AC0 reference select select
#[allow(non_upper_case_globals)]
pub mod vref_ac0refsel {
   /// Voltage reference at 0.55V.
   pub const _0V55: u32 = 0x0;
   /// Voltage reference at 1.1V.
   pub const _1V1: u32 = 0x1;
   /// Voltage reference at 2.5V.
   pub const _2V5: u32 = 0x2;
   /// Voltage reference at 4.34V.
   pub const _4V34: u32 = 0x3;
   /// Voltage reference at 1.5V.
   pub const _1V5: u32 = 0x4;
   /// AVDD.
   pub const AVDD: u32 = 0x7;
}

/// ADC0 reference select select
#[allow(non_upper_case_globals)]
pub mod vref_adc0refsel {
   /// Voltage reference at 0.55V.
   pub const _0V55: u32 = 0x0;
   /// Voltage reference at 1.1V.
   pub const _1V1: u32 = 0x1;
   /// Voltage reference at 2.5V.
   pub const _2V5: u32 = 0x2;
   /// Voltage reference at 4.34V.
   pub const _4V34: u32 = 0x3;
   /// Voltage reference at 1.5V.
   pub const _1V5: u32 = 0x4;
}

/// Period select
#[allow(non_upper_case_globals)]
pub mod wdt_period {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// 8 cycles (8ms).
   pub const _8CLK: u32 = 0x1;
   /// 16 cycles (16ms).
   pub const _16CLK: u32 = 0x2;
   /// 32 cycles (32ms).
   pub const _32CLK: u32 = 0x3;
   /// 64 cycles (64ms).
   pub const _64CLK: u32 = 0x4;
   /// 128 cycles (0.128s).
   pub const _128CLK: u32 = 0x5;
   /// 256 cycles (0.256s).
   pub const _256CLK: u32 = 0x6;
   /// 512 cycles (0.512s).
   pub const _512CLK: u32 = 0x7;
   /// 1K cycles (1.0s).
   pub const _1KCLK: u32 = 0x8;
   /// 2K cycles (2.0s).
   pub const _2KCLK: u32 = 0x9;
   /// 4K cycles (4.1s).
   pub const _4KCLK: u32 = 0xA;
   /// 8K cycles (8.2s).
   pub const _8KCLK: u32 = 0xB;
}

/// Window select
#[allow(non_upper_case_globals)]
pub mod wdt_window {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// 8 cycles (8ms).
   pub const _8CLK: u32 = 0x1;
   /// 16 cycles (16ms).
   pub const _16CLK: u32 = 0x2;
   /// 32 cycles (32ms).
   pub const _32CLK: u32 = 0x3;
   /// 64 cycles (64ms).
   pub const _64CLK: u32 = 0x4;
   /// 128 cycles (0.128s).
   pub const _128CLK: u32 = 0x5;
   /// 256 cycles (0.256s).
   pub const _256CLK: u32 = 0x6;
   /// 512 cycles (0.512s).
   pub const _512CLK: u32 = 0x7;
   /// 1K cycles (1.0s).
   pub const _1KCLK: u32 = 0x8;
   /// 2K cycles (2.0s).
   pub const _2KCLK: u32 = 0x9;
   /// 4K cycles (4.1s).
   pub const _4KCLK: u32 = 0xA;
   /// 8K cycles (8.2s).
   pub const _8KCLK: u32 = 0xB;
}

