//! The AVR ATtiny214 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny214-SSFR | SOIC14 | SOIC14 | -40°C - 125°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny214-SSNR | SOIC14 | SOIC14 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

/// Watchdog Configuration.
pub const WDTCFG: *mut u8 = 0x0 as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x0 as *mut u8;

/// Reset Flags.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SWRF | 10000 |
/// | UPDIRF | 100000 |
/// | WDRF | 1000 |
/// | EXTRF | 100 |
/// | PORF | 1 |
/// | BORF | 10 |
pub const RSTFR: *mut u8 = 0x0 as *mut u8;

/// Asynchronous Channel Strobe.
pub const ASYNCSTROBE: *mut u8 = 0x0 as *mut u8;

/// Receive Data Low Byte.
pub const RXDATAL: *mut u8 = 0x0 as *mut u8;

/// Device ID Byte 0.
pub const DEVICEID0: *mut u8 = 0x0 as *mut u8;

/// Data Direction.
pub const DIR: *mut u8 = 0x0 as *mut u8;

/// Control A.
pub const CTRLA: *mut u8 = 0x0 as *mut u8;

/// Lock bits.
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// User Row Byte 0.
pub const USERROW0: *mut u8 = 0x0 as *mut u8;

/// MCLK Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOUT | 10000000 |
pub const MCLKCTRLA: *mut u8 = 0x0 as *mut u8;

/// User Row Byte 1.
pub const USERROW1: *mut u8 = 0x1 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x1 as *mut u8;

/// Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYNCBUSY | 1 |
pub const STATUS: *mut u8 = 0x1 as *mut u8;

/// Output Value.
pub const OUT: *mut u8 = 0x1 as *mut u8;

/// Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DAC0REFEN | 1 |
/// | ADC0REFEN | 10 |
pub const CTRLB: *mut u8 = 0x1 as *mut u8;

/// Software Reset.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SWRE | 1 |
pub const SWRR: *mut u8 = 0x1 as *mut u8;

/// Synchronous Channel Strobe.
pub const SYNCSTROBE: *mut u8 = 0x1 as *mut u8;

/// Revision ID.
pub const REVID: *mut u8 = 0x1 as *mut u8;

/// Sequential Control 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEQSEL | 111 |
pub const SEQCTRL0: *mut u8 = 0x1 as *mut u8;

/// Data Direction Set.
pub const DIRSET: *mut u8 = 0x1 as *mut u8;

/// BOD Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAMPFREQ | 10000 |
/// | SLEEP | 11 |
/// | LVL | 11100000 |
/// | ACTIVE | 1100 |
pub const BODCFG: *mut u8 = 0x1 as *mut u8;

/// MCLK Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEN | 1 |
/// | PDIV | 11110 |
pub const MCLKCTRLB: *mut u8 = 0x1 as *mut u8;

/// Receive Data High Byte.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PERR | 10 |
/// | FERR | 100 |
/// | BUFOVF | 1000000 |
/// | RXCIF | 10000000 |
pub const RXDATAH: *mut u8 = 0x1 as *mut u8;

/// Device ID Byte 1.
pub const DEVICEID1: *mut u8 = 0x1 as *mut u8;

/// Mux Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INVERT | 10000000 |
/// | MUXNEG | 11 |
pub const MUXCTRLA: *mut u8 = 0x2 as *mut u8;

/// Device ID Byte 2.
pub const DEVICEID2: *mut u8 = 0x2 as *mut u8;

/// User Row Byte 2.
pub const USERROW2: *mut u8 = 0x2 as *mut u8;

/// Interrupt Level 0 Priority.
pub const LVL0PRI: *mut u8 = 0x2 as *mut u8;

/// Asynchronous Channel 0 Generator Selection.
pub const ASYNCCH0: *mut u8 = 0x2 as *mut u8;

/// Oscillator Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OSCLOCK | 10000000 |
/// | FREQSEL | 11 |
pub const OSCCFG: *mut u8 = 0x2 as *mut u8;

/// Input Value.
pub const IN: *mut u8 = 0x2 as *mut u8;

/// Transmit Data Low Byte.
pub const TXDATAL: *mut u8 = 0x2 as *mut u8;

/// External Break.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENEXTBRK | 1 |
pub const EXTBRK: *mut u8 = 0x2 as *mut u8;

/// MCLK Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LOCKEN | 1 |
pub const MCLKLOCK: *mut u8 = 0x2 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x2 as *mut u8;

/// Data Direction Clear.
pub const DIRCLR: *mut u8 = 0x2 as *mut u8;

/// Serial Number Byte 0.
pub const SERNUM0: *mut u8 = 0x3 as *mut u8;

/// MCLK Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OSC32KS | 100000 |
/// | EXTS | 10000000 |
/// | SOSC | 1 |
/// | OSC20MS | 10000 |
/// | XOSC32KS | 1000000 |
pub const MCLKSTATUS: *mut u8 = 0x3 as *mut u8;

/// Master Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TIMEOUT | 1100 |
/// | RIEN | 10000000 |
/// | QCEN | 10000 |
/// | WIEN | 1000000 |
pub const MCTRLA: *mut u8 = 0x3 as *mut u8;

/// Interrupt Level 1 Priority Vector.
pub const LVL1VEC: *mut u8 = 0x3 as *mut u8;

/// Control D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMPBVAL | 11110000 |
/// | CMPAVAL | 1111 |
pub const CTRLD: *mut u8 = 0x3 as *mut u8;

/// Data Direction Toggle.
pub const DIRTGL: *mut u8 = 0x3 as *mut u8;

/// User Row Byte 3.
pub const USERROW3: *mut u8 = 0x3 as *mut u8;

/// Asynchronous Channel 1 Generator Selection.
pub const ASYNCCH1: *mut u8 = 0x3 as *mut u8;

/// General Purpose IO Register 3.
pub const GPIOR3: *mut u8 = 0x3 as *mut u8;

/// Interrupt Flags.
pub const INTFLAGS: *mut u8 = 0x3 as *mut u8;

/// Transmit Data High Byte.
pub const TXDATAH: *mut u8 = 0x3 as *mut u8;

/// Control E.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCAPTUREA | 1000 |
/// | SYNC | 10 |
/// | DISEOC | 10000000 |
/// | RESTART | 100 |
/// | SYNCEOC | 1 |
/// | SCAPTUREB | 10000 |
pub const CTRLE: *mut u8 = 0x4 as *mut u8;

/// Serial Number Byte 1.
pub const SERNUM1: *mut u8 = 0x4 as *mut u8;

/// Control E Clear.
pub const CTRLECLR: *mut u8 = 0x4 as *mut u8;

/// TCD0 Configuration.
pub const TCD0CFG: *mut u8 = 0x4 as *mut u8;

/// User Row Byte 4.
pub const USERROW4: *mut u8 = 0x4 as *mut u8;

/// Asynchronous Channel 2 Generator Selection.
pub const ASYNCCH2: *mut u8 = 0x4 as *mut u8;

/// Data.
pub const DATA: *mut u8 = 0x4 as *mut u8;

/// Master Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FLUSH | 1000 |
/// | MCMD | 11 |
pub const MCTRLB: *mut u8 = 0x4 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x4 as *mut u8;

/// Output Value Set.
pub const OUTSET: *mut u8 = 0x5 as *mut u8;

/// Control E Set.
pub const CTRLESET: *mut u8 = 0x5 as *mut u8;

/// Serial Number Byte 2.
pub const SERNUM2: *mut u8 = 0x5 as *mut u8;

