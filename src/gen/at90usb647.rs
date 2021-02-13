//! The AVR AT90USB647 microcontroller
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
/// | SUT_CKSEL | 111111 |
/// | CKOUT | 1000000 |
/// | CKDIV8 | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
/// | BLB1 | 110000 |
/// | BLB0 | 1100 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIEN | 100000 |
/// | WDTON | 10000 |
/// | BOOTRST | 1 |
/// | EESAVE | 1000 |
/// | BOOTSZ | 110 |
/// | JTAGEN | 1000000 |
/// | OCDEN | 10000000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 111 |
/// | HWBE | 1000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

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

/// Input Pins, Port E.
pub const PINE: *mut u8 = 0x2C as *mut u8;

/// Data Direction Register, Port E.
pub const DDRE: *mut u8 = 0x2D as *mut u8;

/// Data Register, Port E.
pub const PORTE: *mut u8 = 0x2E as *mut u8;

/// Input Pins, Port F.
pub const PINF: *mut u8 = 0x2F as *mut u8;

/// Data Direction Register, Port F.
pub const DDRF: *mut u8 = 0x30 as *mut u8;

/// Data Register, Port F.
pub const PORTF: *mut u8 = 0x31 as *mut u8;

/// Timer/Counter0 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0B | 100 |
/// | TOV0 | 1 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter1 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1B | 100 |
/// | TOV1 | 1 |
/// | OCF1A | 10 |
/// | ICF1 | 100000 |
/// | OCF1C | 1000 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF2B | 100 |
/// | TOV2 | 1 |
/// | OCF2A | 10 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter3 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF3B | 100 |
/// | OCF3C | 1000 |
/// | OCF3A | 10 |
/// | ICF3 | 100000 |
/// | TOV3 | 1 |
pub const TIFR3: *mut u8 = 0x38 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

/// External Interrupt Flag Register.
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GPIOR00 | 1 |
/// | GPIOR04 | 10000 |
/// | GPIOR07 | 10000000 |
/// | GPIOR01 | 10 |
/// | GPIOR02 | 100 |
/// | GPIOR06 | 1000000 |
/// | GPIOR05 | 100000 |
/// | GPIOR03 | 1000 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPM | 110000 |
/// | EERE | 1 |
/// | EEMPE | 100 |
/// | EEPE | 10 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register Low Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register Low Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register Low Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer Counter Control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSRASY | 10 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM02 | 1000 |
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
/// | CS0 | 111 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// PLL Status and Control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLOCK | 1 |
/// | PLLE | 10 |
/// | PLLP | 11100 |
pub const PLLCSR: *mut u8 = 0x49 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIE | 10000000 |
/// | CPOL | 1000 |
/// | CPHA | 100 |
/// | SPE | 1000000 |
/// | SPR | 11 |
/// | DORD | 100000 |
/// | MSTR | 10000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | WCOL | 1000000 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACBG | 1000000 |
/// | ACIC | 100 |
/// | ACD | 10000000 |
/// | ACIE | 1000 |
/// | ACO | 100000 |
/// | ACIS | 11 |
/// | ACI | 10000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// On-Chip Debug Related Register in I/O Memory.
pub const OCDR: *mut u8 = 0x51 as *mut u8;

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
/// | JTRF | 10000 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTD | 10000000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SIGRD | 100000 |
/// | RWWSB | 1000000 |
/// | PGERS | 10 |
/// | BLBSET | 1000 |
/// | SPMIE | 10000000 |
/// | RWWSRE | 10000 |
/// | SPMEN | 1 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// RAM Page Z Select Register.
pub const RAMPZ: *mut u8 = 0x5B as *mut u8;

