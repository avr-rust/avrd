//! The AVR ATxmega256A3BU microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATxmega256A3BU-AU | QFP-QFN-64 | TQFP64 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//! | ATxmega256A3BU-MH | QFP-QFN-64 | QFN64 | -40°C - 85°C | 1.6V - 3.6V | 32 MHz |
//!

#![allow(non_upper_case_globals)]

/// Analog Comparator 0 Control.
pub const AC0CTRL: *mut u8 = 0x0 as *mut u8;

/// JTAG User ID.
pub const FUSEBYTE0: *mut u8 = 0x0 as *mut u8;

/// Device ID byte 0.
pub const DEVID0: *mut u8 = 0x0 as *mut u8;

/// Event Channel 0 Multiplexer.
pub const CH0MUX: *mut u8 = 0x0 as *mut u8;

/// Address Register 0.
pub const ADDR0: *mut u8 = 0x0 as *mut u8;

/// Control Register.
pub const CTRL: *mut u8 = 0x0 as *mut u8;

/// OCD Register 0.
pub const OCDR0: *mut u8 = 0x0 as *mut u8;

/// RCOSC 2 MHz Calibration Value B.
pub const RCOSC2M: *mut u8 = 0x0 as *mut u8;

/// Multi-pin Configuration Mask.
pub const MPCMASK: *mut u8 = 0x0 as *mut u8;

/// Lock Bits.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLBB | 11000000 |
/// | BLBAT | 1100 |
/// | BLBA | 110000 |
/// | LB | 11 |
pub const LOCKBITS: *mut u8 = 0x0 as *mut u8;

/// I/O Port Data Direction.
pub const DIR: *mut u8 = 0x0 as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x0 as *mut u8;

/// General Power Reduction.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVSYS | 10 |
/// | AES | 10000 |
/// | USB | 1000000 |
/// | RTC | 100 |
/// | DMA | 1 |
pub const PRGEN: *mut u8 = 0x0 as *mut u8;

/// IrDA Transmitter Pulse Length Control Register.
pub const TXPLCTRL: *mut u8 = 0x1 as *mut u8;

/// Interrupt Priority.
pub const INTPRI: *mut u8 = 0x1 as *mut u8;

/// Prescaler Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSADIV | 1111100 |
/// | PSBCDIV | 11 |
pub const PSCTRL: *mut u8 = 0x1 as *mut u8;

/// Device ID byte 1.
pub const DEVID1: *mut u8 = 0x1 as *mut u8;

/// Watchdog Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 1111 |
/// | WDWP | 11110000 |
pub const FUSEBYTE1: *mut u8 = 0x1 as *mut u8;

/// Synchronization Control/Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYNCCNT | 10000 |
/// | SYNCBUSY | 1 |
pub const SYNCCTRL: *mut u8 = 0x1 as *mut u8;

/// MUX Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUXINT | 1111000 |
pub const MUXCTRL: *mut u8 = 0x1 as *mut u8;

/// Power Reduction Port A.
pub const PRPA: *mut u8 = 0x1 as *mut u8;

/// I/O Port Data Direction Set.
pub const DIRSET: *mut u8 = 0x1 as *mut u8;

/// RCOSC 2 MHz Calibration Value A.
pub const RCOSC2MA: *mut u8 = 0x1 as *mut u8;

/// Interrupt Control Register.
pub const INTCTRL: *mut u8 = 0x1 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x1 as *mut u8;

/// Event Channel 1 Multiplexer.
pub const CH1MUX: *mut u8 = 0x1 as *mut u8;

/// OCD Register 1.
pub const OCDR1: *mut u8 = 0x1 as *mut u8;

/// Address Register 1.
pub const ADDR1: *mut u8 = 0x1 as *mut u8;

/// Analog Comparator 1 Control.
pub const AC1CTRL: *mut u8 = 0x1 as *mut u8;

/// IrDA Receiver Pulse Length Control Register.
pub const RXPLCTRL: *mut u8 = 0x2 as *mut u8;

/// Reset Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOSCSEL | 100000 |
/// | BODPD | 11 |
/// | BOOTRST | 1000000 |
pub const FUSEBYTE2: *mut u8 = 0x2 as *mut u8;

/// Backup Register 0.
pub const BACKUP0: *mut u8 = 0x2 as *mut u8;

/// Address Register 2.
pub const ADDR2: *mut u8 = 0x2 as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WRCOL | 1000000 |
/// | IF | 10000000 |
pub const STATUS: *mut u8 = 0x2 as *mut u8;

/// Power Reduction Port B.
pub const PRPB: *mut u8 = 0x2 as *mut u8;

/// Event Channel 2 Multiplexer.
pub const CH2MUX: *mut u8 = 0x2 as *mut u8;

/// Lock register.
pub const LOCK: *mut u8 = 0x2 as *mut u8;

/// AES State Register.
pub const STATE: *mut u8 = 0x2 as *mut u8;

/// Virtual Port Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VP1MAP | 11110000 |
/// | VP0MAP | 1111 |
pub const VPCTRLA: *mut u8 = 0x2 as *mut u8;

/// Fault Detection Event Mask.
pub const FDEMASK: *mut u8 = 0x2 as *mut u8;

/// Calibration Register A.
pub const CALA: *mut u8 = 0x2 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x2 as *mut u8;

/// External Oscillator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRQRANGE | 11000000 |
/// | X32KLPM | 100000 |
/// | XOSCPWR | 10000 |
/// | XOSCSEL | 1111 |
pub const XOSCCTRL: *mut u8 = 0x2 as *mut u8;

/// I/O Port Data Direction Clear.
pub const DIRCLR: *mut u8 = 0x2 as *mut u8;

/// RCOSC 32.768 kHz Calibration Value.
pub const RCOSC32K: *mut u8 = 0x2 as *mut u8;

/// Analog Comparator 0 MUX Control.
pub const AC0MUXCTRL: *mut u8 = 0x2 as *mut u8;

/// Address Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCRELOAD | 11000000 |
/// | DESTRELOAD | 1100 |
/// | SRCDIR | 110000 |
/// | DESTDIR | 11 |
pub const ADDRCTRL: *mut u8 = 0x2 as *mut u8;

/// Reference Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BANDGAP | 10 |
/// | REFSEL | 1110000 |
/// | TEMPREF | 1 |
pub const REFCTRL: *mut u8 = 0x2 as *mut u8;

/// Device ID byte 2.
pub const DEVID2: *mut u8 = 0x2 as *mut u8;

/// Event Input Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVSPLIT | 1000 |
pub const EVCTRL: *mut u8 = 0x3 as *mut u8;

/// Channel Trigger Source.
pub const TRIGSRC: *mut u8 = 0x3 as *mut u8;

/// RTC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTCEN | 1 |
/// | RTCSRC | 1110 |
pub const RTCCTRL: *mut u8 = 0x3 as *mut u8;

/// RCOSC 32 MHz Calibration Value B.
pub const RCOSC32M: *mut u8 = 0x3 as *mut u8;

/// Backup Register 1.
pub const BACKUP1: *mut u8 = 0x3 as *mut u8;

/// Revision ID.
pub const REVID: *mut u8 = 0x3 as *mut u8;

/// Control Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EVACT | 11100000 |
/// | EVDLY | 10000 |
pub const CTRLD: *mut u8 = 0x3 as *mut u8;

/// Data Input.
pub const DATAIN: *mut u8 = 0x3 as *mut u8;

/// Event Channel 3 Multiplexer.
pub const CH3MUX: *mut u8 = 0x3 as *mut u8;

/// Oscillator Failure Detection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XOSCFDIF | 10 |
/// | XOSCFDEN | 1 |
/// | PLLFDEN | 100 |
/// | PLLFDIF | 1000 |
pub const XOSCFAIL: *mut u8 = 0x3 as *mut u8;