/// User Row Byte 5.
pub const USERROW5: *mut u8 = 0x5 as *mut u8;

/// Sample Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAMPLEN | 11111 |
pub const SAMPCTRL: *mut u8 = 0x5 as *mut u8;

/// System Configuration 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1 |
/// | RSTPINCFG | 1100 |
/// | CRCSRC | 11000000 |
pub const SYSCFG0: *mut u8 = 0x5 as *mut u8;

/// Asynchronous Channel 3 Generator Selection.
pub const ASYNCCH3: *mut u8 = 0x5 as *mut u8;

/// Master Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ARBLOST | 1000 |
/// | WIF | 1000000 |
/// | BUSSTATE | 11 |
/// | RIF | 10000000 |
pub const MSTATUS: *mut u8 = 0x5 as *mut u8;

/// LUT Control 0 A.
pub const LUT0CTRLA: *mut u8 = 0x5 as *mut u8;

/// Positive mux input.
pub const MUXPOS: *mut u8 = 0x6 as *mut u8;

/// User Row Byte 6.
pub const USERROW6: *mut u8 = 0x6 as *mut u8;

/// Control F Clear.
pub const CTRLFCLR: *mut u8 = 0x6 as *mut u8;

/// Serial Number Byte 3.
pub const SERNUM3: *mut u8 = 0x6 as *mut u8;

/// Output Value Clear.
pub const OUTCLR: *mut u8 = 0x6 as *mut u8;

/// Master Baurd Rate Control.
pub const MBAUD: *mut u8 = 0x6 as *mut u8;

/// System Configuration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SUT | 111 |
pub const SYSCFG1: *mut u8 = 0x6 as *mut u8;

/// LUT Control 0 B.
pub const LUT0CTRLB: *mut u8 = 0x6 as *mut u8;

/// Application Code Section End.
pub const APPEND: *mut u8 = 0x7 as *mut u8;

/// Output Value Toggle.
pub const OUTTGL: *mut u8 = 0x7 as *mut u8;

/// Master Address.
pub const MADDR: *mut u8 = 0x7 as *mut u8;

/// User Row Byte 7.
pub const USERROW7: *mut u8 = 0x7 as *mut u8;

/// Control C.
pub const CTRLC: *mut u8 = 0x7 as *mut u8;

/// Control F Set.
pub const CTRLFSET: *mut u8 = 0x7 as *mut u8;

/// LUT Control 0 C.
pub const LUT0CTRLC: *mut u8 = 0x7 as *mut u8;

/// Clock Select.
pub const CLKSEL: *mut u8 = 0x7 as *mut u8;

/// Serial Number Byte 4.
pub const SERNUM4: *mut u8 = 0x7 as *mut u8;

/// Address.
pub const ADDR: *mut u16 = 0x8 as *mut u16;

/// Voltage level monitor Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VLMLVL | 11 |
pub const VLMCTRLA: *mut u8 = 0x8 as *mut u8;

/// Baud Rate low byte.
pub const BAUDL: *mut u8 = 0x8 as *mut u8;

/// Command.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STCONV | 1 |
pub const COMMAND: *mut u8 = 0x8 as *mut u8;

/// Serial Number Byte 5.
pub const SERNUM5: *mut u8 = 0x8 as *mut u8;

/// Master Data.
pub const MDATA: *mut u8 = 0x8 as *mut u8;

/// Address low byte.
pub const ADDRL: *mut u8 = 0x8 as *mut u8;

/// User Row Byte 8.
pub const USERROW8: *mut u8 = 0x8 as *mut u8;

/// EVCTRLA.
pub const EVCTRLA: *mut u8 = 0x8 as *mut u8;

/// Truth 0.
pub const TRUTH0: *mut u8 = 0x8 as *mut u8;

/// Baud Rate.
pub const BAUD: *mut u16 = 0x8 as *mut u16;

/// Boot Section End.
pub const BOOTEND: *mut u8 = 0x8 as *mut u8;

/// Serial Number Byte 6.
pub const SERNUM6: *mut u8 = 0x9 as *mut u8;

/// Slave Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PMEN | 100 |
/// | DIEN | 10000000 |
/// | PIEN | 100000 |
/// | APIEN | 1000000 |
pub const SCTRLA: *mut u8 = 0x9 as *mut u8;

/// User Row Byte 9.
pub const USERROW9: *mut u8 = 0x9 as *mut u8;

/// EVCTRLB.
pub const EVCTRLB: *mut u8 = 0x9 as *mut u8;

/// LUT Control 1 A.
pub const LUT1CTRLA: *mut u8 = 0x9 as *mut u8;

/// Baud Rate high byte.
pub const BAUDH: *mut u8 = 0x9 as *mut u8;

/// Address high byte.
pub const ADDRH: *mut u8 = 0x9 as *mut u8;

/// Temporary Value.
pub const TEMP: *mut u8 = 0x9 as *mut u8;

/// Synchronous Channel 0 Generator Selection.
pub const SYNCCH0: *mut u8 = 0xA as *mut u8;

/// Count.
pub const CNT: *mut u16 = 0xA as *mut u16;

/// LUT Control 1 B.
pub const LUT1CTRLB: *mut u8 = 0xA as *mut u8;

/// Serial Number Byte 7.
pub const SERNUM7: *mut u8 = 0xA as *mut u8;

/// User Row Byte 10.
pub const USERROW10: *mut u8 = 0xA as *mut u8;

/// Count low byte.
pub const CNTL: *mut u8 = 0xA as *mut u8;

/// Slave Control B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCMD | 11 |
pub const SCTRLB: *mut u8 = 0xA as *mut u8;

/// User Row Byte 11.
pub const USERROW11: *mut u8 = 0xB as *mut u8;

/// Slave Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AP | 1 |
/// | COLL | 1000 |
/// | APIF | 1000000 |
/// | DIF | 10000000 |
pub const SSTATUS: *mut u8 = 0xB as *mut u8;

/// LUT Control 1 C.
pub const LUT1CTRLC: *mut u8 = 0xB as *mut u8;

/// Synchronous Channel 1 Generator Selection.
pub const SYNCCH1: *mut u8 = 0xB as *mut u8;

/// Debug Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ABMBP | 10000000 |
pub const DBGCTRL: *mut u8 = 0xB as *mut u8;

/// Count high byte.
pub const CNTH: *mut u8 = 0xB as *mut u8;

/// Serial Number Byte 8.
pub const SERNUM8: *mut u8 = 0xB as *mut u8;

/// Event Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IREI | 1 |
pub const EVCTRL: *mut u8 = 0xC as *mut u8;

/// Truth 1.
pub const TRUTH1: *mut u8 = 0xC as *mut u8;

/// Slave Address.
pub const SADDR: *mut u8 = 0xC as *mut u8;

/// Serial Number Byte 9.
pub const SERNUM9: *mut u8 = 0xC as *mut u8;

/// Interrupt Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TRIGA | 100 |
/// | TRIGB | 1000 |
/// | OVF | 1 |
pub const INTCTRL: *mut u8 = 0xC as *mut u8;

/// Compare.
pub const CMP: *mut u16 = 0xC as *mut u16;

/// Compare or Capture low byte.
pub const CCMPL: *mut u8 = 0xC as *mut u8;

/// Compare low byte.
pub const CMPL: *mut u8 = 0xC as *mut u8;

/// User Row Byte 12.
pub const USERROW12: *mut u8 = 0xC as *mut u8;

/// Compare or Capture.
pub const CCMP: *mut u16 = 0xC as *mut u16;

