//! The AVR ATtiny261A microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 1.8V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKDIV8 | 10000000 |
/// | CKOUT | 1000000 |
/// | SUT_CKSEL | 111111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | DWEN | 1000000 |
/// | BODLEVEL | 111 |
/// | SPIEN | 100000 |
/// | RSTDISBL | 10000000 |
/// | WDTON | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SELFPRGEN | 1 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Timer/Counter1 Control Register E.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OC1OE | 111111 |
pub const TCCR1E: *mut u8 = 0x20 as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AREFD | 1000 |
/// | ADC1D | 10 |
/// | ADC0D | 1 |
/// | ADC5D | 1000000 |
/// | ADC3D | 10000 |
/// | ADC2D | 100 |
/// | ADC4D | 100000 |
/// | ADC6D | 10000000 |
pub const DIDR0: *mut u8 = 0x21 as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC10D | 10000000 |
/// | ADC9D | 1000000 |
/// | ADC8D | 100000 |
/// | ADC7D | 10000 |
pub const DIDR1: *mut u8 = 0x22 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GSEL | 1000000 |
/// | BIN | 10000000 |
/// | REFS2 | 10000 |
/// | MUX5 | 1000 |
/// | IPR | 100000 |
/// | ADTS | 111 |
pub const ADCSRB: *mut u8 = 0x23 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x24 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x24 as *mut u8;

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
/// | ADATE | 100000 |
/// | ADPS | 111 |
/// | ADIF | 10000 |
/// | ADIE | 1000 |
pub const ADCSRA: *mut u8 = 0x26 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 100000 |
/// | REFS | 11000000 |
/// | MUX | 11111 |
pub const ADMUX: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIS | 11 |
/// | ACD | 10000000 |
/// | ACIE | 1000 |
/// | ACME | 100 |
/// | ACO | 100000 |
/// | ACI | 10000 |
/// | ACBG | 1000000 |
pub const ACSRA: *mut u8 = 0x28 as *mut u8;

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACM | 111 |
/// | HLEV | 1000000 |
/// | HSEL | 10000000 |
pub const ACSRB: *mut u8 = 0x29 as *mut u8;

/// General purpose register 0.
pub const GPIOR0: *mut u8 = 0x2A as *mut u8;

/// General Purpose register 1.
pub const GPIOR1: *mut u8 = 0x2B as *mut u8;

/// General Purpose IO register 2.
pub const GPIOR2: *mut u8 = 0x2C as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USICLK | 10 |
/// | USISIE | 10000000 |
/// | USITC | 1 |
/// | USIOIE | 1000000 |
/// | USIWM | 110000 |
/// | USICS | 1100 |
pub const USICR: *mut u8 = 0x2D as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USIDC | 10000 |
/// | USICNT | 1111 |
/// | USIOIF | 1000000 |
/// | USIPF | 100000 |
/// | USISIF | 10000000 |
pub const USISR: *mut u8 = 0x2E as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0x2F as *mut u8;

/// USI Buffer Register.
pub const USIBR: *mut u8 = 0x30 as *mut u8;

/// USI Pin Position.
pub const USIPP: *mut u8 = 0x31 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x32 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x33 as *mut u8;

/// Timer/Counter0 High.
pub const TCNT0H: *mut u8 = 0x34 as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC0 | 100000 |
/// | ACIC0 | 1000 |
/// | ICES0 | 10000 |
/// | TCW0 | 10000000 |
/// | WGM00 | 1 |
/// | ICEN0 | 1000000 |
pub const TCCR0A: *mut u8 = 0x35 as *mut u8;

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
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | EEMPE | 100 |
/// | EERIE | 1000 |
/// | EEPE | 10 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// debugWire data register.
pub const DWDR: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDIF | 10000000 |
/// | WDCE | 10000 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// Pin Change Enable Mask 1.
pub const PCMSK1: *mut u8 = 0x42 as *mut u8;