/// General Purpose IO Register 3.
pub const GPIOR3: *mut u8 = 0x3 as *mut u8;

/// Virtual Port Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VP3MAP | 11110000 |
/// | VP2MAP | 1111 |
pub const VPCTRLB: *mut u8 = 0x3 as *mut u8;

/// Power Reduction Port C.
pub const PRPC: *mut u8 = 0x3 as *mut u8;

/// Analog Comparator 1 MUX Control.
pub const AC1MUXCTRL: *mut u8 = 0x3 as *mut u8;

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

/// Calibration Register B.
pub const CALB: *mut u8 = 0x3 as *mut u8;

/// I/O Port Data Direction Toggle.
pub const DIRTGL: *mut u8 = 0x3 as *mut u8;

/// Address Register.
pub const ADDR: *mut u8 = 0x3 as *mut u8;

/// AES Key Register.
pub const KEY: *mut u8 = 0x3 as *mut u8;

/// Data Register.
pub const DATA: *mut u8 = 0x3 as *mut u8;

/// I/O Port Output.
pub const OUT: *mut u8 = 0x4 as *mut u8;

/// Channel Block Transfer Count low byte.
pub const TRFCNTL: *mut u8 = 0x4 as *mut u8;

/// Control Register E.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BYTEM | 11 |
pub const CTRLE: *mut u8 = 0x4 as *mut u8;

/// Channel Result low byte.
pub const RESL: *mut u8 = 0x4 as *mut u8;

/// Channel Block Transfer Count.
pub const TRFCNT: *mut u16 = 0x4 as *mut u16;

/// Channel Result.
pub const RES: *mut u16 = 0x4 as *mut u16;

/// Oscillator Compare Register 0.
pub const COMP0: *mut u8 = 0x4 as *mut u8;

/// General Purpose IO Register 4.
pub const GPIOR4: *mut u8 = 0x4 as *mut u8;

/// RCOSC 32 MHz Calibration Value A.
pub const RCOSC32MA: *mut u8 = 0x4 as *mut u8;

/// Checksum byte 0.
pub const CHECKSUM0: *mut u8 = 0x4 as *mut u8;

/// USB Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBPSDIV | 111000 |
/// | USBSEN | 1 |
/// | USBSRC | 110 |
pub const USBCTRL: *mut u8 = 0x4 as *mut u8;

/// Clock Prescaler.
pub const PRESCALER: *mut u8 = 0x4 as *mut u8;

/// Baurd Rate Control Register.
pub const BAUD: *mut u8 = 0x4 as *mut u8;

/// Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLK2X | 100 |
/// | MPCM | 10 |
/// | TXEN | 1000 |
/// | RXEN | 10000 |
/// | TXB8 | 1 |
pub const CTRLB: *mut u8 = 0x4 as *mut u8;

/// Power Reduction Port D.
pub const PRPD: *mut u8 = 0x4 as *mut u8;

/// 32.768 kHz Internal Oscillator Calibration Register.
pub const RC32KCAL: *mut u8 = 0x4 as *mut u8;

/// JTAG User ID.
pub const JTAGUID: *mut u8 = 0x4 as *mut u8;

/// Event Channel 4 Multiplexer.
pub const CH4MUX: *mut u8 = 0x4 as *mut u8;

/// Data Pointer low byte.
pub const DATAPTRL: *mut u8 = 0x4 as *mut u8;

/// Data Register 0.
pub const DATA0: *mut u8 = 0x4 as *mut u8;

/// Start-up Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 10000 |
/// | JTAGEN | 1 |
/// | SUT | 1100 |
/// | WDLOCK | 10 |
pub const FUSEBYTE4: *mut u8 = 0x4 as *mut u8;

/// Data Pointer.
pub const DATAPTR: *mut u16 = 0x4 as *mut u16;

/// Clock and Event Out Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOUTSEL | 1100 |
/// | RTCOUT | 1000000 |
/// | EVOUT | 110000 |
/// | CLKOUT | 11 |
/// | CLKEVPIN | 10000000 |
pub const CLKEVOUT: *mut u8 = 0x4 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x4 as *mut u8;

/// FIFO Write Pointer Register.
pub const FIFOWP: *mut u8 = 0x4 as *mut u8;

/// FIFO Read Pointer Register.
pub const FIFORP: *mut u8 = 0x5 as *mut u8;

/// Power Reduction Port E.
pub const PRPE: *mut u8 = 0x5 as *mut u8;

/// I/O Port Output Set.
pub const OUTSET: *mut u8 = 0x5 as *mut u8;

/// General Purpose IO Register 5.
pub const GPIOR5: *mut u8 = 0x5 as *mut u8;

/// Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PMODE | 110000 |
/// | SBMODE | 1000 |
/// | CMODE | 11000000 |
/// | CHSIZE | 111 |
pub const CTRLC: *mut u8 = 0x5 as *mut u8;

/// Channel Block Transfer Count high byte.
pub const TRFCNTH: *mut u8 = 0x5 as *mut u8;

/// Data Pointer high byte.
pub const DATAPTRH: *mut u8 = 0x5 as *mut u8;

/// Event Channel 5 Multiplexer.
pub const CH5MUX: *mut u8 = 0x5 as *mut u8;

/// Data Register 1.
pub const DATA1: *mut u8 = 0x5 as *mut u8;

/// Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADDREN | 1 |
pub const ADDRMASK: *mut u8 = 0x5 as *mut u8;

/// PLL Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLSRC | 11000000 |
/// | PLLDIV | 100000 |
/// | PLLFAC | 11111 |
pub const PLLCTRL: *mut u8 = 0x5 as *mut u8;

/// EESAVE and BOD Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODACT | 110000 |
/// | EESAVE | 1000 |
/// | BODLVL | 111 |
pub const FUSEBYTE5: *mut u8 = 0x5 as *mut u8;

/// Checksum byte 1.
pub const CHECKSUM1: *mut u8 = 0x5 as *mut u8;

/// Status Set Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FDF | 100 |
/// | DTLSBUFV | 1 |
/// | DTHSBUFV | 10 |
pub const STATUSSET: *mut u8 = 0x5 as *mut u8;

/// Channel Result high byte.
pub const RESH: *mut u8 = 0x5 as *mut u8;

/// Oscillator Compare Register 1.
pub const COMP1: *mut u8 = 0x5 as *mut u8;

/// Data Register 2.
pub const DATA2: *mut u8 = 0x6 as *mut u8;

/// I/O Port Output Clear.
pub const OUTCLR: *mut u8 = 0x6 as *mut u8;

/// Input Channel Scan.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OFFSET | 11110000 |
/// | SCANNUM | 1111 |
pub const SCAN: *mut u8 = 0x6 as *mut u8;

/// Auxiliary Data low byte.
pub const AUXDATAL: *mut u8 = 0x6 as *mut u8;

/// Checksum byte 2.
pub const CHECKSUM2: *mut u8 = 0x6 as *mut u8;

/// Endpoint Configuration Table Pointer low byte.
pub const EPPTRL: *mut u8 = 0x6 as *mut u8;

/// Channel Repeat Count.
pub const REPCNT: *mut u8 = 0x6 as *mut u8;

/// Dead Time Both Sides.
pub const DTBOTH: *mut u8 = 0x6 as *mut u8;

/// Power Reduction Port F.
pub const PRPF: *mut u8 = 0x6 as *mut u8;

/// Baud Rate Control Register A.
pub const BAUDCTRLA: *mut u8 = 0x6 as *mut u8;

/// Oscillator Compare Register 2.
pub const COMP2: *mut u8 = 0x6 as *mut u8;

/// Event Output Select.
pub const EVOUTSEL: *mut u8 = 0x6 as *mut u8;

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
/// | HUNFINTLVL | 1100 |
/// | LUNFINTLVL | 11 |
pub const INTCTRLA: *mut u8 = 0x6 as *mut u8;