/// Compare high byte.
pub const CMPH: *mut u8 = 0xD as *mut u8;

/// Compare or Capture high byte.
pub const CCMPH: *mut u8 = 0xD as *mut u8;

/// Slave Data.
pub const SDATA: *mut u8 = 0xD as *mut u8;

/// Stack Pointer Low.
pub const SPL: *mut u8 = 0xD as *mut u8;

/// User Row Byte 13.
pub const USERROW13: *mut u8 = 0xD as *mut u8;

/// IRCOM Transmitter Pulse Length Control.
pub const TXPLCTRL: *mut u8 = 0xD as *mut u8;

/// Slave Address Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADDREN | 1 |
/// | ADDRMASK | 11111110 |
pub const SADDRMASK: *mut u8 = 0xE as *mut u8;

/// IRCOM Receiver Pulse Length Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXPL | 1111111 |
pub const RXPLCTRL: *mut u8 = 0xE as *mut u8;

/// User Row Byte 14.
pub const USERROW14: *mut u8 = 0xE as *mut u8;

/// Stack Pointer High.
pub const SPH: *mut u8 = 0xE as *mut u8;

/// User Row Byte 15.
pub const USERROW15: *mut u8 = 0xF as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | Z | 10 |
/// | T | 1000000 |
/// | H | 100000 |
/// | C | 1 |
/// | S | 10000 |
/// | I | 10000000 |
/// | N | 100 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0xF as *mut u8;

/// User Row Byte 16.
pub const USERROW16: *mut u8 = 0x10 as *mut u8;

/// PIT Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PITEN | 1 |
pub const PITCTRLA: *mut u8 = 0x10 as *mut u8;

/// OSC20M Control A.
pub const OSC20MCTRLA: *mut u8 = 0x10 as *mut u8;

/// Input Control A.
pub const INPUTCTRLA: *mut u8 = 0x10 as *mut u8;

/// ADC Accumulator Result low byte.
pub const RESL: *mut u8 = 0x10 as *mut u8;

/// Pin 0 Control.
pub const PIN0CTRL: *mut u8 = 0x10 as *mut u8;

/// ADC Accumulator Result.
pub const RES: *mut u16 = 0x10 as *mut u16;

/// Pin 1 Control.
pub const PIN1CTRL: *mut u8 = 0x11 as *mut u8;

/// OSC20M Calibration A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CAL20M | 111111 |
/// | CALSEL20M | 11000000 |
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

/// Input Control B.
pub const INPUTCTRLB: *mut u8 = 0x11 as *mut u8;

/// User Row Byte 18.
pub const USERROW18: *mut u8 = 0x12 as *mut u8;

/// OSC20M Calibration B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TEMPCAL20M | 1111 |
pub const OSC20MCALIBB: *mut u8 = 0x12 as *mut u8;

/// Asynchronous User Ch 0 Input Selection - TCB0.
pub const ASYNCUSER0: *mut u8 = 0x12 as *mut u8;

/// Fault Control.
pub const FAULTCTRL: *mut u8 = 0x12 as *mut u8;

/// Window comparator low threshold low byte.
pub const WINLTL: *mut u8 = 0x12 as *mut u8;

/// Pin 2 Control.
pub const PIN2CTRL: *mut u8 = 0x12 as *mut u8;

/// PIT Interrupt Control.
pub const PITINTCTRL: *mut u8 = 0x12 as *mut u8;

/// Window comparator low threshold.
pub const WINLT: *mut u16 = 0x12 as *mut u16;

/// PIT Interrupt Flags.
pub const PITINTFLAGS: *mut u8 = 0x13 as *mut u8;

/// Window comparator low threshold high byte.
pub const WINLTH: *mut u8 = 0x13 as *mut u8;

/// Pin 3 Control.
pub const PIN3CTRL: *mut u8 = 0x13 as *mut u8;

/// Asynchronous User Ch 1 Input Selection - ADC0.
pub const ASYNCUSER1: *mut u8 = 0x13 as *mut u8;

/// User Row Byte 19.
pub const USERROW19: *mut u8 = 0x13 as *mut u8;

/// Delay Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DLYPRESC | 110000 |
/// | DLYSEL | 11 |
/// | DLYTRIG | 1100 |
pub const DLYCTRL: *mut u8 = 0x14 as *mut u8;

/// Pin 4 Control.
pub const PIN4CTRL: *mut u8 = 0x14 as *mut u8;

/// User Row Byte 20.
pub const USERROW20: *mut u8 = 0x14 as *mut u8;

/// Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0.
pub const ASYNCUSER2: *mut u8 = 0x14 as *mut u8;

/// Window comparator high threshold.
pub const WINHT: *mut u16 = 0x14 as *mut u16;

/// Window comparator high threshold low byte.
pub const WINHTL: *mut u8 = 0x14 as *mut u8;

/// Delay value.
pub const DLYVAL: *mut u8 = 0x15 as *mut u8;

/// Pin 5 Control.
pub const PIN5CTRL: *mut u8 = 0x15 as *mut u8;

/// PIT Debug control.
pub const PITDBGCTRL: *mut u8 = 0x15 as *mut u8;

/// Window comparator high threshold high byte.
pub const WINHTH: *mut u8 = 0x15 as *mut u8;

/// User Row Byte 21.
pub const USERROW21: *mut u8 = 0x15 as *mut u8;

/// Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0.
pub const ASYNCUSER3: *mut u8 = 0x15 as *mut u8;

/// Pin 6 Control.
pub const PIN6CTRL: *mut u8 = 0x16 as *mut u8;

/// User Row Byte 22.
pub const USERROW22: *mut u8 = 0x16 as *mut u8;

/// Asynchronous User Ch 4 Input Selection - CCL LUT0 Event 1.
pub const ASYNCUSER4: *mut u8 = 0x16 as *mut u8;

/// Calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DUTYCYC | 1 |
pub const CALIB: *mut u8 = 0x16 as *mut u8;

/// User Row Byte 23.
pub const USERROW23: *mut u8 = 0x17 as *mut u8;

/// Pin 7 Control.
pub const PIN7CTRL: *mut u8 = 0x17 as *mut u8;

/// Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1.
pub const ASYNCUSER5: *mut u8 = 0x17 as *mut u8;

/// Asynchronous User Ch 6 Input Selection - TCD0 Event 0.
pub const ASYNCUSER6: *mut u8 = 0x18 as *mut u8;

/// User Row Byte 24.
pub const USERROW24: *mut u8 = 0x18 as *mut u8;

/// Dither Control A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DITHERSEL | 11 |
pub const DITCTRL: *mut u8 = 0x18 as *mut u8;

/// OSC32K Control A.
pub const OSC32KCTRLA: *mut u8 = 0x18 as *mut u8;

/// User Row Byte 25.
pub const USERROW25: *mut u8 = 0x19 as *mut u8;

/// Asynchronous User Ch 7 Input Selection - TCD0 Event 1.
pub const ASYNCUSER7: *mut u8 = 0x19 as *mut u8;

/// Dither value.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DITHER | 1111 |
pub const DITVAL: *mut u8 = 0x19 as *mut u8;

/// Asynchronous User Ch 8 Input Selection - Event Out 0.
pub const ASYNCUSER8: *mut u8 = 0x1A as *mut u8;

/// User Row Byte 26.
pub const USERROW26: *mut u8 = 0x1A as *mut u8;

/// Asynchronous User Ch 9 Input Selection - Event Out 1.
pub const ASYNCUSER9: *mut u8 = 0x1B as *mut u8;

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
/// | SEL | 100 |
/// | CSUT | 110000 |
pub const XOSC32KCTRLA: *mut u8 = 0x1C as *mut u8;

