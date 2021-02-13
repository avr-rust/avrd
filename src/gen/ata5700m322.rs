//! The AVR ATA5700M322 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.1V - 4.2V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 1000000 |
/// | BOOTRST | 100 |
/// | SPIEN | 100000 |
/// | EESAVE | 1000 |
/// | WDTON | 10000 |
/// | EEACC | 10 |
/// | CKSTART | 10000000 |
/// | PCEE1 | 1 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x20 as *mut u8;

/// Power reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRT4 | 1000 |
/// | PRLFPH | 10000000 |
/// | PRLFTP | 1000000 |
/// | PRLFR | 100000 |
/// | PRT2 | 10 |
/// | PRT5 | 10000 |
/// | PRT1 | 1 |
/// | PRT3 | 100 |
pub const PRR1: *mut u8 = 0x21 as *mut u8;

/// Power reduction register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSSM | 10000000 |
/// | PRTM | 1000000 |
/// | VXTSDI | 10000 |
/// | PRSF | 100 |
/// | PRTWI2 | 10 |
/// | PRDF | 1000 |
/// | PRSPI2 | 1 |
pub const PRR2: *mut u8 = 0x22 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x26 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x27 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Port D Input Pins.
pub const PIND: *mut u8 = 0x29 as *mut u8;

/// Port D Data Direction Register.
pub const DDRD: *mut u8 = 0x2A as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x2B as *mut u8;

/// Transponder Control 2 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPPSD | 100 |
/// | TPMA | 1 |
/// | TPNFTO | 10000 |
/// | TPWDLV | 1100000 |
/// | TPD | 1000 |
/// | TPMOD | 10 |
pub const TPCR2: *mut u8 = 0x2C as *mut u8;

/// Transponder Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPNFTF | 100 |
/// | TPF | 1 |
/// | TPBERF | 1000 |
/// | TPFTF | 10 |
pub const TPFR: *mut u8 = 0x2D as *mut u8;

/// MCU control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUD | 10000 |
/// | ENPS | 1000 |
/// | IVSEL | 10000000 |
/// | SPIIO | 100 |
/// | TRCCE | 100000 |
/// | TRCEN | 1000000 |
/// | IVL | 11 |
pub const MCUCR: *mut u8 = 0x2E as *mut u8;

/// Timer1 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1TOS | 1000000 |
/// | T1RES | 100000 |
/// | T1TOP | 10000 |
/// | T1CTM | 10 |
/// | T1ENA | 10000000 |
/// | T1OTM | 1 |
/// | T1CRM | 100 |
pub const T1CR: *mut u8 = 0x31 as *mut u8;

/// Timer2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2TOP | 10000 |
/// | T2CTM | 10 |
/// | T2RES | 100000 |
/// | T2OTM | 1 |
/// | T2ENA | 10000000 |
/// | T2CRM | 100 |
/// | T2TOS | 1000000 |
pub const T2CR: *mut u8 = 0x32 as *mut u8;

/// Timer3 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CTM | 10 |
/// | T3CRM | 100 |
/// | T3TOP | 10000 |
/// | T3TOS | 1000000 |
/// | T3CPRM | 1000 |
/// | T3ENA | 10000000 |
/// | T3RES | 100000 |
/// | T3OTM | 1 |
pub const T3CR: *mut u8 = 0x33 as *mut u8;

/// Timer4 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4TOP | 10000 |
/// | T4RES | 100000 |
/// | T4CRM | 100 |
/// | T4CTM | 10 |
/// | T4OTM | 1 |
/// | T4ENA | 10000000 |
/// | T4CPRM | 1000 |
/// | T4TOS | 1000000 |
pub const T4CR: *mut u8 = 0x34 as *mut u8;

/// LF Timer Control Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LTENA | 10000000 |
/// | LTPS1 | 10 |
/// | LTPS0 | 1 |
/// | LTCRM | 1000 |
/// | LTSM | 1000000 |
/// | LTCIM | 10000 |
/// | LTCM | 100000 |
/// | LTPS2 | 100 |
pub const LTCMR: *mut u8 = 0x35 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEBRE | 1 |
/// | E2FF | 1000000 |
/// | E2CIM | 10 |
/// | E2CF | 10000000 |
/// | E2AVF | 100000 |
pub const EECR2: *mut u8 = 0x36 as *mut u8;

/// PH Telegram Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CSM | 10000000 |
/// | CPM | 1000000 |
/// | FRFIFO | 100000 |
pub const PHTCR: *mut u8 = 0x37 as *mut u8;

/// LF Data FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFCLR | 10000000 |
pub const LDFFL: *mut u8 = 0x38 as *mut u8;

/// LF Data FIFO Data Register.
pub const LDFD: *mut u8 = 0x39 as *mut u8;

/// Power reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTXDC | 100 |
/// | PRCU | 1000000 |
/// | PRTWI1 | 10000000 |
/// | PRSPI | 1 |
/// | PRVM | 10000 |
/// | PRCO | 100000 |
/// | PRCRC | 1000 |
/// | PRLFRS | 10 |
pub const PRR0: *mut u8 = 0x3A as *mut u8;

/// Protocol Handler Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHID0F | 10000 |
/// | PHTBLF | 10 |
/// | PHDFF | 100 |
/// | PHID1F | 100000 |
/// | CRCEF | 1 |
/// | PHIDFF | 1000 |
pub const PHFR: *mut u8 = 0x3B as *mut u8;

/// LF Receiver Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSYDF | 1 |
/// | LFSD | 1000000 |
/// | LFES | 10000000 |
/// | LFEOF | 100 |
/// | LFDEF | 10 |
/// | LFTOF | 1000 |
pub const LFFR: *mut u8 = 0x3C as *mut u8;

/// AES Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AESLKM | 1000000 |
/// | AESRES | 100000 |
/// | AESXOR | 10000 |
/// | AESE | 10000000 |
/// | AESIM | 100 |
/// | AESWK | 1 |
/// | AESD | 1000 |
/// | AESWD | 10 |
pub const AESCR: *mut u8 = 0x3D as *mut u8;

/// AES Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AESRF | 1 |
/// | AESERF | 10000000 |
pub const AESSR: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPAGE | 1000000 |
/// | EERIE | 1000 |
/// | NVMBSY | 10000000 |
/// | EEPM | 110000 |
/// | EEMWE | 100 |
/// | EERE | 1 |
/// | EEWE | 10 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// EEPROM Protection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEAP | 1111 |
pub const EEPR: *mut u8 = 0x43 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x44 as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x45 as *mut u8;

/// Pin change Interrupt control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE1 | 10 |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x46 as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
/// | INT1 | 10 |
pub const EIMSK: *mut u8 = 0x47 as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
/// | INTF1 | 10 |
pub const EIFR: *mut u8 = 0x48 as *mut u8;

/// LF Data FIFO Clock Switch Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFSCKS | 10 |
/// | LDFSCSW | 1 |
pub const LDFCKSW: *mut u8 = 0x49 as *mut u8;

/// Voltage Monitor Status and Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMDIH | 10 |
/// | VMF | 1 |
pub const VMSCR: *mut u8 = 0x4A as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWRF | 10000 |
/// | PORF | 1 |
/// | TPRF | 100000 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x4B as *mut u8;

/// SPI control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPHA | 100 |
/// | CPOL | 1000 |
/// | SPE | 1000000 |
/// | SPR | 11 |
/// | SPIE | 10000000 |
/// | DORD | 100000 |
/// | MSTR | 10000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXIF | 100000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
/// | RXIF | 10000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// LF Receiver Control Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFRRT | 11000000 |
/// | LFCE1 | 1 |
/// | LFBR | 11000 |
/// | LFMG | 100000 |
/// | LFCE3 | 100 |
/// | LFCE2 | 10 |
pub const LFCR0: *mut u8 = 0x4F as *mut u8;

/// LF Receiver Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ARMDE | 1000 |
/// | ADTHEN | 100000 |
/// | LFFM1 | 100 |
/// | LFPEEN | 1000000 |
/// | RSST | 11 |
/// | LFRE | 10000000 |
/// | FLLEN | 10000 |
pub const LFCR1: *mut u8 = 0x50 as *mut u8;

/// Debug Wire Data Register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Timer0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0F | 1 |
pub const T0IFR: *mut u8 = 0x52 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | FLSEL | 111000 |
/// | SELFPRGEN | 1 |
/// | RWWSB | 1000000 |
/// | SPMIE | 10000000 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Sleep mode control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 1110 |
pub const SMCR: *mut u8 = 0x58 as *mut u8;

/// Transponder Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPGAP | 10 |
/// | TPA | 1 |
/// | TPBCOK | 1000 |
/// | TPPSW | 100 |
pub const TPSR: *mut u8 = 0x59 as *mut u8;

/// LF Receiver Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFDAMP | 100 |
/// | LFSEN | 11 |
/// | LFVC | 11100000 |
pub const LFCR2: *mut u8 = 0x5A as *mut u8;

/// LF Receiver Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFRCTEN | 1 |
/// | LFRCPM | 100 |
/// | LFRCPCEN | 10 |
/// | LFSBEN | 10000000 |
/// | LFTS | 1110000 |
/// | LFTON | 1000 |
pub const LFCR3: *mut u8 = 0x5B as *mut u8;

/// Stack Pointer low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | S | 10000 |
/// | T | 1000000 |
/// | I | 10000000 |
/// | V | 1000 |
/// | H | 100000 |
/// | Z | 10 |
/// | N | 100 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// External Interrupt control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
pub const EICRA: *mut u8 = 0x6B as *mut u8;

/// Pin change Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT4 | 10000 |
/// | PCINT2 | 100 |
/// | PCINT7 | 10000000 |
/// | PCINT3 | 1000 |
/// | PCINT1 | 10 |
/// | PCINT6 | 1000000 |
/// | PCINT5 | 100000 |
/// | PCINT0 | 1 |
pub const PCMSK0: *mut u8 = 0x6C as *mut u8;

/// Pin change Mask Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT11 | 1000 |
/// | PCINT9 | 10 |
/// | PCINT10 | 100 |
/// | PCINT15 | 10000000 |
/// | PCINT12 | 10000 |
/// | PCINT8 | 1 |
/// | PCINT13 | 100000 |
/// | PCINT14 | 1000000 |
pub const PCMSK1: *mut u8 = 0x6D as *mut u8;

/// Watchdog Timer0 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDPS | 111 |
/// | WDCE | 10000 |
pub const WDTCR: *mut u8 = 0x6E as *mut u8;

/// Timer1 Counter Register.
pub const T1CNT: *mut u8 = 0x6F as *mut u8;

/// Timer1 Compare Register.
pub const T1COR: *mut u8 = 0x70 as *mut u8;

/// Timer1 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1PS | 111100 |
/// | T1CS | 11 |
/// | T1DC | 11000000 |
pub const T1MR: *mut u8 = 0x71 as *mut u8;

/// Timer1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1OIM | 1 |
/// | T1CIM | 10 |
pub const T1IMR: *mut u8 = 0x72 as *mut u8;

/// Timer2 Counter Register.
pub const T2CNT: *mut u8 = 0x73 as *mut u8;

/// Timer2 Compare Register.
pub const T2COR: *mut u8 = 0x74 as *mut u8;

/// Timer2 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2PS | 111100 |
/// | T2CS | 11 |
/// | T2DC | 11000000 |
pub const T2MR: *mut u8 = 0x75 as *mut u8;

