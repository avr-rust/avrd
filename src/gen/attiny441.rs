//! The AVR ATtiny441 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny441-SSU | SOIC-14 | SOIC14 | -40°C - 85°C | 1.7V - 5.5V | 16 MHz |
//! | ATtiny441-MU | QFN-20 | MLF20 | -40°C - 85°C | 1.7V - 5.5V | 16 MHz |
//! | ATtiny441-MMH | QFN-20 | VQFN20 | -40°C - 85°C | 1.7V - 5.5V | 16 MHz |
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
/// | CKOUT | 1000000 |
/// | CKDIV8 | 10000000 |
/// | SUT_CKSEL | 11111 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIEN | 100000 |
/// | DWEN | 1000000 |
/// | BODLEVEL | 111 |
/// | RSTDISBL | 10000000 |
/// | EESAVE | 1000 |
/// | WDTON | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODACT | 110 |
/// | SELFPRGEN | 1 |
/// | BODPD | 11000 |
/// | ULPOSCSEL | 11100000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 1000 |
/// | ADTS | 111 |
pub const ADCSRB: *mut u8 = 0x24 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADATE | 100000 |
/// | ADIE | 1000 |
/// | ADIF | 10000 |
/// | ADEN | 10000000 |
/// | ADPS | 111 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x25 as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x26 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x26 as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x27 as *mut u8;

/// The ADC multiplexer Selection Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11100000 |
/// | GSEL | 11 |
pub const ADMUXB: *mut u8 = 0x28 as *mut u8;

/// The ADC multiplexer Selection Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 111111 |
pub const ADMUXA: *mut u8 = 0x29 as *mut u8;

/// Analog Comparator 0 Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI0 | 10000 |
/// | ACD0 | 10000000 |
/// | ACO0 | 100000 |
/// | ACPMUX2 | 1000000 |
/// | ACIS0 | 11 |
/// | ACIE0 | 1000 |
/// | ACIC0 | 100 |
pub const ACSR0A: *mut u8 = 0x2A as *mut u8;

/// Analog Comparator 0 Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACNMUX | 1100 |
/// | HLEV0 | 1000000 |
/// | HSEL0 | 10000000 |
/// | ACOE0 | 10000 |
/// | ACPMUX | 11 |
pub const ACSR0B: *mut u8 = 0x2B as *mut u8;

/// Analog Comparator 1 Control And Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIE1 | 1000 |
/// | ACIS1 | 11 |
/// | ACO1 | 100000 |
/// | ACIC1 | 100 |
/// | ACBG1 | 1000000 |
/// | ACI1 | 10000 |
/// | ACD1 | 10000000 |
pub const ACSR1A: *mut u8 = 0x2C as *mut u8;

/// Analog Comparator 1 Control And Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACOE1 | 10000 |
/// | HLEV1 | 1000000 |
/// | HSEL1 | 10000000 |
/// | ACME1 | 100 |
pub const ACSR1B: *mut u8 = 0x2D as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1B | 100 |
/// | ICF1 | 100000 |
/// | TOV1 | 1 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x2E as *mut u8;

/// Timer/Counter1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 1 |
/// | ICIE1 | 100000 |
/// | OCIE1A | 10 |
/// | OCIE1B | 100 |
pub const TIMSK1: *mut u8 = 0x2F as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV2 | 1 |
/// | OCF2B | 100 |
/// | OCF2A | 10 |
/// | ICF2 | 100000 |
pub const TIFR2: *mut u8 = 0x30 as *mut u8;

/// Timer/Counter2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE2 | 100000 |
/// | OCIE2B | 100 |
/// | TOIE2 | 1 |
/// | OCIE2A | 10 |
pub const TIMSK2: *mut u8 = 0x31 as *mut u8;