/// Asynchronous User Ch 10 Input Selection - Event Out 2.
pub const ASYNCUSER10: *mut u8 = 0x1C as *mut u8;

/// User Row Byte 29.
pub const USERROW29: *mut u8 = 0x1D as *mut u8;

/// User Row Byte 30.
pub const USERROW30: *mut u8 = 0x1E as *mut u8;

/// User Row Byte 31.
pub const USERROW31: *mut u8 = 0x1F as *mut u8;

/// Low Count.
pub const LCNT: *mut u8 = 0x20 as *mut u8;

/// Temperature Sensor Calibration Byte 0.
pub const TEMPSENSE0: *mut u8 = 0x20 as *mut u8;

/// High Count.
pub const HCNT: *mut u8 = 0x21 as *mut u8;

/// Temperature Sensor Calibration Byte 1.
pub const TEMPSENSE1: *mut u8 = 0x21 as *mut u8;

/// Capture A low byte.
pub const CAPTUREAL: *mut u8 = 0x22 as *mut u8;

/// OSC16 error at 3V.
pub const OSC16ERR3V: *mut u8 = 0x22 as *mut u8;

/// Synchronous User Ch 0 Input Selection - TCA0.
pub const SYNCUSER0: *mut u8 = 0x22 as *mut u8;

/// Capture A.
pub const CAPTUREA: *mut u16 = 0x22 as *mut u16;

/// Synchronous User Ch 1 Input Selection - USART0.
pub const SYNCUSER1: *mut u8 = 0x23 as *mut u8;

/// Capture A high byte.
pub const CAPTUREAH: *mut u8 = 0x23 as *mut u8;

/// OSC16 error at 5V.
pub const OSC16ERR5V: *mut u8 = 0x23 as *mut u8;

/// Capture B low byte.
pub const CAPTUREBL: *mut u8 = 0x24 as *mut u8;

/// OSC20 error at 3V.
pub const OSC20ERR3V: *mut u8 = 0x24 as *mut u8;

/// Capture B.
pub const CAPTUREB: *mut u16 = 0x24 as *mut u16;

/// Capture B high byte.
pub const CAPTUREBH: *mut u8 = 0x25 as *mut u8;

/// OSC20 error at 5V.
pub const OSC20ERR5V: *mut u8 = 0x25 as *mut u8;

/// Period.
pub const PER: *mut u16 = 0x26 as *mut u16;

/// Period low byte.
pub const PERL: *mut u8 = 0x26 as *mut u8;

/// Low Period.
pub const LPER: *mut u8 = 0x26 as *mut u8;

/// High Period.
pub const HPER: *mut u8 = 0x27 as *mut u8;

/// Period high byte.
pub const PERH: *mut u8 = 0x27 as *mut u8;

/// Compare 0 low byte.
pub const CMP0L: *mut u8 = 0x28 as *mut u8;

/// Compare A Set.
pub const CMPASET: *mut u16 = 0x28 as *mut u16;

/// Compare 0.
pub const CMP0: *mut u16 = 0x28 as *mut u16;

/// Compare A Set low byte.
pub const CMPASETL: *mut u8 = 0x28 as *mut u8;

/// Low Compare.
pub const LCMP0: *mut u8 = 0x28 as *mut u8;

/// Compare 0 high byte.
pub const CMP0H: *mut u8 = 0x29 as *mut u8;

/// Compare A Set high byte.
pub const CMPASETH: *mut u8 = 0x29 as *mut u8;

/// High Compare.
pub const HCMP0: *mut u8 = 0x29 as *mut u8;

/// Compare A Clear.
pub const CMPACLR: *mut u16 = 0x2A as *mut u16;

/// Compare 1.
pub const CMP1: *mut u16 = 0x2A as *mut u16;

/// Compare A Clear low byte.
pub const CMPACLRL: *mut u8 = 0x2A as *mut u8;

/// Low Compare.
pub const LCMP1: *mut u8 = 0x2A as *mut u8;

/// Compare 1 low byte.
pub const CMP1L: *mut u8 = 0x2A as *mut u8;

/// High Compare.
pub const HCMP1: *mut u8 = 0x2B as *mut u8;

/// Compare 1 high byte.
pub const CMP1H: *mut u8 = 0x2B as *mut u8;

/// Compare A Clear high byte.
pub const CMPACLRH: *mut u8 = 0x2B as *mut u8;

/// Compare B Set.
pub const CMPBSET: *mut u16 = 0x2C as *mut u16;

/// Low Compare.
pub const LCMP2: *mut u8 = 0x2C as *mut u8;

/// Compare 2 low byte.
pub const CMP2L: *mut u8 = 0x2C as *mut u8;

/// Compare 2.
pub const CMP2: *mut u16 = 0x2C as *mut u16;

/// Compare B Set low byte.
pub const CMPBSETL: *mut u8 = 0x2C as *mut u8;

/// Compare 2 high byte.
pub const CMP2H: *mut u8 = 0x2D as *mut u8;

/// High Compare.
pub const HCMP2: *mut u8 = 0x2D as *mut u8;

/// Compare B Set high byte.
pub const CMPBSETH: *mut u8 = 0x2D as *mut u8;

/// Compare B Clear low byte.
pub const CMPBCLRL: *mut u8 = 0x2E as *mut u8;

/// Compare B Clear.
pub const CMPBCLR: *mut u16 = 0x2E as *mut u16;

/// Compare B Clear high byte.
pub const CMPBCLRH: *mut u8 = 0x2F as *mut u8;

/// Period Buffer.
pub const PERBUF: *mut u16 = 0x36 as *mut u16;

/// Period Buffer low byte.
pub const PERBUFL: *mut u8 = 0x36 as *mut u8;

/// Period Buffer high byte.
pub const PERBUFH: *mut u8 = 0x37 as *mut u8;

/// Compare 0 Buffer.
pub const CMP0BUF: *mut u16 = 0x38 as *mut u16;

/// Compare 0 Buffer low byte.
pub const CMP0BUFL: *mut u8 = 0x38 as *mut u8;

/// Compare 0 Buffer high byte.
pub const CMP0BUFH: *mut u8 = 0x39 as *mut u8;

/// Compare 1 Buffer low byte.
pub const CMP1BUFL: *mut u8 = 0x3A as *mut u8;

/// Compare 1 Buffer.
pub const CMP1BUF: *mut u16 = 0x3A as *mut u16;

/// Compare 1 Buffer high byte.
pub const CMP1BUFH: *mut u8 = 0x3B as *mut u8;

/// Compare 2 Buffer low byte.
pub const CMP2BUFL: *mut u8 = 0x3C as *mut u8;

/// Compare 2 Buffer.
pub const CMP2BUF: *mut u16 = 0x3C as *mut u16;

/// Compare 2 Buffer high byte.
pub const CMP2BUFH: *mut u8 = 0x3D as *mut u8;

