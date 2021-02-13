//! The AVR ATA6286 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2V - 3.6V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODEN | 10 |
/// | CKDIV8 | 10000000 |
/// | CKOUT | 1000000 |
/// | TSRDI | 1 |
/// | FRCFS | 100 |
/// | WDRCON | 1000 |
/// | SUT_CKSEL | 110000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB1 | 110000 |
/// | BLB0 | 1100 |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 1000000 |
/// | EELOCK | 10000000 |
/// | SPIEN | 100000 |
/// | WDTON | 10000 |
/// | BOOTRST | 1 |
/// | BOOTSZ | 110 |
/// | EESAVE | 1000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

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

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECINS | 100000 |
/// | CMCCE | 10000000 |
/// | CMM | 11 |
/// | CMONEN | 1000 |
/// | CCS | 10000 |
/// | SRCD | 100 |
pub const CMCR: *mut u8 = 0x2F as *mut u8;

/// Clock Management Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0x30 as *mut u8;

/// Timer 2 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2ICS | 100000 |
/// | T2CTM | 10 |
/// | T2TS | 1000000 |
/// | T2CR | 100 |
/// | T2E | 10000000 |
/// | T2OTM | 1 |
/// | T2CRM | 1000 |
pub const T2CRA: *mut u8 = 0x31 as *mut u8;

/// Timer 2 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2SCE | 1 |
pub const T2CRB: *mut u8 = 0x32 as *mut u8;

/// Timer 3 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3SCE | 10 |
/// | T3AC | 1 |
/// | T3E | 10000000 |
/// | T3CR | 100 |
/// | T3TS | 1000000 |
pub const T3CRA: *mut u8 = 0x34 as *mut u8;

/// Voltage Monitor Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMEN | 1 |
/// | BODPD | 1000000 |
/// | VMLS | 1110 |
/// | BODLS | 10000000 |
/// | VMF | 100000 |
/// | VMIM | 10000 |
pub const VMCSR: *mut u8 = 0x36 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 111 |
pub const PCIFR: *mut u8 = 0x37 as *mut u8;

/// Low Frequency Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFBF | 10 |
/// | LFWPF | 1 |
/// | LFEDF | 100 |
/// | LFRF | 1000 |
pub const LFFR: *mut u8 = 0x38 as *mut u8;

/// Sensor Status + Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSENO | 10 |
/// | MSENF | 1 |
pub const SSFR: *mut u8 = 0x39 as *mut u8;

/// Timer1/0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0F | 1 |
/// | T1F | 10 |
pub const T10IFR: *mut u8 = 0x3A as *mut u8;

/// Timer2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2ICF | 100 |
/// | T2RXF | 1000 |
/// | T2COF | 10 |
/// | T2TCF | 100000 |
/// | T2TXF | 10000 |
/// | T2OFF | 1 |
pub const T2IFR: *mut u8 = 0x3B as *mut u8;

/// Timer3 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3COBF | 100 |
/// | T3COAF | 10 |
/// | T3OFF | 1 |
/// | T3ICF | 1000 |
pub const T3IFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 11 |
pub const EIFR: *mut u8 = 0x3D as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | EEMWE | 100 |
/// | EEWE | 10 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 111 |
pub const PCICR: *mut u8 = 0x43 as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 11 |
pub const EIMSK: *mut u8 = 0x44 as *mut u8;

/// Sensor Voltage Control Register.
pub const SVCR: *mut u8 = 0x47 as *mut u8;

/// Sensor Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SMS | 1 |
/// | SEN | 110 |
/// | SMEN | 1000 |
pub const SCR: *mut u8 = 0x48 as *mut u8;

/// Sensor Capacitor Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCCS | 11100 |
/// | SRCC | 11 |
pub const SCCR: *mut u8 = 0x49 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPOL | 1000 |
/// | CPHA | 100 |
/// | DORD | 100000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Timer 2 Modulator Data Register.
pub const T2MDR: *mut u8 = 0x4F as *mut u8;

/// LF RSSI Data Register.
pub const LFRR: *mut u8 = 0x50 as *mut u8;

/// LF receiver Control und Data Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFRST | 1000000 |
/// | LFDO | 1 |
/// | LFSCE | 10000000 |
pub const LFCDR: *mut u8 = 0x52 as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 1110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x53 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BORF | 100 |
/// | TSRF | 100000 |
/// | PORF | 1 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | IVSEL | 10 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Low Frequency Receive data Buffer.
pub const LFRB: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSRE | 10000 |
/// | RWWSB | 1000000 |
/// | BLBSET | 1000 |
/// | SPMIE | 10000000 |
/// | SELFPRGEN | 1 |
/// | PGWRT | 100 |
/// | PGERS | 10 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer 1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1PS | 111 |
/// | T1CS | 111000 |
/// | T1IE | 10000000 |
pub const T1CR: *mut u8 = 0x58 as *mut u8;

/// Timer 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PBS | 11100000 |
/// | T0PR | 10000 |
/// | T0IE | 1000 |
/// | T0PAS | 111 |
pub const T0CR: *mut u8 = 0x59 as *mut u8;

