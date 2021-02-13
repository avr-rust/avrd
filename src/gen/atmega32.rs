//! The AVR ATmega32 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega32L-8AU | TQFPQFN44 | TQFP44 | -40°C - 85°C | 2.7V - 5.5V | 8 MHz |
//! | ATmega32L-8PU | PDIP40 | PDIP40 | -40°C - 85°C | 2.7V - 5.5V | 8 MHz |
//! | ATmega32L-8MU | TQFPQFN44 | QFN44 | -40°C - 85°C | 2.7V - 5.5V | 8 MHz |
//! | ATmega32-16AU | TQFPQFN44 | TQFP44 | -40°C - 85°C | 4.5V - 5.5V | 16 MHz |
//! | ATmega32-16PU | PDIP40 | PDIP40 | -40°C - 85°C | 4.5V - 5.5V | 16 MHz |
//! | ATmega32-16MU | TQFPQFN44 | QFN44 | -40°C - 85°C | 4.5V - 5.5V | 16 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODEN | 1000000 |
/// | SUT_CKSEL | 111111 |
/// | BODLEVEL | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB0 | 1100 |
/// | BLB1 | 110000 |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | BOOTSZ | 110 |
/// | JTAGEN | 1000000 |
/// | CKOPT | 10000 |
/// | BOOTRST | 1 |
/// | OCDEN | 10000000 |
/// | SPIEN | 100000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR: *mut u8 = 0x20 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWS | 11111000 |
/// | TWPS | 11 |
pub const TWSR: *mut u8 = 0x21 as *mut u8;

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWGCE | 1 |
/// | TWA | 11111110 |
pub const TWAR: *mut u8 = 0x22 as *mut u8;

/// TWI Data register.
pub const TWDR: *mut u8 = 0x23 as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x24 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x24 as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x25 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000000 |
/// | ADSC | 1000000 |
/// | ADIE | 1000 |
/// | ADPS | 111 |
/// | ADIF | 10000 |
/// | ADATE | 100000 |
pub const ADCSRA: *mut u8 = 0x26 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 100000 |
/// | MUX | 11111 |
/// | REFS | 11000000 |
pub const ADMUX: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACBG | 1000000 |
/// | ACIC | 100 |
/// | ACIE | 1000 |
/// | ACD | 10000000 |
/// | ACIS | 11 |
/// | ACO | 100000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USART Baud Rate Register Low Byte.
pub const UBRRL: *mut u8 = 0x29 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXB8 | 1 |
/// | TXEN | 1000 |
/// | UDRIE | 100000 |
/// | TXCIE | 1000000 |
/// | RXB8 | 10 |
/// | UCSZ2 | 100 |
/// | RXEN | 10000 |
/// | RXCIE | 10000000 |
pub const UCSRB: *mut u8 = 0x2A as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC | 10000000 |
/// | U2X | 10 |
/// | FE | 10000 |
/// | UDRE | 100000 |
/// | TXC | 1000000 |
/// | MPCM | 1 |
/// | DOR | 1000 |
/// | UPE | 100 |
pub const UCSRA: *mut u8 = 0x2B as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0x2C as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPHA | 100 |
/// | MSTR | 10000 |
/// | SPR | 11 |
/// | DORD | 100000 |
/// | SPE | 1000000 |
/// | CPOL | 1000 |
/// | SPIE | 10000000 |
pub const SPCR: *mut u8 = 0x2D as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
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
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | EEWE | 10 |
/// | EEMWE | 100 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access  Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Read/Write Access  Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Read/Write Access  Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// USART Baud Rate Register Hight Byte.
pub const UBRRH: *mut u8 = 0x40 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM | 110000 |
/// | UMSEL | 1000000 |
/// | UCPOL | 1 |
/// | URSEL | 10000000 |
/// | UCSZ | 110 |
/// | USBS | 1000 |
pub const UCSRC: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 111 |
/// | WDE | 1000 |
/// | WDTOE | 10000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// Asynchronous Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCR2UB | 1 |
/// | OCR2UB | 10 |
/// | AS2 | 1000 |
/// | TCN2UB | 100 |
pub const ASSR: *mut u8 = 0x42 as *mut u8;

/// Timer/Counter2 Output Compare Register.
pub const OCR2: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter2.
pub const TCNT2: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM21 | 1000 |
/// | WGM20 | 1000000 |
/// | CS2 | 111 |
/// | COM2 | 110000 |
/// | FOC2 | 10000000 |
pub const TCCR2: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x46 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1B: *mut u16 = 0x48 as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x49 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1A: *mut u16 = 0x4A as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x4A as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
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
/// | ICNC1 | 10000000 |
/// | ICES1 | 1000000 |
pub const TCCR1B: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | FOC1B | 100 |
/// | FOC1A | 1000 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x4F as *mut u8;