/// Bitfield on register `BODCFG`
pub const SAMPFREQ: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BODCFG`
pub const SLEEP: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `BODCFG`
pub const LVL: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `BODCFG`
pub const ACTIVE: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CALIB`
pub const DUTYCYC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `COMMAND`
pub const STCONV: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLB`
pub const DAC0REFEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLB`
pub const ADC0REFEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLD`
pub const CMPBVAL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CTRLD`
pub const CMPAVAL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CTRLE`
pub const SCAPTUREA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CTRLE`
pub const SYNC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLE`
pub const DISEOC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CTRLE`
pub const RESTART: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLE`
pub const SYNCEOC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLE`
pub const SCAPTUREB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DBGCTRL`
pub const ABMBP: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DITCTRL`
pub const DITHERSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DITVAL`
pub const DITHER: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DLYCTRL`
pub const DLYPRESC: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DLYCTRL`
pub const DLYSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DLYCTRL`
pub const DLYTRIG: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EVCTRL`
pub const IREI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EXTBRK`
pub const ENEXTBRK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const TRIGA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const TRIGB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `INTCTRL`
pub const OVF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCLKCTRLA`
pub const CLKOUT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCLKCTRLB`
pub const PEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCLKCTRLB`
pub const PDIV: *mut u8 = 0x1E as *mut u8;

/// Bitfield on register `MCLKLOCK`
pub const LOCKEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const OSC32KS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const EXTS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const SOSC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const OSC20MS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCLKSTATUS`
pub const XOSC32KS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const TIMEOUT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MCTRLA`
pub const RIEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const QCEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCTRLA`
pub const WIEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCTRLB`
pub const FLUSH: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCTRLB`
pub const MCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const ARBLOST: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const WIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const BUSSTATE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MSTATUS`
pub const RIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MUXCTRLA`
pub const INVERT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MUXCTRLA`
pub const MUXNEG: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `OSC20MCALIBA`
pub const CAL20M: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `OSC20MCALIBA`
pub const CALSEL20M: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `OSC20MCALIBB`
pub const TEMPCAL20M: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `OSCCFG`
pub const OSCLOCK: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `OSCCFG`
pub const FREQSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PITCTRLA`
pub const PITEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PITSTATUS`
pub const CTRLBUSY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFR`
pub const SWRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSTFR`
pub const UPDIRF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSTFR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSTFR`
pub const EXTRF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSTFR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFR`
pub const BORF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const PERR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const FERR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const BUFOVF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RXDATAH`
pub const RXCIF: *mut u8 = 0x80 as *mut u8;

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
pub const DIEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SCTRLA`
pub const PIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SCTRLA`
pub const APIEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SCTRLB`
pub const SCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SEQCTRL0`
pub const SEQSEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const AP: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const COLL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const APIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSTATUS`
pub const DIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `STATUS`
pub const SYNCBUSY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SWRR`
pub const SWRE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SYSCFG0`
pub const EESAVE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SYSCFG0`
pub const RSTPINCFG: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `SYSCFG0`
pub const CRCSRC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `SYSCFG1`
pub const SUT: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `VLMCTRLA`
pub const VLMLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `XOSC32KCTRLA`
pub const SEL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `XOSC32KCTRLA`
pub const CSUT: *mut u8 = 0x30 as *mut u8;

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
   /// Voltage Reference.
   pub const VREF: u32 = 0x2;
   /// DAC output.
   pub const DAC: u32 = 0x3;
}

/// Positive Input MUX Selection select
#[allow(non_upper_case_globals)]
pub mod ac_muxpos {
   /// Positive Pin 0.
   pub const PIN0: u32 = 0x0;
   /// Positive Pin 1.
   pub const PIN1: u32 = 0x1;
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
   /// DAC0.
   pub const DAC0: u32 = 0x1C;
   /// Internal Ref.
   pub const INTREF: u32 = 0x1D;
   /// Temp sensor.
   pub const TEMPSENSE: u32 = 0x1E;
   /// GND.
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
   /// Enabled with wakeup halt.
   pub const ENWAKE: u32 = 0x3;
}

/// Bod level select
#[allow(non_upper_case_globals)]
pub mod bod_lvl {
   /// 1.8 V.
   pub const BODLEVEL0: u32 = 0x0;
   /// 2.1 V.
   pub const BODLEVEL1: u32 = 0x1;
   /// 2.6 V.
   pub const BODLEVEL2: u32 = 0x2;
   /// 2.9 V.
   pub const BODLEVEL3: u32 = 0x3;
   /// 3.3 V.
   pub const BODLEVEL4: u32 = 0x4;
   /// 3.7 V.
   pub const BODLEVEL5: u32 = 0x5;
   /// 4.0 V.
   pub const BODLEVEL6: u32 = 0x6;
   /// 4.2 V.
   pub const BODLEVEL7: u32 = 0x7;
}

/// Sample frequency select
#[allow(non_upper_case_globals)]
pub mod bod_sampfreq {
   /// 1kHz sampling.
   pub const _1KHZ: u32 = 0x0;
   /// 125Hz sampling.
   pub const _125Hz: u32 = 0x1;
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
   /// Event input source 0.
   pub const EVENT0: u32 = 0x3;
   /// Event input source 1.
   pub const EVENT1: u32 = 0x4;
   /// IO pin LUTn-IN0 input source.
   pub const IO: u32 = 0x5;
   /// AC0 OUT input source.
   pub const AC0: u32 = 0x6;
   /// TCB0 WO input source.
   pub const TCB0: u32 = 0x7;
   /// TCA0 WO0 input source.
   pub const TCA0: u32 = 0x8;
   /// TCD0 WOA input source.
   pub const TCD0: u32 = 0x9;
   /// USART0 XCK input source.
   pub const USART0: u32 = 0xA;
   /// SPI0 SCK source.
   pub const SPI0: u32 = 0xB;
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
   /// Event input source 0.
   pub const EVENT0: u32 = 0x3;
   /// Event input source 1.
   pub const EVENT1: u32 = 0x4;
   /// IO pin LUTn-N1 input source.
   pub const IO: u32 = 0x5;
   /// AC0 OUT input source.
   pub const AC0: u32 = 0x6;
   /// TCB0 WO input source.
   pub const TCB0: u32 = 0x7;
   /// TCA0 WO1 input source.
   pub const TCA0: u32 = 0x8;
   /// TCD0 WOB input source.
   pub const TCD0: u32 = 0x9;
   /// USART0 TXD input source.
   pub const USART0: u32 = 0xA;
   /// SPI0 MOSI input source.
   pub const SPI0: u32 = 0xB;
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
   /// Event input source 0.
   pub const EVENT0: u32 = 0x3;
   /// Event input source 1.
   pub const EVENT1: u32 = 0x4;
   /// IO pin LUTn-IN2 input source.
   pub const IO: u32 = 0x5;
   /// AC0 OUT input source.
   pub const AC0: u32 = 0x6;
   /// TCB0 WO input source.
   pub const TCB0: u32 = 0x7;
   /// TCA0 WO2 input source.
   pub const TCA0: u32 = 0x8;
   /// TCD0 WOA input source.
   pub const TCD0: u32 = 0x9;
   /// SPI0 MISO source.
   pub const SPI0: u32 = 0xB;
}

/// Sequential Selection select
#[allow(non_upper_case_globals)]
pub mod ccl_seqsel {
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
   /// 20MHz internal oscillator.
   pub const OSC20M: u32 = 0x0;
   /// 32KHz internal Ultra Low Power oscillator.
   pub const OSCULP32K: u32 = 0x1;
   /// 32.768kHz external crystal oscillator.
   pub const XOSC32K: u32 = 0x2;
   /// External clock.
   pub const EXTCLK: u32 = 0x3;
}