/// Clock Management Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x5B as *mut u8;

/// Clock Prescaler Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLTPS | 111000 |
/// | CLKPS | 111 |
/// | CLPCE | 10000000 |
pub const CLKPR: *mut u8 = 0x5C as *mut u8;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | I | 10000000 |
/// | C | 1 |
/// | T | 1000000 |
/// | Z | 10 |
/// | N | 100 |
/// | H | 100000 |
/// | S | 10000 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDPS | 111 |
/// | WDCE | 10000 |
pub const WDTCR: *mut u8 = 0x60 as *mut u8;

/// Sensor Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSIE | 1 |
pub const SIMSK: *mut u8 = 0x61 as *mut u8;

/// Temperature Sensor Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSSD | 1 |
pub const TSCR: *mut u8 = 0x64 as *mut u8;

/// SRC-Oscillator Calibration Register.
pub const SRCCAL: *mut u8 = 0x65 as *mut u8;

/// FRC-Oscillator Calibration Register.
pub const FRCCAL: *mut u8 = 0x66 as *mut u8;

/// Motion Sensor Voltage Calibration Register.
pub const MSVCAL: *mut u8 = 0x67 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 2.
pub const PCMSK2: *mut u8 = 0x6C as *mut u8;

/// Timer 2 Input Capture Register Low Byte.
pub const T2ICRL: *mut u8 = 0x6E as *mut u8;

/// Timer 2 Input Capture Register High Byte.
pub const T2ICR: *mut u8 = 0x6F as *mut u8;

/// Timer2 Compare Register  Bytes low byte.
pub const T2CORL: *mut u8 = 0x70 as *mut u8;

/// Timer2 Compare Register  Bytes.
pub const T2COR: *mut u16 = 0x70 as *mut u16;

/// Timer2 Compare Register  Bytes high byte.
pub const T2CORH: *mut u8 = 0x71 as *mut u8;

/// Timer 2 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2CS | 111 |
/// | T2CNC | 100000 |
/// | T2TP | 11000000 |
/// | T2CE | 11000 |
pub const T2MRA: *mut u8 = 0x72 as *mut u8;

/// Timer 2 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2CPOL | 1000000 |
/// | T2TOP | 10000 |
/// | T2SSIE | 10000000 |
/// | T2M | 1111 |
pub const T2MRB: *mut u8 = 0x73 as *mut u8;

/// Timer 2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2CIM | 10 |
/// | T2RXIM | 1000 |
/// | T2CPIM | 100 |
/// | T2TCIM | 100000 |
/// | T2TXIM | 10000 |
/// | T2OIM | 1 |
pub const T2IMR: *mut u8 = 0x74 as *mut u8;

/// Timer3 Input Capture Register  Bytes low byte.
pub const T3ICRL: *mut u8 = 0x76 as *mut u8;

/// Timer3 Input Capture Register  Bytes.
pub const T3ICR: *mut u16 = 0x76 as *mut u16;

/// Timer3 Input Capture Register  Bytes high byte.
pub const T3ICRH: *mut u8 = 0x77 as *mut u8;

/// Timer3 COmpare Register A  Bytes.
pub const T3CORA: *mut u16 = 0x78 as *mut u16;

/// Timer3 COmpare Register A  Bytes low byte.
pub const T3CORAL: *mut u8 = 0x78 as *mut u8;

/// Timer3 COmpare Register A  Bytes high byte.
pub const T3CORAH: *mut u8 = 0x79 as *mut u8;

/// Timer3 COmpare Register B  Bytes.
pub const T3CORB: *mut u16 = 0x7A as *mut u16;

/// Timer3 COmpare Register B  Bytes low byte.
pub const T3CORBL: *mut u8 = 0x7A as *mut u8;

/// Timer3 COmpare Register B  Bytes high byte.
pub const T3CORBH: *mut u8 = 0x7B as *mut u8;

/// Timer 3 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CNC | 100000 |
/// | T3CS | 111 |
/// | T3ICS | 11000000 |
/// | T3CE | 11000 |
pub const T3MRA: *mut u8 = 0x7C as *mut u8;

/// Timer 3 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3TOP | 10000 |
/// | T3M | 111 |
pub const T3MRB: *mut u8 = 0x7D as *mut u8;

/// Timer 3 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3SAMB | 10000 |
/// | T3CTMB | 1000 |
/// | T3CTMA | 1 |
/// | T3CRMB | 100000 |
/// | T3CRMA | 100 |
/// | T3SAMA | 10 |
/// | T3CPRM | 1000000 |
pub const T3CRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3OIM | 1 |
/// | T3CAIM | 10 |
/// | T3CBIM | 100 |
/// | T3CPIM | 1000 |
pub const T3IMR: *mut u8 = 0x7F as *mut u8;

/// Low Frequency Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFWIM | 1 |
/// | LFBIM | 10 |
/// | LFEIM | 100 |
pub const LFIMR: *mut u8 = 0x81 as *mut u8;