/// Timer2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2OIM | 1 |
/// | T2CIM | 10 |
pub const T2IMR: *mut u8 = 0x76 as *mut u8;

/// Timer3 counter Register low byte.
pub const T3CNTL: *mut u8 = 0x77 as *mut u8;

/// Timer3 counter Register.
pub const T3CNT: *mut u16 = 0x77 as *mut u16;

/// Timer3 counter Register high byte.
pub const T3CNTH: *mut u8 = 0x78 as *mut u8;

/// Timer3 compare Register low byte.
pub const T3CORL: *mut u8 = 0x79 as *mut u8;

/// Timer3 compare Register.
pub const T3COR: *mut u16 = 0x79 as *mut u16;

/// Timer3 compare Register high byte.
pub const T3CORH: *mut u8 = 0x7A as *mut u8;

/// Timer3 input capture Register low byte.
pub const T3ICRL: *mut u8 = 0x7B as *mut u8;

/// Timer3 input capture Register.
pub const T3ICR: *mut u16 = 0x7B as *mut u16;

/// Timer3 input capture Register high byte.
pub const T3ICRH: *mut u8 = 0x7C as *mut u8;

/// Timer3 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3PS | 11100 |
/// | T3CS | 11 |
pub const T3MRA: *mut u8 = 0x7D as *mut u8;

/// Timer3 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CNC | 100 |
/// | T3SCE | 10 |
/// | T3CE | 11000 |
/// | T3ICS | 11100000 |
pub const T3MRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CIM | 10 |
/// | T3CPIM | 100 |
/// | T3OIM | 1 |
pub const T3IMR: *mut u8 = 0x7F as *mut u8;

/// Timer4 counter Register low byte.
pub const T4CNTL: *mut u8 = 0x80 as *mut u8;

/// Timer4 counter Register.
pub const T4CNT: *mut u16 = 0x80 as *mut u16;

/// Timer4 counter Register high byte.
pub const T4CNTH: *mut u8 = 0x81 as *mut u8;

/// Timer4 compare Register low byte.
pub const T4CORL: *mut u8 = 0x82 as *mut u8;

/// Timer4 compare Register.
pub const T4COR: *mut u16 = 0x82 as *mut u16;

/// Timer4 compare Register high byte.
pub const T4CORH: *mut u8 = 0x83 as *mut u8;

/// Timer4 input capture Register low byte.
pub const T4ICRL: *mut u8 = 0x84 as *mut u8;

/// Timer4 input capture Register.
pub const T4ICR: *mut u16 = 0x84 as *mut u16;

/// Timer4 input capture Register high byte.
pub const T4ICRH: *mut u8 = 0x85 as *mut u8;

/// Timer4 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CS | 11 |
/// | T4PS | 11100 |
pub const T4MRA: *mut u8 = 0x86 as *mut u8;

/// Timer4 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CNC | 100 |
/// | T4ICS | 11100000 |
/// | T4SCE | 10 |
/// | T4CE | 11000 |
pub const T4MRB: *mut u8 = 0x87 as *mut u8;

/// Timer4 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4OIM | 1 |
/// | T4CPIM | 100 |
/// | T4CIM | 10 |
pub const T4IMR: *mut u8 = 0x88 as *mut u8;

/// Timer5 Temp Register.
pub const T5TEMP: *mut u8 = 0x89 as *mut u8;

/// Timer5 Output Compare Register low byte.
pub const T5OCRL: *mut u8 = 0x8A as *mut u8;

/// Timer5 Output Compare Register.
pub const T5OCR: *mut u16 = 0x8A as *mut u16;

/// Timer5 Output Compare Register high byte.
pub const T5OCRH: *mut u8 = 0x8B as *mut u8;

/// Timer5 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CS | 111 |
/// | T5CTC | 1000 |
pub const T5CCR: *mut u8 = 0x8C as *mut u8;

/// Timer5 Counter low byte.
pub const T5CNTL: *mut u8 = 0x8D as *mut u8;

/// Timer5 Counter.
pub const T5CNT: *mut u16 = 0x8D as *mut u16;

/// Timer5 Counter high byte.
pub const T5CNTH: *mut u8 = 0x8E as *mut u8;

/// Timer5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OIM | 1 |
/// | T5CIM | 10 |
pub const T5IMR: *mut u8 = 0x8F as *mut u8;

/// LF Receiver Calibration Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSTC | 111 |
/// | ICOMPRT | 11000 |
/// | SEL150M | 11100000 |
pub const LFCALR1: *mut u8 = 0x90 as *mut u8;

/// LF Receiver Calibration Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TIKOMPD | 10000000 |
/// | LFSRM | 1000000 |
/// | LFSTRES | 111111 |
pub const LFCALR2: *mut u8 = 0x91 as *mut u8;

/// LF Receiver Calibration Register 3.
pub const LFCALR3: *mut u8 = 0x92 as *mut u8;

/// LF Receiver Calibration Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCGAIN26 | 1000000 |
/// | TCGAIN25 | 100000 |
/// | TCGAIN24 | 10000 |
/// | TCGAIN21 | 10 |
/// | TCGAIN27 | 10000000 |
/// | TCGAIN20 | 1 |
/// | TCGAIN23 | 1000 |
/// | TCGAIN22 | 100 |
pub const LFCALR4: *mut u8 = 0x93 as *mut u8;

/// LF Receiver Calibration Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCGAIN34 | 10000 |
/// | TCGAIN31 | 10 |
/// | TCGAIN37 | 10000000 |
/// | TCGAIN30 | 1 |
/// | TCGAIN35 | 100000 |
/// | TCGAIN32 | 100 |
/// | TCGAIN36 | 1000000 |
pub const LFCALR5: *mut u8 = 0x94 as *mut u8;

/// LF Receiver Calibration Register 6.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCGAIN41 | 10 |
/// | TCGAIN43 | 1000 |
/// | TCGAIN44 | 10000 |
/// | TCGAIN42 | 100 |
/// | TCGAIN40 | 1 |
pub const LFCALR6: *mut u8 = 0x95 as *mut u8;

/// LF Receiver Calibration Register 7.
pub const LFCALR7: *mut u8 = 0x96 as *mut u8;

/// LF Receiver Calibration Register 8.
pub const LFCALR8: *mut u8 = 0x97 as *mut u8;

/// LF Receiver Calibration Register 9.
pub const LFCALR9: *mut u8 = 0x98 as *mut u8;

/// LF Receiver Calibration Register 10.
pub const LFCALR10: *mut u8 = 0x99 as *mut u8;

/// LF Receiver Calibration Register 11.
pub const LFCALR11: *mut u8 = 0x9A as *mut u8;

/// LF Receiver Calibration Register 12.
pub const LFCALR12: *mut u8 = 0x9B as *mut u8;

/// LF Receiver Calibration Register 13.
pub const LFCALR13: *mut u8 = 0x9C as *mut u8;

/// LF Receiver Calibration Register 14.
pub const LFCALR14: *mut u8 = 0x9D as *mut u8;

/// LF Receiver Calibration Register 15.
pub const LFCALR15: *mut u8 = 0x9E as *mut u8;

/// LF Receiver Calibration Register 16.
pub const LFCALR16: *mut u8 = 0x9F as *mut u8;

/// LF Receiver Calibration Register 17.
pub const LFCALR17: *mut u8 = 0xA0 as *mut u8;

/// LF Receiver Calibration Register 18.
pub const LFCALR18: *mut u8 = 0xA1 as *mut u8;

/// LF Receiver Calibration Register 19.
pub const LFCALR19: *mut u8 = 0xA2 as *mut u8;

/// LF Receiver Calibration Register 20.
pub const LFCALR20: *mut u8 = 0xA3 as *mut u8;

/// LF Receiver Calibration Register 21.
pub const LFCALR21: *mut u8 = 0xA4 as *mut u8;

/// LF Receiver Calibration Register 22.
pub const LFCALR22: *mut u8 = 0xA5 as *mut u8;

/// LF Receiver Calibration Register 23.
pub const LFCALR23: *mut u8 = 0xA6 as *mut u8;

/// LF Receiver Calibration Register 24.
pub const LFCALR24: *mut u8 = 0xA7 as *mut u8;

/// LF Receiver Calibration Register 25.
pub const LFCALR25: *mut u8 = 0xA8 as *mut u8;

/// LF Receiver Calibration Register 26.
pub const LFCALR26: *mut u8 = 0xA9 as *mut u8;

/// LF Receiver Calibration Register 27.
pub const LFCALR27: *mut u8 = 0xAA as *mut u8;

/// LF Receiver Calibration Register 28.
pub const LFCALR28: *mut u8 = 0xAB as *mut u8;

/// LF Receiver Calibration Register 29.
pub const LFCALR29: *mut u8 = 0xAC as *mut u8;

/// LF Receiver Calibration Register 30.
pub const LFCALR30: *mut u8 = 0xAD as *mut u8;

/// LF Receiver Calibration Register 31.
pub const LFCALR31: *mut u8 = 0xAE as *mut u8;

/// LF Receiver Calibration Register 32.
pub const LFCALR32: *mut u8 = 0xAF as *mut u8;

/// LF Receiver Calibration Register 33.
pub const LFCALR33: *mut u8 = 0xB0 as *mut u8;

/// LF Receiver Calibration Register 34.
pub const LFCALR34: *mut u8 = 0xB1 as *mut u8;

/// LF Receiver Calibration Register 35.
pub const LFCALR35: *mut u8 = 0xB2 as *mut u8;

/// LF Receiver Calibration Register 36.
pub const LFCALR36: *mut u8 = 0xB3 as *mut u8;

/// LF Receiver Calibration Register 37.
pub const LFCALR37: *mut u8 = 0xB4 as *mut u8;

/// LF Receiver Calibration Register 38.
pub const LFCALR38: *mut u8 = 0xB5 as *mut u8;

/// LF Receiver Calibration Register 39.
pub const LFCALR39: *mut u8 = 0xB6 as *mut u8;

/// LF Receiver Calibration Register 40.
pub const LFCALR40: *mut u8 = 0xB7 as *mut u8;

/// LF Receiver Calibration Register 41.
pub const LFCALR41: *mut u8 = 0xB8 as *mut u8;

/// LF Receiver Calibration Register 42.
pub const LFCALR42: *mut u8 = 0xB9 as *mut u8;

/// LF Receiver Calibration Register 43.
pub const LFCALR43: *mut u8 = 0xBA as *mut u8;

/// LF Receiver Calibration Register 44.
pub const LFCALR44: *mut u8 = 0xBB as *mut u8;

/// LF Receiver Calibration Register 45.
pub const LFCALR45: *mut u8 = 0xBC as *mut u8;

/// LF Receiver Calibration Register 46.
pub const LFCALR46: *mut u8 = 0xBD as *mut u8;

/// LF Receiver Calibration Register 47.
pub const LFCALR47: *mut u8 = 0xBE as *mut u8;

/// LF Receiver Calibration Register 48.
pub const LFCALR48: *mut u8 = 0xBF as *mut u8;

/// LF Receiver Calibration Register 49.
pub const LFCALR49: *mut u8 = 0xC0 as *mut u8;

/// LF Receiver Calibration Register 50.
pub const LFCALR50: *mut u8 = 0xC1 as *mut u8;

/// LF Receiver Calibration Register 51.
pub const LFCALR51: *mut u8 = 0xC2 as *mut u8;

/// LF Receiver Calibration Register 52.
pub const LFCALR52: *mut u8 = 0xC3 as *mut u8;

