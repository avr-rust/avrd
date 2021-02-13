//! The AVR ATtiny25 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny25V-10PU | SOIC-8 | PDIP8 | -40°C - 85°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10SU | SOIC-8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10SH | SOIC-8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10SSU | SOIC-8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10SSH | SOIC-8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10MU | QFN-20 | MLF20 | -40°C - 85°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10SN | SOIC-8 | SOIC8 | -40°C - 105°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10SSN | SOIC-8 | SOIC8 | -40°C - 105°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-10MF | QFN-20 | MLF20 | -40°C - 125°C | 1.8V - 5.5V | 10 MHz |
//! | ATtiny25V-20PU | SOIC-8 | PDIP8 | -40°C - 85°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20SU | SOIC-8 | SOIC8 | -40°C - 85°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20SH | SOIC-8 | SOIC8 | -40°C - 85°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20SSU | SOIC-8 | SOIC8 | -40°C - 85°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20SSH | SOIC-8 | SOIC8 | -40°C - 85°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20MU | QFN-20 | MLF20 | -40°C - 85°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20SN | SOIC-8 | SOIC8 | -40°C - 105°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20SSN | SOIC-8 | SOIC8 | -40°C - 105°C | 2.7V - 5.5V | 20 MHz |
//! | ATtiny25V-20MF | QFN-20 | MLF20 | -40°C - 125°C | 2.7V - 5.5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SUT_CKSEL | 111111 |
/// | CKDIV8 | 10000000 |
/// | CKOUT | 1000000 |
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
/// | WDTON | 10000 |
/// | SPIEN | 100000 |
/// | RSTDISBL | 10000000 |
/// | DWEN | 1000000 |
/// | EESAVE | 1000 |
/// | BODLEVEL | 111 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SELFPRGEN | 1 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BIN | 10000000 |
/// | ADTS | 111 |
/// | IPR | 100000 |
pub const ADCSRB: *mut u8 = 0x23 as *mut u8;

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
/// | ADATE | 100000 |
/// | ADEN | 10000000 |
/// | ADIF | 10000 |
/// | ADPS | 111 |
/// | ADIE | 1000 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x26 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS2 | 10000 |
/// | MUX | 1111 |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIE | 1000 |
/// | ACD | 10000000 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
/// | ACI | 10000 |
/// | ACO | 100000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USICS | 1100 |
/// | USICLK | 10 |
/// | USIOIE | 1000000 |
/// | USIWM | 110000 |
/// | USISIE | 10000000 |
/// | USITC | 1 |
pub const USICR: *mut u8 = 0x2D as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USISIF | 10000000 |
/// | USICNT | 1111 |
/// | USIPF | 100000 |
/// | USIDC | 10000 |
/// | USIOIF | 1000000 |
pub const USISR: *mut u8 = 0x2E as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0x2F as *mut u8;

/// USI Buffer Register.
pub const USIBR: *mut u8 = 0x30 as *mut u8;

/// General purpose register 0.
pub const GPIOR0: *mut u8 = 0x31 as *mut u8;

/// General Purpose register 1.
pub const GPIOR1: *mut u8 = 0x32 as *mut u8;

/// General Purpose IO register 2.
pub const GPIOR2: *mut u8 = 0x33 as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC0D | 100000 |
/// | ADC2D | 10000 |
/// | ADC3D | 1000 |
/// | ADC1D | 100 |
pub const DIDR0: *mut u8 = 0x34 as *mut u8;

/// Pin Change Enable Mask.
pub const PCMSK: *mut u8 = 0x35 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x36 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x37 as *mut u8;

/// Data Register, Port B.
pub const PORTB: *mut u8 = 0x38 as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERIE | 1000 |
/// | EEMPE | 100 |
/// | EERE | 1 |
/// | EEPE | 10 |
/// | EEPM | 110000 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

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
pub const PRR: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
/// | WDIF | 10000000 |
/// | WDCE | 10000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// debugWire data register.
pub const DWDR: *mut u8 = 0x42 as *mut u8;

/// Dead time prescaler register.
pub const DTPS: *mut u8 = 0x43 as *mut u8;

/// Dead time value B.
pub const DT1B: *mut u8 = 0x44 as *mut u8;

/// Dead time value register.
pub const DT1A: *mut u8 = 0x45 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
/// | CLKPCE | 10000000 |
pub const CLKPR: *mut u8 = 0x46 as *mut u8;

/// PLL Control and status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LSM | 10000000 |
/// | PLLE | 10 |
/// | PLOCK | 1 |
/// | PCKE | 100 |
pub const PLLCSR: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x49 as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x4A as *mut u8;

/// Output Compare Register.
pub const OCR1B: *mut u8 = 0x4B as *mut u8;

/// Timer counter control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PWM1B | 1000000 |
/// | PSR1 | 10 |
/// | COM1B | 110000 |
/// | FOC1B | 1000 |
/// | FOC1A | 100 |
pub const GTCCR: *mut u8 = 0x4C as *mut u8;

/// Output compare register.
pub const OCR1C: *mut u8 = 0x4D as *mut u8;

/// Output Compare Register.
pub const OCR1A: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter Register.
pub const TCNT1: *mut u8 = 0x4F as *mut u8;

/// Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PWM1A | 1000000 |
/// | CTC1 | 10000000 |
/// | CS1 | 1111 |
/// | COM1A | 110000 |
pub const TCCR1: *mut u8 = 0x50 as *mut u8;