/// Special Function IO Register.
pub const SFIOR: *mut u8 = 0x50 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x51 as *mut u8;

/// On-Chip Debug Related Register in I/O Memory.
pub const OCDR: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter Register.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0 | 110000 |
/// | WGM00 | 1000000 |
/// | FOC0 | 10000000 |
/// | CS0 | 111 |
/// | WGM01 | 1000 |
pub const TCCR0: *mut u8 = 0x53 as *mut u8;

/// MCU Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTRF | 10000 |
/// | JTD | 10000000 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | BORF | 100 |
pub const MCUCSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 10000000 |
/// | SM | 1110000 |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWSTA | 100000 |
/// | TWEN | 100 |
/// | TWSTO | 10000 |
/// | TWIE | 1 |
/// | TWINT | 10000000 |
/// | TWEA | 1000000 |
/// | TWWC | 1000 |
pub const TWCR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPMEN | 1 |
/// | PGERS | 10 |
/// | RWWSRE | 10000 |
/// | RWWSB | 1000000 |
/// | BLBSET | 1000 |
/// | SPMIE | 10000000 |
/// | PGWRT | 100 |
pub const SPMCR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 10000 |
/// | ICF1 | 100000 |
/// | OCF1B | 1000 |
/// | TOV1 | 100 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TICIE1 | 100000 |
/// | OCIE1A | 10000 |
/// | OCIE1B | 1000 |
/// | TOIE1 | 100 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 11000000 |
/// | INTF2 | 100000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | INT2 | 100000 |
/// | INT1 | 10000000 |
/// | IVSEL | 10 |
/// | INT0 | 1000000 |
pub const GICR: *mut u8 = 0x5B as *mut u8;

/// Output Compare Register.
pub const OCR0: *mut u8 = 0x5C as *mut u8;

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
/// | V | 1000 |
/// | Z | 10 |
/// | H | 100000 |
/// | C | 1 |
/// | T | 1000000 |
/// | S | 10000 |
/// | N | 100 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2UB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2UB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN2UB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GICR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GICR`
pub const INT2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GICR`
pub const INT1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GICR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GICR`
pub const INT0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const CKOPT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const BODEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const BODLEVEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const JTD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0`
pub const COM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0`
pub const WGM00: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0`
pub const FOC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0`
pub const WGM01: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const FOC1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const FOC1A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2`
pub const WGM21: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2`
pub const WGM20: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR2`
pub const COM2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2`
pub const FOC2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TIFR`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWAR`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWAR`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `UCSRA`
pub const RXC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRA`
pub const U2X: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRA`
pub const FE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UDRE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRA`
pub const TXC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRA`
pub const MPCM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRA`
pub const DOR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXB8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UDRIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXB8: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UCSZ2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXCIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UMSEL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCPOL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRC`
pub const URSEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSRC`
pub const USBS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDTOE: *mut u8 = 0x10 as *mut u8;

/// `ANALOG_ADC_PRESCALER` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_prescaler {
   /// 2.
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

/// `ANALOG_ADC_V_REF2` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref2 {
   /// AREF, Internal Vref turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVCC with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 2.56V Voltage Reference with external capacitor at AREF pin.
   pub const VAL_0x03: u32 = 0x3;
}

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

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Standby.
   pub const STDBY: u32 = 0x6;
   /// Extended Standby.
   pub const ESTDBY: u32 = 0x7;
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
   /// Boot Flash size=256 words start address=$3F00.
   pub const _256W_3F00: u32 = 0x3;
   /// Boot Flash size=512 words start address=$3E00.
   pub const _512W_3E00: u32 = 0x2;
   /// Boot Flash size=1024 words start address=$3C00.
   pub const _1024W_3C00: u32 = 0x1;
   /// Boot Flash size=2048 words start address=$3800.
   pub const _2048W_3800: u32 = 0x0;
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
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms; default value.
   pub const INTRCOSC_1MHZ_6CK_64MS_DEFAULT: u32 = 0x21;
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
   /// 1.0 MHz.
   pub const _1_0_MHz: u32 = 0x0;
   /// 2.0 MHz.
   pub const _2_0_MHz: u32 = 0x1;
   /// 4.0 MHz.
   pub const _4_0_MHz: u32 = 0x2;
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x3;
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