/// LF Receiver Calibration Register 53.
pub const LFCALR53: *mut u8 = 0xC4 as *mut u8;

/// `XFUSE` register
pub const XFUSE: *mut u8 = 0xC5 as *mut u8;

/// Middle RC oscillator calibration Register.
pub const MRCCAL: *mut u8 = 0xC6 as *mut u8;

/// Fast RC oscillator calibration Register.
pub const FRCCAL: *mut u8 = 0xC7 as *mut u8;

/// RC oscillator Temperature Compensation register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DI_MRCBG | 10000 |
/// | MRCTC | 1110 |
/// | FRCTC | 1 |
pub const RCTCAL: *mut u8 = 0xC8 as *mut u8;

/// Clock management status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0xC9 as *mut u8;

/// Clock management override control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCACT | 100 |
/// | MRCAO | 10 |
/// | FRCAO | 1 |
pub const CMOCR: *mut u8 = 0xCA as *mut u8;

/// Supply Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVCCLF | 10 |
/// | AVCCRF | 1 |
pub const SUPFR: *mut u8 = 0xCB as *mut u8;

/// Supply Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVCCLM | 10 |
/// | AVCCRM | 1 |
/// | AVEN | 10000 |
/// | VMEMEN | 10000000 |
/// | VMRESM | 1000000 |
/// | PVEN | 100 |
/// | AVDIC | 1000 |
/// | DVHEN | 100000 |
pub const SUPCR: *mut u8 = 0xCC as *mut u8;

/// Supply calibration register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PV22 | 100 |
/// | PVDIC | 1000 |
/// | PVCAL | 11110000 |
pub const SUPCA1: *mut u8 = 0xCD as *mut u8;

/// Supply calibration register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL | 1111 |
pub const SUPCA2: *mut u8 = 0xCE as *mut u8;

/// Supply calibration register 3.
pub const SUPCA3: *mut u8 = 0xCF as *mut u8;

/// Supply calibration register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICONST | 111111 |
pub const SUPCA4: *mut u8 = 0xD0 as *mut u8;

/// Calibration ready signature.
pub const CALRDY: *mut u8 = 0xD1 as *mut u8;

/// Data FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFOFL | 100 |
/// | DFFLRF | 1 |
/// | DFUFL | 10 |
pub const DFS: *mut u8 = 0xD2 as *mut u8;

/// Data FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFCLR | 10000000 |
/// | DFFLS | 111111 |
pub const DFL: *mut u8 = 0xD5 as *mut u8;

/// Data FIFO Write Pointer.
pub const DFWP: *mut u8 = 0xD6 as *mut u8;

/// Data FIFO Read Pointer.
pub const DFRP: *mut u8 = 0xD7 as *mut u8;

/// Data FIFO Data Register.
pub const DFD: *mut u8 = 0xD8 as *mut u8;

/// Data FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLIM | 1 |
/// | DFERIM | 10 |
pub const DFI: *mut u8 = 0xD9 as *mut u8;

/// Data FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFDRA | 10000000 |
/// | DFFLC | 111111 |
pub const DFC: *mut u8 = 0xDA as *mut u8;

/// Support FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLRF | 1 |
/// | SFUFL | 10 |
/// | SFOFL | 100 |
pub const SFS: *mut u8 = 0xDB as *mut u8;

/// Support FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLS | 11111 |
/// | SFCLR | 10000000 |
pub const SFL: *mut u8 = 0xDC as *mut u8;

/// Support FIFO Write Pointer.
pub const SFWP: *mut u8 = 0xDD as *mut u8;

/// Support FIFO Read Pointer.
pub const SFRP: *mut u8 = 0xDE as *mut u8;

/// Support FIFO Data Register.
pub const SFD: *mut u8 = 0xDF as *mut u8;

/// Support FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFERIM | 10 |
/// | SFFLIM | 1 |
pub const SFI: *mut u8 = 0xE0 as *mut u8;

/// Support FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDRA | 10000000 |
/// | SFFLC | 11111 |
pub const SFC: *mut u8 = 0xE1 as *mut u8;

/// SSM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMTGE | 100 |
/// | SSMPVE | 10000 |
/// | SSMTPE | 1000 |
/// | SSMTAE | 100000 |
pub const SSMCR: *mut u8 = 0xE2 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR10 | 1 |
pub const GTCCR: *mut u8 = 0xE3 as *mut u8;

/// SSM Filter Bandwidth Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMPLDT | 100000 |
pub const SSMFBR: *mut u8 = 0xE4 as *mut u8;

/// SSM Run Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMST | 10 |
/// | SSMR | 1 |
pub const SSMRR: *mut u8 = 0xE5 as *mut u8;

/// SSM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMESM | 1111 |
/// | SSMERR | 10000000 |
pub const SSMSR: *mut u8 = 0xE6 as *mut u8;

/// SSM Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIF | 1 |
pub const SSMIFR: *mut u8 = 0xE7 as *mut u8;

/// SSM interrupt mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIM | 1 |
pub const SSMIMR: *mut u8 = 0xE8 as *mut u8;

/// Master State Machine state register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMMST | 11111 |
pub const MSMSTR: *mut u8 = 0xE9 as *mut u8;

/// SSM State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTA | 111111 |
pub const SSMSTR: *mut u8 = 0xEA as *mut u8;

/// VX Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EN_VX_IN | 10000 |
/// | VX_SEL1 | 10 |
/// | VX_SEL0 | 1 |
/// | EN_VX_OUT | 1000 |
/// | EN_VX | 100 |
pub const VXMCTRL: *mut u8 = 0xEB as *mut u8;

/// Master State Machine Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM0 | 1111 |
/// | MSMSM1 | 11110000 |
pub const MSMCR1: *mut u8 = 0xEC as *mut u8;

/// Master State Machine Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM2 | 1111 |
/// | MSMSM3 | 11110000 |
pub const MSMCR2: *mut u8 = 0xED as *mut u8;

/// Master State Machine Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM4 | 1111 |
/// | MSMSM5 | 11110000 |
pub const MSMCR3: *mut u8 = 0xEE as *mut u8;

/// Master State Machine Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM6 | 1111 |
/// | MSMSM7 | 11110000 |
pub const MSMCR4: *mut u8 = 0xEF as *mut u8;

/// SPI2 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DORD2 | 100000 |
/// | SP2R | 11 |
/// | CPHA2 | 100 |
/// | SP2IE | 10000000 |
/// | SP2E | 1000000 |
/// | CPOL2 | 1000 |
/// | MSTR2 | 10000 |
pub const SP2CR: *mut u8 = 0xF7 as *mut u8;

/// SPI2 Data Register.
pub const SP2DR: *mut u8 = 0xF8 as *mut u8;

/// SPI2 Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SP2IF | 10000000 |
/// | WCOL2 | 1000000 |
/// | SPI22X | 1 |
pub const SP2SR: *mut u8 = 0xF9 as *mut u8;

/// Trace ID Register.
pub const TRCID: *mut u16 = 0xFC as *mut u16;

/// Trace ID Register low byte.
pub const TRCIDL: *mut u8 = 0xFC as *mut u8;

/// Trace ID Register high byte.
pub const TRCIDH: *mut u8 = 0xFD as *mut u8;

/// Trace Data Register.
pub const TRCDR: *mut u8 = 0xFF as *mut u8;

/// Clock output divider settings Register.
pub const CLKOD: *mut u8 = 0x115 as *mut u8;

/// Clock output control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOS | 11 |
/// | CLKOEN | 100 |
pub const CLKOCR: *mut u8 = 0x116 as *mut u8;

/// LF Receiver Decoder Setting Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LOTHA | 11 |
/// | CTTHA | 110000 |
/// | HITHA | 1100 |
pub const LFDSR1: *mut u8 = 0x130 as *mut u8;

/// LF Receiver Decoder Setting Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTTHB | 110000 |
/// | HITHB | 1100 |
/// | LOTHB | 11 |
pub const LFDSR2: *mut u8 = 0x131 as *mut u8;

/// LF Receiver Decoder Setting Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PBDTH | 11 |
/// | QCTH | 111000 |
pub const LFDSR3: *mut u8 = 0x132 as *mut u8;

/// LF Receiver Decoder Setting Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SDTHA | 111 |
/// | SRSTC | 11000000 |
/// | SCTHA | 111000 |
pub const LFDSR4: *mut u8 = 0x133 as *mut u8;

/// LF Decoder Setting 5 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCTHB | 111000 |
/// | SSUTB | 10000000 |
/// | SDTHB | 111 |
/// | SSUTA | 1000000 |
pub const LFDSR5: *mut u8 = 0x134 as *mut u8;

/// LF Decoder Setting 6 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TODU | 111 |
/// | TODS | 111000 |
pub const LFDSR6: *mut u8 = 0x135 as *mut u8;

/// LF Decoder Setting 7 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PBG | 1100 |
/// | MDG | 11000000 |
/// | MDSP | 110000 |
/// | PBSP | 11 |
pub const LFDSR7: *mut u8 = 0x136 as *mut u8;

/// LF Decoder Setting 8 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLD | 111 |
/// | LGFE | 1000 |
/// | ASWTH | 1110000 |
pub const LFDSR8: *mut u8 = 0x137 as *mut u8;

/// LF Decoder Setting 9 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STW | 11111 |
pub const LFDSR9: *mut u8 = 0x138 as *mut u8;

/// LF Decoder Setting 10 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FCL | 111111 |
/// | STBTH | 11000000 |
pub const LFDSR10: *mut u8 = 0x139 as *mut u8;

/// Low Frequency Decoder Setting Register 11.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TINITA | 1111 |
/// | TINITB | 11110000 |
pub const LFDSR11: *mut u8 = 0x13A as *mut u8;

/// EEPROM Protection Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPS4WD | 1 |
/// | EEPS7RD | 10000000 |
/// | EEPS5WD | 100 |
/// | EEPS6RD | 100000 |
/// | EEPS5RD | 1000 |
/// | EEPS6WD | 10000 |
/// | EEPS7WD | 1000000 |
/// | EEPS4RD | 10 |
pub const EEPR1: *mut u8 = 0x13B as *mut u8;

/// EEPROM Protection Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPS10RD | 100000 |
/// | EEPS9WD | 100 |
/// | EEPS11RD | 10000000 |
/// | EEPS11WD | 1000000 |
/// | EEPS8WD | 1 |
/// | EEPS8RD | 10 |
/// | EEPS9RD | 1000 |
/// | EEPS10WD | 10000 |
pub const EEPR2: *mut u8 = 0x13C as *mut u8;

/// EEPROM Protection Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPS12WD | 1 |
/// | EEPS12RD | 10 |
pub const EEPR3: *mut u8 = 0x13D as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFLI | 10 |
/// | REFLO | 100 |
/// | CRCRS | 1 |
pub const CRCCR: *mut u8 = 0x145 as *mut u8;

/// CRC Data Output Register.
pub const CRCDOR: *mut u8 = 0x146 as *mut u8;

/// LF Receiver SRC Tuning MSB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSRCT1 | 1 |
/// | LFSRCT2 | 10 |
/// | LFSRCT6 | 100000 |
/// | LFSRCT8 | 10000000 |
/// | LFSRCT4 | 1000 |
/// | LFSRCT7 | 1000000 |
/// | LFSRCT5 | 10000 |
/// | LFSRCT3 | 100 |
pub const LFSRCTM: *mut u8 = 0x151 as *mut u8;

