//! The AVR ATmega8515 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.7V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 10000000 |
/// | SUT_CKSEL | 111111 |
/// | BODEN | 1000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB0 | 1100 |
/// | LB | 11 |
/// | BLB1 | 110000 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | SPIEN | 100000 |
/// | S8515C | 10000000 |
/// | BOOTSZ | 110 |
/// | WDTON | 1000000 |
/// | BOOTRST | 1 |
/// | CKOPT | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x24 as *mut u8;

/// Port E Input Pins.
pub const PINE: *mut u8 = 0x25 as *mut u8;

/// Port E Data Direction Register.
pub const DDRE: *mut u8 = 0x26 as *mut u8;

/// Port E Data Register.
pub const PORTE: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACIC | 100 |
/// | ACI | 10000 |
/// | ACIE | 1000 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
/// | ACD | 10000000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USART Baud Rate Register Low Byte.
pub const UBRRL: *mut u8 = 0x29 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXCIE | 1000000 |
/// | TXB8 | 1 |
/// | UCSZ2 | 100 |
/// | RXCIE | 10000000 |
/// | RXB8 | 10 |
/// | TXEN | 1000 |
/// | RXEN | 10000 |
/// | UDRIE | 100000 |
pub const UCSRB: *mut u8 = 0x2A as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC | 10000000 |
/// | MPCM | 1 |
/// | UDRE | 100000 |
/// | U2X | 10 |
/// | TXC | 1000000 |
/// | UPE | 100 |
/// | DOR | 1000 |
/// | FE | 10000 |
pub const UCSRA: *mut u8 = 0x2B as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0x2C as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | SPR | 11 |
/// | CPHA | 100 |
/// | SPIE | 10000000 |
/// | DORD | 100000 |
/// | CPOL | 1000 |
pub const SPCR: *mut u8 = 0x2D as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
pub const SPSR: *mut u8 = 0x2E as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x2F as *mut u8;

/// Port D Input Pins.
pub const PIND: *mut u8 = 0x30 as *mut u8;

/// Port D Data Direction Register.
pub const DDRD: *mut u8 = 0x31 as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x32 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x33 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x34 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x35 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x36 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x37 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x38 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x39 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x3A as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x3B as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEMWE | 100 |
/// | EEWE | 10 |
/// | EERIE | 1000 |
/// | EERE | 1 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// USART Baud Rate Register High Byte.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UBRR | 11 |
/// | UBRR1 | 1100 |
pub const UBRRH: *mut u8 = 0x40 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCSZ | 110 |
/// | UPM | 110000 |
/// | UMSEL | 1000000 |
/// | USBS | 1000 |
/// | UCPOL | 1 |
pub const UCSRC: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDP | 111 |
/// | WDE | 1000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x44 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x48 as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x49 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x4A as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x4A as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x4B as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x4C as *mut u16;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x4C as *mut u8;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x4D as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS1 | 111 |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
pub const TCCR1B: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 1000 |
/// | FOC1B | 100 |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x4F as *mut u8;

/// Special Function IO Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR10 | 1 |
/// | XMBK | 1000000 |
/// | PUD | 100 |
/// | XMM | 111000 |
pub const SFIOR: *mut u8 = 0x50 as *mut u8;

/// Timer/Counter 0 Output Compare Register.
pub const OCR0: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter 0 Register.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0 | 10000000 |
/// | WGM00 | 1000000 |
/// | CS0 | 111 |
/// | WGM01 | 1000 |
/// | COM0 | 110000 |
pub const TCCR0: *mut u8 = 0x53 as *mut u8;

