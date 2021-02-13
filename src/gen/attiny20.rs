//! The AVR ATtiny20 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny20-UUR | WLCSP | WLCSP12 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-SSU | DIP | SOIC14 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-XU | DIP | TSSOP14 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-CCU | UFBGA | UFBGA15 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny20-MMH | VQFN | VQFN20 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
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

/// `BYTE0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 1 |
/// | BODLEVEL | 1110000 |
/// | CKOUT | 100 |
/// | WDTON | 10 |
pub const BYTE0: *mut u8 = 0x0 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x0 as *mut u8;

/// Data Direction Register, Port A.
pub const DDRA: *mut u8 = 0x1 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x2 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEA: *mut u8 = 0x3 as *mut u8;

/// Port B Data register.
pub const PINB: *mut u8 = 0x4 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x5 as *mut u8;

/// Input Pins, Port B.
pub const PORTB: *mut u8 = 0x6 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEB: *mut u8 = 0x7 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMA | 1 |
pub const PORTCR: *mut u8 = 0x8 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x9 as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0xA as *mut u8;

/// General Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 110000 |
/// | INTF0 | 1 |
pub const GIFR: *mut u8 = 0xB as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
/// | PCIE | 110000 |
pub const GIMSK: *mut u8 = 0xC as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC2D | 100 |
/// | ADC4D | 10000 |
/// | ADC1D | 10 |
/// | ADC7D | 10000000 |
/// | ADC6D | 1000000 |
/// | ADC0D | 1 |
/// | ADC5D | 100000 |
/// | ADC3D | 1000 |
pub const DIDR0: *mut u8 = 0xD as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0xE as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0xE as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0xF as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 1111 |
/// | REFS | 1000000 |
pub const ADMUX: *mut u8 = 0x10 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 1000 |
/// | ADTS | 111 |
pub const ADCSRB: *mut u8 = 0x11 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADATE | 100000 |
/// | ADIF | 10000 |
/// | ADPS | 111 |
/// | ADIE | 1000 |
/// | ADEN | 10000000 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x12 as *mut u8;

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HLEV | 1000000 |
/// | ACME | 100 |
/// | HSEL | 10000000 |
pub const ACSRB: *mut u8 = 0x13 as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIS | 11 |
/// | ACD | 10000000 |
/// | ACIC | 100 |
/// | ACIE | 1000 |
/// | ACO | 100000 |
/// | ACBG | 1000000 |
/// | ACI | 10000 |
pub const ACSRA: *mut u8 = 0x14 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x15 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x16 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x17 as *mut u8;

/// Timer/Counter 0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0A | 10000000 |
/// | WGM02 | 1000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
pub const TCCR0B: *mut u8 = 0x18 as *mut u8;

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x19 as *mut u8;

/// Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x1A as *mut u16;

/// Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x1A as *mut u8;

/// Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x1B as *mut u8;

/// Timer/Counter1 Output Compare Register B  low byte.
pub const OCR1BL: *mut u8 = 0x1C as *mut u8;

/// Timer/Counter1 Output Compare Register B.
pub const OCR1B: *mut u16 = 0x1C as *mut u16;

/// Timer/Counter1 Output Compare Register B  high byte.
pub const OCR1BH: *mut u8 = 0x1D as *mut u8;

/// Timer/Counter 1 Output Compare Register A.
pub const OCR1A: *mut u16 = 0x1E as *mut u16;

/// Timer/Counter 1 Output Compare Register A  low byte.
pub const OCR1AL: *mut u8 = 0x1E as *mut u8;

/// Timer/Counter 1 Output Compare Register A  high byte.
pub const OCR1AH: *mut u8 = 0x1F as *mut u8;

/// Timer/Counter1.
pub const TCNT1: *mut u16 = 0x20 as *mut u16;

/// Timer/Counter1  low byte.
pub const TCNT1L: *mut u8 = 0x20 as *mut u8;

/// Timer/Counter1  high byte.
pub const TCNT1H: *mut u8 = 0x21 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x22 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS1 | 111 |
/// | ICNC1 | 10000000 |
/// | ICES1 | 1000000 |
pub const TCCR1B: *mut u8 = 0x23 as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x24 as *mut u8;

/// Overflow Interrupt Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 10000 |
/// | OCF0B | 100 |
/// | OCF1B | 100000 |
/// | ICF1 | 10000000 |
/// | TOV | 1001 |
/// | OCF0A | 10 |
pub const TIFR: *mut u8 = 0x25 as *mut u8;

/// Timer Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1B | 100000 |
/// | OCIE0B | 100 |
/// | TOIE | 1001 |
/// | OCIE0A | 10 |
/// | OCIE1A | 10000 |
/// | ICIE1 | 10000000 |
pub const TIMSK: *mut u8 = 0x26 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR | 1 |
pub const GTCCR: *mut u8 = 0x27 as *mut u8;

/// TWI Slave Data Register.
pub const TWSD: *mut u8 = 0x28 as *mut u8;