/// DeBounce Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBCS | 10 |
/// | DBHA | 1000 |
/// | DBMD | 1 |
/// | DBTMS | 100 |
pub const DBCR: *mut u8 = 0x152 as *mut u8;

/// Debounce Timer Compare Register.
pub const DBTC: *mut u8 = 0x153 as *mut u8;

/// DeBounce Enable Port B.
pub const DBENB: *mut u8 = 0x154 as *mut u8;

/// DeBounce Enable Port C.
pub const DBENC: *mut u8 = 0x155 as *mut u8;

/// Debugging Support Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBGGS | 1111 |
/// | ATEST | 10000000 |
/// | CPBFOS | 110000 |
/// | CPBF | 1000000 |
pub const DBGSW: *mut u8 = 0x156 as *mut u8;

/// SPI FIFO Fill Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RFC | 1000 |
/// | RFL | 111 |
/// | TFL | 1110000 |
/// | TFC | 10000000 |
pub const SFFR: *mut u8 = 0x157 as *mut u8;

/// SPI FIFO Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STIE | 10000000 |
/// | SRIE | 1000 |
/// | TIL | 1110000 |
/// | RIL | 111 |
pub const SFIR: *mut u8 = 0x158 as *mut u8;

/// Timer2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2COF | 10 |
/// | T2OFF | 1 |
pub const T2IFR: *mut u8 = 0x159 as *mut u8;

/// Program Memory Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGMSYN | 11111 |
pub const PGMST: *mut u8 = 0x15A as *mut u8;

/// EEPROM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESYN | 1111 |
pub const EEST: *mut u8 = 0x15B as *mut u8;

/// LF Receiver SRC Tuning LSB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSRCT0 | 1 |
pub const LFSRCTL: *mut u8 = 0x15C as *mut u8;

/// Pin change Interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
/// | PCIF1 | 10 |
pub const PCIFR: *mut u8 = 0x161 as *mut u8;

/// Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PS | 111 |
/// | T0IE | 1000 |
/// | T0PR | 10000 |
pub const T0CR: *mut u8 = 0x162 as *mut u8;

/// DeBounce Enable Port D.
pub const DBEND: *mut u8 = 0x164 as *mut u8;

/// Transponder Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPMODE | 10000000 |
/// | TPQPLM | 100 |
/// | TPBR | 10000 |
/// | TPDFCP | 1100000 |
pub const TPCR1: *mut u8 = 0x165 as *mut u8;

/// Transponder Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPNFTIM | 100 |
/// | TPIM | 1 |
/// | TPFTIM | 10 |
/// | TPBERIM | 1000 |
pub const TPIMR: *mut u8 = 0x166 as *mut u8;

/// Transponder Decoder Comparator Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL1 | 111111 |
pub const TPDCR1: *mut u8 = 0x167 as *mut u8;

/// Transponder Decoder Comparator Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL2 | 111111 |
pub const TPDCR2: *mut u8 = 0x168 as *mut u8;

/// Transponder Decoder Comparator Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL3 | 111111 |
pub const TPDCR3: *mut u8 = 0x169 as *mut u8;

/// Transponder Decoder Comparator Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL4 | 111111 |
pub const TPDCR4: *mut u8 = 0x16A as *mut u8;

/// Transponder Decoder Comparator Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPDCL5 | 111111 |
pub const TPDCR5: *mut u8 = 0x16B as *mut u8;

/// Transponder Encoder Comparator Register 1.
pub const TPECR1: *mut u8 = 0x16C as *mut u8;

/// Transponder Encoder Comparator Register 2.
pub const TPECR2: *mut u8 = 0x16D as *mut u8;

/// Transponder Encoder Comparator Register 3.
pub const TPECR3: *mut u8 = 0x16E as *mut u8;

/// Transponder Encoder Comparator Register 4.
pub const TPECR4: *mut u8 = 0x16F as *mut u8;

/// Transponder Encoder Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPECM4 | 11000000 |
/// | TPECM3 | 110000 |
/// | TPECM2 | 1100 |
/// | TPECM1 | 11 |
pub const TPECMR: *mut u8 = 0x170 as *mut u8;

/// Transponder Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPTD | 1 |
/// | TPTLIW | 100 |
/// | TPRD | 10 |
/// | TPRCD | 100000 |
pub const TPCR3: *mut u8 = 0x171 as *mut u8;

/// Transponder Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPBCM | 10000 |
/// | TPBCCS | 1111 |
pub const TPCR4: *mut u8 = 0x172 as *mut u8;

/// Transponder Control Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPMD | 1110000 |
/// | TPMUD | 111 |
pub const TPCR5: *mut u8 = 0x173 as *mut u8;

/// Transponder Calibration Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPBG_IREF | 111111 |
pub const TPCALR1: *mut u8 = 0x175 as *mut u8;

/// Transponder Calibration Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPBG_UREF | 1111111 |
pub const TPCALR2: *mut u8 = 0x176 as *mut u8;

/// Transponder Calibration Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFVCC_TPCAL1 | 10 |
/// | TPORTH | 11000 |
/// | LFVCC_TPCAL0 | 1 |
/// | LFVCC_TPCAL2 | 100 |
pub const TPCALR3: *mut u8 = 0x177 as *mut u8;

/// Transponder Calibration Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPINIT_CAL | 111 |
/// | COMPVC_CAL | 11000 |
pub const TPCALR4: *mut u8 = 0x178 as *mut u8;

/// Transponder Calibration Register 5.
pub const TPCALR5: *mut u8 = 0x179 as *mut u8;

/// Transponder Calibration Register 6.
pub const TPCALR6: *mut u8 = 0x17A as *mut u8;

/// Transponder Calibration Register 7.
pub const TPCALR7: *mut u8 = 0x17B as *mut u8;

/// Transponder Calibration Register 8.
pub const TPCALR8: *mut u8 = 0x17C as *mut u8;

/// Transponder Calibration Register 9.
pub const TPCALR9: *mut u8 = 0x17D as *mut u8;

/// Transponder Calibration Register 10.
pub const TPCALR10: *mut u8 = 0x17E as *mut u8;

/// AES Data Pointer Register.
pub const AESDPR: *mut u8 = 0x17F as *mut u8;

/// AES Key Register.
pub const AESKR: *mut u8 = 0x180 as *mut u8;

/// AES Data Register.
pub const AESDR: *mut u8 = 0x181 as *mut u8;

/// General Purpose I/O Register 3.
pub const GPIOR3: *mut u8 = 0x182 as *mut u8;

/// General Purpose I/O Register 4.
pub const GPIOR4: *mut u8 = 0x183 as *mut u8;

/// General Purpose I/O Register 5.
pub const GPIOR5: *mut u8 = 0x184 as *mut u8;

/// General Purpose I/O Register 6.
pub const GPIOR6: *mut u8 = 0x185 as *mut u8;

/// General Purpose I/O Register 7.
pub const GPIOR7: *mut u8 = 0x186 as *mut u8;

/// General Purpose I/O Register 8.
pub const GPIOR8: *mut u8 = 0x187 as *mut u8;

/// Protocol Handler Bit Counter Read Register.
pub const PHBCRR: *mut u8 = 0x188 as *mut u8;

/// LF Receiver Calibration Protect Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPCD | 1000000 |
/// | LFCALP | 1 |
/// | LFCPCE | 10000000 |
/// | LFCALRY | 10 |
pub const LFCPR: *mut u8 = 0x18E as *mut u8;

/// LF Receiver Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFEOIM | 100 |
/// | LFSYDIM | 1 |
/// | LFDEIM | 10 |
pub const LFIMR: *mut u8 = 0x18F as *mut u8;

/// PH ID0 Register.
pub const PHID0: *mut u32 = 0x190 as *mut u32;

/// PH Identifier 0 Length Register.
pub const PHID0L: *mut u8 = 0x194 as *mut u8;

/// PH ID1 Register.
pub const PHID1: *mut u32 = 0x195 as *mut u32;

/// PH Identifier 1 Length Register.
pub const PHID1L: *mut u8 = 0x199 as *mut u8;

/// Protocol Handler ID Frame Register.
pub const PHIDFR: *mut u8 = 0x19A as *mut u8;

/// LF Receiver Synchronization Symbols Register.
pub const LFSYSY: *mut u32 = 0x19B as *mut u32;

/// LF Receiver Synchronization Length Register.
pub const LFSYLE: *mut u8 = 0x19F as *mut u8;

/// LF Receiver Stop Bit Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFSTL | 1110000 |
/// | LFSTSY | 1111 |
pub const LFSTOP: *mut u8 = 0x1A0 as *mut u8;

/// LF Timer Compare Register.
pub const LTCOR: *mut u8 = 0x1A1 as *mut u8;

/// Timer1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1COF | 10 |
/// | T1OFF | 1 |
pub const T1IFR: *mut u8 = 0x1A2 as *mut u8;

/// Protocol Handler Telegram Bit Length Register.
pub const PHTBLR: *mut u8 = 0x1A4 as *mut u8;

/// Protocol Handler Data Frame end Register.
pub const PHDFR: *mut u8 = 0x1A5 as *mut u8;

/// LF Timer Event Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ID1EM | 10 |
/// | LTCOF | 10000000 |
/// | ID0EM | 1 |
/// | DFEM | 1000 |
/// | IDFEM | 100 |
/// | EOFEM | 1000000 |
/// | FLEM | 100000 |
/// | TBLEM | 10000 |
pub const LTEMR: *mut u8 = 0x1A6 as *mut u8;

/// LF Receiver Channel 3 Quality Faktor Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFQS3 | 1111 |
/// | LFCS3 | 11110000 |
pub const LFQC3: *mut u8 = 0x1A7 as *mut u8;

/// LF Receiver Channel 2 Quality Faktor Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFCS2 | 11110000 |
/// | LFQS2 | 1111 |
pub const LFQC2: *mut u8 = 0x1A8 as *mut u8;

/// LF Receiver Channel 1 Quality Faktor Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFCS1 | 11110000 |
/// | LFQS1 | 1111 |
pub const LFQC1: *mut u8 = 0x1A9 as *mut u8;

/// TWI2 Bit Rate Register.
pub const TW2BR: *mut u8 = 0x1AA as *mut u8;

/// TWI2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2EA | 1000000 |
/// | TW2STA | 100000 |
/// | TW2STO | 10000 |
/// | TW2IE | 1 |
/// | TW2INT | 10000000 |
/// | TW2WC | 1000 |
/// | TW2EN | 100 |
pub const TW2CR: *mut u8 = 0x1AB as *mut u8;

/// TWI2 Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2S | 11111000 |
/// | TW2PS | 11 |
pub const TW2SR: *mut u8 = 0x1AC as *mut u8;

/// TWI2 Data Register.
pub const TW2DR: *mut u8 = 0x1AD as *mut u8;

/// TWI2 (Slave) Address Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2A | 11111110 |
/// | TW2GCE | 1 |
pub const TW2AR: *mut u8 = 0x1AE as *mut u8;

/// TWI2 Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW2AM | 11111110 |
pub const TW2AMR: *mut u8 = 0x1AF as *mut u8;

/// RSSI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES | 10000000 |
/// | RSMODE0 | 10000 |
/// | RSMODE1 | 100000 |
/// | RSOS | 10 |
/// | RSOFM | 1000 |
/// | RSEOR | 100 |
/// | RSSDEN | 1 |
pub const RSCR: *mut u8 = 0x1B0 as *mut u8;