/// Crystal startup time select
#[allow(non_upper_case_globals)]
pub mod clkctrl_csut {
   /// 1K cycles.
   pub const _1K: u32 = 0x0;
   /// 16K cycles.
   pub const _16K: u32 = 0x1;
   /// 32K cycles.
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

/// CRC Flash Access Mode select
#[allow(non_upper_case_globals)]
pub mod crcscan_mode {
   /// Priority to flash.
   pub const PRIORITY: u32 = 0x0;
   /// Reserved.
   pub const RESERVED: u32 = 0x1;
   /// Lowest priority to flash.
   pub const BACKGROUND: u32 = 0x2;
   /// Continuous checks in background.
   pub const CONTINUOUS: u32 = 0x3;
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

/// Asynchronous Channel 0 Generator Selection select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncch0 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Configurable Custom Logic LUT0.
   pub const CCL_LUT0: u32 = 0x1;
   /// Configurable Custom Logic LUT1.
   pub const CCL_LUT1: u32 = 0x2;
   /// Analog Comparator 0 out.
   pub const AC0_OUT: u32 = 0x3;
   /// Timer/Counter D0 compare B clear.
   pub const TCD0_CMPBCLR: u32 = 0x4;
   /// Timer/Counter D0 compare A set.
   pub const TCD0_CMPASET: u32 = 0x5;
   /// Timer/Counter D0 compare B set.
   pub const TCD0_CMPBSET: u32 = 0x6;
   /// Timer/Counter D0 program event.
   pub const TCD0_PROGEV: u32 = 0x7;
   /// Real Time Counter overflow.
   pub const RTC_OVF: u32 = 0x8;
   /// Real Time Counter compare.
   pub const RTC_CMP: u32 = 0x9;
   /// Asynchronous Event from Pin PA0.
   pub const PORTA_PIN0: u32 = 0xA;
   /// Asynchronous Event from Pin PA1.
   pub const PORTA_PIN1: u32 = 0xB;
   /// Asynchronous Event from Pin PA2.
   pub const PORTA_PIN2: u32 = 0xC;
   /// Asynchronous Event from Pin PA3.
   pub const PORTA_PIN3: u32 = 0xD;
   /// Asynchronous Event from Pin PA4.
   pub const PORTA_PIN4: u32 = 0xE;
   /// Asynchronous Event from Pin PA5.
   pub const PORTA_PIN5: u32 = 0xF;
   /// Asynchronous Event from Pin PA6.
   pub const PORTA_PIN6: u32 = 0x10;
   /// Asynchronous Event from Pin PA7.
   pub const PORTA_PIN7: u32 = 0x11;
   /// Unified Program and debug interface.
   pub const UPDI: u32 = 0x12;
}

/// Asynchronous Channel 1 Generator Selection select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncch1 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Configurable custom logic LUT0.
   pub const CCL_LUT0: u32 = 0x1;
   /// Configurable custom logic LUT1.
   pub const CCL_LUT1: u32 = 0x2;
   /// Analog Comparator 0 out.
   pub const AC0_OUT: u32 = 0x3;
   /// Timer/Counter D0 compare B clear.
   pub const TCD0_CMPBCLR: u32 = 0x4;
   /// Timer/Counter D0 compare A set.
   pub const TCD0_CMPASET: u32 = 0x5;
   /// Timer/Counter D0 compare B set.
   pub const TCD0_CMPBSET: u32 = 0x6;
   /// Timer/Counter D0 program event.
   pub const TCD0_PROGEV: u32 = 0x7;
   /// Real Time Counter overflow.
   pub const RTC_OVF: u32 = 0x8;
   /// Real Time Counter compare.
   pub const RTC_CMP: u32 = 0x9;
   /// Asynchronous Event from Pin PB0.
   pub const PORTB_PIN0: u32 = 0xA;
   /// Asynchronous Event from Pin PB1.
   pub const PORTB_PIN1: u32 = 0xB;
   /// Asynchronous Event from Pin PB2.
   pub const PORTB_PIN2: u32 = 0xC;
   /// Asynchronous Event from Pin PB3.
   pub const PORTB_PIN3: u32 = 0xD;
   /// Asynchronous Event from Pin PB4.
   pub const PORTB_PIN4: u32 = 0xE;
   /// Asynchronous Event from Pin PB5.
   pub const PORTB_PIN5: u32 = 0xF;
   /// Asynchronous Event from Pin PB6.
   pub const PORTB_PIN6: u32 = 0x10;
   /// Asynchronous Event from Pin PB7.
   pub const PORTB_PIN7: u32 = 0x11;
}

/// Asynchronous Channel 2 Generator Selection select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncch2 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Configurable Custom Logic LUT0.
   pub const CCL_LUT0: u32 = 0x1;
   /// Configurable Custom Logic LUT1.
   pub const CCL_LUT1: u32 = 0x2;
   /// Analog Comparator 0 out.
   pub const AC0_OUT: u32 = 0x3;
   /// Timer/Counter D0 compare B clear.
   pub const TCD0_CMPBCLR: u32 = 0x4;
   /// Timer/Counter D0 compare A set.
   pub const TCD0_CMPASET: u32 = 0x5;
   /// Timer/Counter D0 compare B set.
   pub const TCD0_CMPBSET: u32 = 0x6;
   /// Timer/Counter D0 program event.
   pub const TCD0_PROGEV: u32 = 0x7;
   /// Real Time Counter overflow.
   pub const RTC_OVF: u32 = 0x8;
   /// Real Time Counter compare.
   pub const RTC_CMP: u32 = 0x9;
   /// Asynchronous Event from Pin PC0.
   pub const PORTC_PIN0: u32 = 0xA;
   /// Asynchronous Event from Pin PC1.
   pub const PORTC_PIN1: u32 = 0xB;
   /// Asynchronous Event from Pin PC2.
   pub const PORTC_PIN2: u32 = 0xC;
   /// Asynchronous Event from Pin PC3.
   pub const PORTC_PIN3: u32 = 0xD;
   /// Asynchronous Event from Pin PC4.
   pub const PORTC_PIN4: u32 = 0xE;
   /// Asynchronous Event from Pin PC5.
   pub const PORTC_PIN5: u32 = 0xF;
}

/// Asynchronous Channel 3 Generator Selection select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncch3 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Configurable custom logic LUT0.
   pub const CCL_LUT0: u32 = 0x1;
   /// Configurable custom logic LUT1.
   pub const CCL_LUT1: u32 = 0x2;
   /// Analog Comparator 0 out.
   pub const AC0_OUT: u32 = 0x3;
   /// Timer/Counter type D compare B clear.
   pub const TCD0_CMPBCLR: u32 = 0x4;
   /// Timer/Counter type D compare A set.
   pub const TCD0_CMPASET: u32 = 0x5;
   /// Timer/Counter type D compare B set.
   pub const TCD0_CMPBSET: u32 = 0x6;
   /// Timer/Counter type D program event.
   pub const TCD0_PROGEV: u32 = 0x7;
   /// Real Time Counter overflow.
   pub const RTC_OVF: u32 = 0x8;
   /// Real Time Counter compare.
   pub const RTC_CMP: u32 = 0x9;
   /// Periodic Interrupt CLK_RTC div 8192.
   pub const PIT_DIV8192: u32 = 0xA;
   /// Periodic Interrupt CLK_RTC div 4096.
   pub const PIT_DIV4096: u32 = 0xB;
   /// Periodic Interrupt CLK_RTC div 2048.
   pub const PIT_DIV2048: u32 = 0xC;
   /// Periodic Interrupt CLK_RTC div 1024.
   pub const PIT_DIV1024: u32 = 0xD;
   /// Periodic Interrupt CLK_RTC div 512.
   pub const PIT_DIV512: u32 = 0xE;
   /// Periodic Interrupt CLK_RTC div 256.
   pub const PIT_DIV256: u32 = 0xF;
   /// Periodic Interrupt CLK_RTC div 128.
   pub const PIT_DIV128: u32 = 0x10;
   /// Periodic Interrupt CLK_RTC div 64.
   pub const PIT_DIV64: u32 = 0x11;
}

