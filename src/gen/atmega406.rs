//! The AVR ATmega406 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 4V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB1 | 110000 |
/// | LB | 11 |
/// | BLB0 | 1100 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BOOTSZ | 110000 |
/// | EESAVE | 1000000 |
/// | SUT_CKSEL | 111 |
/// | BOOTRST | 1000 |
/// | WDTON | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCDEN | 10 |
/// | JTAGEN | 1 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x20 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x21 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x22 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Input Pins, Port D.
pub const PIND: *mut u8 = 0x29 as *mut u8;

/// Data Direction Register, Port D.
pub const DDRD: *mut u8 = 0x2A as *mut u8;

/// Data Register, Port D.
pub const PORTD: *mut u8 = 0x2B as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0B | 100 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV1 | 1 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 11 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 1111 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 1111 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERIE | 1000 |
/// | EEPM | 110000 |
/// | EEPE | 10 |
/// | EEMPE | 100 |
/// | EERE | 1 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSRSYNC | 1 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM02 | 1000 |
/// | CS0 | 111 |
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer Counter 0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Output compare Register A.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Output compare Register B.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 1110 |
pub const SMCR: *mut u8 = 0x53 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | JTRF | 10000 |
/// | BODRF | 100 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTD | 10000000 |
/// | IVCE | 1 |
/// | PUD | 10000 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSB | 1000000 |
/// | SPMEN | 1 |
/// | PGERS | 10 |
/// | RWWSRE | 10000 |
/// | SIGRD | 100000 |
/// | SPMIE | 10000000 |
/// | PGWRT | 100 |
/// | BLBSET | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | I | 10000000 |
/// | H | 100000 |
/// | N | 100 |
/// | T | 1000000 |
/// | S | 10000 |
/// | V | 1000 |
/// | Z | 10 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDE | 1000 |
/// | WDIE | 1000000 |
/// | WDIF | 10000000 |
/// | WDCE | 10000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// Wake-up Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUTR | 10000 |
/// | WUTIF | 10000000 |
/// | WUTE | 1000 |
/// | WUTIE | 1000000 |
/// | WUTCF | 100000 |
/// | WUTP | 111 |
pub const WUTCSR: *mut u8 = 0x62 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRVADC | 1 |
/// | PRTWI | 1000 |
/// | PRTIM1 | 100 |
/// | PRTIM0 | 10 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Fast Oscillator Calibration Value.
pub const FOSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 11 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC2 | 110000 |
/// | ISC3 | 11000000 |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Enable Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Enable Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE0 | 1 |
/// | OCIE0B | 100 |
/// | OCIE0A | 10 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1A | 10 |
/// | TOIE1 | 1 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// VADC Data Register  Bytes low byte.
pub const VADCL: *mut u8 = 0x78 as *mut u8;

/// VADC Data Register  Bytes.
pub const VADC: *mut u16 = 0x78 as *mut u16;

/// VADC Data Register  Bytes high byte.
pub const VADCH: *mut u8 = 0x79 as *mut u8;

/// The VADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADSC | 100 |
/// | VADCCIF | 10 |
/// | VADCCIE | 1 |
/// | VADEN | 1000 |
pub const VADCSR: *mut u8 = 0x7A as *mut u8;

/// The VADC multiplexer Selection Register.
pub const VADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register.
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTC1 | 1000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer Counter 1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer Counter 1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer Counter 1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Output Compare Register 1A Low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Output Compare Register 1A High byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR: *mut u8 = 0xB8 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWS | 11111000 |
/// | TWPS | 11 |
pub const TWSR: *mut u8 = 0xB9 as *mut u8;

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWGCE | 1 |
/// | TWA | 11111110 |
pub const TWAR: *mut u8 = 0xBA as *mut u8;

/// TWI Data register.
pub const TWDR: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWWC | 1000 |
/// | TWSTO | 10000 |
/// | TWEA | 1000000 |
/// | TWIE | 1 |
/// | TWEN | 100 |
/// | TWSTA | 100000 |
/// | TWINT | 10000000 |
pub const TWCR: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM | 11111110 |
pub const TWAMR: *mut u8 = 0xBD as *mut u8;

/// TWI Bus Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWBCIE | 1000000 |
/// | TWBCIP | 1 |
/// | TWBCIF | 10000000 |
/// | TWBDT | 110 |
pub const TWBCSR: *mut u8 = 0xBE as *mut u8;

/// Clock Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACS | 1 |
/// | XOE | 10 |
pub const CCSR: *mut u8 = 0xC0 as *mut u8;

/// Bandgap Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGD | 10000000 |
/// | BGCC | 111111 |
pub const BGCCR: *mut u8 = 0xD0 as *mut u8;