/// RSSI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSSVLD | 10 |
/// | RSRDY | 1 |
pub const RSSR: *mut u8 = 0x1B1 as *mut u8;

/// RSSI Measurement Setting 1 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSSTIM | 10000 |
/// | RSSCAL | 10000000 |
/// | RSSSV | 1000000 |
/// | RSCH2E | 10 |
/// | RSINTM | 1000 |
/// | RSCH3E | 100 |
/// | RSCH1E | 1 |
/// | RSCMS | 100000 |
pub const RSMS1R: *mut u8 = 0x1B2 as *mut u8;

/// RSSI Measurement Setting 2 Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSAVGS3 | 10000000 |
/// | RSAVGS2 | 1000000 |
/// | RSSADR2 | 100 |
/// | RSSADR0 | 1 |
/// | RSSADR1 | 10 |
/// | RSAVGS1 | 100000 |
/// | RSAVGS0 | 10000 |
/// | RSSADR3 | 1000 |
pub const RSMS2R: *mut u8 = 0x1B3 as *mut u8;

/// RSSI Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSOFF | 1000 |
/// | RSAOOR2 | 1000000 |
/// | RSOOR3 | 100 |
/// | RSAOOR3 | 10000000 |
/// | RSOOR2 | 10 |
/// | RSOOR1 | 1 |
/// | RSAOOR1 | 100000 |
pub const RSFR: *mut u8 = 0x1B4 as *mut u8;

/// RSSI Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSCALIB0 | 1 |
/// | RSCALIB4 | 10000 |
/// | RSCALIB7 | 10000000 |
/// | RSCALIB6 | 1000000 |
/// | RSCALIB1 | 10 |
/// | RSCALIB5 | 100000 |
/// | RSCALIB3 | 1000 |
/// | RSCALIB2 | 100 |
pub const RSCALIB: *mut u8 = 0x1B6 as *mut u8;

/// RSSI Delay Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTRD1 | 10 |
/// | RSTRD0 | 1 |
/// | RSRD1 | 10000000 |
/// | RSTRD2 | 100 |
/// | RSTRD3 | 1000 |
/// | RSTRD4 | 10000 |
/// | RSRD0 | 1000000 |
/// | RSTRD5 | 100000 |
pub const RSDLYR: *mut u8 = 0x1B7 as *mut u8;

/// RSSI Result 1 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES1L4 | 10000 |
/// | RSRES1L2 | 100 |
/// | RSRES1L0 | 1 |
/// | RSRES1L6 | 1000000 |
/// | RSRES1L7 | 10000000 |
/// | RSRES1L5 | 100000 |
/// | RSRES1L1 | 10 |
/// | RSRES1L3 | 1000 |
pub const RSRES1L: *mut u8 = 0x1B8 as *mut u8;

/// RSSI Result 1 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES1H5 | 100000 |
/// | RSRES1H1 | 10 |
/// | RSRES1H6 | 1000000 |
/// | RSRES1H4 | 10000 |
/// | RSRES1H3 | 1000 |
/// | RSRES1H7 | 10000000 |
/// | RSRES1H2 | 100 |
/// | RSRES1H0 | 1 |
pub const RSRES1H: *mut u8 = 0x1B9 as *mut u8;

/// RSSI Result 2 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES2L4 | 10000 |
/// | RSRES2L7 | 10000000 |
/// | RSRES2L6 | 1000000 |
/// | RSRES2L0 | 1 |
/// | RSRES2L2 | 100 |
/// | RSRES2L3 | 1000 |
/// | RSRES2L1 | 10 |
/// | RSRES2L5 | 100000 |
pub const RSRES2L: *mut u8 = 0x1BA as *mut u8;

/// RSSI Result 2 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES2H0 | 1 |
/// | RSRES2H1 | 10 |
/// | RSRES2H3 | 1000 |
/// | RSRES2H4 | 10000 |
/// | RSRES2H2 | 100 |
/// | RSRES2H7 | 10000000 |
/// | RSRES2H5 | 100000 |
/// | RSRES2H6 | 1000000 |
pub const RSRES2H: *mut u8 = 0x1BB as *mut u8;

/// RSSI Result 3 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES3L7 | 10000000 |
/// | RSRES3L5 | 100000 |
/// | RSRES3L3 | 1000 |
/// | RSRES3L2 | 100 |
/// | RSRES3L0 | 1 |
/// | RSRES3L6 | 1000000 |
/// | RSRES3L1 | 10 |
/// | RSRES3L4 | 10000 |
pub const RSRES3L: *mut u8 = 0x1BC as *mut u8;

/// RSSI Result 3 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES3H2 | 100 |
/// | RSRES3H4 | 10000 |
/// | RSRES3H7 | 10000000 |
/// | RSRES3H6 | 1000000 |
/// | RSRES3H3 | 1000 |
/// | RSRES3H1 | 10 |
/// | RSRES3H0 | 1 |
/// | RSRES3H5 | 100000 |
pub const RSRES3H: *mut u8 = 0x1BD as *mut u8;

/// RSSI Result 4 Low Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES4L7 | 10000000 |
/// | RSRES4L0 | 1 |
/// | RSRES4L5 | 100000 |
/// | RSRES4L6 | 1000000 |
/// | RSRES4L4 | 10000 |
/// | RSRES4L1 | 10 |
/// | RSRES4L3 | 1000 |
/// | RSRES4L2 | 100 |
pub const RSRES4L: *mut u8 = 0x1BE as *mut u8;

/// RSSI Result 4 High Byte Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSRES4H2 | 100 |
/// | RSRES4H5 | 100000 |
/// | RSRES4H3 | 1000 |
/// | RSRES4H6 | 1000000 |
/// | RSRES4H0 | 1 |
/// | RSRES4H1 | 10 |
/// | RSRES4H7 | 10000000 |
/// | RSRES4H4 | 10000 |
pub const RSRES4H: *mut u8 = 0x1BF as *mut u8;

/// RSSI SRC Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCCLR | 10000 |
/// | SRCSTEP0 | 1000000 |
/// | SRCMODE1 | 10 |
/// | SRCMODE0 | 1 |
/// | SRCSTEP1 | 10000000 |
/// | SRCMIN1 | 1000 |
/// | SRCMIN0 | 100 |
pub const RSSRCR: *mut u8 = 0x1C0 as *mut u8;

/// Sign Detection Channel 1 vs 2 Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD12RR5 | 100000 |
/// | SD12RR4 | 10000 |
/// | SD12RR2 | 100 |
/// | SD12RR3 | 1000 |
/// | SD12RR0 | 1 |
/// | SD12RR7 | 10000000 |
/// | SD12RR1 | 10 |
/// | SD12RR6 | 1000000 |
pub const SD12RR: *mut u8 = 0x1C1 as *mut u8;

/// Sign Detection Channel 1 vs 3 Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD13RR2 | 100 |
/// | SD13RR1 | 10 |
/// | SD13RR6 | 1000000 |
/// | SD13RR5 | 100000 |
/// | SD13RR3 | 1000 |
/// | SD13RR4 | 10000 |
/// | SD13RR0 | 1 |
/// | SD13RR7 | 10000000 |
pub const SD13RR: *mut u8 = 0x1C2 as *mut u8;

/// Sign Detection Channel 2 vs 3 Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD23RR4 | 10000 |
/// | SD23RR1 | 10 |
/// | SD23RR5 | 100000 |
/// | SD23RR2 | 100 |
/// | SD23RR0 | 1 |
/// | SD23RR3 | 1000 |
/// | SD23RR7 | 10000000 |
/// | SD23RR6 | 1000000 |
pub const SD23RR: *mut u8 = 0x1C3 as *mut u8;

/// Sign Detection 360 Degree Result Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SD360R5 | 100000 |
/// | SD360R7 | 10000000 |
/// | SD360R3 | 1000 |
/// | SD360R1 | 10 |
/// | SD360R0 | 1 |
/// | SD360R2 | 100 |
/// | SD360R6 | 1000000 |
/// | SD360R4 | 10000 |
pub const SD360R: *mut u8 = 0x1C4 as *mut u8;

/// RSSI Debug Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSDBGEN | 10000000 |
/// | RSHOME | 10000 |
/// | RSFPD | 1000 |
/// | RSDBGS0 | 100000 |
/// | RSSANA | 1 |
/// | RSINFM | 100 |
/// | RSDBGS1 | 1000000 |
pub const RSDBGR: *mut u8 = 0x1C5 as *mut u8;

/// LF Data FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFOF | 100 |
/// | LDFUF | 10 |
/// | LDFFLR | 1 |
pub const LDFS: *mut u8 = 0x1D1 as *mut u8;

/// Timer4 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4COF | 10 |
/// | T4ICF | 100 |
/// | T4OFF | 1 |
pub const T4IFR: *mut u8 = 0x1D2 as *mut u8;

/// LF Data FIFO Write Pointer.
pub const LDFWP: *mut u8 = 0x1D3 as *mut u8;

/// LF Data FIFO Read Pointer.
pub const LDFRP: *mut u8 = 0x1D4 as *mut u8;

/// Timer5 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OFF | 1 |
/// | T5COF | 10 |
pub const T5IFR: *mut u8 = 0x1D5 as *mut u8;

/// LF Data FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFFLIM | 1 |
/// | LDFEIM | 10 |
pub const LDFIM: *mut u8 = 0x1D6 as *mut u8;

/// LF Data FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDFMSB | 1000000 |
/// | LDFFLC | 111111 |
pub const LDFC: *mut u8 = 0x1D7 as *mut u8;

/// Protocol Handler Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHID0IM | 10000 |
/// | PHDFIM | 100 |
/// | PHID1IM | 100000 |
/// | PHIDFIM | 1000 |
/// | PHTBLIM | 10 |
pub const PHIMR: *mut u8 = 0x1D8 as *mut u8;

/// Protocol Handler CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCEN | 10000000 |
/// | CRCSE1 | 100000 |
/// | CRCFR | 100 |
/// | CRCSE0 | 10000 |
pub const PHCRCR: *mut u8 = 0x1D9 as *mut u8;

/// PH CRC Start Value Register low byte.
pub const PHCSTL: *mut u8 = 0x1DA as *mut u8;

/// PH CRC Start Value Register.
pub const PHCST: *mut u16 = 0x1DA as *mut u16;

/// PH CRC Start Value Register high byte.
pub const PHCSTH: *mut u8 = 0x1DB as *mut u8;

/// PH CRC Polynomial Register.
pub const PHCRP: *mut u16 = 0x1DC as *mut u16;

/// PH CRC Polynomial Register low byte.
pub const PHCRPL: *mut u8 = 0x1DC as *mut u8;

/// PH CRC Polynomial Register high byte.
pub const PHCRPH: *mut u8 = 0x1DD as *mut u8;

/// PH CRC Checksum Register.
pub const PHCSR: *mut u16 = 0x1DE as *mut u16;

/// PH CRC Checksum Register low byte.
pub const PHCSRL: *mut u8 = 0x1DE as *mut u8;

/// PH CRC Checksum Register high byte.
pub const PHCSRH: *mut u8 = 0x1DF as *mut u8;

/// CRC Data Input Register.
pub const CRCDIR: *mut u8 = 0x1E0 as *mut u8;

/// Timer3 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3OFF | 1 |
/// | T3ICF | 100 |
/// | T3COF | 10 |
pub const T3IFR: *mut u8 = 0x1E1 as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMCCE | 10000000 |
/// | CCS | 1000 |
/// | CMM | 111 |
/// | CMONEN | 1000000 |
pub const CMCR: *mut u8 = 0x1E3 as *mut u8;

