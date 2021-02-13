//! The AVR ATtiny104 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny104-SSNR | SOIC-14 | SOIC14 | -40°C - 105°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny104F-SSNR | SOIC-14 | SOIC14 | -40°C - 105°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny104-SSFR | SOIC-14 | SOIC14 | -40°C - 125°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny104F-SSFR | SOIC-14 | SOIC14 | -40°C - 125°C | 1.8V - 5.5V | 12 MHz |
//!

#![allow(non_upper_case_globals)]

/// Input Pins, Port A.
pub const PINA: *mut u8 = 0x0 as *mut u8;

/// `BYTE0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDTON | 10 |
/// | SELFPROGEN | 1000 |
/// | CKOUT | 100 |
/// | RSTDISBL | 1 |
pub const BYTE0: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// Data Direction Register, Port A.
pub const DDRA: *mut u8 = 0x1 as *mut u8;

/// Port A Data register.
pub const PORTA: *mut u8 = 0x2 as *mut u8;

/// Pull-up Enable Control Register for PORTA.
pub const PUEA: *mut u8 = 0x3 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x4 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x5 as *mut u8;

/// Port B Data register.
pub const PORTB: *mut u8 = 0x6 as *mut u8;

/// Pull-up Enable Control Register for PORTB.
pub const PUEB: *mut u8 = 0x7 as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0x8 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR: *mut u16 = 0x9 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRRL: *mut u8 = 0x9 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRRH: *mut u8 = 0xA as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXSIE | 10000000 |
/// | RXS | 1000000 |
/// | SFDE | 100000 |
pub const UCSRD: *mut u8 = 0xB as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM | 110000 |
/// | USBS | 1000 |
/// | UCSZ | 110 |
/// | UMSEL | 11000000 |
/// | UCPOL | 1 |
pub const UCSRC: *mut u8 = 0xC as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB8 | 10 |
/// | UCSZ2 | 100 |
/// | RXCIE | 10000000 |
/// | UDRIE | 100000 |
/// | TXCIE | 1000000 |
/// | RXEN | 10000 |
/// | TXB8 | 1 |
/// | TXEN | 1000 |
pub const UCSRB: *mut u8 = 0xD as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DOR | 1000 |
/// | UPE | 100 |
/// | UDRE | 100000 |
/// | U2X | 10 |
/// | MPCM | 1 |
/// | FE | 10000 |
/// | RXC | 10000000 |
/// | TXC | 1000000 |
pub const UCSRA: *mut u8 = 0xE as *mut u8;

/// Pin Change Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT4 | 10000 |
/// | PCINT0 | 1 |
/// | PCINT5 | 100000 |
/// | PCINT2 | 100 |
/// | PCINT7 | 10000000 |
/// | PCINT6 | 1000000 |
/// | PCINT3 | 1000 |
/// | PCINT1 | 10 |
pub const PCMSK0: *mut u8 = 0xF as *mut u8;

/// Pin Change Mask Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT11 | 1000 |
/// | PCINT9 | 10 |
/// | PCINT10 | 100 |
/// | PCINT8 | 1 |
pub const PCMSK1: *mut u8 = 0x10 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
/// | PCIF1 | 10 |
pub const PCIFR: *mut u8 = 0x11 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE1 | 10 |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x12 as *mut u8;

/// External Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
pub const EIMSK: *mut u8 = 0x13 as *mut u8;

/// External Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x14 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC01 | 10 |
/// | ISC00 | 1 |
pub const EICRA: *mut u8 = 0x15 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMA | 1 |
pub const PORTCR: *mut u8 = 0x16 as *mut u8;

/// `DIDR0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN1D | 10 |
/// | AIN0D | 1 |
pub const DIDR0: *mut u8 = 0x17 as *mut u8;

/// ADC Data Register Low.
pub const ADCL: *mut u8 = 0x19 as *mut u8;

/// ADC Data Register High.
pub const ADCH: *mut u8 = 0x1A as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | MUX | 111 |
pub const ADMUX: *mut u8 = 0x1B as *mut u8;

/// The ADC Control and Status register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 10000000 |
/// | ADTS | 111 |
pub const ADCSRB: *mut u8 = 0x1C as *mut u8;

/// The ADC Control and Status register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000000 |
/// | ADIF | 10000 |
/// | ADPS | 111 |
/// | ADATE | 100000 |
/// | ADSC | 1000000 |
/// | ADIE | 1000 |
pub const ADCSRA: *mut u8 = 0x1D as *mut u8;

/// Analog Comparator Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACPMUX | 1 |
/// | ACOE | 10 |
pub const ACSRB: *mut u8 = 0x1E as *mut u8;

/// Analog Comparator Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACBG | 1000000 |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACIC | 100 |
/// | ACIS | 11 |
/// | ACIE | 1000 |
/// | ACO | 100000 |
pub const ACSRA: *mut u8 = 0x1F as *mut u8;

/// Input Capture Register  Bytes low byte.
pub const ICR0L: *mut u8 = 0x22 as *mut u8;

/// Input Capture Register  Bytes.
pub const ICR0: *mut u16 = 0x22 as *mut u16;