/// Pin Change Enable Mask 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT0 | 1 |
/// | PCINT5 | 100000 |
/// | PCINT6 | 1000000 |
/// | PCINT4 | 10000 |
/// | PCINT2 | 100 |
/// | PCINT1 | 10 |
/// | PCINT3 | 1000 |
/// | PCINT7 | 10000000 |
pub const PCMSK0: *mut u8 = 0x32 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x33 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x34 as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x35 as *mut u8;

/// Port B Data register.
pub const PINB: *mut u8 = 0x36 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x37 as *mut u8;

/// Input Pins, Port B.
pub const PORTB: *mut u8 = 0x38 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x39 as *mut u8;

/// Data Direction Register, Port A.
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
/// | EEMPE | 100 |
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

/// Pin Change Enable Mask 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT11 | 1000 |
/// | PCINT10 | 100 |
/// | PCINT8 | 1 |
/// | PCINT9 | 10 |
pub const PCMSK1: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDE | 1000 |
/// | WDIF | 10000000 |
/// | WDIE | 1000000 |
pub const WDTCSR: *mut u8 = 0x41 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1B | 1000000 |
pub const TCCR1C: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR | 1 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

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

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x4A as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x4A as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x4B as *mut u8;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x4C as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x4C as *mut u16;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x4D as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x4F as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0B | 110000 |
/// | WGM0 | 11 |
/// | COM0A | 11000000 |
pub const TCCR0A: *mut u8 = 0x50 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0B | 1000000 |
/// | FOC0A | 10000000 |
/// | CS0 | 111 |
/// | WGM02 | 1000 |
pub const TCCR0B: *mut u8 = 0x53 as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | WDRF | 1000 |
/// | BORF | 100 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 11000 |
/// | ISC0 | 11 |
/// | SE | 100000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Timer/Counter0 Output Compare Register A.
pub const OCR0A: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | RSIG | 100000 |
/// | RFLB | 1000 |
/// | CTPB | 10000 |
/// | PGWRT | 100 |
/// | SPMEN | 1 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0B | 100 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE0 | 1 |
/// | OCIE0B | 100 |
/// | OCIE0A | 10 |
pub const TIMSK0: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 110000 |
/// | INTF0 | 1000000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1000000 |
/// | PCIE | 110000 |
pub const GIMSK: *mut u8 = 0x5B as *mut u8;

/// Timer/Counter0 Output Compare Register B.
pub const OCR0B: *mut u8 = 0x5C as *mut u8;

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
/// | N | 100 |
/// | S | 10000 |
/// | H | 100000 |
/// | T | 1000000 |
/// | I | 10000000 |
/// | C | 1 |
/// | Z | 10 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC2D | 100 |
/// | ADC5D | 100000 |
/// | ADC0D | 1 |
/// | ADC3D | 1000 |
/// | ADC7D | 10000000 |
/// | ADC1D | 10 |
/// | ADC6D | 1000000 |
/// | ADC4D | 10000 |
pub const DIDR0: *mut u8 = 0x60 as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC9D | 1000 |
/// | ADC11D | 1 |
/// | ADC8D | 100 |
/// | ADC10D | 10 |
pub const DIDR1: *mut u8 = 0x61 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEB: *mut u8 = 0x62 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEA: *mut u8 = 0x63 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMA | 1 |
pub const PORTCR: *mut u8 = 0x64 as *mut u8;

/// Remap Port Pins.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIMAP | 10 |
pub const REMAP: *mut u8 = 0x65 as *mut u8;

/// Timer Output Compare Pin Mux Channel Output Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOCC2OE | 100 |
/// | TOCC4OE | 10000 |
/// | TOCC5OE | 100000 |
/// | TOCC3OE | 1000 |
/// | TOCC0OE | 1 |
/// | TOCC7OE | 10000000 |
/// | TOCC6OE | 1000000 |
/// | TOCC1OE | 10 |
pub const TOCPMCOE: *mut u8 = 0x66 as *mut u8;