/// Clock interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x1E4 as *mut u8;

/// Clock Prescaler Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 111 |
/// | CLPCE | 10000000 |
/// | CLTPS | 111000 |
pub const CLPR: *mut u8 = 0x1E5 as *mut u8;

/// Voltage Monitor Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMRS | 10000000 |
/// | VMPS | 1100000 |
/// | VMIM | 10000 |
/// | VMLS | 1111 |
pub const VMCR: *mut u8 = 0x1E6 as *mut u8;

/// Downbond Test Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BTEST5 | 100000 |
/// | BTEST4 | 10000 |
/// | AGND_BB | 10 |
/// | BBESD | 1 |
/// | BTEST6 | 1000000 |
/// | AGND_LF | 1000 |
/// | ISO_GND | 100 |
pub const DBONDR: *mut u8 = 0x1E7 as *mut u8;

/// Calibration ready signature LFVCC.
pub const CALRDYLF: *mut u8 = 0x1E8 as *mut u8;

/// TWI1 Bit Rate Register.
pub const TW1BR: *mut u8 = 0x1E9 as *mut u8;

/// TWI1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1IE | 1 |
/// | TW1EA | 1000000 |
/// | TW1STO | 10000 |
/// | TW1EN | 100 |
/// | TW1INT | 10000000 |
/// | TW1STA | 100000 |
/// | TW1WC | 1000 |
pub const TW1CR: *mut u8 = 0x1EA as *mut u8;

/// TWI1 Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1PS | 11 |
/// | TW1S | 11111000 |
pub const TW1SR: *mut u8 = 0x1EB as *mut u8;

/// TWI1 Data Register.
pub const TW1DR: *mut u8 = 0x1EC as *mut u8;

/// TWI1 (Slave) Address Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1GCE | 1 |
/// | TW1A | 11111110 |
pub const TW1AR: *mut u8 = 0x1ED as *mut u8;

/// TWI1 Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TW1AM | 11111110 |
pub const TW1AMR: *mut u8 = 0x1EE as *mut u8;

/// Pad Driver Strength Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSSISEL | 1000000 |
/// | STBTEST | 100000 |
/// | PDSC | 11111 |
/// | ATBSEL | 10000000 |
pub const PDSCR: *mut u8 = 0x1EF as *mut u8;

/// Timer Modulator Output Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TO1PIS | 11 |
/// | TO4PIS | 11000000 |
/// | TO2PIS | 1100 |
/// | TO3PIS | 110000 |
pub const TMOCR: *mut u8 = 0x1F0 as *mut u8;

/// Slow RC oscillator calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCCAL3 | 100 |
/// | SRCCAL7 | 1000000 |
/// | SRCCAL2 | 10 |
/// | SRCCAL4 | 1000 |
/// | SRCCAL6 | 100000 |
/// | SRCCAL1 | 1 |
/// | SRCCAL5 | 10000 |
/// | SRCCAL8 | 10000000 |
pub const SRCCAL: *mut u8 = 0x1F1 as *mut u8;

/// SRC oscillator Temperature Compensation register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCTC | 111 |
/// | DIS_SRC | 1000000 |
/// | SRCS | 11000 |
/// | HOLD_SRC | 10000000 |
pub const SRCTCAL: *mut u8 = 0x1F2 as *mut u8;

/// Supply calibration register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IPTAT | 111111 |
pub const SUPCA5: *mut u8 = 0x1F3 as *mut u8;

/// Supply calibration register 6.
pub const SUPCA6: *mut u8 = 0x1F4 as *mut u8;

/// Supply calibration register 7.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFVCCBD | 111000 |
/// | VCCCAL | 111 |
pub const SUPCA7: *mut u8 = 0x1F5 as *mut u8;

/// Supply calibration register 8.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DVCCBD | 111000 |
/// | VSWBD | 111 |
pub const SUPCA8: *mut u8 = 0x1F6 as *mut u8;

/// Supply calibration register 9.
pub const SUPCA9: *mut u8 = 0x1F7 as *mut u8;

/// Supply calibration register 10.
pub const SUPCA10: *mut u8 = 0x1F8 as *mut u8;

/// Transponder Calibration Register 11.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENVSWBD | 10000 |
/// | ENLFBD | 1000 |
/// | TPCALR117 | 10000000 |
/// | TPCALR116 | 1000000 |
/// | ENDVBD | 100 |
/// | MTBTR0 | 1 |
/// | TPCALR115 | 100000 |
/// | MTBTR1 | 10 |
pub const TPCALR11: *mut u8 = 0x1F9 as *mut u8;

/// Transponder Calibration Register 12.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TPCALR123 | 1000 |
/// | TPCALR122 | 100 |
/// | TPCALR124 | 10000 |
/// | TPDMOD | 1 |
/// | TPCALR127 | 10000000 |
/// | TPCALR125 | 100000 |
/// | TPCALR121 | 10 |
/// | TPCALR126 | 1000000 |
pub const TPCALR12: *mut u8 = 0x1FA as *mut u8;

/// Transponder Calibration Register 13.
pub const TPCALR13: *mut u8 = 0x1FB as *mut u8;

/// Power Management Test Enable Register.
pub const PMTER: *mut u8 = 0x1FE as *mut u8;

/// Slow RC oscillator calibration LSB.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCCAL0 | 1 |
pub const SRCCALL: *mut u8 = 0x1FF as *mut u8;