/// DFLL Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RC32MCREF | 110 |
/// | RC2MCREF | 1 |
pub const DFLLCTRL: *mut u8 = 0x6 as *mut u8;

/// General Purpose IO Register 6.
pub const GPIOR6: *mut u8 = 0x6 as *mut u8;

/// Window Mode Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WEN | 10000 |
/// | WINTMODE | 1100 |
/// | WINTLVL | 11 |
pub const WINCTRL: *mut u8 = 0x6 as *mut u8;

/// Auxiliary Data.
pub const AUXDATA: *mut u16 = 0x6 as *mut u16;

/// Event Channel 6 Multiplexer.
pub const CH6MUX: *mut u8 = 0x6 as *mut u8;

/// Endpoint Configuration Table Pointer.
pub const EPPTR: *mut u16 = 0x6 as *mut u16;

/// General Purpose IO Register 7.
pub const GPIOR7: *mut u8 = 0x7 as *mut u8;

/// Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCMPBINTLVL | 1100 |
/// | LCMPAINTLVL | 11 |
/// | LCMPDINTLVL | 11000000 |
/// | LCMPCINTLVL | 110000 |
pub const INTCTRLB: *mut u8 = 0x7 as *mut u8;

/// I/O Port Output Toggle.
pub const OUTTGL: *mut u8 = 0x7 as *mut u8;

/// Checksum byte 3.
pub const CHECKSUM3: *mut u8 = 0x7 as *mut u8;

/// Baud Rate Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BSCALE | 11110000 |
pub const BAUDCTRLB: *mut u8 = 0x7 as *mut u8;

/// Dead Time Both Sides Buffer.
pub const DTBOTHBUF: *mut u8 = 0x7 as *mut u8;

/// Endpoint Configuration Table Pointer high byte.
pub const EPPTRH: *mut u8 = 0x7 as *mut u8;

/// Auxiliary Data high byte.
pub const AUXDATAH: *mut u8 = 0x7 as *mut u8;

/// Analog Startup Delay.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STARTUPDLYB | 1100 |
/// | STARTUPDLYA | 11 |
pub const ANAINIT: *mut u8 = 0x7 as *mut u8;

/// Event Channel 7 Multiplexer.
pub const CH7MUX: *mut u8 = 0x7 as *mut u8;

/// Channel Source Address 0.
pub const SRCADDR0: *mut u8 = 0x8 as *mut u8;

/// Gain Calibration.
pub const CH0GAINCAL: *mut u8 = 0x8 as *mut u8;

/// General Purpose IO Register 8.
pub const GPIOR8: *mut u8 = 0x8 as *mut u8;

/// Ramp D.
pub const RAMPD: *mut u8 = 0x8 as *mut u8;

/// Dead Time Low Side.
pub const DTLS: *mut u8 = 0x8 as *mut u8;

/// Control Register F Clear.
pub const CTRLFCLR: *mut u8 = 0x8 as *mut u8;

/// Channel 0 Control Register.
pub const CH0CTRL: *mut u8 = 0x8 as *mut u8;

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

/// I/O port Input.
pub const IN: *mut u8 = 0x8 as *mut u8;

/// Channel Source Address 1.
pub const SRCADDR1: *mut u8 = 0x9 as *mut u8;

/// Channel 1 Control Register.
pub const CH1CTRL: *mut u8 = 0x9 as *mut u8;

/// Ramp X.
pub const RAMPX: *mut u8 = 0x9 as *mut u8;

/// Offset Calibration.
pub const CH0OFFSETCAL: *mut u8 = 0x9 as *mut u8;

/// Control Register F Set.
pub const CTRLFSET: *mut u8 = 0x9 as *mut u8;

/// AWEX Lock.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AWEXELOCK | 100 |
/// | AWEXDLOCK | 10 |
/// | AWEXFLOCK | 1000 |
/// | AWEXCLOCK | 1 |
pub const AWEXLOCK: *mut u8 = 0x9 as *mut u8;

/// Control Register F.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMDEN | 11 |
pub const CTRLF: *mut u8 = 0x9 as *mut u8;

/// Lot Number Byte 1, ASCII.
pub const LOTNUM1: *mut u8 = 0x9 as *mut u8;

/// General Purpose IO Register 9.
pub const GPIOR9: *mut u8 = 0x9 as *mut u8;

/// Dead Time High Side.
pub const DTHS: *mut u8 = 0x9 as *mut u8;

/// Command.
pub const CMD: *mut u8 = 0xA as *mut u8;

/// Clear Interrupt Flag Register A.
pub const INTFLAGSACLR: *mut u8 = 0xA as *mut u8;

/// General Purpose IO Register 10.
pub const GPIORA: *mut u8 = 0xA as *mut u8;

/// Control Register G Clear.
pub const CTRLGCLR: *mut u8 = 0xA as *mut u8;

/// Channel 2 Control Register.
pub const CH2CTRL: *mut u8 = 0xA as *mut u8;

/// Ramp Y.
pub const RAMPY: *mut u8 = 0xA as *mut u8;

/// Dead Time Low Side Buffer.
pub const DTLSBUF: *mut u8 = 0xA as *mut u8;

/// Channel Source Address 2.
pub const SRCADDR2: *mut u8 = 0xA as *mut u8;

/// Lot Number Byte 2, ASCII.
pub const LOTNUM2: *mut u8 = 0xA as *mut u8;

/// Gain Calibration.
pub const CH1GAINCAL: *mut u8 = 0xA as *mut u8;

/// Port Interrupt 0 Mask.
pub const INT0MASK: *mut u8 = 0xA as *mut u8;

/// Lot Number Byte 3, ASCII.
pub const LOTNUM3: *mut u8 = 0xB as *mut u8;

/// Ramp Z.
pub const RAMPZ: *mut u8 = 0xB as *mut u8;

/// Offset Calibration.
pub const CH1OFFSETCAL: *mut u8 = 0xB as *mut u8;

/// General Purpose IO Register 11.
pub const GPIORB: *mut u8 = 0xB as *mut u8;

/// Control Register G Set.
pub const CTRLGSET: *mut u8 = 0xB as *mut u8;

/// Dead Time High Side Buffer.
pub const DTHSBUF: *mut u8 = 0xB as *mut u8;

/// Set Interrupt Flag Register A.
pub const INTFLAGSASET: *mut u8 = 0xB as *mut u8;

/// Port Interrupt 1 Mask.
pub const INT1MASK: *mut u8 = 0xB as *mut u8;

/// Channel 3 Control Register.
pub const CH3CTRL: *mut u8 = 0xB as *mut u8;

/// Compare Register.
pub const COMP: *mut u32 = 0xC as *mut u32;

/// Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LUNFIF | 1 |
/// | LCMPBIF | 100000 |
/// | HUNFIF | 10 |
/// | LCMPCIF | 1000000 |
/// | LCMPDIF | 10000000 |
/// | LCMPAIF | 10000 |
pub const INTFLAGS: *mut u8 = 0xC as *mut u8;

/// General Purpose IO Register 12.
pub const GPIORC: *mut u8 = 0xC as *mut u8;

/// Extended Indirect Jump.
pub const EIND: *mut u8 = 0xC as *mut u8;

/// Channel Destination Address 0.
pub const DESTADDR0: *mut u8 = 0xC as *mut u8;

/// Output Override Enable.
pub const OUTOVEN: *mut u8 = 0xC as *mut u8;

/// Clear Interrupt Flag Register B.
pub const INTFLAGSBCLR: *mut u8 = 0xC as *mut u8;

/// Calibration Value.
pub const CAL: *mut u16 = 0xC as *mut u16;

/// Calibration Value low byte.
pub const CALL: *mut u8 = 0xC as *mut u8;