/// MCU Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | SM2 | 100000 |
/// | BORF | 100 |
pub const MCUCSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
/// | SRW10 | 1000000 |
/// | SRE | 10000000 |
/// | SM1 | 10000 |
/// | SE | 100000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Extended MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC2 | 1 |
/// | SRL | 1110000 |
/// | SRW11 | 10 |
/// | SM0 | 10000000 |
/// | SRW0 | 1100 |
pub const EMCUCR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSB | 1000000 |
/// | PGWRT | 100 |
/// | BLBSET | 1000 |
/// | RWWSRE | 10000 |
/// | SPMIE | 10000000 |
/// | SPMEN | 1 |
/// | PGERS | 10 |
pub const SPMCR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 1000000 |
/// | ICF1 | 1000 |
/// | TOV1 | 10000000 |
/// | OCF1B | 100000 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1B | 100000 |
/// | TICIE1 | 1000 |
/// | TOIE1 | 10000000 |
/// | OCIE1A | 1000000 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF2 | 100000 |
/// | INTF | 11000000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | IVSEL | 10 |
/// | INT0 | 1000000 |
/// | INT2 | 100000 |
/// | INT1 | 10000000 |
pub const GICR: *mut u8 = 0x5B as *mut u8;

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
/// | V | 1000 |
/// | H | 100000 |
/// | N | 100 |
/// | S | 10000 |
/// | Z | 10 |
/// | T | 1000000 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EMCUCR`
pub const ISC2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EMCUCR`
pub const SRL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `EMCUCR`
pub const SRW11: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EMCUCR`
pub const SM0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EMCUCR`
pub const SRW0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `GICR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GICR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GICR`
pub const INT0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GICR`
pub const INT2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GICR`
pub const INT1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const S8515C: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const CKOPT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOW`
pub const BODLEVEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const BODEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SRW10: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SRE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const SM2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SFIOR`
pub const PSR10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFIOR`
pub const XMBK: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SFIOR`
pub const PUD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SFIOR`
pub const XMM: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCR`
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
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0`
pub const FOC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0`
pub const WGM00: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0`
pub const WGM01: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0`
pub const COM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const FOC1A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const FOC1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR`
pub const ICF1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TICIE1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UBRRH`
pub const UBRR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `UBRRH`
pub const UBRR1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `UCSRA`
pub const RXC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRA`
pub const MPCM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UDRE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRA`
pub const U2X: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRA`
pub const TXC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRA`
pub const DOR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRA`
pub const FE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXB8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UCSZ2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXCIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXB8: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UDRIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UMSEL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRC`
pub const USBS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCPOL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// `ANALOG_COMP_INTERRUPT` value group
#[allow(non_upper_case_globals)]
pub mod analog_comp_interrupt {
   /// Interrupt on Toggle.
   pub const VAL_0x00: u32 = 0x0;
   /// Reserved.
   pub const VAL_0x01: u32 = 0x1;
   /// Interrupt on Falling Edge.
   pub const VAL_0x02: u32 = 0x2;
   /// Interrupt on Rising Edge.
   pub const VAL_0x03: u32 = 0x3;
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

/// `COMM_STOP_BIT_SEL` value group
#[allow(non_upper_case_globals)]
pub mod comm_stop_bit_sel {
   /// 1-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// 2-bit.
   pub const VAL_0x01: u32 = 0x1;
}

/// `COMM_UPM_PARITY_MODE` value group
#[allow(non_upper_case_globals)]
pub mod comm_upm_parity_mode {
   /// Disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Reserved.
   pub const VAL_0x01: u32 = 0x1;
   /// Enabled, Even Parity.
   pub const VAL_0x02: u32 = 0x2;
   /// Enabled, Odd Parity.
   pub const VAL_0x03: u32 = 0x3;
}

/// `COMM_USART_MODE` value group
#[allow(non_upper_case_globals)]
pub mod comm_usart_mode {
   /// Asynchronous Operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Synchronous Operation.
   pub const VAL_0x01: u32 = 0x1;
}

/// `CPU_SECTOR_LIMITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sector_limits {
   /// LS = N/A, US = 0x1100 - 0xFFFF.
   pub const VAL_0x00: u32 = 0x0;
   /// LS = 0x1100 - 0x1FFF, US = 0x2000 - 0xFFFF.
   pub const VAL_0x01: u32 = 0x1;
   /// LS = 0x1100 - 0x3FFF, US = 0x4000 - 0xFFFF.
   pub const VAL_0x02: u32 = 0x2;
   /// LS = 0x1100 - 0x5FFF, US = 0x6000 - 0xFFFF.
   pub const VAL_0x03: u32 = 0x3;
   /// LS = 0x1100 - 0x7FFF, US = 0x8000 - 0xFFFF.
   pub const VAL_0x04: u32 = 0x4;
   /// LS = 0x1100 - 0x9FFF, US = 0xA000 - 0xFFFF.
   pub const VAL_0x05: u32 = 0x5;
   /// LS = 0x1100 - 0xBFFF, US = 0xC000 - 0xFFFF.
   pub const VAL_0x06: u32 = 0x6;
   /// LS = 0x1100 - 0xDFFF, US = 0xE000 - 0xFFFF.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_WAIT_STATES` value group
#[allow(non_upper_case_globals)]
pub mod cpu_wait_states {
   /// No wait-states.
   pub const VAL_0x00: u32 = 0x0;
   /// Wait one cycle during read/write strobe.
   pub const VAL_0x01: u32 = 0x1;
   /// Wait two cycles during read/write strobe.
   pub const VAL_0x02: u32 = 0x2;
   /// Wait two cycles during read/write and wait one cycle before driving out new address.
   pub const VAL_0x03: u32 = 0x3;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=4.0 V.
   pub const _4V0: u32 = 0x0;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flashsize=128 words Boot address=$0F80.
   pub const _128W_0F80: u32 = 0x3;
   /// Boot Flash size=256 words Boot address=$0F00.
   pub const _256W_0F00: u32 = 0x2;
   /// Boot Flash size=512 words Boot address=$0E00.
   pub const _512W_0E00: u32 = 0x1;
   /// Boot Flash size=1024 words Boot address=$0C00.
   pub const _1024W_0C00: u32 = 0x0;
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
   /// Ext. Clock; Start-up time: 6 CK + 0 ms.
   pub const EXTCLK_6CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time: 6 CK + 4 ms.
   pub const EXTCLK_6CK_4MS: u32 = 0x10;
   /// Ext. Clock; Start-up time: 6 CK + 64 ms.
   pub const EXTCLK_6CK_64MS: u32 = 0x20;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_1MHZ_6CK_0MS: u32 = 0x1;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_1MHZ_6CK_4MS: u32 = 0x11;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_1MHZ_6CK_64MS: u32 = 0x21;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_2MHZ_6CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_2MHZ_6CK_4MS: u32 = 0x12;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_2MHZ_6CK_64MS: u32 = 0x22;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_4MHZ_6CK_0MS: u32 = 0x3;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_4MHZ_6CK_4MS: u32 = 0x13;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_4MHZ_6CK_64MS: u32 = 0x23;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_0MS: u32 = 0x4;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_4MS: u32 = 0x14;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_64MS: u32 = 0x24;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_0MS: u32 = 0x5;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_4MS: u32 = 0x15;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_64MS: u32 = 0x25;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_6CK_4MS: u32 = 0x35;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_0MS: u32 = 0x6;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_4MS: u32 = 0x16;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_64MS: u32 = 0x26;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_6CK_4MS: u32 = 0x36;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_0MS: u32 = 0x7;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_4MS: u32 = 0x17;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_64MS: u32 = 0x27;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_6CK_4MS: u32 = 0x37;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_0MS: u32 = 0x8;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_4MS: u32 = 0x18;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_64MS: u32 = 0x28;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_6CK_4MS: u32 = 0x38;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTAL_1KCK_4MS: u32 = 0x9;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTAL_1KCK_64MS: u32 = 0x19;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms.
   pub const EXTLOFXTAL_32KCK_64MS: u32 = 0x29;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTLOFXTALRES_258CK_4MS: u32 = 0xA;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTLOFXTALRES_258CK_64MS: u32 = 0x1A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTALRES_1KCK_0MS: u32 = 0x2A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTALRES_1KCK_4MS: u32 = 0x3A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTALRES_1KCK_64MS: u32 = 0xB;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTLOFXTALRES_16KCK_0MS: u32 = 0x1B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTLOFXTALRES_16KCK_4MS: u32 = 0x2B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTLOFXTALRES_16KCK_64MS: u32 = 0x3B;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTMEDFXTALRES_258CK_4MS: u32 = 0xC;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTMEDFXTALRES_258CK_64MS: u32 = 0x1C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTMEDFXTALRES_1KCK_0MS: u32 = 0x2C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTMEDFXTALRES_1KCK_4MS: u32 = 0x3C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTMEDFXTALRES_1KCK_64MS: u32 = 0xD;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTMEDFXTALRES_16KCK_0MS: u32 = 0x1D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTMEDFXTALRES_16KCK_4MS: u32 = 0x2D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTMEDFXTALRES_16KCK_64MS: u32 = 0x3D;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTHIFXTALRES_258CK_4MS: u32 = 0xE;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTHIFXTALRES_258CK_64MS: u32 = 0x1E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTHIFXTALRES_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTHIFXTALRES_1KCK_4MS: u32 = 0x3E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTHIFXTALRES_1KCK_64MS: u32 = 0xF;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTHIFXTALRES_16KCK_0MS: u32 = 0x1F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTHIFXTALRES_16KCK_4MS: u32 = 0x2F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTHIFXTALRES_16KCK_64MS: u32 = 0x3F;
}

/// `INTERRUPT_SENSE_CONTROL2` value group
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control2 {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any Logical Change in INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 1 Mhz.
   pub const _1_Mhz: u32 = 0x0;
   /// 2 Mhz.
   pub const _2_Mhz: u32 = 0x1;
   /// 4 Mhz.
   pub const _4_Mhz: u32 = 0x2;
   /// 8 Mhz.
   pub const _8_Mhz: u32 = 0x3;
}

/// `WAVEFORM_GEN_MODE` value group
#[allow(non_upper_case_globals)]
pub mod waveform_gen_mode {
   /// Normal.
   pub const VAL_0x00: u32 = 0x0;
   /// PWM, Phase Correct.
   pub const VAL_0x02: u32 = 0x2;
   /// CTC.
   pub const VAL_0x01: u32 = 0x1;
   /// Fast PWM.
   pub const VAL_0x03: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_3bits {
   /// Oscillator Cycles 16K.
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 32K.
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 64K.
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 128K.
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 256K.
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 512K.
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 1024K.
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 2048K.
   pub const VAL_0x07: u32 = 0x7;
}