/// Pin Change Enable Mask 0.
pub const PCMSK0: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter 1 Dead Time Value.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DT1L | 1111 |
/// | DT1H | 11110000 |
pub const DT1: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter 1 Register High.
pub const TC1H: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter Control Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FPES1 | 10000 |
/// | FPIE1 | 10000000 |
/// | FPF1 | 100 |
/// | WGM1 | 11 |
/// | FPAC1 | 1000 |
/// | FPEN1 | 1000000 |
/// | FPNC1 | 100000 |
pub const TCCR1D: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1A0S | 1000000 |
/// | PWM1D | 1 |
/// | COM1B0S | 10000 |
/// | COM1D | 1100 |
/// | COM1B1S | 100000 |
/// | FOC1D | 10 |
/// | COM1A1S | 10000000 |
pub const TCCR1C: *mut u8 = 0x47 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x48 as *mut u8;

/// PLL Control and status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLOCK | 1 |
/// | LSM | 10000000 |
/// | PCKE | 100 |
/// | PLLE | 10 |
pub const PLLCSR: *mut u8 = 0x49 as *mut u8;

/// Output compare register.
pub const OCR1D: *mut u8 = 0x4A as *mut u8;

/// Output compare register.
pub const OCR1C: *mut u8 = 0x4B as *mut u8;

/// Output Compare Register.
pub const OCR1B: *mut u8 = 0x4C as *mut u8;

/// Output Compare Register.
pub const OCR1A: *mut u8 = 0x4D as *mut u8;

/// Timer/Counter Register.
pub const TCNT1: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PWM1X | 10000000 |
/// | DTPS1 | 110000 |
/// | CS1 | 1111 |
/// | PSR1 | 1000000 |
pub const TCCR1B: *mut u8 = 0x4F as *mut u8;

/// Timer/Counter Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | PWM1A | 10 |
/// | PWM1B | 1 |
/// | FOC1A | 1000 |
/// | COM1A | 11000000 |
/// | FOC1B | 100 |
pub const TCCR1A: *mut u8 = 0x50 as *mut u8;

/// Oscillator Calibration Register.
pub const OSCCAL: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter0 Low.
pub const TCNT0L: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000 |
/// | PSR0 | 1000 |
/// | CS0 | 111 |
pub const TCCR0B: *mut u8 = 0x53 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 11000 |
/// | BODS | 10000000 |
/// | BODSE | 100 |
/// | ISC0 | 11 |
/// | SE | 100000 |
/// | PUD | 1000000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 100 |
/// | PRADC | 1 |
/// | PRTIM1 | 1000 |
/// | PRUSI | 10 |
pub const PRR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RFLB | 1000 |
/// | CTPB | 10000 |
/// | PGERS | 10 |
/// | SPMEN | 1 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 1000000 |
/// | OCF1B | 100000 |
/// | TOV1 | 100 |
/// | OCF1D | 10000000 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1B | 100000 |
/// | TOIE1 | 100 |
/// | OCIE1A | 1000000 |
/// | OCIE1D | 10000000 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 100000 |
/// | INTF | 11000000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 11000000 |
/// | PCIE | 110000 |
pub const GIMSK: *mut u8 = 0x5B as *mut u8;