/// Lot Number Byte 4, ASCII.
pub const LOTNUM4: *mut u8 = 0xC as *mut u8;

/// Channel 4 Control Register.
pub const CH4CTRL: *mut u8 = 0xC as *mut u8;

/// Stack Pointer Low.
pub const SPL: *mut u8 = 0xD as *mut u8;

/// Channel 5 Control Register.
pub const CH5CTRL: *mut u8 = 0xD as *mut u8;

/// Channel Destination Address 1.
pub const DESTADDR1: *mut u8 = 0xD as *mut u8;

/// Lot Number Byte 5, ASCII.
pub const LOTNUM5: *mut u8 = 0xD as *mut u8;

/// Calibration Value high byte.
pub const CALH: *mut u8 = 0xD as *mut u8;

/// General Purpose IO Register 13.
pub const GPIORD: *mut u8 = 0xD as *mut u8;

/// Set Interrupt Flag Register B.
pub const INTFLAGSBSET: *mut u8 = 0xD as *mut u8;

/// General Purpose IO Register 14.
pub const GPIORE: *mut u8 = 0xE as *mut u8;

/// Stack Pointer High.
pub const SPH: *mut u8 = 0xE as *mut u8;

/// Channel 6 Control Register.
pub const CH6CTRL: *mut u8 = 0xE as *mut u8;

/// I/O Port Pin Remap Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TC0C | 100 |
/// | TC0B | 10 |
/// | TC0D | 1000 |
/// | TC0A | 1 |
pub const REMAP: *mut u8 = 0xE as *mut u8;

/// Channel Destination Address 2.
pub const DESTADDR2: *mut u8 = 0xE as *mut u8;

/// Temporary Register For 16-bit Access.
pub const TEMP: *mut u8 = 0xF as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | C | 1 |
/// | N | 100 |
/// | V | 1000 |
/// | I | 10000000 |
/// | S | 10000 |
/// | T | 1000000 |
/// | Z | 10 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0xF as *mut u8;

/// General Purpose IO Register 15.
pub const GPIORF: *mut u8 = 0xF as *mut u8;

/// Channel 7 Control Register.
pub const CH7CTRL: *mut u8 = 0xF as *mut u8;

/// Channel 0 Result.
pub const CH0RES: *mut u16 = 0x10 as *mut u16;

/// Wafer Number.
pub const WAFNUM: *mut u8 = 0x10 as *mut u8;

/// Event Strobe.
pub const STROBE: *mut u8 = 0x10 as *mut u8;

/// Pin 0 Control Register.
pub const PIN0CTRL: *mut u8 = 0x10 as *mut u8;

/// Channel 0 Result low byte.
pub const CH0RESL: *mut u8 = 0x10 as *mut u8;

/// Channel 0 Result high byte.
pub const CH0RESH: *mut u8 = 0x11 as *mut u8;

/// Pin 1 Control Register.
pub const PIN1CTRL: *mut u8 = 0x11 as *mut u8;

/// Wafer Coordinate X Byte 0.
pub const COORDX0: *mut u8 = 0x12 as *mut u8;

/// Channel 1 Result low byte.
pub const CH1RESL: *mut u8 = 0x12 as *mut u8;

/// Pin 2 Control Register.
pub const PIN2CTRL: *mut u8 = 0x12 as *mut u8;

/// Channel 1 Result.
pub const CH1RES: *mut u16 = 0x12 as *mut u16;

/// Channel 1 Result high byte.
pub const CH1RESH: *mut u8 = 0x13 as *mut u8;

/// Pin 3 Control Register.
pub const PIN3CTRL: *mut u8 = 0x13 as *mut u8;

/// Wafer Coordinate X Byte 1.
pub const COORDX1: *mut u8 = 0x13 as *mut u8;

/// Channel 2 Result.
pub const CH2RES: *mut u16 = 0x14 as *mut u16;

/// Wafer Coordinate Y Byte 0.
pub const COORDY0: *mut u8 = 0x14 as *mut u8;

/// Pin 4 Control Register.
pub const PIN4CTRL: *mut u8 = 0x14 as *mut u8;

/// Channel 2 Result low byte.
pub const CH2RESL: *mut u8 = 0x14 as *mut u8;

/// Wafer Coordinate Y Byte 1.
pub const COORDY1: *mut u8 = 0x15 as *mut u8;

/// Pin 5 Control Register.
pub const PIN5CTRL: *mut u8 = 0x15 as *mut u8;

/// Channel 2 Result high byte.
pub const CH2RESH: *mut u8 = 0x15 as *mut u8;

/// Pin 6 Control Register.
pub const PIN6CTRL: *mut u8 = 0x16 as *mut u8;

/// Channel 3 Result low byte.
pub const CH3RESL: *mut u8 = 0x16 as *mut u8;

/// Channel 3 Result.
pub const CH3RES: *mut u16 = 0x16 as *mut u16;

/// Channel 3 Result high byte.
pub const CH3RESH: *mut u8 = 0x17 as *mut u8;

/// Pin 7 Control Register.
pub const PIN7CTRL: *mut u8 = 0x17 as *mut u8;

/// Channel 0 Data.
pub const CH0DATA: *mut u16 = 0x18 as *mut u16;

/// Compare Value low byte.
pub const CMPL: *mut u8 = 0x18 as *mut u8;

/// Channel 0 Data low byte.
pub const CH0DATAL: *mut u8 = 0x18 as *mut u8;

/// Compare Value.
pub const CMP: *mut u16 = 0x18 as *mut u16;

/// Channel 0 Data high byte.
pub const CH0DATAH: *mut u8 = 0x19 as *mut u8;

/// Compare Value high byte.
pub const CMPH: *mut u8 = 0x19 as *mut u8;

/// Channel 1 Data.
pub const CH1DATA: *mut u16 = 0x1A as *mut u16;

/// Channel 1 Data low byte.
pub const CH1DATAL: *mut u8 = 0x1A as *mut u8;

/// USB Calibration Byte 0.
pub const USBCAL0: *mut u8 = 0x1A as *mut u8;

/// USB Calibration Byte 1.
pub const USBCAL1: *mut u8 = 0x1B as *mut u8;

/// Channel 1 Data high byte.
pub const CH1DATAH: *mut u8 = 0x1B as *mut u8;

/// USB RCOSC Calibration Value B.
pub const USBRCOSC: *mut u8 = 0x1C as *mut u8;

/// USB RCOSC Calibration Value A.
pub const USBRCOSCA: *mut u8 = 0x1D as *mut u8;

/// ADCA Calibration Byte 0.
pub const ADCACAL0: *mut u8 = 0x20 as *mut u8;

/// Low Byte Count.
pub const LCNT: *mut u8 = 0x20 as *mut u8;

/// Count.
pub const CNT: *mut u16 = 0x20 as *mut u16;

/// Count low byte.
pub const CNTL: *mut u8 = 0x20 as *mut u8;

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

/// Period low byte.
pub const PERL: *mut u8 = 0x26 as *mut u8;

/// Period.
pub const PER: *mut u16 = 0x26 as *mut u16;

/// Low Byte Period.
pub const LPER: *mut u8 = 0x26 as *mut u8;

/// Period high byte.
pub const PERH: *mut u8 = 0x27 as *mut u8;

/// High Byte Period.
pub const HPER: *mut u8 = 0x27 as *mut u8;

/// Compare or Capture A low byte.
pub const CCAL: *mut u8 = 0x28 as *mut u8;

/// Low Byte Compare A.
pub const LCMPA: *mut u8 = 0x28 as *mut u8;

/// Compare or Capture A.
pub const CCA: *mut u16 = 0x28 as *mut u16;

/// Compare or Capture A high byte.
pub const CCAH: *mut u8 = 0x29 as *mut u8;