/// Timer Output Compare Pin Mux Selection 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOCC2S | 110000 |
/// | TOCC3S | 11000000 |
/// | TOCC0S | 11 |
/// | TOCC1S | 1100 |
pub const TOCPMSA0: *mut u8 = 0x67 as *mut u8;

/// Timer Output Compare Pin Mux Selection 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOCC5S | 1100 |
/// | TOCC4S | 11 |
/// | TOCC7S | 11000000 |
/// | TOCC6S | 110000 |
pub const TOCPMSA1: *mut u8 = 0x68 as *mut u8;

/// Port High Drive Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PHDEA | 11 |
pub const PHDE: *mut u8 = 0x6A as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUSART1 | 1000000 |
/// | PRADC | 1 |
/// | PRSPI | 10000 |
/// | PRUSART0 | 100000 |
/// | PRTWI | 10000000 |
/// | PRTIM1 | 100 |
/// | PRTIM0 | 10 |
/// | PRTIM2 | 1000 |
pub const PRR: *mut u8 = 0x70 as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x71 as *mut u8;

/// Clock Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKSEL | 1111 |
/// | OSCRDY | 10000000 |
/// | CSTR | 1000000 |
/// | CKOUTC | 100000 |
/// | SUT | 10000 |
pub const CLKCR: *mut u8 = 0x72 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x73 as *mut u8;

/// Oscillator Calibration Register 8MHz.
pub const OSCCAL0: *mut u8 = 0x74 as *mut u8;

/// Oscillator Temperature Calibration Register A.
pub const OSCTCAL0A: *mut u8 = 0x75 as *mut u8;

/// Oscillator Temperature Calibration Register B.
pub const OSCTCAL0B: *mut u8 = 0x76 as *mut u8;

/// Oscillator Calibration Register 32kHz.
pub const OSCCAL1: *mut u8 = 0x77 as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0x80 as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR0L: *mut u8 = 0x81 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR0: *mut u16 = 0x81 as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR0H: *mut u8 = 0x82 as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDE0 | 100000 |
/// | RXSIE0 | 10000000 |
/// | RXS0 | 1000000 |
pub const UCSR0D: *mut u8 = 0x83 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS0 | 1000 |
/// | UCPOL0 | 1 |
/// | UCSZ0 | 110 |
/// | UMSEL0 | 11000000 |
/// | UPM0 | 110000 |
pub const UCSR0C: *mut u8 = 0x84 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCSZ02 | 100 |
/// | RXB80 | 10 |
/// | TXEN0 | 1000 |
/// | RXCIE0 | 10000000 |
/// | RXEN0 | 10000 |
/// | UDRIE0 | 100000 |
/// | TXB80 | 1 |
/// | TXCIE0 | 1000000 |
pub const UCSR0B: *mut u8 = 0x85 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC0 | 10000000 |
/// | TXC0 | 1000000 |
/// | UPE0 | 100 |
/// | MPCM0 | 1 |
/// | UDRE0 | 100000 |
/// | FE0 | 10000 |
/// | U2X0 | 10 |
/// | DOR0 | 1000 |
pub const UCSR0A: *mut u8 = 0x86 as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0x90 as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0x91 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0x91 as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0x92 as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXS1 | 1000000 |
/// | SFDE1 | 100000 |
/// | RXSIE1 | 10000000 |
pub const UCSR1D: *mut u8 = 0x93 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM1 | 110000 |
/// | UCSZ1 | 110 |
/// | UMSEL1 | 11000000 |
/// | USBS1 | 1000 |
/// | UCPOL1 | 1 |
pub const UCSR1C: *mut u8 = 0x94 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXCIE1 | 10000000 |
/// | TXB81 | 1 |
/// | TXCIE1 | 1000000 |
/// | UDRIE1 | 100000 |
/// | UCSZ12 | 100 |
/// | RXB81 | 10 |
/// | TXEN1 | 1000 |
/// | RXEN1 | 10000 |
pub const UCSR1B: *mut u8 = 0x95 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DOR1 | 1000 |
/// | FE1 | 10000 |
/// | TXC1 | 1000000 |
/// | MPCM1 | 1 |
/// | UDRE1 | 100000 |
/// | U2X1 | 10 |
/// | RXC1 | 10000000 |
/// | UPE1 | 100 |
pub const UCSR1A: *mut u8 = 0x96 as *mut u8;