/// Bitfield on register `AESCR`
pub const AESLKM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESRES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESXOR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESWK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AESCR`
pub const AESWD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `AESSR`
pub const AESRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `AESSR`
pub const AESERF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKOCR`
pub const CLKOS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKOCR`
pub const CLKOEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CLPR`
pub const CLKPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CLPR`
pub const CLPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLPR`
pub const CLTPS: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMCCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CMCR`
pub const CCS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMONEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CMIMR`
pub const ECIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMOCR`
pub const FRCACT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CMOCR`
pub const MRCAO: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CMOCR`
pub const FRCAO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMSR`
pub const ECF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CRCCR`
pub const REFLI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CRCCR`
pub const REFLO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CRCCR`
pub const CRCRS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBCS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBHA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBMD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBTMS: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DBGSW`
pub const DBGGS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DBGSW`
pub const ATEST: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DBGSW`
pub const CPBFOS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DBGSW`
pub const CPBF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DBONDR`
pub const BTEST5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DBONDR`
pub const BTEST4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DBONDR`
pub const AGND_BB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DBONDR`
pub const BBESD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DBONDR`
pub const BTEST6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DBONDR`
pub const AGND_LF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DBONDR`
pub const ISO_GND: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DFC`
pub const DFDRA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFC`
pub const DFFLC: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `DFI`
pub const DFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DFI`
pub const DFERIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DFL`
pub const DFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFL`
pub const DFFLS: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `DFS`
pub const DFOFL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DFS`
pub const DFFLRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DFS`
pub const DFUFL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPAGE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR2`
pub const EEBRE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR2`
pub const E2FF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EECR2`
pub const E2CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR2`
pub const E2CF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR2`
pub const E2AVF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EEPR`
pub const EEAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS4WD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS7RD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS5WD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS6RD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS5RD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS6WD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS7WD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EEPR1`
pub const EEPS4RD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS10RD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS9WD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS11RD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS11WD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS8WD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS8RD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS9RD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EEPR2`
pub const EEPS10WD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EEPR3`
pub const EEPS12WD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EEPR3`
pub const EEPS12RD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EEST`
pub const EESYN: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LDFC`
pub const LDFMSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LDFC`
pub const LDFFLC: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LDFCKSW`
pub const LDFSCKS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LDFCKSW`
pub const LDFSCSW: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LDFFL`
pub const LDFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LDFIM`
pub const LDFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LDFIM`
pub const LDFEIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LDFS`
pub const LDFOF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LDFS`
pub const LDFUF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LDFS`
pub const LDFFLR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCALR1`
pub const LFSTC: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LFCALR1`
pub const ICOMPRT: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `LFCALR1`
pub const SEL150M: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `LFCALR2`
pub const TIKOMPD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFCALR2`
pub const LFSRM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFCALR2`
pub const LFSTRES: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN26: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN25: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN24: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN21: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN27: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN20: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN23: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFCALR4`
pub const TCGAIN22: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN34: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN31: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN37: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN30: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN35: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN32: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCALR5`
pub const TCGAIN36: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFCALR6`
pub const TCGAIN41: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFCALR6`
pub const TCGAIN43: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFCALR6`
pub const TCGAIN44: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LFCALR6`
pub const TCGAIN42: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCALR6`
pub const TCGAIN40: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCPR`
pub const TPCD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFCPR`
pub const LFCALP: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCPR`
pub const LFCPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFCPR`
pub const LFCALRY: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFCR0`
pub const LFRRT: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LFCR0`
pub const LFCE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCR0`
pub const LFBR: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `LFCR0`
pub const LFMG: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LFCR0`
pub const LFCE3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCR0`
pub const LFCE2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFCR1`
pub const ARMDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFCR1`
pub const ADTHEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LFCR1`
pub const LFFM1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCR1`
pub const LFPEEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFCR1`
pub const RSST: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LFCR1`
pub const LFRE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFCR1`
pub const FLLEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LFCR2`
pub const LFDAMP: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCR2`
pub const LFSEN: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LFCR2`
pub const LFVC: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `LFCR3`
pub const LFRCTEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCR3`
pub const LFRCPM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFCR3`
pub const LFRCPCEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFCR3`
pub const LFSBEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFCR3`
pub const LFTS: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `LFCR3`
pub const LFTON: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFDSR1`
pub const LOTHA: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LFDSR1`
pub const CTTHA: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LFDSR1`
pub const HITHA: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LFDSR10`
pub const FCL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LFDSR10`
pub const STBTH: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LFDSR11`
pub const TINITA: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LFDSR11`
pub const TINITB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `LFDSR2`
pub const CTTHB: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LFDSR2`
pub const HITHB: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LFDSR2`
pub const LOTHB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LFDSR3`
pub const PBDTH: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LFDSR3`
pub const QCTH: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `LFDSR4`
pub const SDTHA: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LFDSR4`
pub const SRSTC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LFDSR4`
pub const SCTHA: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `LFDSR5`
pub const SCTHB: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `LFDSR5`
pub const SSUTB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFDSR5`
pub const SDTHB: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LFDSR5`
pub const SSUTA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFDSR6`
pub const TODU: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LFDSR6`
pub const TODS: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `LFDSR7`
pub const PBG: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LFDSR7`
pub const MDG: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LFDSR7`
pub const MDSP: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LFDSR7`
pub const PBSP: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LFDSR8`
pub const CLD: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LFDSR8`
pub const LGFE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFDSR8`
pub const ASWTH: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `LFDSR9`
pub const STW: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `LFFR`
pub const LFSYDF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFSD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFES: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFEOF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFDEF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFTOF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFIMR`
pub const LFEOIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFIMR`
pub const LFSYDIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFIMR`
pub const LFDEIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFQC1`
pub const LFCS1: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `LFQC1`
pub const LFQS1: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LFQC2`
pub const LFCS2: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `LFQC2`
pub const LFQS2: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LFQC3`
pub const LFQS3: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LFQC3`
pub const LFCS3: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `LFSRCTL`
pub const LFSRCT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT6: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT4: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT7: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT5: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LFSRCTM`
pub const LFSRCT3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFSTOP`
pub const LFSTL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `LFSTOP`
pub const LFSTSY: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const BOOTRST: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOW`
pub const EEACC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LOW`
pub const CKSTART: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const PCEE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTPS1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTPS0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTCRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTSM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTCIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTCM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LTCMR`
pub const LTPS2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LTEMR`
pub const ID1EM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LTEMR`
pub const LTCOF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LTEMR`
pub const ID0EM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LTEMR`
pub const DFEM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LTEMR`
pub const IDFEM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LTEMR`
pub const EOFEM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LTEMR`
pub const FLEM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LTEMR`
pub const TBLEM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ENPS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SPIIO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUCR`
pub const TRCCE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const TRCEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUSR`
pub const DWRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const TPRF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MSMCR1`
pub const MSMSM0: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR1`
pub const MSMSM1: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR2`
pub const MSMSM2: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR2`
pub const MSMSM3: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR3`
pub const MSMSM4: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR3`
pub const MSMSM5: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR4`
pub const MSMSM6: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR4`
pub const MSMSM7: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMSTR`
pub const SSMMST: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT11: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT9: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT10: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT15: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT12: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT13: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT14: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PDSCR`
pub const RSSISEL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PDSCR`
pub const STBTEST: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PDSCR`
pub const PDSC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PDSCR`
pub const ATBSEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PGMST`
pub const PGMSYN: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PHCRCR`
pub const CRCEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PHCRCR`
pub const CRCSE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PHCRCR`
pub const CRCFR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PHCRCR`
pub const CRCSE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PHFR`
pub const PHID0F: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PHFR`
pub const PHTBLF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PHFR`
pub const PHDFF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PHFR`
pub const PHID1F: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PHFR`
pub const CRCEF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PHFR`
pub const PHIDFF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PHIMR`
pub const PHID0IM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PHIMR`
pub const PHDFIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PHIMR`
pub const PHID1IM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PHIMR`
pub const PHIDFIM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PHIMR`
pub const PHTBLIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PHTCR`
pub const CSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PHTCR`
pub const CPM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PHTCR`
pub const FRFIFO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTXDC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCU: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRVM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCRC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRLFRS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT4: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRLFPH: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRLFTP: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRLFR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT5: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRTM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR2`
pub const VXTSDI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRTWI2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRDF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSPI2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RCTCAL`
pub const DI_MRCBG: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RCTCAL`
pub const MRCTC: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `RCTCAL`
pub const FRCTC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSCALIB`
pub const RSCALIB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSRES: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSMODE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSMODE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSOS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSOFM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSEOR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSCR`
pub const RSSDEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSDBGEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSHOME: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSFPD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSDBGS0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSSANA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSINFM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSDBGR`
pub const RSDBGS1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSTRD1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSTRD0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSRD1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSTRD2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSTRD3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSTRD4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSRD0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSDLYR`
pub const RSTRD5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSOFF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSAOOR2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSOOR3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSAOOR3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSOOR2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSOOR1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSFR`
pub const RSAOOR1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSSTIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSSCAL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSSSV: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSCH2E: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSINTM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSCH3E: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSCH1E: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSMS1R`
pub const RSCMS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSAVGS3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSAVGS2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSSADR2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSSADR0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSSADR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSAVGS1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSAVGS0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSMS2R`
pub const RSSADR3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES1H`
pub const RSRES1H0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES1L`
pub const RSRES1L3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES2H`
pub const RSRES2H6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES2L`
pub const RSRES2L5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES3H`
pub const RSRES3H5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES3L`
pub const RSRES3L4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES4H`
pub const RSRES4H4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSRES4L`
pub const RSRES4L2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSSR`
pub const RSSVLD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSSR`
pub const RSRDY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCCLR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCSTEP0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCMODE1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCMODE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCSTEP1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCMIN1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSSRCR`
pub const SRCMIN0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SD12RR`
pub const SD12RR6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SD13RR`
pub const SD13RR7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SD23RR`
pub const SD23RR6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SD360R`
pub const SD360R4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SFC`
pub const SFDRA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFC`
pub const SFFLC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFFR`
pub const RFC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SFFR`
pub const RFL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SFFR`
pub const TFL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `SFFR`
pub const TFC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFI`
pub const SFERIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFI`
pub const SFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFIR`
pub const STIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIR`
pub const SRIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SFIR`
pub const TIL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `SFIR`
pub const RIL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SFL`
pub const SFFLS: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFL`
pub const SFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFS`
pub const SFFLRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFS`
pub const SFUFL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFS`
pub const SFOFL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SP2CR`
pub const DORD2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SP2CR`
pub const SP2R: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SP2CR`
pub const CPHA2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SP2CR`
pub const SP2IE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SP2CR`
pub const SP2E: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SP2CR`
pub const CPOL2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SP2CR`
pub const MSTR2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SP2SR`
pub const SP2IF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SP2SR`
pub const WCOL2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SP2SR`
pub const SPI22X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const FLSEL: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPSR`
pub const TXIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const RXIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL7: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL4: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL6: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL5: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCCAL8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SRCCALL`
pub const SRCCAL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SRCTCAL`
pub const SRCTC: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SRCTCAL`
pub const DIS_SRC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SRCTCAL`
pub const SRCS: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `SRCTCAL`
pub const HOLD_SRC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTGE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMPVE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTPE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTAE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMPLDT: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMIFR`
pub const SSMIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMIMR`
pub const SSMIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMRR`
pub const SSMST: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMRR`
pub const SSMR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMSR`
pub const SSMESM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SSMSR`
pub const SSMERR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMSTR`
pub const SSMSTA: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SUPCA1`
pub const PV22: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPCA1`
pub const PVDIC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SUPCA1`
pub const PVCAL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SUPCA2`
pub const BGCAL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SUPCA4`
pub const ICONST: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SUPCA5`
pub const IPTAT: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SUPCA7`
pub const LFVCCBD: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `SUPCA7`
pub const VCCCAL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SUPCA8`
pub const DVCCBD: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `SUPCA8`
pub const VSWBD: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVCCLM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVCCRM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SUPCR`
pub const VMEMEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SUPCR`
pub const VMRESM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SUPCR`
pub const PVEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVDIC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SUPCR`
pub const DVHEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SUPFR`
pub const AVCCLF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPFR`
pub const AVCCRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0IE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T0IFR`
pub const T0F: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T1IFR`
pub const T1COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1IFR`
pub const T1OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1IMR`
pub const T1OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1IMR`
pub const T1CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1PS: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `T1MR`
pub const T1CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1DC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2PS: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `T2MR`
pub const T2CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2DC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CPRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3PS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3CNC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3ICS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CPRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4MRA`
pub const T4CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T4MRA`
pub const T4PS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4CNC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4ICS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T5CCR`
pub const T5CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T5CCR`
pub const T5CTC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T5IFR`
pub const T5OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T5IFR`
pub const T5COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T5IMR`
pub const T5OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T5IMR`
pub const T5CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TMOCR`
pub const TO1PIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TMOCR`
pub const TO4PIS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TMOCR`
pub const TO2PIS: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TMOCR`
pub const TO3PIS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TPCALR1`
pub const TPBG_IREF: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TPCALR11`
pub const ENVSWBD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const ENLFBD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const TPCALR117: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const TPCALR116: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const ENDVBD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const MTBTR0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const TPCALR115: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TPCALR11`
pub const MTBTR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR123: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR122: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR124: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPDMOD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR127: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR125: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR121: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPCALR12`
pub const TPCALR126: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TPCALR2`
pub const TPBG_UREF: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `TPCALR3`
pub const LFVCC_TPCAL1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPCALR3`
pub const TPORTH: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `TPCALR3`
pub const LFVCC_TPCAL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPCALR3`
pub const LFVCC_TPCAL2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPCALR4`
pub const TPINIT_CAL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TPCALR4`
pub const COMPVC_CAL: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `TPCR1`
pub const TPMODE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TPCR1`
pub const TPQPLM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPCR1`
pub const TPBR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TPCR1`
pub const TPDFCP: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `TPCR2`
pub const TPPSD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPCR2`
pub const TPMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPCR2`
pub const TPNFTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TPCR2`
pub const TPWDLV: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `TPCR2`
pub const TPD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TPCR2`
pub const TPMOD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPCR3`
pub const TPTD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPCR3`
pub const TPTLIW: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPCR3`
pub const TPRD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPCR3`
pub const TPRCD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TPCR4`
pub const TPBCM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TPCR4`
pub const TPBCCS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `TPCR5`
pub const TPMD: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `TPCR5`
pub const TPMUD: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TPDCR1`
pub const TPDCL1: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TPDCR2`
pub const TPDCL2: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TPDCR3`
pub const TPDCL3: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TPDCR4`
pub const TPDCL4: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TPDCR5`
pub const TPDCL5: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TPECMR`
pub const TPECM4: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TPECMR`
pub const TPECM3: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TPECMR`
pub const TPECM2: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TPECMR`
pub const TPECM1: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TPFR`
pub const TPNFTF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPFR`
pub const TPF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPFR`
pub const TPBERF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TPFR`
pub const TPFTF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPIMR`
pub const TPNFTIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TPIMR`
pub const TPIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPIMR`
pub const TPFTIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPIMR`
pub const TPBERIM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TPSR`
pub const TPGAP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TPSR`
pub const TPA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TPSR`
pub const TPBCOK: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TPSR`
pub const TPPSW: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TW1AMR`
pub const TW1AM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TW1AR`
pub const TW1GCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TW1AR`
pub const TW1A: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1IE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1EA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1STO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1EN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1INT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1STA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TW1CR`
pub const TW1WC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TW1SR`
pub const TW1PS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TW1SR`
pub const TW1S: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TW2AMR`
pub const TW2AM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TW2AR`
pub const TW2A: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TW2AR`
pub const TW2GCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2EA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2STA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2STO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2IE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2INT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2WC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TW2CR`
pub const TW2EN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TW2SR`
pub const TW2S: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TW2SR`
pub const TW2PS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `VMCR`
pub const VMRS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `VMCR`
pub const VMPS: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `VMCR`
pub const VMIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `VMCR`
pub const VMLS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VMSCR`
pub const VMDIH: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `VMSCR`
pub const VMF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VXMCTRL`
pub const EN_VX_IN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `VXMCTRL`
pub const VX_SEL1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `VXMCTRL`
pub const VX_SEL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VXMCTRL`
pub const EN_VX_OUT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `VXMCTRL`
pub const EN_VX: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// `CLKOUT_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod clkout_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_mrc.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CLK_SEL_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, No Prescaling.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK/8.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK/32.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK/64.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, CLK/128.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, CLK/256.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, CLK/1024.
   pub const VAL_0x07: u32 = 0x7;
}

/// `COMM_TWI1_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi1_prescale {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 4.
   pub const VAL_0x01: u32 = 0x1;
   /// 16.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_TWI2_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi2_prescale {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 4.
   pub const VAL_0x01: u32 = 0x1;
   /// 16.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_BUSY_OUT` value group