/// Extended Indirect Register.
pub const EIND: *mut u8 = 0x5C as *mut u8;

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
/// | Z | 10 |
/// | N | 100 |
/// | I | 10000000 |
/// | S | 10000 |
/// | T | 1000000 |
/// | C | 1 |
/// | V | 1000 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIF | 10000000 |
/// | WDE | 1000 |
/// | WDIE | 1000000 |
/// | WDCE | 10000 |
/// | WDP | 100111 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// `CLKPR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTWI | 10000000 |
/// | PRTIM2 | 1000000 |
/// | PRADC | 1 |
/// | PRTIM0 | 100000 |
/// | PRSPI | 100 |
/// | PRTIM1 | 1000 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Power Reduction Register1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM3 | 1000 |
/// | PRUSART1 | 1 |
/// | PRUSB | 10000000 |
pub const PRR1: *mut u8 = 0x65 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC3 | 11000000 |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
/// | ISC2 | 110000 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC5 | 1100 |
/// | ISC4 | 11 |
/// | ISC6 | 110000 |
/// | ISC7 | 11000000 |
pub const EICRB: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 10 |
/// | OCIE0B | 100 |
/// | TOIE0 | 1 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE1 | 100000 |
/// | TOIE1 | 1 |
/// | OCIE1B | 100 |
/// | OCIE1C | 1000 |
/// | OCIE1A | 10 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE2 | 1 |
/// | OCIE2B | 100 |
/// | OCIE2A | 10 |
pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

/// Timer/Counter3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE3A | 10 |
/// | OCIE3C | 1000 |
/// | ICIE3 | 100000 |
/// | OCIE3B | 100 |
/// | TOIE3 | 1 |
pub const TIMSK3: *mut u8 = 0x71 as *mut u8;

/// External Memory Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRW0 | 11 |
/// | SRL | 1110000 |
/// | SRW1 | 1100 |
/// | SRE | 10000000 |
pub const XMCRA: *mut u8 = 0x74 as *mut u8;

/// External Memory Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XMM | 111 |
/// | XMBK | 10000000 |
pub const XMCRB: *mut u8 = 0x75 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADSC | 1000000 |
/// | ADEN | 10000000 |
/// | ADPS | 111 |
/// | ADIF | 10000 |
/// | ADATE | 100000 |
/// | ADIE | 1000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACME | 1000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
/// | MUX | 11111 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC7D | 10000000 |
/// | ADC0D | 1 |
/// | ADC6D | 1000000 |
/// | ADC5D | 100000 |
/// | ADC3D | 1000 |
/// | ADC2D | 100 |
/// | ADC4D | 10000 |
/// | ADC1D | 10 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// `DIDR1` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN0D | 1 |
/// | AIN1D | 10 |
pub const DIDR1: *mut u8 = 0x7F as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1C | 1100 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS1 | 111 |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter 1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1C | 100000 |
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes low byte.
pub const OCR1CL: *mut u8 = 0x8C as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes.
pub const OCR1C: *mut u16 = 0x8C as *mut u16;

/// Timer/Counter1 Output Compare Register C  Bytes high byte.
pub const OCR1CH: *mut u8 = 0x8D as *mut u8;

/// Timer/Counter3 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM3A | 11000000 |
/// | COM3C | 1100 |
/// | COM3B | 110000 |
pub const TCCR3A: *mut u8 = 0x90 as *mut u8;

/// Timer/Counter3 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES3 | 1000000 |
/// | ICNC3 | 10000000 |
/// | CS3 | 111 |
pub const TCCR3B: *mut u8 = 0x91 as *mut u8;

/// Timer/Counter 3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3A | 10000000 |
/// | FOC3C | 100000 |
/// | FOC3B | 1000000 |
pub const TCCR3C: *mut u8 = 0x92 as *mut u8;

/// Timer/Counter3  Bytes low byte.
pub const TCNT3L: *mut u8 = 0x94 as *mut u8;

/// Timer/Counter3  Bytes.
pub const TCNT3: *mut u16 = 0x94 as *mut u16;

/// Timer/Counter3  Bytes high byte.
pub const TCNT3H: *mut u8 = 0x95 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes.
pub const ICR3: *mut u16 = 0x96 as *mut u16;

/// Timer/Counter3 Input Capture Register  Bytes low byte.
pub const ICR3L: *mut u8 = 0x96 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes high byte.
pub const ICR3H: *mut u8 = 0x97 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes low byte.
pub const OCR3AL: *mut u8 = 0x98 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes.
pub const OCR3A: *mut u16 = 0x98 as *mut u16;