/// Low Frequency Receiver Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFBM | 10 |
/// | LFRSS | 10000 |
/// | LFEN | 1 |
/// | LFCS | 11100000 |
/// | LFWM | 1100 |
pub const LFRCR: *mut u8 = 0x82 as *mut u8;

/// LF Header Compare Register.
pub const LFHCR: *mut u8 = 0x83 as *mut u8;

/// LF ID Compare Register  low byte.
pub const LFIDCL: *mut u8 = 0x84 as *mut u8;

/// LF ID Compare Register.
pub const LFIDC: *mut u16 = 0x84 as *mut u16;

/// LF ID Compare Register  high byte.
pub const LFIDCH: *mut u8 = 0x85 as *mut u8;

/// LF Calibration Register  Bytes.
pub const LFCAL: *mut u16 = 0x86 as *mut u16;

/// LF Calibration Register  Bytes low byte.
pub const LFCALL: *mut u8 = 0x86 as *mut u8;

/// LF Calibration Register  Bytes high byte.
pub const LFCALH: *mut u8 = 0x87 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLTPS: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CMCR`
pub const ECINS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMCCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMM: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMONEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CMCR`
pub const CCS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CMCR`
pub const SRCD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CMIMR`
pub const ECIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMSR`
pub const ECF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const EELOCK: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFCDR`
pub const LFRST: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LFCDR`
pub const LFDO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFCDR`
pub const LFSCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFBF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFWPF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFEDF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFFR`
pub const LFRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LFIMR`
pub const LFWIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFIMR`
pub const LFBIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFIMR`
pub const LFEIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LFRCR`
pub const LFBM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LFRCR`
pub const LFRSS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LFRCR`
pub const LFEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LFRCR`
pub const LFCS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `LFRCR`
pub const LFWM: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const BODEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const TSRDI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOW`
pub const FRCFS: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LOW`
pub const WDRCON: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const TSRF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SCCR`
pub const SCCS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `SCCR`
pub const SRCC: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SCR`
pub const SMS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SCR`
pub const SEN: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `SCR`
pub const SMEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SIMSK`
pub const MSIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSFR`
pub const MSENO: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSFR`
pub const MSENF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PBS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0IE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PAS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T10IFR`
pub const T0F: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T10IFR`
pub const T1F: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1PS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CS: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1IE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2ICS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2TS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2CR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2E: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2CRA`
pub const T2CRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T2CRB`
pub const T2SCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2RXF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2TCF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2TXF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2RXIM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2TCIM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2TXIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2MRA`
pub const T2CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T2MRA`
pub const T2CNC: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2MRA`
pub const T2TP: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T2MRA`
pub const T2CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T2MRB`
pub const T2CPOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T2MRB`
pub const T2TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T2MRB`
pub const T2SSIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T2MRB`
pub const T2M: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `T3CRA`
pub const T3SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3CRA`
pub const T3AC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3CRA`
pub const T3E: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T3CRA`
pub const T3CR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3CRA`
pub const T3TS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3SAMB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3CTMB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3CTMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3CRMB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3CRMA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3SAMA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3CRB`
pub const T3CPRM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3COBF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3COAF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3ICF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CAIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CBIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CPIM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3CNC: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3ICS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TSCR`
pub const TSSD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VMCSR`
pub const BODPD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMLS: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `VMCSR`
pub const BODLS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// `COMM_SCK_RATE_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate_3bit {
   /// fosc/4.
   pub const VAL_0x00: u32 = 0x0;
   /// fosc/16.
   pub const VAL_0x01: u32 = 0x1;
   /// fosc/64.
   pub const VAL_0x02: u32 = 0x2;
   /// fosc/128.
   pub const VAL_0x03: u32 = 0x3;
   /// fosc/2.
   pub const VAL_0x04: u32 = 0x4;
   /// fosc/8.
   pub const VAL_0x05: u32 = 0x5;
   /// fosc/32.
   pub const VAL_0x06: u32 = 0x6;
   /// fosc/64.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Sensor Noise Reduction.
   pub const SENSORNR: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
}

/// `ENUM_BLB` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb {
   /// LPM and SPM prohibited in Application Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Application Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Application Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_BLB2` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb2 {
   /// LPM and SPM prohibited in Boot Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Boot Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Boot Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Boot Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot size = 128 words.
   pub const VAL_0x03: u32 = 0x3;
   /// Boot size = 256 words.
   pub const VAL_0x02: u32 = 0x2;
   /// Boot size = 512 words.
   pub const VAL_0x01: u32 = 0x1;
   /// Boot size = 1024 words.
   pub const VAL_0x00: u32 = 0x0;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Further programming disabled.
   pub const VAL_0x02: u32 = 0x2;
   /// No memory lock features enabled.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_SUT_CKSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut_cksel {
   /// Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms;   \[SUT=00\].
   pub const VAL_0x00: u32 = 0x0;
   /// Start-up time PWRDWN/RESET: 6 CK/14 CK + 5.7 ms; \[SUT=01\].
   pub const VAL_0x01: u32 = 0x1;
   /// Start-up time PWRDWN/RESET: 6 CK/14 CK + 90 ms;  \[SUT=10\].
   pub const VAL_0x02: u32 = 0x2;
}