/// Oscillator Calibration Register.
pub const OSCCAL: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0B | 1000000 |
/// | CS0 | 111 |
/// | WGM02 | 1000 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x53 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | SM | 11000 |
/// | SE | 100000 |
/// | PUD | 1000000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | SPMEN | 1 |
/// | PGWRT | 100 |
/// | RSIG | 100000 |
/// | CTPB | 10000 |
/// | RFLB | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV1 | 100 |
/// | OCF1A | 1000000 |
/// | OCF1B | 100000 |
pub const TIFR: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 100 |
/// | OCIE1B | 100000 |
/// | OCIE1A | 1000000 |
pub const TIMSK: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1000000 |
/// | PCIF | 100000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1000000 |
/// | PCIE | 100000 |
pub const GIMSK: *mut u8 = 0x5B as *mut u8;

/// Stack Pointer Low Byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | Z | 10 |
/// | S | 10000 |
/// | C | 1 |
/// | T | 1000000 |
/// | I | 10000000 |
/// | H | 100000 |
/// | N | 100 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const BIN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const IPR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIFR`
pub const PCIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GIMSK`
pub const INT0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIMSK`
pub const PCIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PWM1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GTCCR`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `GTCCR`
pub const FOC1B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GTCCR`
pub const FOC1A: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const LSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PCKE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RSIG: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const CTPB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RFLB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1`
pub const PWM1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1`
pub const CTC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1`
pub const CS1: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `TCCR1`
pub const COM1A: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USICR`
pub const USICS: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `USICR`
pub const USICLK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `USICR`
pub const USIOIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USICR`
pub const USIWM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `USICR`
pub const USISIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USICR`
pub const USITC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USISR`
pub const USISIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USISR`
pub const USICNT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `USISR`
pub const USIPF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `USISR`
pub const USIDC: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `USISR`
pub const USIOIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// `ANALOG_ADC_AUTO_TRIGGER2` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger2 {
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
   /// Timer/Counter1 Capture Event.
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

/// `CLK_COMP_MATCH_OUT_MODE` value group
#[allow(non_upper_case_globals)]
pub mod clk_comp_match_out_mode {
   /// Disconnected from OCn/PWMn.
   pub const VAL_0x00: u32 = 0x0;
   /// Toggle OCn/PWMn output line.
   pub const VAL_0x01: u32 = 0x1;
   /// Clear OCn/PWMn output line.
   pub const VAL_0x02: u32 = 0x2;
   /// set OCn/PWMn output line.
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
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
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
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
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
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms.
   pub const EXTCLK_6CK_14CK_4MS1: u32 = 0x10;
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms.
   pub const EXTCLK_6CK_14CK_65MS: u32 = 0x20;
   /// PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4 ms.
   pub const PLLCLK_1KCK_14CK_4MS: u32 = 0x1;
   /// PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4 ms.
   pub const PLLCLK_16KCK_14CK_4MS: u32 = 0x11;
   /// PLL Clock; Start-up time PWRDWN/RESET: 1K CK/14 CK + 64 ms.
   pub const PLLCLK_1KCK_14CK_64MS: u32 = 0x21;
   /// PLL Clock; Start-up time PWRDWN/RESET: 16K CK/14 CK + 64 ms.
   pub const PLLCLK_16KCK_14CK_64MS: u32 = 0x31;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_4MS: u32 = 0x12;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_64MS: u32 = 0x22;
   /// ATtiny15 Comp: Int. RC Osc. 6.4 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const INTRCOSC_4MHZ_6CK_14CK_4MS: u32 = 0x23;
   /// ATtiny15 Comp: Int. RC Osc. 6.4 MHz; Start-up time PWRDWN/RESET: 1 CK/14 CK + 0 ms.
   pub const INTRCOSC_4MHZ_1CK_14CK_0MS: u32 = 0x33;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const WDOSC_128KHZ_6CK_14CK_0MS: u32 = 0x4;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4 ms.
   pub const WDOSC_128KHZ_6CK_14CK_4MS: u32 = 0x14;
   /// WD. Osc. 128 kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 64 ms.
   pub const WDOSC_128KHZ_6CK_14CK_64MS: u32 = 0x24;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const EXTLOFXTAL_1KCK_14CK_0MS: u32 = 0x6;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4 ms.
   pub const EXTLOFXTAL_1KCK_14CK_4MS: u32 = 0x16;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/14 CK + 64 ms.
   pub const EXTLOFXTAL_32KCK_14CK_64MS: u32 = 0x26;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_14CK_4MS1: u32 = 0x8;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_14CK_65MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_14CK_0MS: u32 = 0x28;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_14CK_4MS1: u32 = 0x38;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_14CK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_14CK_65MS: u32 = 0x39;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_14CK_4MS1: u32 = 0xA;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_14CK_65MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_14CK_0MS: u32 = 0x2A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_14CK_4MS1: u32 = 0x3A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_14CK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_14CK_65MS: u32 = 0x3B;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_14CK_4MS1: u32 = 0xC;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_14CK_65MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_14CK_0MS: u32 = 0x2C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_14CK_4MS1: u32 = 0x3C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_14CK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_14CK_65MS: u32 = 0x3D;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_14CK_4MS1: u32 = 0xE;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 258 CK/14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_14CK_65MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_14CK_0MS: u32 = 0x2E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_14CK_4MS1: u32 = 0x3E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time PWRDWN/RESET: 1K CK /14 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_14CK_65MS: u32 = 0xF;
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
   /// Any Logical Change on INTX.
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
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
   /// 6.4 MHz.
   pub const _6_4_MHz: u32 = 0x1;
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