/// Timer/Counter3 Output Compare Register A  Bytes high byte.
pub const OCR3AH: *mut u8 = 0x99 as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes.
pub const OCR3B: *mut u16 = 0x9A as *mut u16;

/// Timer/Counter3 Output Compare Register B  Bytes low byte.
pub const OCR3BL: *mut u8 = 0x9A as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes high byte.
pub const OCR3BH: *mut u8 = 0x9B as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes low byte.
pub const OCR3CL: *mut u8 = 0x9C as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes.
pub const OCR3C: *mut u16 = 0x9C as *mut u16;

/// Timer/Counter3 Output Compare Register B  Bytes high byte.
pub const OCR3CH: *mut u8 = 0x9D as *mut u8;

/// `UHCON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOFEN | 1 |
/// | RESUME | 100 |
/// | RESET | 10 |
pub const UHCON: *mut u8 = 0x9E as *mut u8;

/// `UHINT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXRSMI | 10000 |
/// | UHUPI | 1000000 |
/// | RSTI | 100 |
/// | DDISCI | 10 |
/// | RSMEDI | 1000 |
/// | HSOFI | 100000 |
/// | DCONNI | 1 |
pub const UHINT: *mut u8 = 0x9F as *mut u8;

/// `UHIEN` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DDISCE | 10 |
/// | HWUPE | 1000000 |
/// | RXRSME | 10000 |
/// | HSOFE | 100000 |
/// | RSTE | 100 |
/// | RSMEDE | 1000 |
/// | DCONNE | 1 |
pub const UHIEN: *mut u8 = 0xA0 as *mut u8;

/// `UHADDR` register
pub const UHADDR: *mut u8 = 0xA1 as *mut u8;

/// `UHFNUM` register
pub const UHFNUM: *mut u16 = 0xA2 as *mut u16;

/// low byte.
pub const UHFNUML: *mut u8 = 0xA2 as *mut u8;

/// high byte.
pub const UHFNUMH: *mut u8 = 0xA3 as *mut u8;

/// `UHFLEN` register
pub const UHFLEN: *mut u8 = 0xA4 as *mut u8;

/// `UPINRQX` register
pub const UPINRQX: *mut u8 = 0xA5 as *mut u8;

/// `UPINTX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXSTALLI | 10 |
/// | TXSTPI | 1000 |
/// | NAKEDI | 1000000 |
/// | RXINI | 1 |
/// | PERRI | 10000 |
/// | TXOUTI | 100 |
pub const UPINTX: *mut u8 = 0xA6 as *mut u8;

/// `UPNUM` register
pub const UPNUM: *mut u8 = 0xA7 as *mut u8;

/// `UPRST` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRST | 1111111 |
pub const UPRST: *mut u8 = 0xA8 as *mut u8;

/// `UPCONX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEN | 1 |
/// | PFREEZE | 1000000 |
/// | INMODE | 100000 |
pub const UPCONX: *mut u8 = 0xA9 as *mut u8;

/// `UPCFG0X` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PTYPE | 11000000 |
/// | PTOKEN | 110000 |
/// | PEPNUM | 1111 |
pub const UPCFG0X: *mut u8 = 0xAA as *mut u8;

/// `UPCFG1X` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PBK | 1100 |
/// | PSIZE | 1110000 |
pub const UPCFG1X: *mut u8 = 0xAB as *mut u8;

/// `UPSTAX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NBUSYK | 11 |
pub const UPSTAX: *mut u8 = 0xAC as *mut u8;

/// `UPCFG2X` register
pub const UPCFG2X: *mut u8 = 0xAD as *mut u8;

/// `UPIENX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXOUTE | 100 |
/// | RXINE | 1 |
/// | NAKEDE | 1000000 |
/// | RXSTALLE | 10 |
/// | TXSTPE | 1000 |
/// | PERRE | 10000 |
pub const UPIENX: *mut u8 = 0xAE as *mut u8;

/// `UPDATX` register
pub const UPDATX: *mut u8 = 0xAF as *mut u8;

/// Timer/Counter2 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM2 | 11 |
/// | COM2A | 11000000 |
/// | COM2B | 110000 |
pub const TCCR2A: *mut u8 = 0xB0 as *mut u8;