/// High Byte Compare A.
pub const HCMPA: *mut u8 = 0x29 as *mut u8;

/// Compare or Capture B.
pub const CCB: *mut u16 = 0x2A as *mut u16;

/// Compare or Capture B low byte.
pub const CCBL: *mut u8 = 0x2A as *mut u8;

/// Low Byte Compare B.
pub const LCMPB: *mut u8 = 0x2A as *mut u8;

/// High Byte Compare B.
pub const HCMPB: *mut u8 = 0x2B as *mut u8;

/// Compare or Capture B high byte.
pub const CCBH: *mut u8 = 0x2B as *mut u8;

/// Low Byte Compare C.
pub const LCMPC: *mut u8 = 0x2C as *mut u8;

/// Compare or Capture C low byte.
pub const CCCL: *mut u8 = 0x2C as *mut u8;

/// Compare or Capture C.
pub const CCC: *mut u16 = 0x2C as *mut u16;

/// Compare or Capture C high byte.
pub const CCCH: *mut u8 = 0x2D as *mut u8;

/// High Byte Compare C.
pub const HCMPC: *mut u8 = 0x2D as *mut u8;

/// Compare or Capture D.
pub const CCD: *mut u16 = 0x2E as *mut u16;

/// Compare or Capture D low byte.
pub const CCDL: *mut u8 = 0x2E as *mut u8;

/// Low Byte Compare D.
pub const LCMPD: *mut u8 = 0x2E as *mut u8;

/// Temperature Sensor Calibration Byte 0.
pub const TEMPSENSE0: *mut u8 = 0x2E as *mut u8;

/// Compare or Capture D high byte.
pub const CCDH: *mut u8 = 0x2F as *mut u8;

/// High Byte Compare D.
pub const HCMPD: *mut u8 = 0x2F as *mut u8;

/// Temperature Sensor Calibration Byte 1.
pub const TEMPSENSE1: *mut u8 = 0x2F as *mut u8;

/// DACA0 Calibration Byte 0.
pub const DACA0OFFCAL: *mut u8 = 0x30 as *mut u8;

/// DACA0 Calibration Byte 1.
pub const DACA0GAINCAL: *mut u8 = 0x31 as *mut u8;

/// DACB0 Calibration Byte 0.
pub const DACB0OFFCAL: *mut u8 = 0x32 as *mut u8;

/// DACB0 Calibration Byte 1.
pub const DACB0GAINCAL: *mut u8 = 0x33 as *mut u8;

/// DACA1 Calibration Byte 0.
pub const DACA1OFFCAL: *mut u8 = 0x34 as *mut u8;

/// DACA1 Calibration Byte 1.
pub const DACA1GAINCAL: *mut u8 = 0x35 as *mut u8;

/// Period Buffer low byte.
pub const PERBUFL: *mut u8 = 0x36 as *mut u8;

/// Period Buffer.
pub const PERBUF: *mut u16 = 0x36 as *mut u16;

/// DACB1 Calibration Byte 0.
pub const DACB1OFFCAL: *mut u8 = 0x36 as *mut u8;

/// Period Buffer high byte.
pub const PERBUFH: *mut u8 = 0x37 as *mut u8;

/// DACB1 Calibration Byte 1.
pub const DACB1GAINCAL: *mut u8 = 0x37 as *mut u8;

/// Compare Or Capture A Buffer low byte.
pub const CCABUFL: *mut u8 = 0x38 as *mut u8;

/// Compare Or Capture A Buffer.
pub const CCABUF: *mut u16 = 0x38 as *mut u16;

/// Compare Or Capture A Buffer high byte.
pub const CCABUFH: *mut u8 = 0x39 as *mut u8;

/// Compare Or Capture B Buffer.
pub const CCBBUF: *mut u16 = 0x3A as *mut u16;

/// Calibration Byte 0.
pub const CAL0: *mut u8 = 0x3A as *mut u8;

/// Compare Or Capture B Buffer low byte.
pub const CCBBUFL: *mut u8 = 0x3A as *mut u8;

/// Calibration Byte 1.
pub const CAL1: *mut u8 = 0x3B as *mut u8;

/// Compare Or Capture B Buffer high byte.
pub const CCBBUFH: *mut u8 = 0x3B as *mut u8;

/// Compare Or Capture C Buffer.
pub const CCCBUF: *mut u16 = 0x3C as *mut u16;

/// Compare Or Capture C Buffer low byte.
pub const CCCBUFL: *mut u8 = 0x3C as *mut u8;

/// Compare Or Capture C Buffer high byte.
pub const CCCBUFH: *mut u8 = 0x3D as *mut u8;

/// Compare Or Capture D Buffer.
pub const CCDBUF: *mut u16 = 0x3E as *mut u16;

/// Compare Or Capture D Buffer low byte.
pub const CCDBUFL: *mut u8 = 0x3E as *mut u8;

/// Compare Or Capture D Buffer high byte.
pub const CCDBUFH: *mut u8 = 0x3F as *mut u8;

/// Frame Number Low Byte.
pub const FRAMENUML: *mut u8 = 0x110 as *mut u8;

/// Frame Number High Byte.
pub const FRAMENUMH: *mut u8 = 0x111 as *mut u8;

/// Bitfield on register `ADDRCTRL`
pub const SRCRELOAD: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADDRCTRL`
pub const DESTRELOAD: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `ADDRCTRL`
pub const SRCDIR: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `ADDRCTRL`
pub const DESTDIR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ADDRMASK`
pub const ADDREN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ANAINIT`
pub const STARTUPDLYB: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `ANAINIT`
pub const STARTUPDLYA: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `AWEXLOCK`
pub const AWEXELOCK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `AWEXLOCK`
pub const AWEXDLOCK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `AWEXLOCK`
pub const AWEXFLOCK: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AWEXLOCK`
pub const AWEXCLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BAUDCTRLB`
pub const BSCALE: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const CLKOUTSEL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const RTCOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const EVOUT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const CLKOUT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKEVOUT`
pub const CLKEVPIN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CTRLA`
pub const RXCINTLVL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLA`
pub const DREINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLA`
pub const TXCINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CTRLB`
pub const CLK2X: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CTRLB`
pub const MPCM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CTRLB`
pub const TXEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CTRLB`
pub const RXEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CTRLB`
pub const TXB8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CTRLC`
pub const PMODE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CTRLC`
pub const SBMODE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CTRLC`
pub const CMODE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `CTRLC`
pub const CHSIZE: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CTRLD`
pub const EVACT: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `CTRLD`
pub const EVDLY: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CTRLE`
pub const BYTEM: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CTRLF`
pub const CMDEN: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DFLLCTRL`
pub const RC32MCREF: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `DFLLCTRL`
pub const RC2MCREF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EVCTRL`
pub const EVSPLIT: *mut u8 = 0x8 as *mut u8;

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
pub const TOSCSEL: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const BODPD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FUSEBYTE2`
pub const BOOTRST: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const RSTDISBL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const JTAGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const SUT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `FUSEBYTE4`
pub const WDLOCK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const BODACT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FUSEBYTE5`
pub const BODLVL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const HUNFINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTCTRLA`
pub const LUNFINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPBINTLVL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPAINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPDINTLVL: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `INTCTRLB`
pub const LCMPCINTLVL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LUNFIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPBIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const HUNFIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPCIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `INTFLAGS`
pub const LCMPAIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBB: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBAT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const BLBA: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBITS`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const JTAGD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MUXCTRL`
pub const MUXINT: *mut u8 = 0x78 as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLSRC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLDIV: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PLLCTRL`
pub const PLLFAC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PRGEN`
pub const EVSYS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRGEN`
pub const AES: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRGEN`
pub const USB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRGEN`
pub const RTC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRGEN`
pub const DMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PSCTRL`
pub const PSADIV: *mut u8 = 0x7C as *mut u8;