/// TWI Slave Address Mask Register.
pub const TWSAM: *mut u8 = 0x29 as *mut u8;

/// TWI Slave Address Register.
pub const TWSA: *mut u8 = 0x2A as *mut u8;

/// TWI Slave Status Register A.
pub const TWSSRA: *mut u8 = 0x2B as *mut u8;

/// TWI Slave Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWCMD | 11 |
/// | TWAA | 100 |
pub const TWSCRB: *mut u8 = 0x2C as *mut u8;

/// TWI Slave Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWSIE | 100 |
/// | TWASIE | 10000 |
/// | TWPME | 10 |
/// | TWDIE | 100000 |
/// | TWSHE | 10000000 |
/// | TWEN | 1000 |
/// | TWSME | 1 |
pub const TWSCRA: *mut u8 = 0x2D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x2E as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x2F as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | DORD | 100000 |
/// | CPHA | 100 |
/// | SPIE | 10000000 |
/// | SPR | 11 |
/// | CPOL | 1000 |
pub const SPCR: *mut u8 = 0x30 as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIF | 10000000 |
/// | WDIE | 1000000 |
/// | WDP | 100111 |
/// | WDE | 1000 |
pub const WDTCSR: *mut u8 = 0x31 as *mut u8;

/// Non-Volatile Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NVMBSY | 10000000 |
pub const NVMCSR: *mut u8 = 0x32 as *mut u8;

/// Non-Volatile Memory Command.
pub const NVMCMD: *mut u8 = 0x33 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSPI | 1000 |
/// | PRTIM1 | 100 |
/// | PRTWI | 10000 |
/// | PRTIM0 | 10 |
/// | PRADC | 1 |
pub const PRR: *mut u8 = 0x35 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
pub const CLKPSR: *mut u8 = 0x36 as *mut u8;

/// Clock Main Settings Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKMS | 11 |
pub const CLKMSR: *mut u8 = 0x37 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x39 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC01 | 10000000 |
/// | ISC00 | 1000000 |
pub const MCUCR: *mut u8 = 0x3A as *mut u8;

/// Reset Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
pub const RSTFLR: *mut u8 = 0x3B as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x3C as *mut u8;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x3D as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x3D as *mut u16;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x3E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T | 1000000 |
/// | V | 1000 |
/// | N | 100 |
/// | I | 10000000 |
/// | C | 1 |
/// | Z | 10 |
/// | H | 100000 |
/// | S | 10000 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSRB`
pub const HLEV: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSRB`
pub const ACME: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSRB`
pub const HSEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADLAR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `BYTE0`
pub const RSTDISBL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BYTE0`
pub const BODLEVEL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `BYTE0`
pub const CKOUT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BYTE0`
pub const WDTON: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CLKMSR`
pub const CLKMS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKPSR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GIFR`
pub const PCIF: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GIMSK`
pub const PCIE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC01: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC00: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `NVMCSR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PORTCR`
pub const BBMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRSPI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTWI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR`
pub const ICF1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV: *mut u8 = 0x9 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE: *mut u8 = 0x9 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TIMSK`
pub const ICIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWSIE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWASIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWPME: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWDIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWSHE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWSME: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWSCRB`
pub const TWCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWSCRB`
pub const TWAA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// `ADC_MUX` value group
#[allow(non_upper_case_globals)]
pub mod adc_mux {
   /// ADC0.
   pub const ADC0: u32 = 0x0;
   /// ADC1.
   pub const ADC1: u32 = 0x1;
   /// ADC2.
   pub const ADC2: u32 = 0x2;
   /// ADC3.
   pub const ADC3: u32 = 0x3;
   /// ADC4.
   pub const ADC4: u32 = 0x4;
   /// ADC5.
   pub const ADC5: u32 = 0x5;
   /// ADC6.
   pub const ADC6: u32 = 0x6;
   /// ADC7.
   pub const ADC7: u32 = 0x7;
   /// 0V (AGND).
   pub const ADC_GND: u32 = 0x8;
   /// 1.1V Internal Reference.
   pub const ADC_VBG: u32 = 0x9;
   /// Temperature sensor.
   pub const TEMPSENS: u32 = 0xA;
}

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

/// `ANALOG_ADC_V_REF` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref {
   /// Vcc used as internal reference.
   pub const VAL_0x00: u32 = 0x0;
   /// Internal 1.1V Referemce.
   pub const VAL_0x01: u32 = 0x1;
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

/// `COMM_SCK_RATE` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate {
   /// fcl/4.
   pub const VAL_0x00: u32 = 0x0;
   /// fcl/16.
   pub const VAL_0x01: u32 = 0x1;
   /// fcl/64.
   pub const VAL_0x02: u32 = 0x2;
   /// fcl/128.
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

/// Sleep Mode
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC noise reduction.
   pub const ADC: u32 = 0x1;
   /// Power-down.
   pub const PDOWN: u32 = 0x2;
   /// Standby.
   pub const STDBY: u32 = 0x4;
}

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection disabled; \[BODLEVEL=111\].
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

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any logical change on INTX.
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