/// Bandgap Calibration of Resistor Ladder.
pub const BGCRR: *mut u8 = 0xD1 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC0: *mut u8 = 0xE0 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC1: *mut u8 = 0xE1 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC2: *mut u8 = 0xE2 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC3: *mut u8 = 0xE3 as *mut u8;

/// CC-ADC Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADUB | 100000 |
/// | CADEN | 10000000 |
/// | CADSE | 1 |
/// | CADAS | 11000 |
/// | CADSI | 110 |
pub const CADCSRA: *mut u8 = 0xE4 as *mut u8;

/// CC-ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADRCIF | 10 |
/// | CADICIE | 10000 |
/// | CADACIF | 100 |
/// | CADACIE | 1000000 |
/// | CADICIF | 1 |
/// | CADRCIE | 100000 |
pub const CADCSRB: *mut u8 = 0xE5 as *mut u8;

/// CC-ADC Regular Charge Current.
pub const CADRCC: *mut u8 = 0xE6 as *mut u8;

/// CC-ADC Regular Discharge Current.
pub const CADRDC: *mut u8 = 0xE7 as *mut u8;

/// CC-ADC Instantaneous Current low byte.
pub const CADICL: *mut u8 = 0xE8 as *mut u8;

/// CC-ADC Instantaneous Current.
pub const CADIC: *mut u16 = 0xE8 as *mut u16;

/// CC-ADC Instantaneous Current high byte.
pub const CADICH: *mut u8 = 0xE9 as *mut u8;

/// `FCSR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PFD | 1 |
/// | CPS | 1000 |
/// | PWMOPC | 10000 |
/// | CFE | 10 |
/// | DFE | 100 |
/// | PWMOC | 100000 |
pub const FCSR: *mut u8 = 0xF0 as *mut u8;

/// Cell Balancing Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CBE | 1111 |
pub const CBCR: *mut u8 = 0xF1 as *mut u8;

/// Battery Protection Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCIE | 1 |
/// | DUVIF | 10000000 |
/// | DOCIE | 10 |
/// | COCIE | 100 |
/// | DUVIE | 1000 |
/// | SCIF | 10000 |
/// | COCIF | 1000000 |
/// | DOCIF | 100000 |
pub const BPIR: *mut u8 = 0xF2 as *mut u8;

/// Battery Protection Deep Under Voltage Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DUDL | 1111 |
/// | DUVT | 110000 |
pub const BPDUV: *mut u8 = 0xF3 as *mut u8;

/// Battery Protection Short-Circuit Detection Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCDL | 1111 |
pub const BPSCD: *mut u8 = 0xF4 as *mut u8;

/// Battery Protection OverCurrent Detection Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCDL | 11110000 |
/// | CCDL | 1111 |
pub const BPOCD: *mut u8 = 0xF5 as *mut u8;

/// Current Battery Protection Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCPT | 11110000 |
/// | OCPT | 1111 |
pub const CBPTR: *mut u8 = 0xF6 as *mut u8;

/// Battery Protection Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DUVD | 1000 |
/// | DCD | 10 |
/// | SCD | 100 |
/// | CCD | 1 |
pub const BPCR: *mut u8 = 0xF7 as *mut u8;

/// Battery Protection Parameter Lock Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BPPL | 1 |
/// | BPPLE | 10 |
pub const BPPLR: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `BGCCR`
pub const BGD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `BGCCR`
pub const BGCC: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `BPCR`
pub const DUVD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `BPCR`
pub const DCD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPCR`
pub const SCD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BPCR`
pub const CCD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPDUV`
pub const DUDL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `BPDUV`
pub const DUVT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `BPIR`
pub const SCIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPIR`
pub const DUVIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `BPIR`
pub const DOCIE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPIR`
pub const COCIE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BPIR`
pub const DUVIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `BPIR`
pub const SCIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BPIR`
pub const COCIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `BPIR`
pub const DOCIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `BPOCD`
pub const DCDL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `BPOCD`
pub const CCDL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `BPPLR`
pub const BPPL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPPLR`
pub const BPPLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPSCD`
pub const SCDL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADUB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADSE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADAS: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADSI: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADRCIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADICIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADACIF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADACIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADICIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADRCIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CBCR`
pub const CBE: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CBPTR`
pub const SCPT: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CBPTR`
pub const OCPT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CCSR`
pub const ACS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CCSR`
pub const XOE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FCSR`
pub const PFD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FCSR`
pub const CPS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FCSR`
pub const PWMOPC: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FCSR`
pub const CFE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FCSR`
pub const DFE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FCSR`
pub const PWMOC: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOW`
pub const BOOTSZ: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LOW`
pub const BOOTRST: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const JTD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BODRF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRVADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CTC1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWAMR`
pub const TWAM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWAR`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBCIP: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBCIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBDT: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADSC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADCCIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADCCIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTCF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTP: *mut u8 = 0x7 as *mut u8;