/// TWI Slave Data Register.
pub const TWSD: *mut u8 = 0xA0 as *mut u8;

/// TWI Slave Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAE | 1 |
pub const TWSAM: *mut u8 = 0xA1 as *mut u8;

/// TWI Slave Address Register.
pub const TWSA: *mut u8 = 0xA2 as *mut u8;

/// TWI Slave Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWDIF | 10000000 |
/// | TWCH | 100000 |
/// | TWAS | 1 |
/// | TWDIR | 10 |
/// | TWASIF | 1000000 |
/// | TWBE | 100 |
/// | TWRA | 10000 |
/// | TWC | 1000 |
pub const TWSSRA: *mut u8 = 0xA3 as *mut u8;

/// TWI Slave Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWHNM | 1000 |
/// | TWCMD | 11 |
/// | TWAA | 100 |
pub const TWSCRB: *mut u8 = 0xA4 as *mut u8;

/// TWI Slave Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWSME | 1 |
/// | TWASIE | 10000 |
/// | TWSIE | 100 |
/// | TWDIE | 100000 |
/// | TWEN | 1000 |
/// | TWSHE | 10000000 |
/// | TWPME | 10 |
pub const TWSCRA: *mut u8 = 0xA5 as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0xB0 as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
/// | SPI2X | 1 |
pub const SPSR: *mut u8 = 0xB1 as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPR | 11 |
/// | CPOL | 1000 |
/// | DORD | 100000 |
/// | SPE | 1000000 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
/// | CPHA | 100 |
pub const SPCR: *mut u8 = 0xB2 as *mut u8;

/// Timer/Counter2 Input Capture Register  Bytes low byte.
pub const ICR2L: *mut u8 = 0xC0 as *mut u8;

/// Timer/Counter2 Input Capture Register  Bytes.
pub const ICR2: *mut u16 = 0xC0 as *mut u16;

/// Timer/Counter2 Input Capture Register  Bytes high byte.
pub const ICR2H: *mut u8 = 0xC1 as *mut u8;

/// Timer/Counter2 Output Compare Register B  Bytes low byte.
pub const OCR2BL: *mut u8 = 0xC2 as *mut u8;

/// Timer/Counter2 Output Compare Register B  Bytes.
pub const OCR2B: *mut u16 = 0xC2 as *mut u16;

/// Timer/Counter2 Output Compare Register B  Bytes high byte.
pub const OCR2BH: *mut u8 = 0xC3 as *mut u8;

/// Timer/Counter2 Output Compare Register A  Bytes.
pub const OCR2A: *mut u16 = 0xC4 as *mut u16;

/// Timer/Counter2 Output Compare Register A  Bytes low byte.
pub const OCR2AL: *mut u8 = 0xC4 as *mut u8;

/// Timer/Counter2 Output Compare Register A  Bytes high byte.
pub const OCR2AH: *mut u8 = 0xC5 as *mut u8;

/// Timer/Counter2  Bytes low byte.
pub const TCNT2L: *mut u8 = 0xC6 as *mut u8;

/// Timer/Counter2  Bytes.
pub const TCNT2: *mut u16 = 0xC6 as *mut u16;

/// Timer/Counter2  Bytes high byte.
pub const TCNT2H: *mut u8 = 0xC7 as *mut u8;

/// Timer/Counter2 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC2A | 10000000 |
/// | FOC2B | 1000000 |
pub const TCCR2C: *mut u8 = 0xC8 as *mut u8;