/// Timer/Counter2 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM22 | 1000 |
/// | FOC2B | 1000000 |
/// | CS2 | 111 |
/// | FOC2A | 10000000 |
pub const TCCR2B: *mut u8 = 0xB1 as *mut u8;

/// Timer/Counter2.
pub const TCNT2: *mut u8 = 0xB2 as *mut u8;

/// Timer/Counter2 Output Compare Register A.
pub const OCR2A: *mut u8 = 0xB3 as *mut u8;

/// Timer/Counter2 Output Compare Register B.
pub const OCR2B: *mut u8 = 0xB4 as *mut u8;

/// Asynchronous Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCR2AUB | 1000 |
/// | TCR2AUB | 10 |
/// | AS2 | 100000 |
/// | OCR2BUB | 100 |
/// | TCN2UB | 10000 |
/// | TCR2BUB | 1 |
/// | EXCLK | 1000000 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR: *mut u8 = 0xB8 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWPS | 11 |
/// | TWS | 11111000 |
pub const TWSR: *mut u8 = 0xB9 as *mut u8;

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWA | 11111110 |
/// | TWGCE | 1 |
pub const TWAR: *mut u8 = 0xBA as *mut u8;

/// TWI Data register.
pub const TWDR: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWSTO | 10000 |
/// | TWEN | 100 |
/// | TWSTA | 100000 |
/// | TWEA | 1000000 |
/// | TWIE | 1 |
/// | TWWC | 1000 |
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

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FE1 | 10000 |
/// | UPE1 | 100 |
/// | MPCM1 | 1 |
/// | UDRE1 | 100000 |
/// | TXC1 | 1000000 |
/// | DOR1 | 1000 |
/// | U2X1 | 10 |
/// | RXC1 | 10000000 |
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXB81 | 1 |
/// | RXCIE1 | 10000000 |
/// | RXB81 | 10 |
/// | UCSZ12 | 100 |
/// | TXEN1 | 1000 |
/// | UDRIE1 | 100000 |
/// | RXEN1 | 10000 |
/// | TXCIE1 | 1000000 |
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS1 | 1000 |
/// | UCSZ1 | 110 |
/// | UMSEL1 | 11000000 |
/// | UPM1 | 110000 |
/// | UCPOL1 | 1 |
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// USB Hardware Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UIMOD | 10000000 |
/// | UIDE | 1000000 |
/// | UVCONE | 10000 |
/// | UVREGE | 1 |
pub const UHWCON: *mut u8 = 0xD7 as *mut u8;

/// USB General Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VBUSTE | 1 |
/// | IDTE | 10 |
/// | FRZCLK | 100000 |
/// | OTGPADE | 10000 |
/// | HOST | 1000000 |
/// | USBE | 10000000 |
pub const USBCON: *mut u8 = 0xD8 as *mut u8;

/// `USBSTA` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VBUS | 1 |
/// | SPEED | 1000 |
/// | ID | 10 |
pub const USBSTA: *mut u8 = 0xD9 as *mut u8;

/// `USBINT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDTI | 10 |
/// | VBUSTI | 1 |
pub const USBINT: *mut u8 = 0xDA as *mut u8;

/// `OTGCON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VBUSREQ | 10 |
/// | SRPREQ | 10000 |
/// | VBUSHWC | 100 |
/// | VBUSRQC | 1 |
/// | HNPREQ | 100000 |
/// | SRPSEL | 1000 |
pub const OTGCON: *mut u8 = 0xDD as *mut u8;

/// `OTGIEN` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ROLEEXE | 1000 |
/// | STOE | 100000 |
/// | HNPERRE | 10000 |
/// | SRPE | 1 |
/// | BCERRE | 100 |
/// | VBERRE | 10 |
pub const OTGIEN: *mut u8 = 0xDE as *mut u8;

/// `OTGINT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ROLEEXI | 1000 |
/// | STOI | 100000 |
/// | SRPI | 1 |
/// | HNPERRI | 10000 |
/// | VBERRI | 10 |
/// | BCERRI | 100 |
pub const OTGINT: *mut u8 = 0xDF as *mut u8;