/// Bitfield on register `PSCTRL`
pub const PSBCDIV: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const BANDGAP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const REFSEL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `REFCTRL`
pub const TEMPREF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0C: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0B: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `REMAP`
pub const TC0A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RTCCTRL`
pub const RTCEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RTCCTRL`
pub const RTCSRC: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SCAN`
pub const OFFSET: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SCAN`
pub const SCANNUM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `STATUS`
pub const WRCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `STATUS`
pub const IF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `STATUSSET`
pub const FDF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `STATUSSET`
pub const DTLSBUFV: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `STATUSSET`
pub const DTHSBUFV: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SYNCCTRL`
pub const SYNCCNT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SYNCCTRL`
pub const SYNCBUSY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USBCTRL`
pub const USBPSDIV: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `USBCTRL`
pub const USBSEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USBCTRL`
pub const USBSRC: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `VPCTRLA`
pub const VP1MAP: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `VPCTRLA`
pub const VP0MAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VPCTRLB`
pub const VP3MAP: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `VPCTRLB`
pub const VP2MAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WINTMODE: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `WINCTRL`
pub const WINTLVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const FRQRANGE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const X32KLPM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const XOSCPWR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `XOSCCTRL`
pub const XOSCSEL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const XOSCFDIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const XOSCFDEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const PLLFDEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `XOSCFAIL`
pub const PLLFDIF: *mut u8 = 0x8 as *mut u8;

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
   /// Input pin 12.
   pub const PIN12: u32 = 0xC;
   /// Input pin 13.
   pub const PIN13: u32 = 0xD;
   /// Input pin 14.
   pub const PIN14: u32 = 0xE;
   /// Input pin 15.
   pub const PIN15: u32 = 0xF;
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

/// Channel sweep selection
#[allow(non_upper_case_globals)]
pub mod adc_sweep {
   /// ADC Channel 0.
   pub const _0: u32 = 0x0;
   /// ADC Channel 0,1.
   pub const _01: u32 = 0x1;
   /// ADC Channel 0,1,2.
   pub const _012: u32 = 0x2;
   /// ADC Channel 0,1,2,3.
   pub const _0123: u32 = 0x3;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod aes_intlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
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
   pub const SAMPLED: u32 = 0x1;
   /// BOD enabled continuously.
   pub const CONTINUOUS: u32 = 0x2;
   /// BOD Disabled.
   pub const DISABLED: u32 = 0x3;
}

/// BOD operation
#[allow(non_upper_case_globals)]
pub mod bodact {
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
}

/// USB Prescaler Division Factor
#[allow(non_upper_case_globals)]
pub mod clk_usbpsdiv {
   /// Divide by 1.
   pub const _1: u32 = 0x0;
   /// Divide by 2.
   pub const _2: u32 = 0x1;
   /// Divide by 4.
   pub const _4: u32 = 0x2;
   /// Divide by 8.
   pub const _8: u32 = 0x3;
   /// Divide by 16.
   pub const _16: u32 = 0x4;
   /// Divide by 32.
   pub const _32: u32 = 0x5;
}

/// USB Clock Source
#[allow(non_upper_case_globals)]
pub mod clk_usbsrc {
   /// PLL.
   pub const PLL: u32 = 0x0;
   /// Internal 32 MHz RC Oscillator.
   pub const RC32M: u32 = 0x1;
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
   /// External reference on AREF on PORTB.
   pub const AREFB: u32 = 0x3;
}

/// Burst mode
#[allow(non_upper_case_globals)]
pub mod dma_ch_burstlen {
   /// 1-byte burst mode.
   pub const _1BYTE: u32 = 0x0;
   /// 2-byte burst mode.
   pub const _2BYTE: u32 = 0x1;
   /// 4-byte burst mode.
   pub const _4BYTE: u32 = 0x2;
   /// 8-byte burst mode.
   pub const _8BYTE: u32 = 0x3;
}

/// Destination adressing mode
#[allow(non_upper_case_globals)]
pub mod dma_ch_destdir {
   /// Fixed.
   pub const FIXED: u32 = 0x0;
   /// Increment.
   pub const INC: u32 = 0x1;
   /// Decrement.
   pub const DEC: u32 = 0x2;
}

/// Destination adress reload mode
#[allow(non_upper_case_globals)]
pub mod dma_ch_destreload {
   /// No reload.
   pub const NONE: u32 = 0x0;
   /// Reload at end of block.
   pub const BLOCK: u32 = 0x1;
   /// Reload at end of burst.
   pub const BURST: u32 = 0x2;
   /// Reload at end of transaction.
   pub const TRANSACTION: u32 = 0x3;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod dma_ch_errintlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Source addressing mode
#[allow(non_upper_case_globals)]
pub mod dma_ch_srcdir {
   /// Fixed.
   pub const FIXED: u32 = 0x0;
   /// Increment.
   pub const INC: u32 = 0x1;
   /// Decrement.
   pub const DEC: u32 = 0x2;
}

/// Source address reload mode
#[allow(non_upper_case_globals)]
pub mod dma_ch_srcreload {
   /// No reload.
   pub const NONE: u32 = 0x0;
   /// Reload at end of block.
   pub const BLOCK: u32 = 0x1;
   /// Reload at end of burst.
   pub const BURST: u32 = 0x2;
   /// Reload at end of transaction.
   pub const TRANSACTION: u32 = 0x3;
}

/// Transfer trigger source
#[allow(non_upper_case_globals)]
pub mod dma_ch_trigsrc {
   /// Off software triggers only.
   pub const OFF: u32 = 0x0;
   /// Event System Channel 0.
   pub const EVSYS_CH0: u32 = 0x1;
   /// Event System Channel 1.
   pub const EVSYS_CH1: u32 = 0x2;
   /// Event System Channel 2.
   pub const EVSYS_CH2: u32 = 0x3;
   /// AES.
   pub const AES: u32 = 0x4;
   /// ADCA Channel 0.
   pub const ADCA_CH0: u32 = 0x10;
   /// ADCA Channel 1.
   pub const ADCA_CH1: u32 = 0x11;
   /// ADCA Channel 2.
   pub const ADCA_CH2: u32 = 0x12;
   /// ADCA Channel 3.
   pub const ADCA_CH3: u32 = 0x13;
   /// ADCA Channel 0,1,2,3 combined.
   pub const ADCA_CH4: u32 = 0x14;
   /// DACA Channel 0.
   pub const DACA_CH0: u32 = 0x15;
   /// DACA Channel 1.
   pub const DACA_CH1: u32 = 0x16;
   /// ADCB Channel 0.
   pub const ADCB_CH0: u32 = 0x20;
   /// ADCB Channel 1.
   pub const ADCB_CH1: u32 = 0x21;
   /// ADCB Channel 2.
   pub const ADCB_CH2: u32 = 0x22;
   /// ADCB Channel 3.
   pub const ADCB_CH3: u32 = 0x23;
   /// ADCB Channel 0,1,2,3 combined.
   pub const ADCB_CH4: u32 = 0x24;
   /// DACB Channel 0.
   pub const DACB_CH0: u32 = 0x25;
   /// DACB Channel 1.
   pub const DACB_CH1: u32 = 0x26;
   /// Timer/Counter C0 Overflow.
   pub const TCC0_OVF: u32 = 0x40;
   /// Timer/Counter C0 Error.
   pub const TCC0_ERR: u32 = 0x41;
   /// Timer/Counter C0 Compare or Capture A.
   pub const TCC0_CCA: u32 = 0x42;
   /// Timer/Counter C0 Compare or Capture B.
   pub const TCC0_CCB: u32 = 0x43;
   /// Timer/Counter C0 Compare or Capture C.
   pub const TCC0_CCC: u32 = 0x44;
   /// Timer/Counter C0 Compare or Capture D.
   pub const TCC0_CCD: u32 = 0x45;
   /// Timer/Counter C1 Overflow.
   pub const TCC1_OVF: u32 = 0x46;
   /// Timer/Counter C1 Error.
   pub const TCC1_ERR: u32 = 0x47;
   /// Timer/Counter C1 Compare or Capture A.
   pub const TCC1_CCA: u32 = 0x48;
   /// Timer/Counter C1 Compare or Capture B.
   pub const TCC1_CCB: u32 = 0x49;
   /// SPI C Transfer Complete.
   pub const SPIC: u32 = 0x4A;
   /// USART C0 Receive Complete.
   pub const USARTC0_RXC: u32 = 0x4B;
   /// USART C0 Data Register Empty.
   pub const USARTC0_DRE: u32 = 0x4C;
   /// USART C1 Receive Complete.
   pub const USARTC1_RXC: u32 = 0x4E;
   /// USART C1 Data Register Empty.
   pub const USARTC1_DRE: u32 = 0x4F;
   /// Timer/Counter D0 Overflow.
   pub const TCD0_OVF: u32 = 0x60;
   /// Timer/Counter D0 Error.
   pub const TCD0_ERR: u32 = 0x61;
   /// Timer/Counter D0 Compare or Capture A.
   pub const TCD0_CCA: u32 = 0x62;
   /// Timer/Counter D0 Compare or Capture B.
   pub const TCD0_CCB: u32 = 0x63;
   /// Timer/Counter D0 Compare or Capture C.
   pub const TCD0_CCC: u32 = 0x64;
   /// Timer/Counter D0 Compare or Capture D.
   pub const TCD0_CCD: u32 = 0x65;
   /// Timer/Counter D1 Overflow.
   pub const TCD1_OVF: u32 = 0x66;
   /// Timer/Counter D1 Error.
   pub const TCD1_ERR: u32 = 0x67;
   /// Timer/Counter D1 Compare or Capture A.
   pub const TCD1_CCA: u32 = 0x68;
   /// Timer/Counter D1 Compare or Capture B.
   pub const TCD1_CCB: u32 = 0x69;
   /// SPI D Transfer Complete.
   pub const SPID: u32 = 0x6A;
   /// USART D0 Receive Complete.
   pub const USARTD0_RXC: u32 = 0x6B;
   /// USART D0 Data Register Empty.
   pub const USARTD0_DRE: u32 = 0x6C;
   /// USART D1 Receive Complete.
   pub const USARTD1_RXC: u32 = 0x6E;
   /// USART D1 Data Register Empty.
   pub const USARTD1_DRE: u32 = 0x6F;
   /// Timer/Counter E0 Overflow.
   pub const TCE0_OVF: u32 = 0x80;
   /// Timer/Counter E0 Error.
   pub const TCE0_ERR: u32 = 0x81;
   /// Timer/Counter E0 Compare or Capture A.
   pub const TCE0_CCA: u32 = 0x82;
   /// Timer/Counter E0 Compare or Capture B.
   pub const TCE0_CCB: u32 = 0x83;
   /// Timer/Counter E0 Compare or Capture C.
   pub const TCE0_CCC: u32 = 0x84;
   /// Timer/Counter E0 Compare or Capture D.
   pub const TCE0_CCD: u32 = 0x85;
   /// Timer/Counter E1 Overflow.
   pub const TCE1_OVF: u32 = 0x86;
   /// Timer/Counter E1 Error.
   pub const TCE1_ERR: u32 = 0x87;
   /// Timer/Counter E1 Compare or Capture A.
   pub const TCE1_CCA: u32 = 0x88;
   /// Timer/Counter E1 Compare or Capture B.
   pub const TCE1_CCB: u32 = 0x89;
   /// SPI E Transfer Complete.
   pub const SPIE: u32 = 0x8A;
   /// USART E0 Receive Complete.
   pub const USARTE0_RXC: u32 = 0x8B;
   /// USART E0 Data Register Empty.
   pub const USARTE0_DRE: u32 = 0x8C;
   /// USART E1 Receive Complete.
   pub const USARTE1_RXC: u32 = 0x8E;
   /// USART E1 Data Register Empty.
   pub const USARTE1_DRE: u32 = 0x8F;
   /// Timer/Counter F0 Overflow.
   pub const TCF0_OVF: u32 = 0xA0;
   /// Timer/Counter F0 Error.
   pub const TCF0_ERR: u32 = 0xA1;
   /// Timer/Counter F0 Compare or Capture A.
   pub const TCF0_CCA: u32 = 0xA2;
   /// Timer/Counter F0 Compare or Capture B.
   pub const TCF0_CCB: u32 = 0xA3;
   /// Timer/Counter F0 Compare or Capture C.
   pub const TCF0_CCC: u32 = 0xA4;
   /// Timer/Counter F0 Compare or Capture D.
   pub const TCF0_CCD: u32 = 0xA5;
   /// Timer/Counter F1 Overflow.
   pub const TCF1_OVF: u32 = 0xA6;
   /// Timer/Counter F1 Error.
   pub const TCF1_ERR: u32 = 0xA7;
   /// Timer/Counter F1 Compare or Capture A.
   pub const TCF1_CCA: u32 = 0xA8;
   /// Timer/Counter F1 Compare or Capture B.
   pub const TCF1_CCB: u32 = 0xA9;
   /// SPI F Transfer Complete.
   pub const SPIF: u32 = 0xAA;
   /// USART F0 Receive Complete.
   pub const USARTF0_RXC: u32 = 0xAB;
   /// USART F0 Data Register Empty.
   pub const USARTF0_DRE: u32 = 0xAC;
   /// USART F1 Receive Complete.
   pub const USARTF1_RXC: u32 = 0xAE;
   /// USART F1 Data Register Empty.
   pub const USARTF1_DRE: u32 = 0xAF;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod dma_ch_trnintlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
   pub const HI: u32 = 0x3;
}

/// Double buffering mode
#[allow(non_upper_case_globals)]
pub mod dma_dbufmode {
   /// Double buffering disabled.
   pub const DISABLED: u32 = 0x0;
   /// Double buffering enabled on channel 0/1.
   pub const CH01: u32 = 0x1;
   /// Double buffering enabled on channel 2/3.
   pub const CH23: u32 = 0x2;
   /// Double buffering enabled on ch. 0/1 and ch. 2/3.
   pub const CH01CH23: u32 = 0x3;
}

/// Priority mode
#[allow(non_upper_case_globals)]
pub mod dma_primode {
   /// Round Robin.
   pub const RR0123: u32 = 0x0;
   /// Channel 0 > Round Robin on channel 1/2/3.
   pub const CH0RR123: u32 = 0x1;
   /// Channel 0 > channel 1 > Round Robin on channel 2/3.
   pub const CH01RR23: u32 = 0x2;
   /// Channel 0 > channel 1 > channel 2 > channel 3.
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
   /// USB Setup, SOF, CRC error and UNF/OVF.
   pub const USB: u32 = 0xA;
   /// Analog Comparator A Channel 0.
   pub const ACA_CH0: u32 = 0x10;
   /// Analog Comparator A Channel 1.
   pub const ACA_CH1: u32 = 0x11;
   /// Analog Comparator A Window.
   pub const ACA_WIN: u32 = 0x12;
   /// Analog Comparator B Channel 0.
   pub const ACB_CH0: u32 = 0x13;
   /// Analog Comparator B Channel 1.
   pub const ACB_CH1: u32 = 0x14;
   /// Analog Comparator B Window.
   pub const ACB_WIN: u32 = 0x15;
   /// ADC A Channel 0.
   pub const ADCA_CH0: u32 = 0x20;
   /// ADC A Channel 1.
   pub const ADCA_CH1: u32 = 0x21;
   /// ADC A Channel 2.
   pub const ADCA_CH2: u32 = 0x22;
   /// ADC A Channel 3.
   pub const ADCA_CH3: u32 = 0x23;
   /// ADC B Channel 0.
   pub const ADCB_CH0: u32 = 0x24;
   /// ADC B Channel 1.
   pub const ADCB_CH1: u32 = 0x25;
   /// ADC B Channel 2.
   pub const ADCB_CH2: u32 = 0x26;
   /// ADC B Channel 3.
   pub const ADCB_CH3: u32 = 0x27;
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
   /// Timer/Counter D1 Overflow.
   pub const TCD1_OVF: u32 = 0xD8;
   /// Timer/Counter D1 Error.
   pub const TCD1_ERR: u32 = 0xD9;
   /// Timer/Counter D1 Compare or Capture A.
   pub const TCD1_CCA: u32 = 0xDC;
   /// Timer/Counter D1 Compare or Capture B.
   pub const TCD1_CCB: u32 = 0xDD;
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
   /// Timer/Counter E1 Overflow.
   pub const TCE1_OVF: u32 = 0xE8;
   /// Timer/Counter E1 Error.
   pub const TCE1_ERR: u32 = 0xE9;
   /// Timer/Counter E1 Compare or Capture A.
   pub const TCE1_CCA: u32 = 0xEC;
   /// Timer/Counter E1 Compare or Capture B.
   pub const TCE1_CCB: u32 = 0xED;
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
   /// Timer/Counter F1 Overflow.
   pub const TCF1_OVF: u32 = 0xF8;
   /// Timer/Counter F1 Error.
   pub const TCF1_ERR: u32 = 0xF9;
   /// Timer/Counter F1 Compare or Capture A.
   pub const TCF1_CCA: u32 = 0xFC;
   /// Timer/Counter F1 Compare or Capture B.
   pub const TCF1_CCB: u32 = 0xFD;
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
   /// Load EEPROM page buffer.
   pub const LOAD_EEPROM_BUFFER: u32 = 0x33;
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
   /// Internal 32 MHz RC Oscillator.
   pub const RC32M: u32 = 0x2;
   /// External Oscillator.
   pub const XOSC: u32 = 0x3;
}

/// 2 MHz DFLL Calibration Reference
#[allow(non_upper_case_globals)]
pub mod osc_rc2mcref {
   /// Internal 32.768 kHz RC Oscillator.
   pub const RC32K: u32 = 0x0;
   /// External 32.768 kHz Crystal Oscillator.
   pub const XOSC32K: u32 = 0x1;
}

/// 32 MHz DFLL Calibration Reference
#[allow(non_upper_case_globals)]
pub mod osc_rc32mcref {
   /// Internal 32.768 kHz RC Oscillator.
   pub const RC32K: u32 = 0x0;
   /// External 32.768 kHz Crystal Oscillator.
   pub const XOSC32K: u32 = 0x1;
   /// USB Start of Frame.
   pub const USBSOF: u32 = 0x2;
}

/// External Oscillator Selection and Startup Time
#[allow(non_upper_case_globals)]
pub mod osc_xoscsel {
   /// External Clock - 6 CLK.
   pub const EXTCLK: u32 = 0x0;
   /// 32.768 kHz TOSC - 32K CLK.
   pub const _32KHz: u32 = 0x2;
   /// 0.4-16 MHz XTAL - 256 CLK.
   pub const XTAL_256CLK: u32 = 0x3;
   /// 0.4-16 MHz XTAL - 1K CLK.
   pub const XTAL_1KCLK: u32 = 0x7;
   /// 0.4-16 MHz XTAL - 16K CLK.
   pub const XTAL_16KCLK: u32 = 0xB;
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
   /// System Clock Output on Port E pin 7.
   pub const PE7: u32 = 0x3;
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
   /// Event Channel 7 Output on Port C pin 7.
   pub const PC7: u32 = 0x1;
   /// Event Channel 7 Output on Port D pin 7.
   pub const PD7: u32 = 0x2;
   /// Event Channel 7 Output on Port E pin 7.
   pub const PE7: u32 = 0x3;
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

/// Virtual Port Mapping
#[allow(non_upper_case_globals)]
pub mod portcfg_vp02map {
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

/// Virtual Port Mapping
#[allow(non_upper_case_globals)]
pub mod portcfg_vp13map {
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
pub mod rtc32_compintlvl {
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
pub mod rtc32_ovfintlvl {
   /// Interrupt Disabled.
   pub const OFF: u32 = 0x0;
   /// Low Level.
   pub const LO: u32 = 0x1;
   /// Medium Level.
   pub const MED: u32 = 0x2;
   /// High Level.
   pub const HI: u32 = 0x3;
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

/// Byte Mode
#[allow(non_upper_case_globals)]
pub mod tc_bytem {
   /// 16-bit mode.
   pub const NORMAL: u32 = 0x0;
   /// Timer/Counter operating in byte mode only.
   pub const BYTEMODE: u32 = 0x1;
   /// Timer/Counter split into two 8-bit Counters (TC2).
   pub const SPLITMODE: u32 = 0x2;
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
   pub const SINGLESLOPE: u32 = 0x3;
   /// Single Slope.
   pub const SS: u32 = 0x3;
   /// Dual Slope, Update on TOP.
   pub const DSTOP: u32 = 0x5;
   /// Dual Slope, Update on TOP.
   pub const DS_T: u32 = 0x5;
   /// Dual Slope, Update on both TOP and BOTTOM.
   pub const DSBOTH: u32 = 0x6;
   /// Dual Slope, Update on both TOP and BOTTOM.
   pub const DS_TB: u32 = 0x6;
   /// Dual Slope, Update on BOTTOM.
   pub const DSBOTTOM: u32 = 0x7;
   /// Dual Slope, Update on BOTTOM.
   pub const DS_B: u32 = 0x7;
}

/// Timer Oscillator pin location
#[allow(non_upper_case_globals)]
pub mod toscsel {
   /// TOSC1 / TOSC2 on separate pins.
   pub const ALTERNATE: u32 = 0x0;
   /// TOSC1 / TOSC2 shared with XTAL1 / XTAL2.
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

/// USB Endpoint Buffersize
#[allow(non_upper_case_globals)]
pub mod usb_ep_bufsize {
   /// 8 bytes buffer size.
   pub const _8: u32 = 0x0;
   /// 16 bytes buffer size.
   pub const _16: u32 = 0x1;
   /// 32 bytes buffer size.
   pub const _32: u32 = 0x2;
   /// 64 bytes buffer size.
   pub const _64: u32 = 0x3;
   /// 128 bytes buffer size.
   pub const _128: u32 = 0x4;
   /// 256 bytes buffer size.
   pub const _256: u32 = 0x5;
   /// 512 bytes buffer size.
   pub const _512: u32 = 0x6;
   /// 1023 bytes buffer size.
   pub const _1023: u32 = 0x7;
}

/// USB Endpoint Type
#[allow(non_upper_case_globals)]
pub mod usb_ep_type {
   /// Endpoint Disabled.
   pub const DISABLE: u32 = 0x0;
   /// Control.
   pub const CONTROL: u32 = 0x1;
   /// Bulk/Interrupt.
   pub const BULK: u32 = 0x2;
   /// Isochronous.
   pub const ISOCHRONOUS: u32 = 0x3;
}

/// Interrupt level
#[allow(non_upper_case_globals)]
pub mod usb_intlvl {
   /// Interrupt disabled.
   pub const OFF: u32 = 0x0;
   /// Low level.
   pub const LO: u32 = 0x1;
   /// Medium level.
   pub const MED: u32 = 0x2;
   /// High level.
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

/// Watchdog (Window) Timeout Period
#[allow(non_upper_case_globals)]
pub mod wdp {
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