/// Asynchronous User Ch 0 Input Selection - TCB0 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser0 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 1 Input Selection - ADC0 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser1 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 10 Input Selection - Event Out 2 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser10 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser2 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser3 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// synchronous User Ch 4 Input Selection - CCL LUT0 Event 1 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser4 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser5 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 6 Input Selection - TCD0 Event 0 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser6 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 7 Input Selection - TCD0 Event 1 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser7 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 8 Input Selection - Event Out 0 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser8 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Asynchronous User Ch 9 Input Selection - Event Out 1 select
#[allow(non_upper_case_globals)]
pub mod evsys_asyncuser9 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
   /// Asynchronous Event Channel 0.
   pub const ASYNCCH0: u32 = 0x3;
   /// Asynchronous Event Channel 1.
   pub const ASYNCCH1: u32 = 0x4;
   /// Asynchronous Event Channel 2.
   pub const ASYNCCH2: u32 = 0x5;
   /// Asynchronous Event Channel 3.
   pub const ASYNCCH3: u32 = 0x6;
}

/// Synchronous Channel 0 Generator Selection select
#[allow(non_upper_case_globals)]
pub mod evsys_syncch0 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Timer/Counter B0.
   pub const TCB0: u32 = 0x1;
   /// Timer/Counter A0 overflow.
   pub const TCA0_OVF_LUNF: u32 = 0x2;
   /// Timer/Counter A0 underflow high byte (split mode).
   pub const TCA0_HUNF: u32 = 0x3;
   /// Timer/Counter A0 compare 0.
   pub const TCA0_CMP0: u32 = 0x4;
   /// Timer/Counter A0 compare 1.
   pub const TCA0_CMP1: u32 = 0x5;
   /// Timer/Counter A0 compare 2.
   pub const TCA0_CMP2: u32 = 0x6;
   /// Synchronous Event from Pin PC0.
   pub const PORTC_PIN0: u32 = 0x7;
   /// Synchronous Event from Pin PC1.
   pub const PORTC_PIN1: u32 = 0x8;
   /// Synchronous Event from Pin PC2.
   pub const PORTC_PIN2: u32 = 0x9;
   /// Synchronous Event from Pin PC3.
   pub const PORTC_PIN3: u32 = 0xA;
   /// Synchronous Event from Pin PC4.
   pub const PORTC_PIN4: u32 = 0xB;
   /// Synchronous Event from Pin PC5.
   pub const PORTC_PIN5: u32 = 0xC;
   /// Synchronous Event from Pin PA0.
   pub const PORTA_PIN0: u32 = 0xD;
   /// Synchronous Event from Pin PA1.
   pub const PORTA_PIN1: u32 = 0xE;
   /// Synchronous Event from Pin PA2.
   pub const PORTA_PIN2: u32 = 0xF;
   /// Synchronous Event from Pin PA3.
   pub const PORTA_PIN3: u32 = 0x10;
   /// Synchronous Event from Pin PA4.
   pub const PORTA_PIN4: u32 = 0x11;
   /// Synchronous Event from Pin PA5.
   pub const PORTA_PIN5: u32 = 0x12;
   /// Synchronous Event from Pin PA6.
   pub const PORTA_PIN6: u32 = 0x13;
   /// Synchronous Event from Pin PA7.
   pub const PORTA_PIN7: u32 = 0x14;
}

/// Synchronous Channel 1 Generator Selection select
#[allow(non_upper_case_globals)]
pub mod evsys_syncch1 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Timer/Counter B0.
   pub const TCB0: u32 = 0x1;
   /// Timer/Counter A0 overflow.
   pub const TCA0_OVF_LUNF: u32 = 0x2;
   /// Timer/Counter A0 underflow high byte (split mode).
   pub const TCA0_HUNF: u32 = 0x3;
   /// Timer/Counter A0 compare 0.
   pub const TCA0_CMP0: u32 = 0x4;
   /// Timer/Counter A0 compare 1.
   pub const TCA0_CMP1: u32 = 0x5;
   /// Timer/Counter A0 compare 2.
   pub const TCA0_CMP2: u32 = 0x6;
   /// Synchronous Event from Pin PB0.
   pub const PORTB_PIN0: u32 = 0x8;
   /// Synchronous Event from Pin PB1.
   pub const PORTB_PIN1: u32 = 0x9;
   /// Synchronous Event from Pin PB2.
   pub const PORTB_PIN2: u32 = 0xA;
   /// Synchronous Event from Pin PB3.
   pub const PORTB_PIN3: u32 = 0xB;
   /// Synchronous Event from Pin PB4.
   pub const PORTB_PIN4: u32 = 0xC;
   /// Synchronous Event from Pin PB5.
   pub const PORTB_PIN5: u32 = 0xD;
   /// Synchronous Event from Pin PB6.
   pub const PORTB_PIN6: u32 = 0xE;
   /// Synchronous Event from Pin PB7.
   pub const PORTB_PIN7: u32 = 0xF;
}

/// Synchronous User Ch 0 Input Selection - TCA0 select
#[allow(non_upper_case_globals)]
pub mod evsys_syncuser0 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
}

/// Synchronous User Ch 1 Input Selection - USART0 select
#[allow(non_upper_case_globals)]
pub mod evsys_syncuser1 {
   /// Off.
   pub const OFF: u32 = 0x0;
   /// Synchronous Event Channel 0.
   pub const SYNCCH0: u32 = 0x1;
   /// Synchronous Event Channel 1.
   pub const SYNCCH1: u32 = 0x2;
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
   /// 2.1 V.
   pub const BODLEVEL1: u32 = 0x1;
   /// 2.6 V.
   pub const BODLEVEL2: u32 = 0x2;
   /// 2.9 V.
   pub const BODLEVEL3: u32 = 0x3;
   /// 3.3 V.
   pub const BODLEVEL4: u32 = 0x4;
   /// 3.7 V.
   pub const BODLEVEL5: u32 = 0x5;
   /// 4.0 V.
   pub const BODLEVEL6: u32 = 0x6;
   /// 4.2 V.
   pub const BODLEVEL7: u32 = 0x7;
}

/// Watchdog Timeout Period select
#[allow(non_upper_case_globals)]
pub mod fuse_period {
   /// Watch-Dog timer Off.
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
   /// UPDI mode.
   pub const UPDI: u32 = 0x1;
   /// Reset mode.
   pub const RST: u32 = 0x2;
}