/// `UDCON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RMWKUP | 10 |
/// | DETACH | 1 |
/// | LSM | 100 |
pub const UDCON: *mut u8 = 0xE0 as *mut u8;

/// `UDINT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOFI | 100 |
/// | EORSTI | 1000 |
/// | SUSPI | 1 |
/// | EORSMI | 100000 |
/// | WAKEUPI | 10000 |
/// | UPRSMI | 1000000 |
pub const UDINT: *mut u8 = 0xE1 as *mut u8;

/// `UDIEN` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SOFE | 100 |
/// | SUSPE | 1 |
/// | WAKEUPE | 10000 |
/// | EORSTE | 1000 |
/// | UPRSME | 1000000 |
/// | EORSME | 100000 |
pub const UDIEN: *mut u8 = 0xE2 as *mut u8;

/// `UDADDR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UADD | 1111111 |
/// | ADDEN | 10000000 |
pub const UDADDR: *mut u8 = 0xE3 as *mut u8;

/// low byte.
pub const UDFNUML: *mut u8 = 0xE4 as *mut u8;

/// `UDFNUM` register
pub const UDFNUM: *mut u16 = 0xE4 as *mut u16;

/// high byte.
pub const UDFNUMH: *mut u8 = 0xE5 as *mut u8;

/// `UDMFN` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FNCERR | 10000 |
pub const UDMFN: *mut u8 = 0xE6 as *mut u8;

/// `UEINTX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NAKINI | 1000000 |
/// | NAKOUTI | 10000 |
/// | RXSTPI | 1000 |
/// | TXINI | 1 |
/// | STALLEDI | 10 |
/// | RXOUTI | 100 |
pub const UEINTX: *mut u8 = 0xE8 as *mut u8;

/// `UENUM` register
pub const UENUM: *mut u8 = 0xE9 as *mut u8;

/// `UERST` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPRST | 1111111 |
pub const UERST: *mut u8 = 0xEA as *mut u8;

/// `UECONX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STALLRQC | 10000 |
/// | STALLRQ | 100000 |
/// | EPEN | 1 |
pub const UECONX: *mut u8 = 0xEB as *mut u8;

/// `UECFG0X` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPTYPE | 11000000 |
/// | EPDIR | 1 |
pub const UECFG0X: *mut u8 = 0xEC as *mut u8;

/// `UECFG1X` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPSIZE | 1110000 |
/// | EPBK | 1100 |
pub const UECFG1X: *mut u8 = 0xED as *mut u8;

/// `UESTA0X` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NBUSYBK | 11 |
pub const UESTA0X: *mut u8 = 0xEE as *mut u8;

/// `UESTA1X` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CURRBK | 11 |
/// | CTRLDIR | 100 |
pub const UESTA1X: *mut u8 = 0xEF as *mut u8;

/// `UEIENX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NAKINE | 1000000 |
/// | RXOUTE | 100 |
/// | RXSTPE | 1000 |
/// | STALLEDE | 10 |
/// | NAKOUTE | 10000 |
/// | TXINE | 1 |
pub const UEIENX: *mut u8 = 0xF0 as *mut u8;

/// `UEDATX` register
pub const UEDATX: *mut u8 = 0xF1 as *mut u8;

/// `UEBCLX` register
pub const UEBCLX: *mut u8 = 0xF2 as *mut u8;

/// `UEBCHX` register
pub const UEBCHX: *mut u8 = 0xF3 as *mut u8;

/// `UEINT` register
pub const UEINT: *mut u8 = 0xF4 as *mut u8;

/// `UPERRX` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRC16 | 10000 |
/// | COUNTER | 1100000 |
/// | TIMEOUT | 1000 |
/// | PID | 100 |
/// | DATAPID | 10 |
/// | DATATGL | 1 |
pub const UPERRX: *mut u8 = 0xF5 as *mut u8;

/// `UPBCLX` register
pub const UPBCLX: *mut u8 = 0xF6 as *mut u8;

/// `UPBCHX` register
pub const UPBCHX: *mut u8 = 0xF7 as *mut u8;