/// `ANALOG_CADA_ACC_TIME` value group
#[allow(non_upper_case_globals)]
pub mod analog_cada_acc_time {
   /// 125ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 250ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 500ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 1000ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `BAT_DEEP_UNDER_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod bat_deep_under_delay {
   /// 750ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 1000ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 1250ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 1500ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `BAT_DEEP_UNDER_LEVEL` value group
#[allow(non_upper_case_globals)]
pub mod bat_deep_under_level {
   /// 4.71V.
   pub const VAL_0x00: u32 = 0x0;
   /// 5.03V.
   pub const VAL_0x01: u32 = 0x1;
   /// 5.34V.
   pub const VAL_0x02: u32 = 0x2;
   /// 5.66V.
   pub const VAL_0x03: u32 = 0x3;
   /// 5.97V.
   pub const VAL_0x04: u32 = 0x4;
   /// 6.29V.
   pub const VAL_0x05: u32 = 0x5;
   /// 6.60V.
   pub const VAL_0x06: u32 = 0x6;
   /// 6.91V.
   pub const VAL_0x07: u32 = 0x7;
   /// 7.23V.
   pub const VAL_0x08: u32 = 0x8;
   /// 7.54V.
   pub const VAL_0x09: u32 = 0x9;
   /// 7.86V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 8.17V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 8.49V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 8.80V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 9.11V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 9.43V.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_OVER_CURRENT_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod bat_over_current_delay {
   /// 1 ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 2 ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 4 ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 6 ms.
   pub const VAL_0x03: u32 = 0x3;
   /// 8 ms.
   pub const VAL_0x04: u32 = 0x4;
   /// 10 ms.
   pub const VAL_0x05: u32 = 0x5;
   /// 12 ms.
   pub const VAL_0x06: u32 = 0x6;
   /// 14 ms.
   pub const VAL_0x07: u32 = 0x7;
   /// 16 ms.
   pub const VAL_0x08: u32 = 0x8;
   /// 18 ms.
   pub const VAL_0x09: u32 = 0x9;
   /// 20 ms.
   pub const VAL_0x0A: u32 = 0xA;
   /// 22 ms.
   pub const VAL_0x0B: u32 = 0xB;
   /// 24 ms.
   pub const VAL_0x0C: u32 = 0xC;
   /// 26 ms.
   pub const VAL_0x0D: u32 = 0xD;
   /// 28 ms.
   pub const VAL_0x0E: u32 = 0xE;
   /// 30 ms.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_SHORT_CIRC_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod bat_short_circ_delay {
   /// 61 us.
   pub const VAL_0x00: u32 = 0x0;
   /// 122 us.
   pub const VAL_0x01: u32 = 0x1;
   /// 183 us.
   pub const VAL_0x02: u32 = 0x2;
   /// 244 us.
   pub const VAL_0x03: u32 = 0x3;
   /// 305 us.
   pub const VAL_0x04: u32 = 0x4;
   /// 366 us.
   pub const VAL_0x05: u32 = 0x5;
   /// 427 us.
   pub const VAL_0x06: u32 = 0x6;
   /// 488 us.
   pub const VAL_0x07: u32 = 0x7;
   /// 610 us.
   pub const VAL_0x08: u32 = 0x8;
   /// 732 us.
   pub const VAL_0x09: u32 = 0x9;
   /// 854 us.
   pub const VAL_0x0A: u32 = 0xA;
   /// 976 us.
   pub const VAL_0x0B: u32 = 0xB;
   /// 1098 us.
   pub const VAL_0x0C: u32 = 0xC;
   /// 1220 us.
   pub const VAL_0x0D: u32 = 0xD;
   /// 1342 us.
   pub const VAL_0x0E: u32 = 0xE;
   /// 1464 us.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_VOLT_SENSE` value group
#[allow(non_upper_case_globals)]
pub mod bat_volt_sense {
   /// 0.050V.
   pub const VAL_0x00: u32 = 0x0;
   /// 0.055V.
   pub const VAL_0x01: u32 = 0x1;
   /// 0.060V.
   pub const VAL_0x02: u32 = 0x2;
   /// 0.065V.
   pub const VAL_0x03: u32 = 0x3;
   /// 0.070V.
   pub const VAL_0x04: u32 = 0x4;
   /// 0.080V.
   pub const VAL_0x05: u32 = 0x5;
   /// 0.090V.
   pub const VAL_0x06: u32 = 0x6;
   /// 0.100V.
   pub const VAL_0x07: u32 = 0x7;
   /// 0.110V.
   pub const VAL_0x08: u32 = 0x8;
   /// 0.120V.
   pub const VAL_0x09: u32 = 0x9;
   /// 0.130V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 0.140V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 0.160V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 0.180V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 0.200V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 0.220V.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `BAT_VOLT_SENSE2` value group
#[allow(non_upper_case_globals)]
pub mod bat_volt_sense2 {
   /// 0.100V.
   pub const VAL_0x00: u32 = 0x0;
   /// 0.110V.
   pub const VAL_0x01: u32 = 0x1;
   /// 0.120V.
   pub const VAL_0x02: u32 = 0x2;
   /// 0.130V.
   pub const VAL_0x03: u32 = 0x3;
   /// 0.140V.
   pub const VAL_0x04: u32 = 0x4;
   /// 0.160V.
   pub const VAL_0x05: u32 = 0x5;
   /// 0.180V.
   pub const VAL_0x06: u32 = 0x6;
   /// 0.200V.
   pub const VAL_0x07: u32 = 0x7;
   /// 0.220V.
   pub const VAL_0x08: u32 = 0x8;
   /// 0.240V.
   pub const VAL_0x09: u32 = 0x9;
   /// 0.260V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 0.280V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 0.320V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 0.360V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 0.400V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 0.440V.
   pub const VAL_0x0F: u32 = 0xF;
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

/// `CLK_SEL_3BIT_EXT` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit_ext {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, No Prescaling.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK/8.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK/64.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK/256.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, CLK/1024.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, ExtClk Tx Falling Edge.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, ExtClk Tx Rising Edge.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CLK_SEL_3BIT_ONLY_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit_only_prescale {
   /// 4K(Slow RC) / 1K (32kHz).
   pub const VAL_0x00: u32 = 0x0;
   /// 8K(Slow RC) / 2K (32kHz).
   pub const VAL_0x01: u32 = 0x1;
   /// 16K(Slow RC) / 4K (32kHz).
   pub const VAL_0x02: u32 = 0x2;
   /// 32K(Slow RC) / 8K (32kHz).
   pub const VAL_0x03: u32 = 0x3;
   /// 64K(Slow RC) / 16K (32kHz).
   pub const VAL_0x04: u32 = 0x4;
   /// 128K(Slow RC) / 32K (32kHz).
   pub const VAL_0x05: u32 = 0x5;
   /// 256K(Slow RC) / 64K (32kHz).
   pub const VAL_0x06: u32 = 0x6;
   /// 512K(Slow RC) / 128K (32kHz).
   pub const VAL_0x07: u32 = 0x7;
}

/// `COMM_TWI_PRESACLE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi_presacle {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 4.
   pub const VAL_0x01: u32 = 0x1;
   /// 16.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_TW_BUS_TIMEOUT` value group
#[allow(non_upper_case_globals)]
pub mod comm_tw_bus_timeout {
   /// 250ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 500ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 1000ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 2000ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_SLEEP_MODE_3BITS3` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits3 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Power Off.
   pub const POFF: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
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

/// `ENUM_BLB` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb {
   /// LPM and SPM prohibited in Application Section.
   pub const LPM_SPM_DISABLE: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const LPM_DISABLE: u32 = 0x1;
   /// SPM prohibited in Application Section.
   pub const SPM_DISABLE: u32 = 0x2;
   /// No lock on SPM and LPM in Application Section.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_BLB2` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb2 {
   /// LPM and SPM prohibited in Boot Section.
   pub const LPM_SPM_DISABLE: u32 = 0x0;
   /// LPM prohibited in Boot Section.
   pub const LPM_DISABLE: u32 = 0x1;
   /// SPM prohibited in Boot Section.
   pub const SPM_DISABLE: u32 = 0x2;
   /// No lock on SPM and LPM in Boot Section.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=256 words Boot address=$4F00.
   pub const _256W_4F00: u32 = 0x3;
   /// Boot Flash size=512 words Boot address=$4E00.
   pub const _512W_4E00: u32 = 0x2;
   /// Boot Flash size=1024 words Boot address=$4C00.
   pub const _1024W_4C00: u32 = 0x1;
   /// Boot Flash size=2048 words Boot address=$4800.
   pub const _2048W_4800: u32 = 0x0;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const PROG_VER_DISABLED: u32 = 0x0;
   /// Further programming disabled.
   pub const PROG_DISABLED: u32 = 0x2;
   /// No memory lock features enabled.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_SUT_CKSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut_cksel {
}

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any Logical Change of INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_4BITS` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_4bits {
   /// Oscillator Cycles 2K.
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 4K.
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 8K.
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 16K.
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 32K.
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 64K.
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 128K.
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 256K.
   pub const VAL_0x07: u32 = 0x7;
   /// Oscillator Cycles 512K.
   pub const VAL_0x08: u32 = 0x8;
   /// Oscillator Cycles 1024K.
   pub const VAL_0x09: u32 = 0x9;
}