/// Input Capture Register  Bytes high byte.
pub const ICR0H: *mut u8 = 0x23 as *mut u8;

/// Timer/Counter0 Output Compare Register B.
pub const OCR0B: *mut u16 = 0x24 as *mut u16;

/// Timer/Counter0 Output Compare Register B  low byte.
pub const OCR0BL: *mut u8 = 0x24 as *mut u8;

/// Timer/Counter0 Output Compare Register B  high byte.
pub const OCR0BH: *mut u8 = 0x25 as *mut u8;

/// Timer/Counter 0 Output Compare Register A  low byte.
pub const OCR0AL: *mut u8 = 0x26 as *mut u8;

/// Timer/Counter 0 Output Compare Register A.
pub const OCR0A: *mut u16 = 0x26 as *mut u16;

/// Timer/Counter 0 Output Compare Register A  high byte.
pub const OCR0AH: *mut u8 = 0x27 as *mut u8;

/// Timer/Counter0  low byte.
pub const TCNT0L: *mut u8 = 0x28 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u16 = 0x28 as *mut u16;

/// Timer/Counter0  high byte.
pub const TCNT0H: *mut u8 = 0x29 as *mut u8;

/// Overflow Interrupt Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0A | 10 |
/// | ICF0 | 100000 |
/// | OCF0B | 100 |
pub const TIFR0: *mut u8 = 0x2A as *mut u8;

/// Timer Interrupt Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0B | 100 |
/// | OCIE0A | 10 |
/// | TOIE0 | 1 |
/// | ICIE0 | 100000 |
pub const TIMSK0: *mut u8 = 0x2B as *mut u8;

/// Timer/Counter 0 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0B | 1000000 |
/// | FOC0A | 10000000 |
pub const TCCR0C: *mut u8 = 0x2C as *mut u8;

/// Timer/Counter 0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC0 | 10000000 |
/// | ICES0 | 1000000 |
/// | CS0 | 111 |
pub const TCCR0B: *mut u8 = 0x2D as *mut u8;

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0B | 110000 |
/// | COM0A | 11000000 |
pub const TCCR0A: *mut u8 = 0x2E as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | REMAP | 10 |
/// | PSR | 1 |
pub const GTCCR: *mut u8 = 0x2F as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
/// | WDIF | 10000000 |
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

/// Vcc Level Monitoring Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VLMF | 10000000 |
/// | VLMIE | 1000000 |
/// | VLM | 111 |
pub const VLMCSR: *mut u8 = 0x34 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 1 |
/// | PRUSART | 100 |
/// | PRADC | 10 |
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

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 1110 |
pub const SMCR: *mut u8 = 0x3A as *mut u8;

/// Reset Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
/// | PORF | 1 |
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
/// | C | 1 |
/// | S | 10000 |
/// | N | 100 |
/// | Z | 10 |
/// | T | 1000000 |
/// | V | 1000 |
/// | H | 100000 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSRA`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSRB`
pub const ACPMUX: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ACSRB`
pub const ACOE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADLAR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `BYTE0`
pub const WDTON: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BYTE0`
pub const SELFPROGEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `BYTE0`
pub const CKOUT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BYTE0`
pub const RSTDISBL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CLKMSR`
pub const CLKMS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKPSR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const REMAP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `NVMCSR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

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
pub const PCINT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT11: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT9: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT10: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PORTCR`
pub const BBMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSART: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const ICNC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const ICES0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0C`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0C`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const ICF0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const ICIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRA`
pub const DOR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UDRE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRA`
pub const U2X: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRA`
pub const MPCM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRA`
pub const FE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRA`
pub const RXC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRA`
pub const TXC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXB8: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UCSZ2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXCIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UDRIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXB8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSRC`
pub const USBS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UMSEL: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCPOL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRD`
pub const RXSIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRD`
pub const RXS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRD`
pub const SFDE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `VLMCSR`
pub const VLMF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `VLMCSR`
pub const VLMIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `VLMCSR`
pub const VLM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// `ADC_MUX_TINY10X` value group
#[allow(non_upper_case_globals)]
pub mod adc_mux_tiny10x {
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
}

/// `ANALOG_ADC_AUTO_TRIGGER_T10` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger_t10 {
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
   /// Timer/Counter0 Compare Match B.
   pub const VAL_0x05: u32 = 0x5;
   /// Pin Change Interrupt 0 Request.
   pub const VAL_0x06: u32 = 0x6;
   /// Timer/Counter0 Capture Event.
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

/// `ANALOG_ADC_V_REF_TINY10X` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref_tiny10x {
   /// Vcc.
   pub const VAL_0x00: u32 = 0x0;
   /// Internal 1.1V Referemce.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal 2.2V Referemce.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 4.3V Referemce.
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

/// CCP signature select
#[allow(non_upper_case_globals)]
pub mod cpu_ccp {
   /// SPM Instruction Protection.
   pub const SPM: u32 = 0xE7;
   /// IO Register Protection.
   pub const IOREG: u32 = 0xD8;
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
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction.
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
   /// Standby.
   pub const STDBY: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
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