/// `UPINT` register
pub const UPINT: *mut u8 = 0xF8 as *mut u8;

/// `OTGTCON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VALUE_2 | 111 |
/// | PAGE | 1100000 |
/// | OTGTCON_7 | 10000000 |
pub const OTGTCON: *mut u8 = 0xF9 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ACME: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2AUB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2AUB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2BUB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN2UB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2BUB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const EXCLK: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC5: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC4: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC6: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC7: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const HWBE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRASY: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const JTD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `OTGCON`
pub const VBUSREQ: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `OTGCON`
pub const SRPREQ: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `OTGCON`
pub const VBUSHWC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `OTGCON`
pub const VBUSRQC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `OTGCON`
pub const HNPREQ: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `OTGCON`
pub const SRPSEL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `OTGIEN`
pub const ROLEEXE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `OTGIEN`
pub const STOE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `OTGIEN`
pub const HNPERRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `OTGIEN`
pub const SRPE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `OTGIEN`
pub const BCERRE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `OTGIEN`
pub const VBERRE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `OTGINT`
pub const ROLEEXI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `OTGINT`
pub const STOI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `OTGINT`
pub const SRPI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `OTGINT`
pub const HNPERRI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `OTGINT`
pub const VBERRI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `OTGINT`
pub const BCERRI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `OTGTCON`
pub const VALUE_2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `OTGTCON`
pub const PAGE: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `OTGTCON`
pub const OTGTCON_7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLP: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRUSART1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRUSB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1C: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const WGM2: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const WGM22: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3C: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICES3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICNC3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const CS3: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR2`
pub const TOV2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR3`
pub const ICF3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR3`
pub const TOV3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const TOIE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const ICIE3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const TOIE3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWAMR`
pub const TWAM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const FE1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UPE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const MPCM1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UDRE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const TXC1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const DOR1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const U2X1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const RXC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXB81: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXB81: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXEN1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXEN1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const USBS1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UPM1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UDADDR`
pub const UADD: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `UDADDR`
pub const ADDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UDCON`
pub const RMWKUP: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UDCON`
pub const DETACH: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UDCON`
pub const LSM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UDIEN`
pub const SOFE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UDIEN`
pub const SUSPE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UDIEN`
pub const WAKEUPE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UDIEN`
pub const EORSTE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UDIEN`
pub const UPRSME: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UDIEN`
pub const EORSME: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UDINT`
pub const SOFI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UDINT`
pub const EORSTI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UDINT`
pub const SUSPI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UDINT`
pub const EORSMI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UDINT`
pub const WAKEUPI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UDINT`
pub const UPRSMI: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UDMFN`
pub const FNCERR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UECFG0X`
pub const EPTYPE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UECFG0X`
pub const EPDIR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UECFG1X`
pub const EPSIZE: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `UECFG1X`
pub const EPBK: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `UECONX`
pub const STALLRQC: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UECONX`
pub const STALLRQ: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UECONX`
pub const EPEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UEIENX`
pub const NAKINE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UEIENX`
pub const RXOUTE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UEIENX`
pub const RXSTPE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UEIENX`
pub const STALLEDE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UEIENX`
pub const NAKOUTE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UEIENX`
pub const TXINE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UEINTX`
pub const NAKINI: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UEINTX`
pub const NAKOUTI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UEINTX`
pub const RXSTPI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UEINTX`
pub const TXINI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UEINTX`
pub const STALLEDI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UEINTX`
pub const RXOUTI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UERST`
pub const EPRST: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `UESTA0X`
pub const NBUSYBK: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `UESTA1X`
pub const CURRBK: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `UESTA1X`
pub const CTRLDIR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UHCON`
pub const SOFEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UHCON`
pub const RESUME: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UHCON`
pub const RESET: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UHIEN`
pub const DDISCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UHIEN`
pub const HWUPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UHIEN`
pub const RXRSME: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UHIEN`
pub const HSOFE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UHIEN`
pub const RSTE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UHIEN`
pub const RSMEDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UHIEN`
pub const DCONNE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UHINT`
pub const RXRSMI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UHINT`
pub const UHUPI: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UHINT`
pub const RSTI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UHINT`
pub const DDISCI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UHINT`
pub const RSMEDI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UHINT`
pub const HSOFI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UHINT`
pub const DCONNI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UHWCON`
pub const UIMOD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UHWCON`
pub const UIDE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UHWCON`
pub const UVCONE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UHWCON`
pub const UVREGE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UPCFG0X`
pub const PTYPE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UPCFG0X`
pub const PTOKEN: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UPCFG0X`
pub const PEPNUM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `UPCFG1X`
pub const PBK: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `UPCFG1X`
pub const PSIZE: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `UPCONX`
pub const PEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UPCONX`
pub const PFREEZE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UPCONX`
pub const INMODE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UPERRX`
pub const CRC16: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UPERRX`
pub const COUNTER: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `UPERRX`
pub const TIMEOUT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UPERRX`
pub const PID: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UPERRX`
pub const DATAPID: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UPERRX`
pub const DATATGL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UPIENX`
pub const TXOUTE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UPIENX`
pub const RXINE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UPIENX`
pub const NAKEDE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UPIENX`
pub const RXSTALLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UPIENX`
pub const TXSTPE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UPIENX`
pub const PERRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UPINTX`
pub const RXSTALLI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UPINTX`
pub const TXSTPI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UPINTX`
pub const NAKEDI: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UPINTX`
pub const RXINI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UPINTX`
pub const PERRI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UPINTX`
pub const TXOUTI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UPRST`
pub const PRST: *mut u8 = 0x7F as *mut u8;

/// Bitfield on register `UPSTAX`
pub const NBUSYK: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `USBCON`
pub const VBUSTE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USBCON`
pub const IDTE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `USBCON`
pub const FRZCLK: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `USBCON`
pub const OTGPADE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `USBCON`
pub const HOST: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USBCON`
pub const USBE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USBINT`
pub const IDTI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `USBINT`
pub const VBUSTI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USBSTA`
pub const VBUS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USBSTA`
pub const SPEED: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `USBSTA`
pub const ID: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRW0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRW1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `XMCRB`
pub const XMM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `XMCRB`
pub const XMBK: *mut u8 = 0x80 as *mut u8;

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