#[allow(non_upper_case_globals)]
pub mod cpu_busy_out {
   /// disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// PB0.
   pub const VAL_0x01: u32 = 0x1;
   /// PB3.
   pub const VAL_0x02: u32 = 0x2;
   /// PC1.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_CLK_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_3bits {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.
   pub const VAL_0x01: u32 = 0x1;
   /// 4.
   pub const VAL_0x02: u32 = 0x2;
   /// 8.
   pub const VAL_0x03: u32 = 0x3;
   /// 16.
   pub const VAL_0x04: u32 = 0x4;
   /// 32.
   pub const VAL_0x05: u32 = 0x5;
   /// 64.
   pub const VAL_0x06: u32 = 0x6;
   /// 128.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_CLT_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clt_prescale_3bits {
   /// disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// 1.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.
   pub const VAL_0x02: u32 = 0x2;
   /// 4.
   pub const VAL_0x03: u32 = 0x3;
   /// 8.
   pub const VAL_0x04: u32 = 0x4;
   /// 16.
   pub const VAL_0x05: u32 = 0x5;
   /// 32.
   pub const VAL_0x06: u32 = 0x6;
   /// 64.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_FUSE_LOCK_SEL_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_fuse_lock_sel_3bits {
   /// ROM/FLASH.
   pub const VAL_0x00: u32 = 0x0;
   /// Lockbits.
   pub const VAL_0x01: u32 = 0x8;
   /// Security Fuses.
   pub const VAL_0x03: u32 = 0x18;
   /// EEPROM Protection Fuse Low.
   pub const VAL_0x05: u32 = 0x28;
   /// EEPROM Protection Fuse High.
   pub const VALR_0x07: u32 = 0x38;
}

/// `CPU_IVL_2BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_ivl_2bits {
   /// 0x3600.
   pub const VAL_0x00: u32 = 0x0;
   /// 0x4000.
   pub const VAL_0x01: u32 = 0x1;
   /// 0x7000.
   pub const VAL_0x02: u32 = 0x2;
   /// 0x8000.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Power save.
   pub const PSAVE: u32 = 0x1;
   /// Power down.
   pub const PDOWN: u32 = 0x2;
   /// Extended power save.
   pub const EPSAVE: u32 = 0x3;
   /// Extended power down.
   pub const EPDOWN: u32 = 0x4;
   /// Power off.
   pub const POFF: u32 = 0x5;
}

/// `EEP_MODE` value group
#[allow(non_upper_case_globals)]
pub mod eep_mode {
   /// Erase and Write in one operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Erase Only.
   pub const VAL_0x01: u32 = 0x1;
   /// Write Only.
   pub const VAL_0x02: u32 = 0x2;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Further programming disabled.
   pub const VAL_0x02: u32 = 0x2;
   /// No memory lock features enable.
   pub const VAL_0x03: u32 = 0x3;
}

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Logical Change of INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFREC_BIT_RATE` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_bit_rate {
   /// 1.95 kBit/s.
   pub const VAL_0x00: u32 = 0x0;
   /// 3.90 kBit/s.
   pub const VAL_0x01: u32 = 0x1;
   /// 7.81 kBit/s.
   pub const VAL_0x02: u32 = 0x2;
}

/// `LFREC_RESET_TIME` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_reset_time {
   /// 128 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 160 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 192 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 224 us.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFREC_RSSI_RESET_TIME` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_rssi_reset_time {
   /// 256 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 384 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 512 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 640 us.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFREC_SENSITIVITY_MODE` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_sensitivity_mode {
   /// High Sensitivity.
   pub const VAL_0x00: u32 = 0x0;
   /// Medium Sensitivity.
   pub const VAL_0x01: u32 = 0x1;
   /// Low Sensitivity.
   pub const VAL_0x02: u32 = 0x2;
}

/// `LFREC_STANDBY_TIME` value group
#[allow(non_upper_case_globals)]
pub mod lfrec_standby_time {
   /// 384 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 768 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 1152 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 1536 us.
   pub const VAL_0x03: u32 = 0x3;
   /// 2304 us.
   pub const VAL_0x04: u32 = 0x4;
   /// 3072 us.
   pub const VAL_0x05: u32 = 0x5;
   /// 4608 us.
   pub const VAL_0x06: u32 = 0x6;
   /// 6144 us.
   pub const VAL_0x07: u32 = 0x7;
}

/// `LFTP_FIELDCLK_PRESCALER` value group
#[allow(non_upper_case_globals)]
pub mod lftp_fieldclk_prescaler {
   /// Field Clock / 1.
   pub const VAL_0x00: u32 = 0x0;
   /// Field Clock / 1.
   pub const VAL_0x01: u32 = 0x1;
   /// Field Clock / 2.
   pub const VAL_0x02: u32 = 0x2;
   /// Field Clock / 4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFTP_TPECM` value group
#[allow(non_upper_case_globals)]
pub mod lftp_tpecm {
   /// Manchester.
   pub const VAL_0x00: u32 = 0x0;
   /// Biphase.
   pub const VAL_0x01: u32 = 0x1;
   /// NRZ.
   pub const VAL_0x02: u32 = 0x2;
   /// Manchester.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LFTP_TPMUD` value group
#[allow(non_upper_case_globals)]
pub mod lftp_tpmud {
   /// 5.0 V.
   pub const VAL_0x00: u32 = 0x0;
   /// 5.4 V.
   pub const VAL_0x01: u32 = 0x1;
   /// 5.8 V.
   pub const VAL_0x02: u32 = 0x2;
   /// 6.2 V.
   pub const VAL_0x03: u32 = 0x3;
   /// 6.6 V.
   pub const VAL_0x04: u32 = 0x4;
   /// 7.0 V.
   pub const VAL_0x05: u32 = 0x5;
   /// Up to OVP.
   pub const VAL_0x07: u32 = 0x7;
}

/// `LFTP_TPWDLV` value group
#[allow(non_upper_case_globals)]
pub mod lftp_tpwdlv {
   /// 1.024 ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.048 ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 3.072 ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 4.096 ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SPI2_SP2R` value group
#[allow(non_upper_case_globals)]
pub mod spi2_sp2r {
   /// clkio/4.
   pub const VAL_0x00: u32 = 0x0;
   /// clkio/16.
   pub const VAL_0x01: u32 = 0x1;
   /// clkio/64.
   pub const VAL_0x02: u32 = 0x2;
   /// clkio/128.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SPI_SPR` value group
#[allow(non_upper_case_globals)]
pub mod spi_spr {
   /// clkio/4.
   pub const VAL_0x00: u32 = 0x0;
   /// clkio/16.
   pub const VAL_0x01: u32 = 0x1;
   /// clkio/64.
   pub const VAL_0x02: u32 = 0x2;
   /// clkio/128.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SSM_SUB_STATE_MACHINE` value group
#[allow(non_upper_case_globals)]
pub mod ssm_sub_state_machine {
   /// None/Stop.
   pub const VAL_0x00: u32 = 0x0;
   /// PLL en.
   pub const VAL_0x01: u32 = 0x1;
   /// PLL lock.
   pub const VAL_0x02: u32 = 0x2;
   /// TX DSP enable.
   pub const VAL_0x03: u32 = 0x3;
   /// TX DSP disable.
   pub const VAL_0x04: u32 = 0x4;
   /// Send telegram.
   pub const VAL_0x05: u32 = 0x5;
   /// Shut down.
   pub const VAL_0x06: u32 = 0x6;
   /// VCO Tuning.
   pub const VAL_0x07: u32 = 0x7;
   /// Antenna Tuning.
   pub const VAL_0x08: u32 = 0x8;
}

/// `TIM0_PS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim0_ps_select {
   /// 0.256ms typ.
   pub const VAL_0x00: u32 = 0x0;
   /// 1ms typ.
   pub const VAL_0x01: u32 = 0x1;
   /// 8ms typ.
   pub const VAL_0x02: u32 = 0x2;
   /// 0.5s typ.
   pub const VAL_0x03: u32 = 0x3;
   /// 1s typ.
   pub const VAL_0x04: u32 = 0x4;
   /// 8s typ.
   pub const VAL_0x05: u32 = 0x5;
   /// 67s typ.
   pub const VAL_0x06: u32 = 0x6;
   /// 134s typ.
   pub const VAL_0x07: u32 = 0x7;
}

/// `TIM0_WDPS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim0_wdps_select {
   /// 1ms typ (0.85ms min).
   pub const VAL_0x00: u32 = 0x0;
   /// 4ms typ (3.4ms min).
   pub const VAL_0x01: u32 = 0x1;
   /// 32ms typ (27ms min).
   pub const VAL_0x02: u32 = 0x2;
   /// 2.1s typ (1.75s min).
   pub const VAL_0x03: u32 = 0x3;
   /// 4.2s typ (3.5s min).
   pub const VAL_0x04: u32 = 0x4;
   /// 16.8s typ (14s min).
   pub const VAL_0x05: u32 = 0x5;
   /// 134s typ (110s min).
   pub const VAL_0x06: u32 = 0x6;
   /// 268s typ (220s min).
   pub const VAL_0x07: u32 = 0x7;
}

/// `TIM1_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim1_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_mrc.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM1_DC_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim1_dc_select {
   /// Bypass.
   pub const VAL_0x00: u32 = 0x0;
   /// Duty cycle 1/1 (div 2).
   pub const VAL_0x01: u32 = 0x1;
   /// Duty cycle 1/2 (div 3).
   pub const VAL_0x02: u32 = 0x2;
   /// Duty cycle 1/3 (div 4).
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM2_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim2_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_mrc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM2_DC_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim2_dc_select {
   /// Bypass.
   pub const VAL_0x00: u32 = 0x0;
   /// Duty cycle 1/1 (div 2).
   pub const VAL_0x01: u32 = 0x1;
   /// Duty cycle 1/2 (div 3).
   pub const VAL_0x02: u32 = 0x2;
   /// Duty cycle 1/3 (div 4).
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM3_CAPTURE_EDGE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim3_capture_edge_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// rising edge.
   pub const VAL_0x01: u32 = 0x1;
   /// falling edge.
   pub const VAL_0x02: u32 = 0x2;
   /// both edges.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM3_CAPTURE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim3_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T1.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T4.
   pub const VAL_0x02: u32 = 0x2;
   /// TICP.
   pub const VAL_0x03: u32 = 0x3;
   /// LFES.
   pub const VAL_0x04: u32 = 0x4;
   /// clk_src.
   pub const VAL_0x05: u32 = 0x5;
   /// TPGAP.
   pub const VAL_0x06: u32 = 0x6;
}

/// `TIM3_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim3_clock_select {
   /// clk_frc.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_xto4.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_TEI.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM4_CAPTURE_EDGE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_capture_edge_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// rising edge.
   pub const VAL_0x01: u32 = 0x1;
   /// falling edge.
   pub const VAL_0x02: u32 = 0x2;
   /// both edges.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM4_CAPTURE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T1.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T3.
   pub const VAL_0x02: u32 = 0x2;
   /// TICP.
   pub const VAL_0x03: u32 = 0x3;
   /// LFES.
   pub const VAL_0x04: u32 = 0x4;
   /// clk_src.
   pub const VAL_0x05: u32 = 0x5;
   /// TPGAP.
   pub const VAL_0x06: u32 = 0x6;
}

/// `TIM4_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_mrc.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_frc.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO1PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to1pis_select {
   /// Port D2 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x02: u32 = 0x2;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO2PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to2pis_select {
   /// Port D3 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO3PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to3pis_select {
   /// Port D4 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TO4PIS_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod to4pis_select {
   /// Port D5 Data Register.
   pub const VAL_0x00: u32 = 0x0;
   /// M1 - Toggle Register Timer1.
   pub const VAL_0x01: u32 = 0x1;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x02: u32 = 0x2;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x03: u32 = 0x3;
}