/// Timer/Counter2 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC2 | 10000000 |
/// | ICES2 | 1000000 |
/// | CS2 | 111 |
pub const TCCR2B: *mut u8 = 0xC9 as *mut u8;

/// Timer/Counter2 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM2B | 110000 |
/// | COM2A | 11000000 |
pub const TCCR2A: *mut u8 = 0xCA as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACI0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACD0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACO0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACPMUX2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACIS0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACIE0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR0A`
pub const ACIC0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR0B`
pub const ACNMUX: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `ACSR0B`
pub const HLEV0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR0B`
pub const HSEL0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR0B`
pub const ACOE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR0B`
pub const ACPMUX: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACIE1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACIS1: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACO1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACIC1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACBG1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACI1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR1A`
pub const ACD1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR1B`
pub const ACOE1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR1B`
pub const HLEV1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR1B`
pub const HSEL1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR1B`
pub const ACME1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADLAR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADMUXA`
pub const MUX: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ADMUXB`
pub const REFS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `ADMUXB`
pub const GSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKCR`
pub const CKSEL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKCR`
pub const OSCRDY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKCR`
pub const CSTR: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CLKCR`
pub const CKOUTC: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CLKCR`
pub const SUT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC9D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC11D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC8D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC10D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODACT: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODPD: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const ULPOSCSEL: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `GIFR`
pub const PCIF: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIMSK`
pub const INT0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIMSK`
pub const PCIE: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT11: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT10: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT9: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PHDE`
pub const PHDEA: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PORTCR`
pub const BBMA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSART1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRSPI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSART0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTWI: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `REMAP`
pub const SPIMAP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RSIG: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RFLB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const CTPB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const ICNC2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const ICES2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR2C`
pub const FOC2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2C`
pub const FOC2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR2`
pub const TOV2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR2`
pub const ICF2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const ICIE2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const TOIE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC2OE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC4OE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC5OE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC3OE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC0OE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC7OE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC6OE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TOCPMCOE`
pub const TOCC1OE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TOCPMSA0`
pub const TOCC2S: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TOCPMSA0`
pub const TOCC3S: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TOCPMSA0`
pub const TOCC0S: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TOCPMSA0`
pub const TOCC1S: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TOCPMSA1`
pub const TOCC5S: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TOCPMSA1`
pub const TOCC4S: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TOCPMSA1`
pub const TOCC7S: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TOCPMSA1`
pub const TOCC6S: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TWSAM`
pub const TWAE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWSME: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWASIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWSIE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWDIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWSHE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSCRA`
pub const TWPME: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TWSCRB`
pub const TWHNM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWSCRB`
pub const TWCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWSCRB`
pub const TWAA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWCH: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWAS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWDIR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWASIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWBE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWRA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWSSRA`
pub const TWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const RXC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const TXC0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UPE0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const MPCM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UDRE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const FE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const U2X0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const DOR0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXB80: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXEN0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXEN0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXB80: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const USBS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UPM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR0D`
pub const SFDE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0D`
pub const RXSIE0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0D`
pub const RXS0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const DOR1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const FE1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const TXC1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const MPCM1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UDRE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const U2X1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const RXC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UPE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXB81: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXB81: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXEN1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXEN1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UPM1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const USBS1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1D`
pub const RXS1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1D`
pub const SFDE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1D`
pub const RXSIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// `ANALOG_ADC_AUTO_TRIGGER_T841` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger_t841 {
   /// Free Running mode.
   pub const VAL_0x00: u32 = 0x0;
   /// Analog Comparator 0.
   pub const VAL_0x01: u32 = 0x1;
   /// External Interrupt Request 0.
   pub const VAL_0x02: u32 = 0x2;
   /// Timer/Counter0 Compare Match A.
   pub const VAL_0x03: u32 = 0x3;
   /// Timer/Counter0 Overflow.
   pub const VAL_0x04: u32 = 0x4;
   /// Timer/Counter1 Compare Match A.
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