/// `CPU_PIN_RELEASE` value group
#[allow(non_upper_case_globals)]
pub mod cpu_pin_release {
   /// None.
   pub const VAL_0x00: u32 = 0x0;
   /// Px7.
   pub const VAL_0x01: u32 = 0x1;
   /// Px7-Px6.
   pub const VAL_0x02: u32 = 0x2;
   /// Px7-Px5.
   pub const VAL_0x03: u32 = 0x3;
   /// Px7-Px4.
   pub const VAL_0x04: u32 = 0x4;
   /// Px7-Px3.
   pub const VAL_0x05: u32 = 0x5;
   /// Px7-Px2.
   pub const VAL_0x06: u32 = 0x6;
   /// Full Port X.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SECTOR_LIMITS3` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sector_limits3 {
   /// LS = N/A, US = 0x1100 - 0xFFFF.
   pub const VAL_0x00: u32 = 0x0;
   /// LS = 0x2100 - 0x1FFF, US = 0x2000 - 0xFFFF.
   pub const VAL_0x01: u32 = 0x1;
   /// LS = 0x2100 - 0x3FFF, US = 0x4000 - 0xFFFF.
   pub const VAL_0x02: u32 = 0x2;
   /// LS = 0x2100 - 0x5FFF, US = 0x6000 - 0xFFFF.
   pub const VAL_0x03: u32 = 0x3;
   /// LS = 0x2100 - 0x7FFF, US = 0x8000 - 0xFFFF.
   pub const VAL_0x04: u32 = 0x4;
   /// LS = 0x2100 - 0x9FFF, US = 0xA000 - 0xFFFF.
   pub const VAL_0x05: u32 = 0x5;
   /// LS = 0x2100 - 0xBFFF, US = 0xC000 - 0xFFFF.
   pub const VAL_0x06: u32 = 0x6;
   /// LS = 0x2100 - 0xDFFF, US = 0xE000 - 0xFFFF.
   pub const VAL_0x07: u32 = 0x7;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection disabled; \[BODLEVEL=111\].
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=2.0 V.
   pub const _2V0: u32 = 0x6;
   /// Brown-out detection at VCC=2.2 V.
   pub const _2V2: u32 = 0x5;
   /// Brown-out detection at VCC=2.4 V.
   pub const _2V4: u32 = 0x4;
   /// Brown-out detection at VCC=2.6 V.
   pub const _2V6: u32 = 0x3;
   /// Brown-out detection at VCC=3.4 V.
   pub const _3V4: u32 = 0x2;
   /// Brown-out detection at VCC=3.5 V.
   pub const _3V5: u32 = 0x1;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x0;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=512 words start address=$7F00.
   pub const _512W_7F00: u32 = 0x3;
   /// Boot Flash size=1024 words start address=$7E00.
   pub const _1024W_7E00: u32 = 0x2;
   /// Boot Flash size=2408 words start address=$7C00.
   pub const _2408W_7C00: u32 = 0x1;
   /// Boot Flash size=4096 words start address=$7800.
   pub const _4096W_7800: u32 = 0x0;
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
   /// Ext. Clock; Start-up time: 6 CK + 4.1 ms.
   pub const EXTCLK_6CK_4MS1: u32 = 0x10;
   /// Ext. Clock; Start-up time: 6 CK + 65 ms.
   pub const EXTCLK_6CK_65MS: u32 = 0x20;
   /// Int. RC Osc.; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_6CK_0MS: u32 = 0x2;
   /// Int. RC Osc.; Start-up time: 6 CK + 4.1 ms.
   pub const INTRCOSC_6CK_4MS1: u32 = 0x12;
   /// Int. RC Osc.; Start-up time: 6 CK + 65 ms.
   pub const INTRCOSC_6CK_65MS: u32 = 0x22;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms; Int. Cap.
   pub const EXTLOFXTAL_32KCK_0MS_INTCAP: u32 = 0x7;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms; Int. Cap.
   pub const EXTLOFXTAL_32KCK_4MS1_INTCAP: u32 = 0x17;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms; Int. Cap.
   pub const EXTLOFXTAL_32KCK_65MS_INTCAP: u32 = 0x27;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms; Int. Cap.
   pub const EXTLOFXTAL_1KCK_0MS_INTCAP: u32 = 0x6;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms; Int. Cap.
   pub const EXTLOFXTAL_1KCK_4MS1_INTCAP: u32 = 0x16;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms; Int. Cap.
   pub const EXTLOFXTAL_1KCK_65MS_INTCAP: u32 = 0x26;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms.
   pub const EXTLOFXTAL_32KCK_0MS: u32 = 0x5;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms.
   pub const EXTLOFXTAL_32KCK_4MS1: u32 = 0x15;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms.
   pub const EXTLOFXTAL_32KCK_65MS: u32 = 0x25;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTAL_1KCK_0MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms.
   pub const EXTLOFXTAL_1KCK_4MS1: u32 = 0x14;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms.
   pub const EXTLOFXTAL_1KCK_65MS: u32 = 0x24;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1: u32 = 0x8;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_65MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS: u32 = 0x28;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1: u32 = 0x38;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS: u32 = 0x39;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_4MS1: u32 = 0xA;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_65MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_0MS: u32 = 0x2A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1: u32 = 0x3A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_65MS: u32 = 0x3B;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_4MS1: u32 = 0xC;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_65MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_0MS: u32 = 0x2C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_4MS1: u32 = 0x3C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_65MS: u32 = 0x3D;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_4MS1: u32 = 0xE;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_65MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_4MS1: u32 = 0x3E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_65MS: u32 = 0xF;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_0MS: u32 = 0x1F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_4MS1: u32 = 0x2F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_65MS: u32 = 0x3F;
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

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
}

/// `PLL_INPUT_PRESCALER` value group
#[allow(non_upper_case_globals)]
pub mod pll_input_prescaler {
   /// Clock/4.
   pub const VAL_0x03: u32 = 0x3;
   /// Clock/8.
   pub const VAL_0x05: u32 = 0x5;
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