/// BOD Sample Frequency select
#[allow(non_upper_case_globals)]
pub mod fuse_sampfreq {
   /// 1kHz sampling frequency.
   pub const _1KHz: u32 = 0x0;
   /// 125Hz sampling frequency.
   pub const _125Hz: u32 = 0x1;
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
   /// Window mode off.
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

/// Configurable Custom Logic LUT0 select
#[allow(non_upper_case_globals)]
pub mod portmux_lut0 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Configurable Custom Logic LUT1 select
#[allow(non_upper_case_globals)]
pub mod portmux_lut1 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer SPI0 select
#[allow(non_upper_case_globals)]
pub mod portmux_spi0 {
   /// Default pins.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pins.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCA0 Output 0 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca00 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCA0 output 1 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca01 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCA0 Output 2 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca02 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCA0 Output 3 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca03 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCA0 Output 4 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca04 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCA0 Output 5 select
#[allow(non_upper_case_globals)]
pub mod portmux_tca05 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TCB select
#[allow(non_upper_case_globals)]
pub mod portmux_tcb0 {
   /// Default pin.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pin.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer TWI0 select
#[allow(non_upper_case_globals)]
pub mod portmux_twi0 {
   /// Default pins.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pins.
   pub const ALTERNATE: u32 = 0x1;
}

/// Port Multiplexer USART0 select
#[allow(non_upper_case_globals)]
pub mod portmux_usart0 {
   /// Default pins.
   pub const DEFAULT: u32 = 0x0;
   /// Alternate pins.
   pub const ALTERNATE: u32 = 0x1;
}

/// Input/Sense Configuration select
#[allow(non_upper_case_globals)]
pub mod port_isc {
   /// Iterrupt disabled but input buffer enabled.
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

/// event action select
#[allow(non_upper_case_globals)]
pub mod tcd_action {
   /// Event trigger a fault.
   pub const FAULT: u32 = 0x0;
   /// Event trigger a fault and capture.
   pub const CAPTURE: u32 = 0x1;
}

/// event config select
#[allow(non_upper_case_globals)]
pub mod tcd_cfg {
   /// Neither Filter nor Asynchronous Event is enabled.
   pub const NEITHER: u32 = 0x0;
   /// Input Capture Noise Cancellation Filter enabled.
   pub const FILTER: u32 = 0x1;
   /// Asynchronous Event output qualification enabled.
   pub const ASYNC: u32 = 0x2;
}

/// clock select select
#[allow(non_upper_case_globals)]
pub mod tcd_clksel {
   /// 20 MHz oscillator.
   pub const _20MHZ: u32 = 0x0;
   /// External clock.
   pub const EXTCLK: u32 = 0x2;
   /// System clock.
   pub const SYSCLK: u32 = 0x3;
}

/// Compare C output select select
#[allow(non_upper_case_globals)]
pub mod tcd_cmpcsel {
   /// PWM A output.
   pub const PWMA: u32 = 0x0;
   /// PWM B output.
   pub const PWMB: u32 = 0x1;
}

/// Compare D output select select
#[allow(non_upper_case_globals)]
pub mod tcd_cmpdsel {
   /// PWM A output.
   pub const PWMA: u32 = 0x0;
   /// PWM B output.
   pub const PWMB: u32 = 0x1;
}

/// counter prescaler select
#[allow(non_upper_case_globals)]
pub mod tcd_cntpres {
   /// Sync clock divided by 1.
   pub const DIV1: u32 = 0x0;
   /// Sync clock divided by 4.
   pub const DIV4: u32 = 0x1;
   /// Sync clock divided by 32.
   pub const DIV32: u32 = 0x2;
}

/// dither select select
#[allow(non_upper_case_globals)]
pub mod tcd_dithersel {
   /// On-time ramp B.
   pub const ONTIMEB: u32 = 0x0;
   /// On-time ramp A and B.
   pub const ONTIMEAB: u32 = 0x1;
   /// Dead-time rampB.
   pub const DEADTIMEB: u32 = 0x2;
   /// Dead-time ramp A and B.
   pub const DEADTIMEAB: u32 = 0x3;
}

/// Delay prescaler select
#[allow(non_upper_case_globals)]
pub mod tcd_dlypresc {
   /// No prescaling.
   pub const DIV1: u32 = 0x0;
   /// Prescale with 2.
   pub const DIV2: u32 = 0x1;
   /// Prescale with 4.
   pub const DIV4: u32 = 0x2;
   /// Prescale with 8.
   pub const DIV8: u32 = 0x3;
}

/// Delay select select
#[allow(non_upper_case_globals)]
pub mod tcd_dlysel {
   /// No delay.
   pub const OFF: u32 = 0x0;
   /// Input blanking enabled.
   pub const INBLANK: u32 = 0x1;
   /// Event delay enabled.
   pub const EVENT: u32 = 0x2;
}

/// Delay trigger select
#[allow(non_upper_case_globals)]
pub mod tcd_dlytrig {
   /// Compare A set.
   pub const CMPASET: u32 = 0x0;
   /// Compare A clear.
   pub const CMPACLR: u32 = 0x1;
   /// Compare B set.
   pub const CMPBSET: u32 = 0x2;
   /// Compare B clear.
   pub const CMPBCLR: u32 = 0x3;
}

/// edge select select
#[allow(non_upper_case_globals)]
pub mod tcd_edge {
   /// The falling edge or low level of event generates retrigger or fault action.
   pub const FALL_LOW: u32 = 0x0;
   /// The rising edge or high level of event generates retrigger or fault action.
   pub const RISE_HIGH: u32 = 0x1;
}

/// Input mode select
#[allow(non_upper_case_globals)]
pub mod tcd_inputmode {
   /// Input has no actions.
   pub const NONE: u32 = 0x0;
   /// Stop output, jump to opposite compare cycle and wait.
   pub const JMPWAIT: u32 = 0x1;
   /// Stop output, execute opposite compare cycle and wait.
   pub const EXECWAIT: u32 = 0x2;
   /// stop output, execute opposite compare cycle while fault active.
   pub const EXECFAULT: u32 = 0x3;
   /// Stop all outputs, maintain frequency.
   pub const FREQ: u32 = 0x4;
   /// Stop all outputs, execute dead time while fault active.
   pub const EXECDT: u32 = 0x5;
   /// Stop all outputs, jump to next compare cycle and wait.
   pub const WAIT: u32 = 0x6;
   /// Stop all outputs, wait for software action.
   pub const WAITSW: u32 = 0x7;
   /// Stop output on edge, jump to next compare cycle.
   pub const EDGETRIG: u32 = 0x8;
   /// Stop output on edge, maintain frequency.
   pub const EDGETRIGFREQ: u32 = 0x9;
   /// Stop output at level, maintain frequency.
   pub const LVLTRIGFREQ: u32 = 0xA;
}

/// Syncronization prescaler select
#[allow(non_upper_case_globals)]
pub mod tcd_syncpres {
   /// Selevted clock source divided by 1.
   pub const DIV1: u32 = 0x0;
   /// Selevted clock source divided by 2.
   pub const DIV2: u32 = 0x1;
   /// Selevted clock source divided by 4.
   pub const DIV4: u32 = 0x2;
   /// Selevted clock source divided by 8.
   pub const DIV8: u32 = 0x3;
}

/// Waveform generation mode select
#[allow(non_upper_case_globals)]
pub mod tcd_wgmode {
   /// One ramp mode.
   pub const ONERAMP: u32 = 0x0;
   /// Two ramp mode.
   pub const TWORAMP: u32 = 0x1;
   /// Four ramp mode.
   pub const FOURRAMP: u32 = 0x2;
   /// Dual slope mode.
   pub const DS: u32 = 0x3;
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

/// SDA Hold Time select
#[allow(non_upper_case_globals)]
pub mod twi_sdahold {
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
pub mod twi_sdasetup {
   /// SDA setup time is 4 clock cycles.
   pub const _4CYC: u32 = 0x0;
   /// SDA setup time is 8 clock cycles.
   pub const _8CYC: u32 = 0x1;
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

/// DAC0/AC0 reference select select
#[allow(non_upper_case_globals)]
pub mod vref_dac0refsel {
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
   /// Watch-Dog timer Off.
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
   /// Window mode off.
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