/// Stack Pointer Low Byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | N | 100 |
/// | S | 10000 |
/// | C | 1 |
/// | T | 1000000 |
/// | H | 100000 |
/// | I | 10000000 |
/// | Z | 10 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACME: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSRB`
pub const ACM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ACSRB`
pub const HLEV: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSRB`
pub const HSEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const GSEL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const BIN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const REFS2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const MUX5: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const IPR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const AREFD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC10D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC9D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC8D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC7D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DT1`
pub const DT1L: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DT1`
pub const DT1H: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GIFR`
pub const PCIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `GIMSK`
pub const INT: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `GIMSK`
pub const PCIE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `MCUCR`
pub const BODS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const BODSE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const LSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PCKE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RFLB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const CTPB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICNC0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ACIC0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICES0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const TCW0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICEN0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const TSM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const PSR0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const PWM1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const PWM1B: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const FOC1A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const FOC1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const PWM1X: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const DTPS1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `TCCR1B`
pub const PSR1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const COM1A0S: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const PWM1D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const COM1B0S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const COM1D: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR1C`
pub const COM1B1S: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const COM1A1S: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const FPES1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const FPIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const FPF1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const WGM1: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const FPAC1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const FPEN1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1D`
pub const FPNC1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR1E`
pub const OC1OE: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USICR`
pub const USICLK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `USICR`
pub const USISIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USICR`
pub const USITC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USICR`
pub const USIOIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USICR`
pub const USIWM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `USICR`
pub const USICS: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `USISR`
pub const USIDC: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `USISR`
pub const USICNT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `USISR`
pub const USIOIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USISR`
pub const USIPF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `USISR`
pub const USISIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// `ANALOG_ADC_AUTO_TRIGGER3` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger3 {
   /// Free Running mode.
   pub const VAL_0x00: u32 = 0x0;
   /// Analog Comparator.
   pub const VAL_0x01: u32 = 0x1;
   /// External Interrupt Request 0.
   pub const VAL_0x02: u32 = 0x2;
   /// Timer/Counter0 Compare Match A.
   pub const VAL_0x03: u32 = 0x3;
   /// Timer/Counter0 Overflow.
   pub const VAL_0x04: u32 = 0x4;
   /// Timer/Counter1 Compare Match B.
   pub const VAL_0x05: u32 = 0x5;
   /// Timer/Counter1 Overflow.
   pub const VAL_0x06: u32 = 0x6;
   /// Watchdog Interrupt Request.
   pub const VAL_0x07: u32 = 0x7;
}

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

/// `CLK_SEL_4BIT` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_4bit {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, No Prescaling.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK/2.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK/4.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK/8.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, CLK/16.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, CLK/32.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, CLK/64.
   pub const VAL_0x07: u32 = 0x7;
   /// Running, CLK/128.
   pub const VAL_0x08: u32 = 0x8;
   /// Running, CLK/256.
   pub const VAL_0x09: u32 = 0x9;
   /// Running, CLK/512.
   pub const VAL_0x0A: u32 = 0xA;
   /// Running, CLK/1024.
   pub const VAL_0x0B: u32 = 0xB;
   /// Running, CLK/2048.
   pub const VAL_0x0C: u32 = 0xC;
   /// Running, CLK/4096.
   pub const VAL_0x0D: u32 = 0xD;
   /// Running, CLK/8192.
   pub const VAL_0x0E: u32 = 0xE;
   /// Running, CLK/16384.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `COMM_USI_OP` value group
#[allow(non_upper_case_globals)]
pub mod comm_usi_op {
   /// Normal Operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Three-Wire Mode.
   pub const VAL_0x01: u32 = 0x1;
   /// Two-Wire Mode.
   pub const VAL_0x02: u32 = 0x2;
   /// Two-Wire Mode Held Low.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_CLK_PRESCALE_4_BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_4_bits_small {
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
   /// 256.
   pub const VAL_0x08: u32 = 0x8;
}

/// `CPU_SLEEP_MODE2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode2 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Standby.
   pub const STDBY: u32 = 0x3;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=2.3 V.
   pub const _2V3: u32 = 0x3;
   /// Brown-out detection at VCC=2.2 V.
   pub const _2V2: u32 = 0x2;
   /// Brown-out detection at VCC=1.9 V.
   pub const _1V9: u32 = 0x1;
   /// Brown-out detection at VCC=2.0 V.
   pub const _2V0: u32 = 0x0;
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
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
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const EXTCLK_6CK_14CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const EXTCLK_6CK_14CK_4MS: u32 = 0x10;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const EXTCLK_6CK_14CK_64MS: u32 = 0x20;
   /// PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 8 ms.
   pub const PLLCLK_1KCK_14CK_8MS: u32 = 0x1;
   /// PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 8 ms.
   pub const PLLCLK_16KCK_14CK_8MS: u32 = 0x11;
   /// PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 68 ms.
   pub const PLLCLK_1KCK_14CK_68MS: u32 = 0x21;
   /// PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 68 ms.
   pub const PLLCLK_16KCK_14CK_68MS: u32 = 0x31;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_4MS: u32 = 0x12;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_64MS: u32 = 0x22;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const WDOSC_128KHZ_6CK_14CK_0MS: u32 = 0x3;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const WDOSC_128KHZ_6CK_14CK_4MS: u32 = 0x13;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const WDOSC_128KHZ_6CK_14CK_64MS: u32 = 0x23;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK 4 ms.
   pub const EXTLOFXTAL_1CK_4MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1 CK + 64 ms.
   pub const EXTLOFXTAL_1CK_64MS: u32 = 0x14;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32 CK + 64 ms.
   pub const EXTLOFXTAL_32CK_64MS: u32 = 0x24;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_258CK_14CK_4MS1: u32 = 0x8;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_258CK_14CK_65MS: u32 = 0x18;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_0MS: u32 = 0x28;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_4MS1: u32 = 0x38;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_14CK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS: u32 = 0x39;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ9_3MHZ_258CK_14CK_4MS1: u32 = 0xA;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_0MHZ9_3MHZ_258CK_14CK_65MS: u32 = 0x1A;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_14CK_0MS: u32 = 0x2A;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_14CK_4MS1: u32 = 0x3A;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_14CK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS: u32 = 0x3B;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_3MHZ_8MHZ_258CK_14CK_4MS1: u32 = 0xC;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_3MHZ_8MHZ_258CK_14CK_65MS: u32 = 0x1C;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_14CK_0MS: u32 = 0x2C;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_14CK_4MS1: u32 = 0x3C;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_14CK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS: u32 = 0x3D;
   /// Ext. Ceramic Res. 8.0-    MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTCRES_8MHZ_XX_258CK_14CK_4MS1: u32 = 0xE;
   /// Ext. Ceramic Res. 8.0-    MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTCRES_8MHZ_XX_258CK_14CK_65MS: u32 = 0x1E;
   /// Ext. Ceramic Res. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTCRES_8MHZ_XX_1KCK_14CK_0MS: u32 = 0x2E;
   /// Ext. Ceramic Res. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTCRES_8MHZ_XX_1KCK_14CK_4MS1: u32 = 0x3E;
   /// Ext. Ceramic Res. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTCRES_8MHZ_XX_1KCK_14CK_65MS: u32 = 0xF;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_14CK_0MS: u32 = 0x1F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_14CK_4MS1: u32 = 0x2F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_14CK_65MS: u32 = 0x3F;
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

/// `MISC_2BIT_SCALE` value group
#[allow(non_upper_case_globals)]
pub mod misc_2bit_scale {
   /// 1x.
   pub const VAL_0x00: u32 = 0x0;
   /// 2x.
   pub const VAL_0x01: u32 = 0x1;
   /// 4x.
   pub const VAL_0x02: u32 = 0x2;
   /// 8x.
   pub const VAL_0x03: u32 = 0x3;
}

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
}

/// `PULSE_WIDTH_MODU2` value group
#[allow(non_upper_case_globals)]
pub mod pulse_width_modu2 {
   /// Fast PWM (NB! PWMx must be set!).
   pub const VAL_0x00: u32 = 0x0;
   /// Phase and Frequency Correct PWM.
   pub const VAL_0x01: u32 = 0x1;
   /// PWM6/Single-slope.
   pub const VAL_0x02: u32 = 0x2;
   /// PWM6/Dual-slope.
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