/// `COMM_USART_MODE_2BIT` value group
#[allow(non_upper_case_globals)]
pub mod comm_usart_mode_2bit {
   /// Asynchronous USART.
   pub const VAL_0x00: u32 = 0x0;
   /// Synchronous USART.
   pub const VAL_0x01: u32 = 0x1;
   /// Master SPI.
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

/// `CPU_SLEEP_MODE` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode {
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
}

/// `ENUM_BODMODE` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodmode {
   /// Sampled.
   pub const BOD_SAMPLED: u32 = 0x1;
   /// Enabled.
   pub const BOD_ENABLED: u32 = 0x2;
   /// Disabled.
   pub const BOD_DISABLED: u32 = 0x3;
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
   /// Ext. Clock; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms.
   pub const EXTCLK_6CK_16CK_16MS: u32 = 0x0;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms.
   pub const INTRCOSC_8MHZ_6CK_16CK_16MS: u32 = 0x2;
   /// Int. ULP Osc.; Start-up time PWRDWN/RESET: 6 CK/16 CK + 16 ms.
   pub const INTULPOSC_32KHZ_6CK_16CK_16MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTLOFXTAL_1KCK_16CK_16MS: u32 = 0x6;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/16 CK + 16 ms.
   pub const EXTLOFXTAL_32KCK_14CK_16MS: u32 = 0x16;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_258CK_16CK_16MS: u32 = 0x8;
   /// Ext. Ceramic Res. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ4_0MHZ9_1KCK_16CK_16MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_16CK_16MS: u32 = 0x9;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ9_3MHZ_258CK_16CK_16MS: u32 = 0xA;
   /// Ext. Ceramic Res. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_0MHZ9_3MHZ_1KCK_16CK_16MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_16CK_16MS: u32 = 0xB;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_3MHZ_8MHZ_258CK_16CK_16MS: u32 = 0xC;
   /// Ext. Ceramic Res. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_3MHZ_8MHZ_1KCK_16CK_16MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_16CK_16MS: u32 = 0xD;
   /// Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 258 CK/16 CK + 16 ms.
   pub const EXTCRES_8MHZ_XX_258CK_16CK_16MS: u32 = 0xE;
   /// Ext. Ceramic Res. 8.0- MHz; Start-up time PWRDWN/RESET: 1K CK/16 CK + 16 ms.
   pub const EXTCRES_8MHZ_XX_1KCK_16CK_16MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0- MHz; Start-up time PWRDWN/RESET: 16 K CK/16 CK + 16 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_16CK_16MS: u32 = 0xF;
}

/// `ENUM_ULPOSCSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_ulposcsel {
   /// 32 kHz.
   pub const ULPOSC_32KHZ: u32 = 0x7;
   /// 64 kHz.
   pub const ULPOSC_64KHZ: u32 = 0x6;
   /// 128 kHz.
   pub const ULPOSC_128KHZ: u32 = 0x5;
   /// 256 kHz.
   pub const ULPOSC_256KHZ: u32 = 0x4;
   /// 512 kHz.
   pub const ULPOSC_512KHZ: u32 = 0x3;
}

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Reserved.
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
   /// 32 kHz.
   pub const _32_kHz: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_4BITS_32KHZ` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_4bits_32khz {
   /// Oscillator Cycles 512 (16 ms).
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 1K (32 ms).
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 2K (64 ms).
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 4K (0.125 s).
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 8K (0.25 s).
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 16K (0.5 s).
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 32K (1.0 s).
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 64K (2.0 s).
   pub const VAL_0x07: u32 = 0x7;
   /// Oscillator Cycles 128K (4.0 s).
   pub const VAL_0x08: u32 = 0x8;
   /// Oscillator Cycles 256K (8.0 s).
   pub const VAL_0x09: u32 = 0x9;
}

