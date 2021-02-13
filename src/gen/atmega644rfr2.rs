//! The AVR ATmega644RFR2 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega644RFR2-ZU |  | VQFN48 | -40°C - 85°C | 1.8V - 3.6V | 16 MHz |
//! | ATmega644RFR2-ZUR |  | VQFN48 | -40°C - 85°C | 1.8V - 3.6V | 16 MHz |
//! | ATmega644RFR2-ZF |  | VQFN48 | -40°C - 125°C | 1.8V - 3.6V | 16 MHz |
//! | ATmega644RFR2-ZFR |  | VQFN48 | -40°C - 125°C | 1.8V - 3.6V | 16 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOUT | 1000000 |
/// | CKSEL_SUT | 111111 |
/// | CKDIV8 | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

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

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCDEN | 10000000 |
/// | BOOTRST | 1 |
/// | SPIEN | 100000 |
/// | WDTON | 10000 |
/// | EESAVE | 1000 |
/// | JTAGEN | 1000000 |
/// | BOOTSZ | 110 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 111 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Port A Input Pins Address.
pub const PINA: *mut u8 = 0x20 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x21 as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x22 as *mut u8;

/// Port B Input Pins Address.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

/// Port C Input Pins Address.
pub const PINC: *mut u8 = 0x26 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x27 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Port D Input Pins Address.
pub const PIND: *mut u8 = 0x29 as *mut u8;

/// Port D Data Direction Register.
pub const DDRD: *mut u8 = 0x2A as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x2B as *mut u8;

/// Port E Input Pins Address.
pub const PINE: *mut u8 = 0x2C as *mut u8;

/// Port E Data Direction Register.
pub const DDRE: *mut u8 = 0x2D as *mut u8;

/// Port E Data Register.
pub const PORTE: *mut u8 = 0x2E as *mut u8;

/// Port F Input Pins Address.
pub const PINF: *mut u8 = 0x2F as *mut u8;

/// Port F Data Direction Register.
pub const DDRF: *mut u8 = 0x30 as *mut u8;

/// Port F Data Register.
pub const PORTF: *mut u8 = 0x31 as *mut u8;

/// Port G Input Pins Address.
pub const PING: *mut u8 = 0x32 as *mut u8;

/// Port G Data Direction Register.
pub const DDRG: *mut u8 = 0x33 as *mut u8;

/// Port G Data Register.
pub const PORTG: *mut u8 = 0x34 as *mut u8;

/// Timer/Counter0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0A | 10 |
/// | OCF0B | 100 |
/// | TOV0 | 1 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1C | 1000 |
/// | TOV1 | 1 |
/// | ICF1 | 100000 |
/// | OCF1A | 10 |
/// | OCF1B | 100 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV2 | 1 |
/// | OCF2A | 10 |
/// | OCF2B | 100 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter3 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF3 | 100000 |
/// | OCF3A | 10 |
/// | OCF3B | 100 |
/// | TOV3 | 1 |
/// | OCF3C | 1000 |
pub const TIFR3: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter4 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF4C | 1000 |
/// | TOV4 | 1 |
/// | ICF4 | 100000 |
/// | OCF4A | 10 |
/// | OCF4B | 100 |
pub const TIFR4: *mut u8 = 0x39 as *mut u8;

/// Timer/Counter5 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF5A | 10 |
/// | OCF5B | 100 |
/// | ICF5 | 100000 |
/// | TOV5 | 1 |
/// | OCF5C | 1000 |
pub const TIFR5: *mut u8 = 0x3A as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 111 |
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
/// | GPIOR01 | 10 |
/// | GPIOR00 | 1 |
/// | GPIOR07 | 10000000 |
/// | GPIOR03 | 1000 |
/// | GPIOR04 | 10000 |
/// | GPIOR05 | 100000 |
/// | GPIOR02 | 100 |
/// | GPIOR06 | 1000000 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPM | 110000 |
/// | EEPE | 10 |
/// | EEMPE | 100 |
/// | EERE | 1 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer Counter Control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSRASY | 10 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0B | 110000 |
/// | WGM0 | 11 |
/// | COM0A | 11000000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS0 | 111 |
/// | WGM02 | 1000 |
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0 Register.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register B.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | DORD | 100000 |
/// | CPOL | 1000 |
/// | CPHA | 100 |
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

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACIC | 100 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// On-Chip Debug Register.
pub const OCDR: *mut u8 = 0x51 as *mut u8;

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
/// | JTRF | 10000 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | BORF | 100 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | RWWSB | 1000000 |
/// | PGERS | 10 |
/// | BLBSET | 1000 |
/// | RWWSRE | 10000 |
/// | SPMEN | 1 |
/// | SIGRD | 100000 |
/// | SPMIE | 10000000 |
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
/// | S | 10000 |
/// | N | 100 |
/// | V | 1000 |
/// | C | 1 |
/// | H | 100000 |
/// | I | 10000000 |
/// | T | 1000000 |
/// | Z | 10 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
/// | WDIF | 10000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRRAM2 | 100 |
/// | PRRAM1 | 10 |
/// | PRRAM3 | 1000 |
/// | PRRAM0 | 1 |
pub const PRR2: *mut u8 = 0x63 as *mut u8;

/// Power Reduction Register0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRPGA | 10000 |
/// | PRSPI | 100 |
/// | PRTWI | 10000000 |
/// | PRTIM1 | 1000 |
/// | PRUSART0 | 10 |
/// | PRTIM0 | 100000 |
/// | PRADC | 1 |
/// | PRTIM2 | 1000000 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Power Reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM5 | 100000 |
/// | PRUSART1 | 1 |
/// | PRTRX24 | 1000000 |
/// | PRTIM3 | 1000 |
/// | PRTIM4 | 10000 |
pub const PRR1: *mut u8 = 0x65 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// Reference Voltage Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL_FINE | 1111000 |
/// | BGCAL | 111 |
pub const BGCR: *mut u8 = 0x67 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 111 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
/// | ISC3 | 11000000 |
/// | ISC2 | 110000 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC5 | 1100 |
/// | ISC7 | 11000000 |
/// | ISC4 | 11 |
/// | ISC6 | 110000 |
pub const EICRB: *mut u8 = 0x6A as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Pin Change Mask Register 2.
pub const PCMSK2: *mut u8 = 0x6D as *mut u8;

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
/// | OCIE1B | 100 |
/// | ICIE1 | 100000 |
/// | OCIE1C | 1000 |
/// | TOIE1 | 1 |
/// | OCIE1A | 10 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE2 | 1 |
/// | OCIE2A | 10 |
/// | OCIE2B | 100 |
pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

/// Timer/Counter3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE3 | 1 |
/// | ICIE3 | 100000 |
/// | OCIE3A | 10 |
/// | OCIE3C | 1000 |
/// | OCIE3B | 100 |
pub const TIMSK3: *mut u8 = 0x71 as *mut u8;

/// Timer/Counter4 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE4B | 100 |
/// | ICIE4 | 100000 |
/// | OCIE4A | 10 |
/// | OCIE4C | 1000 |
/// | TOIE4 | 1 |
pub const TIMSK4: *mut u8 = 0x72 as *mut u8;

/// Timer/Counter5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE5A | 10 |
/// | OCIE5C | 1000 |
/// | ICIE5 | 100000 |
/// | OCIE5B | 100 |
/// | TOIE5 | 1 |
pub const TIMSK5: *mut u8 = 0x73 as *mut u8;

/// Flash Extended-Mode Control-Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ENEAM | 1000000 |
/// | AEAM | 110000 |
pub const NEMCR: *mut u8 = 0x75 as *mut u8;

/// The ADC Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADSUT | 11111 |
/// | ADTHT | 11000000 |
pub const ADCSRC: *mut u8 = 0x77 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADIE | 1000 |
/// | ADPS | 111 |
/// | ADATE | 100000 |
/// | ADEN | 10000000 |
/// | ADSC | 1000000 |
/// | ADIF | 10000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// The ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVDDOK | 10000000 |
/// | MUX5 | 1000 |
/// | ACCH | 10000 |
/// | REFOK | 100000 |
/// | ADTS | 111 |
/// | ACME | 1000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC Multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 11111 |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC10D | 100 |
/// | ADC11D | 1000 |
/// | ADC12D | 10000 |
/// | ADC14D | 1000000 |
/// | ADC15D | 10000000 |
/// | ADC8D | 1 |
/// | ADC9D | 10 |
/// | ADC13D | 100000 |
pub const DIDR2: *mut u8 = 0x7D as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC4D | 10000 |
/// | ADC5D | 100000 |
/// | ADC0D | 1 |
/// | ADC6D | 1000000 |
/// | ADC3D | 1000 |
/// | ADC2D | 100 |
/// | ADC1D | 10 |
/// | ADC7D | 10000000 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN1D | 10 |
/// | AIN0D | 1 |
pub const DIDR1: *mut u8 = 0x7F as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1C | 1100 |
/// | COM1B | 110000 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
/// | ICES1 | 1000000 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1B | 1000000 |
/// | FOC1C | 100000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register A  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register A  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter1 Output Compare Register C  Bytes.
pub const OCR1C: *mut u16 = 0x8C as *mut u16;

/// Timer/Counter1 Output Compare Register C  Bytes low byte.
pub const OCR1CL: *mut u8 = 0x8C as *mut u8;

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
/// | CS3 | 111 |
/// | ICNC3 | 10000000 |
/// | ICES3 | 1000000 |
pub const TCCR3B: *mut u8 = 0x91 as *mut u8;

/// Timer/Counter3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3A | 10000000 |
/// | FOC3C | 100000 |
/// | FOC3B | 1000000 |
pub const TCCR3C: *mut u8 = 0x92 as *mut u8;

/// Timer/Counter3  Bytes.
pub const TCNT3: *mut u16 = 0x94 as *mut u16;

/// Timer/Counter3  Bytes low byte.
pub const TCNT3L: *mut u8 = 0x94 as *mut u8;

/// Timer/Counter3  Bytes high byte.
pub const TCNT3H: *mut u8 = 0x95 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes low byte.
pub const ICR3L: *mut u8 = 0x96 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes.
pub const ICR3: *mut u16 = 0x96 as *mut u16;

/// Timer/Counter3 Input Capture Register  Bytes high byte.
pub const ICR3H: *mut u8 = 0x97 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes.
pub const OCR3A: *mut u16 = 0x98 as *mut u16;

/// Timer/Counter3 Output Compare Register A  Bytes low byte.
pub const OCR3AL: *mut u8 = 0x98 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes high byte.
pub const OCR3AH: *mut u8 = 0x99 as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes low byte.
pub const OCR3BL: *mut u8 = 0x9A as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes.
pub const OCR3B: *mut u16 = 0x9A as *mut u16;

/// Timer/Counter3 Output Compare Register B  Bytes high byte.
pub const OCR3BH: *mut u8 = 0x9B as *mut u8;

/// Timer/Counter3 Output Compare Register C  Bytes low byte.
pub const OCR3CL: *mut u8 = 0x9C as *mut u8;

/// Timer/Counter3 Output Compare Register C  Bytes.
pub const OCR3C: *mut u16 = 0x9C as *mut u16;

/// Timer/Counter3 Output Compare Register C  Bytes high byte.
pub const OCR3CH: *mut u8 = 0x9D as *mut u8;

/// Timer/Counter4 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM4A | 11000000 |
/// | COM4C | 1100 |
/// | COM4B | 110000 |
pub const TCCR4A: *mut u8 = 0xA0 as *mut u8;

/// Timer/Counter4 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC4 | 10000000 |
/// | ICES4 | 1000000 |
/// | CS4 | 111 |
pub const TCCR4B: *mut u8 = 0xA1 as *mut u8;

/// Timer/Counter4 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC4A | 10000000 |
/// | FOC4B | 1000000 |
/// | FOC4C | 100000 |
pub const TCCR4C: *mut u8 = 0xA2 as *mut u8;

/// Timer/Counter4  Bytes low byte.
pub const TCNT4L: *mut u8 = 0xA4 as *mut u8;

/// Timer/Counter4  Bytes.
pub const TCNT4: *mut u16 = 0xA4 as *mut u16;

/// Timer/Counter4  Bytes high byte.
pub const TCNT4H: *mut u8 = 0xA5 as *mut u8;

/// Timer/Counter4 Input Capture Register  Bytes.
pub const ICR4: *mut u16 = 0xA6 as *mut u16;

/// Timer/Counter4 Input Capture Register  Bytes low byte.
pub const ICR4L: *mut u8 = 0xA6 as *mut u8;

/// Timer/Counter4 Input Capture Register  Bytes high byte.
pub const ICR4H: *mut u8 = 0xA7 as *mut u8;

/// Timer/Counter4 Output Compare Register A  Bytes low byte.
pub const OCR4AL: *mut u8 = 0xA8 as *mut u8;

/// Timer/Counter4 Output Compare Register A  Bytes.
pub const OCR4A: *mut u16 = 0xA8 as *mut u16;

/// Timer/Counter4 Output Compare Register A  Bytes high byte.
pub const OCR4AH: *mut u8 = 0xA9 as *mut u8;

/// Timer/Counter4 Output Compare Register B  Bytes low byte.
pub const OCR4BL: *mut u8 = 0xAA as *mut u8;

/// Timer/Counter4 Output Compare Register B  Bytes.
pub const OCR4B: *mut u16 = 0xAA as *mut u16;

/// Timer/Counter4 Output Compare Register B  Bytes high byte.
pub const OCR4BH: *mut u8 = 0xAB as *mut u8;

/// Timer/Counter4 Output Compare Register C  Bytes low byte.
pub const OCR4CL: *mut u8 = 0xAC as *mut u8;

/// Timer/Counter4 Output Compare Register C  Bytes.
pub const OCR4C: *mut u16 = 0xAC as *mut u16;

/// Timer/Counter4 Output Compare Register C  Bytes high byte.
pub const OCR4CH: *mut u8 = 0xAD as *mut u8;

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
/// | FOC2B | 1000000 |
/// | WGM22 | 1000 |
/// | FOC2A | 10000000 |
/// | CS2 | 111 |
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
/// | AS2 | 100000 |
/// | TCN2UB | 10000 |
/// | TCR2BUB | 1 |
/// | OCR2AUB | 1000 |
/// | OCR2BUB | 100 |
/// | EXCLK | 1000000 |
/// | EXCLKAMR | 10000000 |
/// | TCR2AUB | 10 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

/// TWI Bit Rate Register.
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

/// TWI (Slave) Address Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWA | 11111110 |
/// | TWGCE | 1 |
pub const TWAR: *mut u8 = 0xBA as *mut u8;

/// TWI Data Register.
pub const TWDR: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWEA | 1000000 |
/// | TWSTO | 10000 |
/// | TWIE | 1 |
/// | TWINT | 10000000 |
/// | TWWC | 1000 |
/// | TWEN | 100 |
/// | TWSTA | 100000 |
pub const TWCR: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM | 11111110 |
pub const TWAMR: *mut u8 = 0xBD as *mut u8;

/// Transceiver Interrupt Enable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MAF_2_AMI_EN | 1000 |
/// | TX_START_EN | 1 |
/// | MAF_3_AMI_EN | 10000 |
/// | MAF_1_AMI_EN | 100 |
/// | MAF_0_AMI_EN | 10 |
pub const IRQ_MASK1: *mut u8 = 0xBE as *mut u8;

/// Transceiver Interrupt Status Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MAF_3_AMI | 10000 |
/// | MAF_1_AMI | 100 |
/// | TX_START | 1 |
/// | MAF_2_AMI | 1000 |
/// | MAF_0_AMI | 10 |
pub const IRQ_STATUS1: *mut u8 = 0xBF as *mut u8;

/// USART0 MSPIM Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXC0 | 1000000 |
/// | UDRE0 | 100000 |
/// | RXC0 | 10000000 |
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

/// USART0 MSPIM Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UDRIE0 | 100000 |
/// | TXEN0 | 1000 |
/// | RXEN0 | 10000 |
/// | RXCIE0 | 10000000 |
/// | TXCIE0 | 1000000 |
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

/// USART0 MSPIM Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UDORD0 | 100 |
/// | UCPOL0 | 1 |
/// | UCPHA0 | 10 |
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

/// USART0 Baud Rate Register  Bytes low byte.
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

/// USART0 Baud Rate Register  Bytes.
pub const UBRR0: *mut u16 = 0xC4 as *mut u16;

/// USART0 Baud Rate Register  Bytes high byte.
pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

/// USART0 I/O Data Register.
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

/// USART1 MSPIM Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC1 | 10000000 |
/// | UDRE1 | 100000 |
/// | TXC1 | 1000000 |
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART1 MSPIM Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXCIE1 | 1000000 |
/// | UDRIE1 | 100000 |
/// | TXEN1 | 1000 |
/// | RXEN1 | 10000 |
/// | RXCIE1 | 10000000 |
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART1 MSPIM Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCPHA1 | 10 |
/// | UDORD1 | 100 |
/// | UCPOL1 | 1 |
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART1 Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART1 Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART1 Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART1 I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// Symbol Counter Received Frame Timestamp Register LL-Byte.
pub const SCRSTRLL: *mut u8 = 0xD7 as *mut u8;

/// Symbol Counter Received Frame Timestamp Register LH-Byte.
pub const SCRSTRLH: *mut u8 = 0xD8 as *mut u8;

/// Symbol Counter Received Frame Timestamp Register HL-Byte.
pub const SCRSTRHL: *mut u8 = 0xD9 as *mut u8;

/// Symbol Counter Received Frame Timestamp Register HH-Byte.
pub const SCRSTRHH: *mut u8 = 0xDA as *mut u8;

/// Symbol Counter Compare Source Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCCS2 | 1100 |
/// | SCCS3 | 110000 |
/// | SCCS1 | 11 |
pub const SCCSR: *mut u8 = 0xDB as *mut u8;

/// Symbol Counter Control Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCCMP | 111 |
/// | SCRES | 10000000 |
/// | SCTSE | 1000 |
/// | SCEN | 100000 |
/// | SCMBTS | 1000000 |
/// | SCCKSEL | 10000 |
pub const SCCR0: *mut u8 = 0xDC as *mut u8;

/// Symbol Counter Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCENBO | 1 |
/// | SCCKDIV | 11100 |
/// | SCBTSM | 100000 |
/// | SCEECLK | 10 |
pub const SCCR1: *mut u8 = 0xDD as *mut u8;

/// Symbol Counter Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCBSY | 1 |
pub const SCSR: *mut u8 = 0xDE as *mut u8;

/// Symbol Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IRQMBO | 10000 |
/// | IRQMOF | 1000 |
/// | IRQMCP | 111 |
pub const SCIRQM: *mut u8 = 0xDF as *mut u8;

/// Symbol Counter Interrupt Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IRQSCP | 111 |
/// | IRQSBO | 10000 |
/// | IRQSOF | 1000 |
pub const SCIRQS: *mut u8 = 0xE0 as *mut u8;

/// Symbol Counter Register LL-Byte.
pub const SCCNTLL: *mut u8 = 0xE1 as *mut u8;

/// Symbol Counter Register LH-Byte.
pub const SCCNTLH: *mut u8 = 0xE2 as *mut u8;

/// Symbol Counter Register HL-Byte.
pub const SCCNTHL: *mut u8 = 0xE3 as *mut u8;

/// Symbol Counter Register HH-Byte.
pub const SCCNTHH: *mut u8 = 0xE4 as *mut u8;

/// Symbol Counter Beacon Timestamp Register LL-Byte.
pub const SCBTSRLL: *mut u8 = 0xE5 as *mut u8;

/// Symbol Counter Beacon Timestamp Register LH-Byte.
pub const SCBTSRLH: *mut u8 = 0xE6 as *mut u8;

/// Symbol Counter Beacon Timestamp Register HL-Byte.
pub const SCBTSRHL: *mut u8 = 0xE7 as *mut u8;

/// Symbol Counter Beacon Timestamp Register HH-Byte.
pub const SCBTSRHH: *mut u8 = 0xE8 as *mut u8;

/// Symbol Counter Frame Timestamp Register LL-Byte.
pub const SCTSRLL: *mut u8 = 0xE9 as *mut u8;

/// Symbol Counter Frame Timestamp Register LH-Byte.
pub const SCTSRLH: *mut u8 = 0xEA as *mut u8;

/// Symbol Counter Frame Timestamp Register HL-Byte.
pub const SCTSRHL: *mut u8 = 0xEB as *mut u8;

/// Symbol Counter Frame Timestamp Register HH-Byte.
pub const SCTSRHH: *mut u8 = 0xEC as *mut u8;

/// Symbol Counter Output Compare Register 3 LL-Byte.
pub const SCOCR3LL: *mut u8 = 0xED as *mut u8;

/// Symbol Counter Output Compare Register 3 LH-Byte.
pub const SCOCR3LH: *mut u8 = 0xEE as *mut u8;

/// Symbol Counter Output Compare Register 3 HL-Byte.
pub const SCOCR3HL: *mut u8 = 0xEF as *mut u8;

/// Symbol Counter Output Compare Register 3 HH-Byte.
pub const SCOCR3HH: *mut u8 = 0xF0 as *mut u8;

/// Symbol Counter Output Compare Register 2 LL-Byte.
pub const SCOCR2LL: *mut u8 = 0xF1 as *mut u8;

/// Symbol Counter Output Compare Register 2 LH-Byte.
pub const SCOCR2LH: *mut u8 = 0xF2 as *mut u8;

/// Symbol Counter Output Compare Register 2 HL-Byte.
pub const SCOCR2HL: *mut u8 = 0xF3 as *mut u8;

/// Symbol Counter Output Compare Register 2 HH-Byte.
pub const SCOCR2HH: *mut u8 = 0xF4 as *mut u8;

/// Symbol Counter Output Compare Register 1 LL-Byte.
pub const SCOCR1LL: *mut u8 = 0xF5 as *mut u8;

/// Symbol Counter Output Compare Register 1 LH-Byte.
pub const SCOCR1LH: *mut u8 = 0xF6 as *mut u8;

/// Symbol Counter Output Compare Register 1 HL-Byte.
pub const SCOCR1HL: *mut u8 = 0xF7 as *mut u8;

/// Symbol Counter Output Compare Register 1 HH-Byte.
pub const SCOCR1HH: *mut u8 = 0xF8 as *mut u8;

/// Symbol Counter Transmit Frame Timestamp Register LL-Byte.
pub const SCTSTRLL: *mut u8 = 0xF9 as *mut u8;

/// Symbol Counter Transmit Frame Timestamp Register LH-Byte.
pub const SCTSTRLH: *mut u8 = 0xFA as *mut u8;

/// Symbol Counter Transmit Frame Timestamp Register HL-Byte.
pub const SCTSTRHL: *mut u8 = 0xFB as *mut u8;

/// Symbol Counter Transmit Frame Timestamp Register HH-Byte.
pub const SCTSTRHH: *mut u8 = 0xFC as *mut u8;

/// Multiple Address Filter Configuration Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MAF0EN | 1 |
/// | MAF2EN | 100 |
/// | MAF3EN | 1000 |
/// | MAF1EN | 10 |
pub const MAFCR0: *mut u8 = 0x10C as *mut u8;

/// Multiple Address Filter Configuration Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AACK_2_I_AM_COORD | 10000 |
/// | AACK_3_SET_PD | 10000000 |
/// | AACK_0_I_AM_COORD | 1 |
/// | AACK_1_I_AM_COORD | 100 |
/// | AACK_1_SET_PD | 1000 |
/// | AACK_0_SET_PD | 10 |
/// | AACK_2_SET_PD | 100000 |
/// | AACK_3_I_AM_COORD | 1000000 |
pub const MAFCR1: *mut u8 = 0x10D as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 0 (Low Byte).
pub const MAFSA0L: *mut u8 = 0x10E as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 0 (High Byte).
pub const MAFSA0H: *mut u8 = 0x10F as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 0 (Low Byte).
pub const MAFPA0L: *mut u8 = 0x110 as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 0 (High Byte).
pub const MAFPA0H: *mut u8 = 0x111 as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 1 (Low Byte).
pub const MAFSA1L: *mut u8 = 0x112 as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 1 (High Byte).
pub const MAFSA1H: *mut u8 = 0x113 as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 1 (Low Byte).
pub const MAFPA1L: *mut u8 = 0x114 as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 1 (High Byte).
pub const MAFPA1H: *mut u8 = 0x115 as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 2 (Low Byte).
pub const MAFSA2L: *mut u8 = 0x116 as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 2 (High Byte).
pub const MAFSA2H: *mut u8 = 0x117 as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 2 (Low Byte).
pub const MAFPA2L: *mut u8 = 0x118 as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 2 (High Byte).
pub const MAFPA2H: *mut u8 = 0x119 as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 3 (Low Byte).
pub const MAFSA3L: *mut u8 = 0x11A as *mut u8;

/// Transceiver MAC Short Address Register for Frame Filter 3 (High Byte).
pub const MAFSA3H: *mut u8 = 0x11B as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 3 (Low Byte).
pub const MAFPA3L: *mut u8 = 0x11C as *mut u8;

/// Transceiver Personal Area Network ID Register for Frame Filter 3 (High Byte).
pub const MAFPA3H: *mut u8 = 0x11D as *mut u8;

/// Timer/Counter5 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM5A | 11000000 |
/// | COM5B | 110000 |
/// | COM5C | 1100 |
pub const TCCR5A: *mut u8 = 0x120 as *mut u8;

/// Timer/Counter5 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC5 | 10000000 |
/// | CS5 | 111 |
/// | ICES5 | 1000000 |
pub const TCCR5B: *mut u8 = 0x121 as *mut u8;

/// Timer/Counter5 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC5C | 100000 |
/// | FOC5B | 1000000 |
/// | FOC5A | 10000000 |
pub const TCCR5C: *mut u8 = 0x122 as *mut u8;

/// Timer/Counter5  Bytes low byte.
pub const TCNT5L: *mut u8 = 0x124 as *mut u8;

/// Timer/Counter5  Bytes.
pub const TCNT5: *mut u16 = 0x124 as *mut u16;

/// Timer/Counter5  Bytes high byte.
pub const TCNT5H: *mut u8 = 0x125 as *mut u8;

/// Timer/Counter5 Input Capture Register  Bytes low byte.
pub const ICR5L: *mut u8 = 0x126 as *mut u8;

/// Timer/Counter5 Input Capture Register  Bytes.
pub const ICR5: *mut u16 = 0x126 as *mut u16;

/// Timer/Counter5 Input Capture Register  Bytes high byte.
pub const ICR5H: *mut u8 = 0x127 as *mut u8;

/// Timer/Counter5 Output Compare Register A  Bytes.
pub const OCR5A: *mut u16 = 0x128 as *mut u16;

/// Timer/Counter5 Output Compare Register A  Bytes low byte.
pub const OCR5AL: *mut u8 = 0x128 as *mut u8;

/// Timer/Counter5 Output Compare Register A  Bytes high byte.
pub const OCR5AH: *mut u8 = 0x129 as *mut u8;

/// Timer/Counter5 Output Compare Register B  Bytes low byte.
pub const OCR5BL: *mut u8 = 0x12A as *mut u8;

/// Timer/Counter5 Output Compare Register B  Bytes.
pub const OCR5B: *mut u16 = 0x12A as *mut u16;

/// Timer/Counter5 Output Compare Register B  Bytes high byte.
pub const OCR5BH: *mut u8 = 0x12B as *mut u8;

/// Timer/Counter5 Output Compare Register C  Bytes.
pub const OCR5C: *mut u16 = 0x12C as *mut u16;

/// Timer/Counter5 Output Compare Register C  Bytes low byte.
pub const OCR5CL: *mut u8 = 0x12C as *mut u8;

/// Timer/Counter5 Output Compare Register C  Bytes high byte.
pub const OCR5CH: *mut u8 = 0x12D as *mut u8;

/// Low Leakage Voltage Regulator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LLTCO | 100 |
/// | LLCAL | 1000 |
/// | LLDONE | 100000 |
/// | LLCOMP | 10000 |
/// | LLENCAL | 1 |
/// | LLSHORT | 10 |
pub const LLCR: *mut u8 = 0x12F as *mut u8;

/// Low Leakage Voltage Regulator Data Register (Low-Byte).
pub const LLDRL: *mut u8 = 0x130 as *mut u8;

/// Low Leakage Voltage Regulator Data Register (High-Byte).
pub const LLDRH: *mut u8 = 0x131 as *mut u8;

/// Data Retention Configuration Register #3.
pub const DRTRAM3: *mut u8 = 0x132 as *mut u8;

/// Data Retention Configuration Register #2.
pub const DRTRAM2: *mut u8 = 0x133 as *mut u8;

/// Data Retention Configuration Register #1.
pub const DRTRAM1: *mut u8 = 0x134 as *mut u8;

/// Data Retention Configuration Register #0.
pub const DRTRAM0: *mut u8 = 0x135 as *mut u8;

/// Port Driver Strength Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PFDRV | 11000000 |
/// | PEDRV | 110000 |
/// | PDDRV | 1100 |
/// | PBDRV | 11 |
pub const DPDS0: *mut u8 = 0x136 as *mut u8;

/// Port Driver Strength Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGDRV | 11 |
pub const DPDS1: *mut u8 = 0x137 as *mut u8;

/// Power Amplifier Ramp up/down Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PARDFI | 10 |
/// | PARUFI | 1 |
/// | PALTU | 11100 |
/// | PALTD | 11100000 |
pub const PARCR: *mut u8 = 0x138 as *mut u8;

/// Transceiver Pin Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SLPTR | 10 |
/// | TRXRST | 1 |
pub const TRXPR: *mut u8 = 0x139 as *mut u8;

/// AES Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AES_DIR | 1000 |
/// | AES_REQUEST | 10000000 |
/// | AES_IM | 100 |
/// | AES_MODE | 100000 |
pub const AES_CTRL: *mut u8 = 0x13C as *mut u8;

/// AES Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AES_DONE | 1 |
/// | AES_ER | 10000000 |
pub const AES_STATUS: *mut u8 = 0x13D as *mut u8;

/// AES Plain and Cipher Text Buffer Register.
pub const AES_STATE: *mut u8 = 0x13E as *mut u8;

/// AES Encryption and Decryption Key Buffer Register.
pub const AES_KEY: *mut u8 = 0x13F as *mut u8;

/// Transceiver Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCA_DONE | 10000000 |
/// | TST_STATUS | 100000 |
/// | CCA_STATUS | 1000000 |
pub const TRX_STATUS: *mut u8 = 0x141 as *mut u8;

/// Transceiver State Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TRAC_STATUS | 11100000 |
/// | TRX_CMD | 11111 |
pub const TRX_STATE: *mut u8 = 0x142 as *mut u8;

/// Reserved.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PMU_START | 100000 |
/// | PMU_EN | 1000000 |
/// | Res7 | 10000000 |
/// | PMU_IF_INV | 10000 |
pub const TRX_CTRL_0: *mut u8 = 0x143 as *mut u8;

/// Transceiver Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TX_AUTO_CRC_ON | 100000 |
/// | IRQ_2_EXT_EN | 1000000 |
/// | PLL_TX_FLT | 10000 |
/// | PA_EXT_EN | 10000000 |
pub const TRX_CTRL_1: *mut u8 = 0x144 as *mut u8;

/// Transceiver Transmit Power Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TX_PWR | 1111 |
pub const PHY_TX_PWR: *mut u8 = 0x145 as *mut u8;

/// Receiver Signal Strength Indicator Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSSI | 11111 |
/// | RND_VALUE | 1100000 |
/// | RX_CRC_VALID | 10000000 |
pub const PHY_RSSI: *mut u8 = 0x146 as *mut u8;

/// Transceiver Energy Detection Level Register.
pub const PHY_ED_LEVEL: *mut u8 = 0x147 as *mut u8;

/// Transceiver Clear Channel Assessment (CCA) Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCA_REQUEST | 10000000 |
/// | CHANNEL | 11111 |
/// | CCA_MODE | 1100000 |
pub const PHY_CC_CCA: *mut u8 = 0x148 as *mut u8;

/// Transceiver CCA Threshold Setting Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCA_CS_THRES | 11110000 |
/// | CCA_ED_THRES | 1111 |
pub const CCA_THRES: *mut u8 = 0x149 as *mut u8;

/// Transceiver Receive Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PDT_THRES | 1111 |
pub const RX_CTRL: *mut u8 = 0x14A as *mut u8;

/// Start of Frame Delimiter Value Register.
pub const SFD_VALUE: *mut u8 = 0x14B as *mut u8;

/// Transceiver Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RX_SAFE_MODE | 10000000 |
/// | OQPSK_DATA_RATE | 11 |
pub const TRX_CTRL_2: *mut u8 = 0x14C as *mut u8;

/// Antenna Diversity Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ANT_DIV_EN | 1000 |
/// | ANT_CTRL | 11 |
/// | ANT_EXT_SW_EN | 100 |
/// | ANT_SEL | 10000000 |
pub const ANT_DIV: *mut u8 = 0x14D as *mut u8;

/// Transceiver Interrupt Enable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RX_START_EN | 100 |
/// | PLL_UNLOCK_EN | 10 |
/// | AWAKE_EN | 10000000 |
/// | CCA_ED_DONE_EN | 10000 |
/// | RX_END_EN | 1000 |
/// | PLL_LOCK_EN | 1 |
/// | TX_END_EN | 1000000 |
/// | AMI_EN | 100000 |
pub const IRQ_MASK: *mut u8 = 0x14E as *mut u8;

/// Transceiver Interrupt Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CCA_ED_DONE | 10000 |
/// | AWAKE | 10000000 |
/// | RX_START | 100 |
/// | PLL_LOCK | 1 |
/// | TX_END | 1000000 |
/// | AMI | 100000 |
/// | PLL_UNLOCK | 10 |
/// | RX_END | 1000 |
pub const IRQ_STATUS: *mut u8 = 0x14F as *mut u8;

/// Voltage Regulator Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DVREG_EXT | 1000 |
/// | DVDD_OK | 100 |
/// | AVREG_EXT | 10000000 |
/// | AVDD_OK | 1000000 |
pub const VREG_CTRL: *mut u8 = 0x150 as *mut u8;

/// Battery Monitor Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BATMON_VTH | 1111 |
/// | BATMON_HR | 10000 |
/// | BAT_LOW | 10000000 |
/// | BAT_LOW_EN | 1000000 |
/// | BATMON_OK | 100000 |
pub const BATMON: *mut u8 = 0x151 as *mut u8;

/// Crystal Oscillator Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XTAL_TRIM | 1111 |
/// | XTAL_MODE | 11110000 |
pub const XOSC_CTRL: *mut u8 = 0x152 as *mut u8;

/// Channel Control Register 0.
pub const CC_CTRL_0: *mut u8 = 0x153 as *mut u8;

/// Channel Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CC_BAND | 1111 |
pub const CC_CTRL_1: *mut u8 = 0x154 as *mut u8;

/// Transceiver Receiver Sensitivity Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RX_PDT_DIS | 10000000 |
/// | RX_OVERRIDE | 1000000 |
/// | RX_PDT_LEVEL | 1111 |
pub const RX_SYN: *mut u8 = 0x155 as *mut u8;

/// Transceiver Reduced Power Consumption Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLL_RPC_EN | 1000 |
/// | RX_RPC_CTRL | 11000000 |
/// | IPAN_RPC_EN | 10 |
/// | PDT_RPC_EN | 10000 |
/// | XAH_RPC_EN | 1 |
/// | RX_RPC_EN | 100000 |
pub const TRX_RPC: *mut u8 = 0x156 as *mut u8;

/// Transceiver Acknowledgment Frame Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AACK_UPLD_RES_FT | 10000 |
/// | AACK_PROM_MODE | 10 |
/// | AACK_FLTR_RES_FT | 100000 |
/// | AACK_ACK_TIME | 100 |
pub const XAH_CTRL_1: *mut u8 = 0x157 as *mut u8;

/// Transceiver Filter Tuning Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FTN_START | 10000000 |
pub const FTN_CTRL: *mut u8 = 0x158 as *mut u8;

/// Transceiver Center Frequency Calibration Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLL_CF_START | 10000000 |
pub const PLL_CF: *mut u8 = 0x15A as *mut u8;

/// Transceiver Delay Cell Calibration Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLL_DCU_START | 10000000 |
pub const PLL_DCU: *mut u8 = 0x15B as *mut u8;

/// Device Identification Register (Part Number).
pub const PART_NUM: *mut u8 = 0x15C as *mut u8;

/// Device Identification Register (Version Number).
pub const VERSION_NUM: *mut u8 = 0x15D as *mut u8;

/// Device Identification Register (Manufacture ID Low Byte).
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MAN_ID_03 | 1000 |
/// | MAN_ID_04 | 10000 |
/// | MAN_ID_02 | 100 |
/// | MAN_ID_07 | 10000000 |
/// | MAN_ID_01 | 10 |
/// | MAN_ID_00 | 1 |
/// | MAN_ID_06 | 1000000 |
/// | MAN_ID_05 | 100000 |
pub const MAN_ID_0: *mut u8 = 0x15E as *mut u8;

/// Device Identification Register (Manufacture ID High Byte).
pub const MAN_ID_1: *mut u8 = 0x15F as *mut u8;

/// Transceiver MAC Short Address Register (Low Byte).
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SHORT_ADDR_04 | 10000 |
/// | SHORT_ADDR_00 | 1 |
/// | SHORT_ADDR_05 | 100000 |
/// | SHORT_ADDR_02 | 100 |
/// | SHORT_ADDR_06 | 1000000 |
/// | SHORT_ADDR_03 | 1000 |
/// | SHORT_ADDR_01 | 10 |
/// | SHORT_ADDR_07 | 10000000 |
pub const SHORT_ADDR_0: *mut u8 = 0x160 as *mut u8;

/// Transceiver MAC Short Address Register (High Byte).
pub const SHORT_ADDR_1: *mut u8 = 0x161 as *mut u8;

/// Transceiver Personal Area Network ID Register (Low Byte).
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PAN_ID_00 | 1 |
/// | PAN_ID_06 | 1000000 |
/// | PAN_ID_03 | 1000 |
/// | PAN_ID_01 | 10 |
/// | PAN_ID_02 | 100 |
/// | PAN_ID_05 | 100000 |
/// | PAN_ID_04 | 10000 |
/// | PAN_ID_07 | 10000000 |
pub const PAN_ID_0: *mut u8 = 0x162 as *mut u8;

/// Transceiver Personal Area Network ID Register (High Byte).
pub const PAN_ID_1: *mut u8 = 0x163 as *mut u8;

/// Transceiver MAC IEEE Address Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IEEE_ADDR_00 | 1 |
/// | IEEE_ADDR_01 | 10 |
/// | IEEE_ADDR_02 | 100 |
/// | IEEE_ADDR_03 | 1000 |
/// | IEEE_ADDR_06 | 1000000 |
/// | IEEE_ADDR_05 | 100000 |
/// | IEEE_ADDR_07 | 10000000 |
/// | IEEE_ADDR_04 | 10000 |
pub const IEEE_ADDR_0: *mut u8 = 0x164 as *mut u8;

/// Transceiver MAC IEEE Address Register 1.
pub const IEEE_ADDR_1: *mut u8 = 0x165 as *mut u8;

/// Transceiver MAC IEEE Address Register 2.
pub const IEEE_ADDR_2: *mut u8 = 0x166 as *mut u8;

/// Transceiver MAC IEEE Address Register 3.
pub const IEEE_ADDR_3: *mut u8 = 0x167 as *mut u8;

/// Transceiver MAC IEEE Address Register 4.
pub const IEEE_ADDR_4: *mut u8 = 0x168 as *mut u8;

/// Transceiver MAC IEEE Address Register 5.
pub const IEEE_ADDR_5: *mut u8 = 0x169 as *mut u8;

/// Transceiver MAC IEEE Address Register 6.
pub const IEEE_ADDR_6: *mut u8 = 0x16A as *mut u8;

/// Transceiver MAC IEEE Address Register 7.
pub const IEEE_ADDR_7: *mut u8 = 0x16B as *mut u8;

/// Transceiver Extended Operating Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MAX_CSMA_RETRIES | 1110 |
/// | SLOTTED_OPERATION | 1 |
/// | MAX_FRAME_RETRIES | 11110000 |
pub const XAH_CTRL_0: *mut u8 = 0x16C as *mut u8;

/// Transceiver CSMA-CA Random Number Generator Seed Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CSMA_SEED_01 | 10 |
/// | CSMA_SEED_07 | 10000000 |
/// | CSMA_SEED_05 | 100000 |
/// | CSMA_SEED_04 | 10000 |
/// | CSMA_SEED_03 | 1000 |
/// | CSMA_SEED_02 | 100 |
/// | CSMA_SEED_00 | 1 |
/// | CSMA_SEED_06 | 1000000 |
pub const CSMA_SEED_0: *mut u8 = 0x16D as *mut u8;

/// Transceiver Acknowledgment Frame Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AACK_FVN_MODE | 11000000 |
/// | AACK_SET_PD | 100000 |
/// | AACK_DIS_ACK | 10000 |
/// | AACK_I_AM_COORD | 1000 |
pub const CSMA_SEED_1: *mut u8 = 0x16E as *mut u8;

/// Transceiver CSMA-CA Back-off Exponent Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MIN_BE | 1111 |
/// | MAX_BE | 11110000 |
pub const CSMA_BE: *mut u8 = 0x16F as *mut u8;

/// Transceiver Digital Test Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TST_CTRL_DIG | 1111 |
pub const TST_CTRL_DIGI: *mut u8 = 0x176 as *mut u8;

/// Transceiver Received Frame Length Register.
pub const TST_RX_LENGTH: *mut u8 = 0x17B as *mut u8;

/// Start of frame buffer.
pub const TRXFBST: *mut u8 = 0x180 as *mut u8;

/// End of frame buffer.
pub const TRXFBEND: *mut u8 = 0x1FF as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const AVDDOK: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const MUX5: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ACCH: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const REFOK: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ACME: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRC`
pub const ADSUT: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ADCSRC`
pub const ADTHT: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AES_CTRL`
pub const AES_DIR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AES_CTRL`
pub const AES_REQUEST: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AES_CTRL`
pub const AES_IM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `AES_CTRL`
pub const AES_MODE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AES_STATUS`
pub const AES_DONE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `AES_STATUS`
pub const AES_ER: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ANT_DIV`
pub const ANT_DIV_EN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ANT_DIV`
pub const ANT_CTRL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ANT_DIV`
pub const ANT_EXT_SW_EN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ANT_DIV`
pub const ANT_SEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN2UB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2BUB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2AUB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2BUB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ASSR`
pub const EXCLK: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ASSR`
pub const EXCLKAMR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2AUB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BATMON`
pub const BATMON_VTH: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `BATMON`
pub const BATMON_HR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BATMON`
pub const BAT_LOW: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `BATMON`
pub const BAT_LOW_EN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `BATMON`
pub const BATMON_OK: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `BGCR`
pub const BGCAL_FINE: *mut u8 = 0x78 as *mut u8;

/// Bitfield on register `BGCR`
pub const BGCAL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CCA_THRES`
pub const CCA_CS_THRES: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CCA_THRES`
pub const CCA_ED_THRES: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CC_CTRL_1`
pub const CC_BAND: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CSMA_BE`
pub const MIN_BE: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CSMA_BE`
pub const MAX_BE: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CSMA_SEED_0`
pub const CSMA_SEED_06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CSMA_SEED_1`
pub const AACK_FVN_MODE: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `CSMA_SEED_1`
pub const AACK_SET_PD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CSMA_SEED_1`
pub const AACK_DIS_ACK: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CSMA_SEED_1`
pub const AACK_I_AM_COORD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC10D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC11D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC12D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC14D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC15D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC8D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC9D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR2`
pub const ADC13D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DPDS0`
pub const PFDRV: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `DPDS0`
pub const PEDRV: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DPDS0`
pub const PDDRV: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `DPDS0`
pub const PBDRV: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DPDS1`
pub const PGDRV: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC5: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC7: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC4: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC6: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `FTN_CTRL`
pub const FTN_START: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRASY: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `IEEE_ADDR_0`
pub const IEEE_ADDR_04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const RX_START_EN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const PLL_UNLOCK_EN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const AWAKE_EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const CCA_ED_DONE_EN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const RX_END_EN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const PLL_LOCK_EN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const TX_END_EN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `IRQ_MASK`
pub const AMI_EN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `IRQ_MASK1`
pub const MAF_2_AMI_EN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `IRQ_MASK1`
pub const TX_START_EN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `IRQ_MASK1`
pub const MAF_3_AMI_EN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `IRQ_MASK1`
pub const MAF_1_AMI_EN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `IRQ_MASK1`
pub const MAF_0_AMI_EN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const CCA_ED_DONE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const AWAKE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const RX_START: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const PLL_LOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const TX_END: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const AMI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const PLL_UNLOCK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `IRQ_STATUS`
pub const RX_END: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `IRQ_STATUS1`
pub const MAF_3_AMI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `IRQ_STATUS1`
pub const MAF_1_AMI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `IRQ_STATUS1`
pub const TX_START: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `IRQ_STATUS1`
pub const MAF_2_AMI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `IRQ_STATUS1`
pub const MAF_0_AMI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LLCR`
pub const LLTCO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LLCR`
pub const LLCAL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LLCR`
pub const LLDONE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LLCR`
pub const LLCOMP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LLCR`
pub const LLENCAL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LLCR`
pub const LLSHORT: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const CKSEL_SUT: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MAFCR0`
pub const MAF0EN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MAFCR0`
pub const MAF2EN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MAFCR0`
pub const MAF3EN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MAFCR0`
pub const MAF1EN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_2_I_AM_COORD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_3_SET_PD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_0_I_AM_COORD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_1_I_AM_COORD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_1_SET_PD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_0_SET_PD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_2_SET_PD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MAFCR1`
pub const AACK_3_I_AM_COORD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MAN_ID_0`
pub const MAN_ID_05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `NEMCR`
pub const ENEAM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `NEMCR`
pub const AEAM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PAN_ID_0`
pub const PAN_ID_07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PARCR`
pub const PARDFI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PARCR`
pub const PARUFI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PARCR`
pub const PALTU: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `PARCR`
pub const PALTD: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `PHY_CC_CCA`
pub const CCA_REQUEST: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PHY_CC_CCA`
pub const CHANNEL: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PHY_CC_CCA`
pub const CCA_MODE: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `PHY_RSSI`
pub const RSSI: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PHY_RSSI`
pub const RND_VALUE: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `PHY_RSSI`
pub const RX_CRC_VALID: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PHY_TX_PWR`
pub const TX_PWR: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PLL_CF`
pub const PLL_CF_START: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PLL_DCU`
pub const PLL_DCU_START: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRPGA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRUSART0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRUSART1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTRX24: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRRAM2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRRAM1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRRAM3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRRAM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RX_CTRL`
pub const PDT_THRES: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `RX_SYN`
pub const RX_PDT_DIS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RX_SYN`
pub const RX_OVERRIDE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RX_SYN`
pub const RX_PDT_LEVEL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SCCR0`
pub const SCCMP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SCCR0`
pub const SCRES: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SCCR0`
pub const SCTSE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SCCR0`
pub const SCEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SCCR0`
pub const SCMBTS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SCCR0`
pub const SCCKSEL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SCCR1`
pub const SCENBO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SCCR1`
pub const SCCKDIV: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `SCCR1`
pub const SCBTSM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SCCR1`
pub const SCEECLK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SCCSR`
pub const SCCS2: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `SCCSR`
pub const SCCS3: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `SCCSR`
pub const SCCS1: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SCIRQM`
pub const IRQMBO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SCIRQM`
pub const IRQMOF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SCIRQM`
pub const IRQMCP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SCIRQS`
pub const IRQSCP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SCIRQS`
pub const IRQSBO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SCIRQS`
pub const IRQSOF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SCSR`
pub const SCBSY: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SHORT_ADDR_0`
pub const SHORT_ADDR_07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1C: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const WGM2: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const WGM22: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3C: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const CS3: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICNC3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICES3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4C: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const ICNC4: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const ICES4: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const CS4: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR5A`
pub const COM5A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR5A`
pub const COM5B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR5A`
pub const COM5C: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `TCCR5B`
pub const ICNC5: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR5B`
pub const CS5: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR5B`
pub const ICES5: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR5C`
pub const FOC5C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR5C`
pub const FOC5B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR5C`
pub const FOC5A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR2`
pub const TOV2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR3`
pub const ICF3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR3`
pub const TOV3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR4`
pub const TOV4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR4`
pub const ICF4: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR5`
pub const OCF5A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR5`
pub const OCF5B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR5`
pub const ICF5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR5`
pub const TOV5: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR5`
pub const OCF5C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const TOIE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const TOIE3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const ICIE3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const ICIE4: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const TOIE4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK5`
pub const OCIE5A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK5`
pub const OCIE5C: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK5`
pub const ICIE5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK5`
pub const OCIE5B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK5`
pub const TOIE5: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TRXPR`
pub const SLPTR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TRXPR`
pub const TRXRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TRX_CTRL_0`
pub const PMU_START: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TRX_CTRL_0`
pub const PMU_EN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TRX_CTRL_0`
pub const Res7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TRX_CTRL_0`
pub const PMU_IF_INV: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TRX_CTRL_1`
pub const TX_AUTO_CRC_ON: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TRX_CTRL_1`
pub const IRQ_2_EXT_EN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TRX_CTRL_1`
pub const PLL_TX_FLT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TRX_CTRL_1`
pub const PA_EXT_EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TRX_CTRL_2`
pub const RX_SAFE_MODE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TRX_CTRL_2`
pub const OQPSK_DATA_RATE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TRX_RPC`
pub const PLL_RPC_EN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TRX_RPC`
pub const RX_RPC_CTRL: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TRX_RPC`
pub const IPAN_RPC_EN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TRX_RPC`
pub const PDT_RPC_EN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TRX_RPC`
pub const XAH_RPC_EN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TRX_RPC`
pub const RX_RPC_EN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TRX_STATE`
pub const TRAC_STATUS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `TRX_STATE`
pub const TRX_CMD: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `TRX_STATUS`
pub const CCA_DONE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TRX_STATUS`
pub const TST_STATUS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TRX_STATUS`
pub const CCA_STATUS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TST_CTRL_DIGI`
pub const TST_CTRL_DIG: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `TWAMR`
pub const TWAM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const TXC0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UDRE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const RXC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXEN0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXEN0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UDORD0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCPHA0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const RXC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UDRE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const TXC1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXEN1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXEN1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCPHA1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UDORD1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VREG_CTRL`
pub const DVREG_EXT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `VREG_CTRL`
pub const DVDD_OK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `VREG_CTRL`
pub const AVREG_EXT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `VREG_CTRL`
pub const AVDD_OK: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `XAH_CTRL_0`
pub const MAX_CSMA_RETRIES: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `XAH_CTRL_0`
pub const SLOTTED_OPERATION: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `XAH_CTRL_0`
pub const MAX_FRAME_RETRIES: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `XAH_CTRL_1`
pub const AACK_UPLD_RES_FT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `XAH_CTRL_1`
pub const AACK_PROM_MODE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `XAH_CTRL_1`
pub const AACK_FLTR_RES_FT: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `XAH_CTRL_1`
pub const AACK_ACK_TIME: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `XOSC_CTRL`
pub const XTAL_TRIM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `XOSC_CTRL`
pub const XTAL_MODE: *mut u8 = 0xF0 as *mut u8;

/// `AACK_ACK_TIME_bitf` value group
#[allow(non_upper_case_globals)]
pub mod aack_ack_time_bitf {
   /// 12 symbols acknowledgment time.
   pub const AACK_ACK_TIME_12_SYM: u32 = 0x0;
   /// 2 symbols acknowledgment time.
   pub const AACK_ACK_TIME_2_SYM: u32 = 0x1;
}

/// `AACK_FVN_MODE_bitf` value group
#[allow(non_upper_case_globals)]
pub mod aack_fvn_mode_bitf {
   /// Acknowledge frames with version number 0.
   pub const VAL_0: u32 = 0x0;
   /// Acknowledge frames with version number 0 or 1.
   pub const VAL_1: u32 = 0x1;
   /// Acknowledge frames with version number 0 or 1 or 2.
   pub const VAL_2: u32 = 0x2;
   /// Acknowledge frames independent of frame version number.
   pub const VAL_3: u32 = 0x3;
}

/// `AES_DIRECTION_BITF` value group
#[allow(non_upper_case_globals)]
pub mod aes_direction_bitf {
   /// AES operation is encryption.
   pub const AES_DIR_ENC: u32 = 0x0;
   /// AES operation is decryption.
   pub const AES_DIR_DEC: u32 = 0x1;
}

/// `AES_MODE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod aes_mode_bitf {
   /// AES Mode is ECB (Electronic Code Book).
   pub const AES_MODE_ECB: u32 = 0x0;
   /// AES Mode is CBC (Cipher Block Chaining).
   pub const AES_MODE_CBC: u32 = 0x1;
}

/// `ANALOG_ADC_AUTO_TRIGGER` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger {
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

/// `ANALOG_ADC_STARTUP_TIME` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_startup_time {
   /// 3 ADC clock cycles.
   pub const VAL_0x00: u32 = 0x0;
   /// 7 ADC clock cycles.
   pub const VAL_0x01: u32 = 0x1;
   /// 11 ADC clock cycles.
   pub const VAL_0x02: u32 = 0x2;
   /// 15 ADC clock cycles.
   pub const VAL_0x03: u32 = 0x3;
   /// ...
   pub const VAL_0x04: u32 = 0x4;
   /// 251 ADC clock cycles.
   pub const VAL_0x3E: u32 = 0x3E;
   /// 255 ADC clock cycles.
   pub const VAL_0x3F: u32 = 0x3F;
}

/// `ANALOG_ADC_TRACK_AND_HOLD_TIME` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_track_and_hold_time {
   /// Single ended: 1, differential 3 ADC clock cycles.
   pub const VAL_0x00: u32 = 0x0;
   /// Single ended: 2, differential 5 ADC clock cycles.
   pub const VAL_0x01: u32 = 0x1;
   /// Single ended: 3, differential 7 ADC clock cycles.
   pub const VAL_0x02: u32 = 0x2;
   /// Single ended: 4, differential 9 ADC clock cycles.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ANALOG_ADC_V_REF9` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref9 {
   /// AREF, Internal reference voltage generation turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVDD with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal 1.5V Voltage Reference (no external capacitor at AREF pin).
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 1.6V Voltage Reference (no external capacitor at AREF pin).
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

/// `ANT_CTRL_bitf` value group
#[allow(non_upper_case_globals)]
pub mod ant_ctrl_bitf {
   /// Reserved.
   pub const VAL_0: u32 = 0x0;
   /// Antenna 1: DIG1=H, DIG2=L.
   pub const ANT_1: u32 = 0x1;
   /// Antenna 0: DIG1=L, DIG2=H.
   pub const ANT_0: u32 = 0x2;
   /// Default value for ANT_EXT_SW_EN=0; Mandatory setting for applications not using Antenna Diversity.
   pub const ANT_RESET: u32 = 0x3;
}

/// `ANT_DIV_EN_bitf` value group
#[allow(non_upper_case_globals)]
pub mod ant_div_en_bitf {
   /// Antenna Diversity algorithm disabled.
   pub const VAL_0: u32 = 0x0;
   /// Antenna Diversity algorithm enabled.
   pub const VAL_1: u32 = 0x1;
}

/// `ANT_EXT_SW_EN_bitf` value group
#[allow(non_upper_case_globals)]
pub mod ant_ext_sw_en_bitf {
   /// Antenna Diversity RF switch control disabled.
   pub const ANT_DIV_EXT_SW_DIS: u32 = 0x0;
   /// Antenna Diversity RF switch control enabled.
   pub const ANT_DIV_EXT_SW_EN: u32 = 0x1;
}

/// `ANT_SEL_bitf` value group
#[allow(non_upper_case_globals)]
pub mod ant_sel_bitf {
   /// Antenna 0.
   pub const ANTENNA_0: u32 = 0x0;
   /// Antenna 1.
   pub const ANTENNA_1: u32 = 0x1;
}

/// `AVDD_OK_BITF` value group
#[allow(non_upper_case_globals)]
pub mod avdd_ok_bitf {
   /// Analog voltage regulator disabled or supply voltage not stable.
   pub const VAL_0: u32 = 0x0;
   /// Analog supply voltage has settled.
   pub const VAL_1: u32 = 0x1;
}

/// `AVREG_EXT_BITF` value group
#[allow(non_upper_case_globals)]
pub mod avreg_ext_bitf {
   /// Internal AVDD voltage regulator for the analog section is enabled.
   pub const AVDD_INT: u32 = 0x0;
   /// Internal AVDD voltage regulator is disabled; use external regulated 1.8V supply voltage for the analog section.
   pub const AVDD_EXT: u32 = 0x1;
}

/// `BATMON_HR_bitf` value group
#[allow(non_upper_case_globals)]
pub mod batmon_hr_bitf {
   /// Enables the low range, see BATMON_VTH.
   pub const BATMON_HR_DIS: u32 = 0x0;
   /// Enables the high range, see BATMON_VTH.
   pub const BATMON_HR_EN: u32 = 0x1;
}

/// `BATMON_OK_bitf` value group
#[allow(non_upper_case_globals)]
pub mod batmon_ok_bitf {
   /// The battery voltage is below the threshold.
   pub const VAL_0: u32 = 0x0;
   /// The battery voltage is above the threshold.
   pub const VAL_1: u32 = 0x1;
}

/// `BATMON_VTH_bitf` value group
#[allow(non_upper_case_globals)]
pub mod batmon_vth_bitf {
   /// 2.550V / 1.70V (BATMON_HR=1/0).
   pub const VAL_0x0: u32 = 0x0;
   /// 2.625V / 1.75V (BATMON_HR=1/0).
   pub const VAL_0x1: u32 = 0x1;
   /// 2.700V / 1.80V (BATMON_HR=1/0).
   pub const VAL_0x2: u32 = 0x2;
   /// 2.775V / 1.85V (BATMON_HR=1/0).
   pub const VAL_0x3: u32 = 0x3;
   /// 2.850V / 1.90V (BATMON_HR=1/0).
   pub const VAL_0x4: u32 = 0x4;
   /// 2.925V / 1.95V (BATMON_HR=1/0).
   pub const VAL_0x5: u32 = 0x5;
   /// 3.000V / 2.00V (BATMON_HR=1/0).
   pub const VAL_0x6: u32 = 0x6;
   /// 3.075V / 2.05V (BATMON_HR=1/0).
   pub const VAL_0x7: u32 = 0x7;
   /// 3.150V / 2.10V (BATMON_HR=1/0).
   pub const VAL_0x8: u32 = 0x8;
   /// 3.225V / 2.15V (BATMON_HR=1/0).
   pub const VAL_0x9: u32 = 0x9;
   /// 3.300V / 2.20V (BATMON_HR=1/0).
   pub const VAL_0xA: u32 = 0xA;
   /// 3.375V / 2.25V (BATMON_HR=1/0).
   pub const VAL_0xB: u32 = 0xB;
   /// 3.450V / 2.30V (BATMON_HR=1/0).
   pub const VAL_0xC: u32 = 0xC;
   /// 3.525V / 2.35V (BATMON_HR=1/0).
   pub const VAL_0xD: u32 = 0xD;
   /// 3.600V / 2.40V (BATMON_HR=1/0).
   pub const VAL_0xE: u32 = 0xE;
   /// 3.675V / 2.45V (BATMON_HR=1/0).
   pub const VAL_0xF: u32 = 0xF;
}

/// `BGCAL_BITF` value group
#[allow(non_upper_case_globals)]
pub mod bgcal_bitf {
   /// Center value.
   pub const VAL_4: u32 = 0x4;
   /// Voltage step up.
   pub const VAL_3: u32 = 0x3;
   /// Voltage step down.
   pub const VAL_5: u32 = 0x5;
   /// Setting for highest voltage.
   pub const VAL_0: u32 = 0x0;
   /// Setting for lowest voltage.
   pub const VAL_7: u32 = 0x7;
}

/// `BGCAL_FINE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod bgcal_fine_bitf {
   /// Center value.
   pub const VAL_0: u32 = 0x0;
   /// Voltage step up.
   pub const VAL_1: u32 = 0x1;
   /// Voltage step down.
   pub const VAL_8: u32 = 0x8;
   /// Setting for highest voltage.
   pub const VAL_7: u32 = 0x7;
   /// Setting for lowest voltage.
   pub const VAL_15: u32 = 0xF;
}

/// `CCA_DONE_bitf` value group
#[allow(non_upper_case_globals)]
pub mod cca_done_bitf {
   /// CCA calculation not finished.
   pub const CCA_NOT_FIN: u32 = 0x0;
   /// CCA calculation finished.
   pub const CCA_FIN: u32 = 0x1;
}

/// `CCA_MODE_bitf` value group
#[allow(non_upper_case_globals)]
pub mod cca_mode_bitf {
   /// Mode 3a, Carrier sense OR energy above threshold.
   pub const CCA_CS_OR_ED: u32 = 0x0;
   /// Mode 1, Energy above threshold.
   pub const CCA_ED: u32 = 0x1;
   /// Mode 2, Carrier sense only.
   pub const CCA_CS: u32 = 0x2;
   /// Mode 3b, Carrier sense AND energy above threshold.
   pub const CCA_CS_AND_ED: u32 = 0x3;
}

/// `CCA_STATUS_bitf` value group
#[allow(non_upper_case_globals)]
pub mod cca_status_bitf {
   /// Channel indicated as busy.
   pub const CCA_BUSY: u32 = 0x0;
   /// Channel indicated as idle.
   pub const CCA_IDLE: u32 = 0x1;
}

/// `CHANNEL_bitf` value group
#[allow(non_upper_case_globals)]
pub mod channel_bitf {
   /// 2405 MHz.
   pub const F_2405MHZ: u32 = 0xB;
   /// 2410 MHz.
   pub const F_2410MHZ: u32 = 0xC;
   /// 2415 MHz.
   pub const F_2415MHZ: u32 = 0xD;
   /// 2420 MHz.
   pub const F_2420MHZ: u32 = 0xE;
   /// 2425 MHz.
   pub const F_2425MHZ: u32 = 0xF;
   /// 2430 MHz.
   pub const F_2430MHZ: u32 = 0x10;
   /// 2435 MHz.
   pub const F_2435MHZ: u32 = 0x11;
   /// 2440 MHz.
   pub const F_2440MHZ: u32 = 0x12;
   /// 2445 MHz.
   pub const F_2445MHZ: u32 = 0x13;
   /// 2450 MHz.
   pub const F_2450MHZ: u32 = 0x14;
   /// 2455 MHz.
   pub const F_2455MHZ: u32 = 0x15;
   /// 2460 MHz.
   pub const F_2460MHZ: u32 = 0x16;
   /// 2465 MHz.
   pub const F_2465MHZ: u32 = 0x17;
   /// 2470 MHz.
   pub const F_2470MHZ: u32 = 0x18;
   /// 2475 MHz.
   pub const F_2475MHZ: u32 = 0x19;
   /// 2480 MHz.
   pub const F_2480MHZ: u32 = 0x1A;
}

/// `CLK_SEL_3BIT_EXT_MEGARF` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit_ext_megarf {
   /// No clock source (Timer/Counter stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// clk_IO/1 (no prescaling).
   pub const VAL_0x01: u32 = 0x1;
   /// clk_IO/8 (from prescaler).
   pub const VAL_0x02: u32 = 0x2;
   /// clk_IO/64 (from prescaler).
   pub const VAL_0x03: u32 = 0x3;
   /// clk_IO/256 (from prescaler).
   pub const VAL_0x04: u32 = 0x4;
   /// clk_IO/1024 (from prescaler).
   pub const VAL_0x05: u32 = 0x5;
   /// External clock source on Tn pin, clock on falling edge.
   pub const VAL_0x06: u32 = 0x6;
   /// External clock source on Tn pin, clock on rising edge.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CLK_SEL_3BIT_NOEXT_MEGARF` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_3bit_noext_megarf {
   /// No clock source (Timer/Counter stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// clk_IO/1 (no prescaling).
   pub const VAL_0x01: u32 = 0x1;
   /// clk_IO/8 (from prescaler).
   pub const VAL_0x02: u32 = 0x2;
   /// clk_IO/64 (from prescaler).
   pub const VAL_0x03: u32 = 0x3;
   /// clk_IO/256 (from prescaler).
   pub const VAL_0x04: u32 = 0x4;
   /// clk_IO/1024 (from prescaler).
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
}

/// `COMM_SCK_RATE_SPI2X` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate_spi2x {
   /// fosc/4 / fosc/2 (SPI2X=0/1).
   pub const VAL_0x00: u32 = 0x0;
   /// fosc/16 / fosc/8 (SPI2X=0/1).
   pub const VAL_0x01: u32 = 0x1;
   /// fosc/64 / fosc/32 (SPI2X=0/1).
   pub const VAL_0x02: u32 = 0x2;
   /// fosc/128 / fosc/64 (SPI2X=0/1).
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

/// `COMM_USART_MODE_2BIT_MEGARF` value group
#[allow(non_upper_case_globals)]
pub mod comm_usart_mode_2bit_megarf {
   /// Asynchronous USART.
   pub const VAL_0x00: u32 = 0x0;
   /// Synchronous USART.
   pub const VAL_0x01: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Master SPI (MSPIM).
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_CLK_PRESCALE_4_BITS_SMALL_MEGARF` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_4_bits_small_megarf {
   /// Division factor 1   / RC-Oscillator   2.
   pub const VAL_0x0: u32 = 0x0;
   /// Division factor 2   / RC-Oscillator   4.
   pub const VAL_0x1: u32 = 0x1;
   /// Division factor 4   / RC-Oscillator   8.
   pub const VAL_0x2: u32 = 0x2;
   /// Division factor 8   / RC-Oscillator  16.
   pub const VAL_0x3: u32 = 0x3;
   /// Division factor 16  / RC-Oscillator  32.
   pub const VAL_0x4: u32 = 0x4;
   /// Division factor 32  / RC-Oscillator  64.
   pub const VAL_0x5: u32 = 0x5;
   /// Division factor 64  / RC-Oscillator 128.
   pub const VAL_0x6: u32 = 0x6;
   /// Division factor 128 / RC-Oscillator 256.
   pub const VAL_0x7: u32 = 0x7;
   /// Division factor 256 / RC-Oscillator 512.
   pub const VAL_0x8: u32 = 0x8;
   /// Reserved.
   pub const VAL_0x9: u32 = 0x9;
   /// Reserved.
   pub const VAL_0xA: u32 = 0xA;
   /// Reserved.
   pub const VAL_0xB: u32 = 0xB;
   /// Reserved.
   pub const VAL_0xC: u32 = 0xC;
   /// Reserved.
   pub const VAL_0xD: u32 = 0xD;
   /// Reserved.
   pub const VAL_0xE: u32 = 0xE;
   /// Division factor 1 only permitted for RC-Oscillator. Flash and EEPROM programming is not allowed.
   pub const VAL_0xF: u32 = 0xF;
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

/// `DVDD_OK_BITF` value group
#[allow(non_upper_case_globals)]
pub mod dvdd_ok_bitf {
   /// Digital voltage regulator disabled or supply voltage not stable.
   pub const VAL_0: u32 = 0x0;
   /// Digital supply voltage has settled.
   pub const VAL_1: u32 = 0x1;
}

/// `DVREG_EXT_BITF` value group
#[allow(non_upper_case_globals)]
pub mod dvreg_ext_bitf {
   /// Internal DVDD voltage regulator for the digital section is enabled.
   pub const DVDD_INT: u32 = 0x0;
   /// Internal DVDD voltage regulator is disabled; use external regulated 1.8V supply voltage for the digital section.
   pub const DVDD_EXT: u32 = 0x1;
}

/// `ED_LEVEL_BITF` value group
#[allow(non_upper_case_globals)]
pub mod ed_level_bitf {
   /// Minimum result of last ED measurement.
   pub const ED_MIN: u32 = 0x0;
   /// P(RF) = RSSI_BASE_VAL+ED \[dBm\].
   pub const ED_MIN_PLUS_1dB: u32 = 0x1;
   /// ...
   pub const VAL_0x02: u32 = 0x2;
   /// Maximum result of last ED measurement.
   pub const ED_MAX: u32 = 0x54;
   /// Reset value.
   pub const ED_RESET: u32 = 0xFF;
}

/// `EEP_MODE2` value group
#[allow(non_upper_case_globals)]
pub mod eep_mode2 {
   /// Erase and Write in one operation (Atomic Operation).
   pub const VAL_0x00: u32 = 0x0;
   /// Erase only.
   pub const VAL_0x01: u32 = 0x1;
   /// Write only.
   pub const VAL_0x02: u32 = 0x2;
   /// Reserved for future use.
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
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=1.9 V.
   pub const _1V9: u32 = 0x5;
   /// Brown-out detection at VCC=2.0 V.
   pub const _2V0: u32 = 0x4;
   /// Brown-out detection at VCC=2.1 V.
   pub const _2V1: u32 = 0x3;
   /// Brown-out detection at VCC=2.2 V.
   pub const _2V2: u32 = 0x2;
   /// Brown-out detection at VCC=2.3 V.
   pub const _2V3: u32 = 0x1;
   /// Brown-out detection at VCC=2.4 V.
   pub const _2V4: u32 = 0x0;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=512 words start address=$7E00.
   pub const _512W_7E00: u32 = 0x3;
   /// Boot Flash size=1024 words start address=$7C00.
   pub const _1024W_7C00: u32 = 0x2;
   /// Boot Flash size=2048 words start address=$7800.
   pub const _2048W_7800: u32 = 0x1;
   /// Boot Flash size=4096 words start address=$7000.
   pub const _4096W_7000: u32 = 0x0;
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
   /// Int. 128kHz RC Osc.; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_128KHZ_6CK_0MS: u32 = 0x3;
   /// Int. 128kHz RC Osc.; Start-up time: 6 CK + 4.1 ms.
   pub const INTRCOSC_128KHZ_6CK_4MS1: u32 = 0x13;
   /// Int. 128kHz RC Osc.; Start-up time: 6 CK + 65 ms.
   pub const INTRCOSC_128KHZ_6CK_65MS: u32 = 0x23;
}

/// `INTERRUPT_EXT_FLAG_BITF` value group
#[allow(non_upper_case_globals)]
pub mod interrupt_ext_flag_bitf {
   /// No edge or logic change on INT7:0 occurred.
   pub const VAL_0x00: u32 = 0x0;
   /// A edge or logic change on INT0 occurred and triggered an interrupt request.
   pub const VAL_0x01: u32 = 0x1;
   /// ...
   pub const VAL_0x02: u32 = 0x2;
   /// A edge or logic change on INT7 occurred and triggered an interrupt request.
   pub const VAL_0x80: u32 = 0x80;
}

/// `INTERRUPT_REQ_ENABLE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod interrupt_req_enable_bitf {
   /// All external pin interrupts are disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// All external pin interrupts are enabled.
   pub const VAL_0xff: u32 = 0xFF;
}

/// `INTERRUPT_SENSE_CONTROL3` value group
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control3 {
   /// The low level of INTn generates an interrupt request.
   pub const VAL_0x00: u32 = 0x0;
   /// Any edge of INTn generates asynchronously an interrupt request.
   pub const VAL_0x01: u32 = 0x1;
   /// The falling edge of INTn generates asynchronously an interrupt request.
   pub const VAL_0x02: u32 = 0x2;
   /// The rising edge of INTn generates asynchronously an interrupt request.
   pub const VAL_0x03: u32 = 0x3;
}

/// `LLDRH_VALUE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod lldrh_value_bitf {
   /// Calibration limit for fast process corner/high output voltage.
   pub const VAL_0x00: u32 = 0x0;
   /// Calibration limit for slow process corner/low output voltage.
   pub const VAL_0x10: u32 = 0x10;
}

/// `LLDRL_VALUE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod lldrl_value_bitf {
   /// Calibration limit for fast process corner/high output voltage.
   pub const VAL_0x00: u32 = 0x0;
   /// Calibration limit for slow process corner/low output voltage.
   pub const VAL_0x08: u32 = 0x8;
}

/// `MAN_ID_0_BITF` value group
#[allow(non_upper_case_globals)]
pub mod man_id_0_bitf {
   /// Atmel JEDEC manufacturer ID, bits \[7:0\] of 32 bit manufacturer ID: 00 00 00 1F.
   pub const ATMEL_BYTE_0: u32 = 0x1F;
}

/// `MAN_ID_1_BITF` value group
#[allow(non_upper_case_globals)]
pub mod man_id_1_bitf {
   /// Atmel JEDEC manufacturer ID, bits \[15:8\] of 32 bit manufacturer ID: 00 00 00 1F.
   pub const ATMEL_BYTE_1: u32 = 0x0;
}

/// `MAX_BE_bitf` value group
#[allow(non_upper_case_globals)]
pub mod max_be_bitf {
   /// This value is not valid for the maximum back-off exponent.
   pub const VAL_1: u32 = 0x1;
   /// This value is not valid for the maximum back-off exponent.
   pub const VAL_2: u32 = 0x2;
   /// Minimum, IEEE compliant value for the maximum back-off exponent.
   pub const VAL_3: u32 = 0x3;
   /// ...
   pub const VAL_4: u32 = 0x4;
   /// Maximum, IEEE compliant value for the maximum back-off exponent.
   pub const VAL_8: u32 = 0x8;
}

/// `MAX_CSMA_RETRIES_bitf` value group
#[allow(non_upper_case_globals)]
pub mod max_csma_retries_bitf {
   /// No repetition of CSMA-CA procedure.
   pub const VAL_0x0: u32 = 0x0;
   /// One repetition of CSMA-CA procedure.
   pub const VAL_0x1: u32 = 0x1;
   /// ...
   pub const VAL_0x2: u32 = 0x2;
   /// Five repetitions (highest IEEE 802.15.4 compliant value).
   pub const VAL_0x5: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x6: u32 = 0x6;
   /// Immediate frame re-transmission without performing CSMA-CA.
   pub const VAL_0x7: u32 = 0x7;
}

/// `MAX_FRAME_RETRIES_bitf` value group
#[allow(non_upper_case_globals)]
pub mod max_frame_retries_bitf {
   /// Retransmission of frame is not attempted.
   pub const VAL_0x0: u32 = 0x0;
   /// Retransmission of frame is attempted once.
   pub const VAL_0x1: u32 = 0x1;
   /// ...
   pub const VAL_0x2: u32 = 0x2;
   /// Retransmission of frame is attempted 15 times.
   pub const VAL_0xF: u32 = 0xF;
}

/// `MIN_BE_bitf` value group
#[allow(non_upper_case_globals)]
pub mod min_be_bitf {
   /// Minimum value of minimum back-off exponent.
   pub const VAL_0: u32 = 0x0;
   /// ...
   pub const VAL_1: u32 = 0x1;
   /// Maximum value of minimum back-off exponent. MIN_BE must be smaller or equal to MAX_BE.
   pub const VAL_8: u32 = 0x8;
}

/// `NEMCR_ADDRESS_BITF` value group
#[allow(non_upper_case_globals)]
pub mod nemcr_address_bitf {
   /// Factory Row.
   pub const VAL_0: u32 = 0x0;
   /// User Row 1.
   pub const VAL_1: u32 = 0x1;
   /// User Row 2.
   pub const VAL_2: u32 = 0x2;
   /// User Row 3.
   pub const VAL_3: u32 = 0x3;
}

/// `OCDR_DATA_BITF` value group
#[allow(non_upper_case_globals)]
pub mod ocdr_data_bitf {
   /// Refer to the debugger documentation for further information on how to use this register.
   pub const VAL_0: u32 = 0x0;
}

/// `OQPSK_DATA_RATE_bitf` value group
#[allow(non_upper_case_globals)]
pub mod oqpsk_data_rate_bitf {
   /// 250 kb/s (IEEE 802.15.4 compliant).
   pub const RATE_250KB: u32 = 0x0;
   /// 500 kb/s.
   pub const RATE_500KB: u32 = 0x1;
   /// 1000 kb/s.
   pub const RATE_1000KB: u32 = 0x2;
   /// 2000 kb/s.
   pub const RATE_2000KB: u32 = 0x3;
}

/// `OSCCAL_BITF` value group
#[allow(non_upper_case_globals)]
pub mod osccal_bitf {
   /// Calibration value for lowest oscillator frequency.
   pub const VAL_0x00: u32 = 0x0;
   /// End value of low frequency range calibration.
   pub const VAL_0x7f: u32 = 0x7F;
   /// Start value of high frequency range calibration.
   pub const VAL_0x80: u32 = 0x80;
   /// Calibration value for highest oscillator frequency.
   pub const VAL_0xff: u32 = 0xFF;
}

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 16.0 MHz.
   pub const _16_0_MHz: u32 = 0x0;
}

/// `PAD_IO_bitf` value group
#[allow(non_upper_case_globals)]
pub mod pad_io_bitf {
   /// 2 mA.
   pub const PAD_IO_2MA: u32 = 0x0;
   /// 4 mA.
   pub const PAD_IO_4MA: u32 = 0x1;
   /// 6 mA.
   pub const PAD_IO_6MA: u32 = 0x2;
   /// 8 mA.
   pub const PAD_IO_8MA: u32 = 0x3;
}

/// `PALTD_bitf` value group
#[allow(non_upper_case_globals)]
pub mod paltd_bitf {
   /// -3us.
   pub const PALTD_MINUS_3US: u32 = 0x0;
   /// -2us.
   pub const PALTD_MINUS_2US: u32 = 0x1;
   /// -1us.
   pub const PALTD_MINUS_1US: u32 = 0x2;
   /// 0us.
   pub const PALTD_0US: u32 = 0x3;
   /// 1us.
   pub const PALTD_1US: u32 = 0x4;
   /// 2us.
   pub const PALTD_2US: u32 = 0x5;
   /// 3us.
   pub const PALTD_3US: u32 = 0x6;
   /// 4us.
   pub const PALTD_4US: u32 = 0x7;
}

/// `PALTU_bitf` value group
#[allow(non_upper_case_globals)]
pub mod paltu_bitf {
   /// -3us.
   pub const PALTU_MINUS_3US: u32 = 0x0;
   /// -2us.
   pub const PALTU_MINUS_2US: u32 = 0x1;
   /// -1us.
   pub const PALTU_MINUS_1US: u32 = 0x2;
   /// 0us.
   pub const PALTU_0US: u32 = 0x3;
   /// 1us.
   pub const PALTU_1US: u32 = 0x4;
   /// 2us.
   pub const PALTU_2US: u32 = 0x5;
   /// 3us.
   pub const PALTU_3US: u32 = 0x6;
   /// 4us.
   pub const PALTU_4US: u32 = 0x7;
}

/// `PART_NUM_bitf` value group
#[allow(non_upper_case_globals)]
pub mod part_num_bitf {
   /// ATmega128RFA1 part number.
   pub const P_ATmega128RFA1: u32 = 0x83;
   /// RFA2 family.
   pub const P_RFA2: u32 = 0x93;
   /// RFR2 family.
   pub const P_RFR2: u32 = 0x94;
}

/// `PDT_THRES_bitf` value group
#[allow(non_upper_case_globals)]
pub mod pdt_thres_bitf {
   /// Reset value, to be used if Antenna Diversity algorithm is disabled.
   pub const PDT_THRES_ANT_DIV_OFF: u32 = 0x7;
   /// Recommended correlator threshold for Antenna Diversity operation.
   pub const PDT_THRES_ANT_DIV_ON: u32 = 0x3;
}

/// `RSSI_VALUE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod rssi_value_bitf {
   /// Minimum RSSI value: P(RF) < -90 dBm.
   pub const RSSI_MIN: u32 = 0x0;
   /// P(RF) = RSSI_BASE_VAL+3 · (RSSI-1) \[dBm\].
   pub const RSSI_MIN_PLUS_3dB: u32 = 0x1;
   /// ...
   pub const VAL_2: u32 = 0x2;
   /// Maximum RSSI value: P(RF) ≥ -10 dBm.
   pub const RSSI_MAX: u32 = 0x1C;
}

/// `RX_CRC_VALID_bitf` value group
#[allow(non_upper_case_globals)]
pub mod rx_crc_valid_bitf {
   /// CRC (FCS) not valid.
   pub const CRC_INVALID: u32 = 0x0;
   /// CRC (FCS) valid.
   pub const CRC_VALID: u32 = 0x1;
}

/// `RX_PDT_LEVEL_BITF` value group
#[allow(non_upper_case_globals)]
pub mod rx_pdt_level_bitf {
   /// RX_THRES ≤ RSSI_BASE_VAL (Reset value); RSSI value not considered.
   pub const RX_PDT_LEVEL_MIN: u32 = 0x0;
   /// RX_THRES > RSSI_BASE_VAL + 0 · 3; RSSI > -90 dBm.
   pub const VAL_0x1: u32 = 0x1;
   /// ...
   pub const VAL_0x2: u32 = 0x2;
   /// RX_THRES > RSSI_BASE_VAL + 13 · 3; RSSI > -51 dBm.
   pub const VAL_0xE: u32 = 0xE;
   /// RX_THRES > RSSI_BASE_VAL + 14 · 3; RSSI > -48 dBm.
   pub const RX_PDT_LEVEL_MAX: u32 = 0xF;
}

/// `RX_RPC_CTRL_BITF` value group
#[allow(non_upper_case_globals)]
pub mod rx_rpc_ctrl_bitf {
   /// Activates minimum power saving behaviour for smart receiving mode.
   pub const VAL_0: u32 = 0x0;
   /// Reserved.
   pub const VAL_1: u32 = 0x1;
   /// Reserved.
   pub const VAL_2: u32 = 0x2;
   /// Activates maximum power saving behaviour for smart receiving mode.
   pub const VAL_3: u32 = 0x3;
}

/// `SCCKDIV_BITF` value group
#[allow(non_upper_case_globals)]
pub mod scckdiv_bitf {
   /// Transceiver Clock divided by 256, (62.5kHz).
   pub const VAL_0: u32 = 0x0;
   /// Transceiver Clock divided by 128, (125kHz).
   pub const VAL_1: u32 = 0x1;
   /// Transceiver Clock divided by 64,  (250kHz).
   pub const VAL_2: u32 = 0x2;
   /// Transceiver Clock divided by 32,  (500kHz).
   pub const VAL_3: u32 = 0x3;
   /// Transceiver Clock divided by 16,  (1MHz).
   pub const VAL_4: u32 = 0x4;
   /// Transceiver Clock divided by 8,   (2MHz).
   pub const VAL_5: u32 = 0x5;
   /// Transceiver Clock divided by 4,   (4MHz).
   pub const VAL_6: u32 = 0x6;
}

/// `SCCS1_BITF` value group
#[allow(non_upper_case_globals)]
pub mod sccs1_bitf {
   /// Compare Unit 1 Relative Compare Source = Beacon Timestamp Register.
   pub const VAL_0: u32 = 0x0;
   /// Compare Unit 1 Relative Compare Source = Transmit Frame Timestamp Register.
   pub const VAL_1: u32 = 0x1;
   /// Compare Unit 1 Relative Compare Source = Received Frame Timestamp Register.
   pub const VAL_2: u32 = 0x2;
}

/// `SCCS2_BITF` value group
#[allow(non_upper_case_globals)]
pub mod sccs2_bitf {
   /// Compare Unit 2 Relative Compare Source = Beacon Timestamp Register.
   pub const VAL_0: u32 = 0x0;
   /// Compare Unit 2 Relative Compare Source = Transmit Frame Timestamp Register.
   pub const VAL_1: u32 = 0x1;
   /// Compare Unit 2 Relative Compare Source = Received Frame Timestamp Register.
   pub const VAL_2: u32 = 0x2;
}

/// `SCCS3_BITF` value group
#[allow(non_upper_case_globals)]
pub mod sccs3_bitf {
   /// Compare Unit 3 Relative Compare Source = Beacon Timestamp Register.
   pub const VAL_0: u32 = 0x0;
   /// Compare Unit 3 Relative Compare Source = Transmit Frame Timestamp Register.
   pub const VAL_1: u32 = 0x1;
   /// Compare Unit 3 Relative Compare Source = Received Frame Timestamp Register.
   pub const VAL_2: u32 = 0x2;
}

/// `SFD_VALUE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod sfd_value_bitf {
   /// IEEE 802.15.4 compliant value of the SFD.
   pub const IEEE_SFD: u32 = 0xA7;
}

/// `SLOTTED_OPERATION_BITF` value group
#[allow(non_upper_case_globals)]
pub mod slotted_operation_bitf {
   /// The radio transceiver operates in unslotted mode. An acknowledgment frame is automatically sent if requested.
   pub const SLOTTED_OP_DIS: u32 = 0x0;
   /// The transmission of an acknowledgment frame has to be controlled by the microcontroller.
   pub const SLOTTED_OP_EN: u32 = 0x1;
}

/// `SPI_CPHA_BITF` value group
#[allow(non_upper_case_globals)]
pub mod spi_cpha_bitf {
   /// Sample (Leading Edge), Setup (Trailing Edge).
   pub const VAL_0: u32 = 0x0;
   /// Setup (Leading Edge), Sample (Trailing Edge).
   pub const VAL_1: u32 = 0x1;
}

/// `SPI_CPOL_BITF` value group
#[allow(non_upper_case_globals)]
pub mod spi_cpol_bitf {
   /// Rising (Leading Edge), Falling (Trailing Edge).
   pub const VAL_0: u32 = 0x0;
   /// Falling (Leading Egde), Rising (Trailing Edge).
   pub const VAL_1: u32 = 0x1;
}

/// `TC0_CLK_SEL_3BIT_EXT` value group
#[allow(non_upper_case_globals)]
pub mod tc0_clk_sel_3bit_ext {
   /// No clock source (Timer/Counter0 stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// clk_IO/1 (no prescaling).
   pub const VAL_0x01: u32 = 0x1;
   /// clk_IO/8 (from prescaler).
   pub const VAL_0x02: u32 = 0x2;
   /// clk_IO/64 (from prescaler).
   pub const VAL_0x03: u32 = 0x3;
   /// clk_IO/256 (from prescaler).
   pub const VAL_0x04: u32 = 0x4;
   /// clk_IO/1024 (from prescaler).
   pub const VAL_0x05: u32 = 0x5;
   /// External clock source on T0 pin, clock on falling edge.
   pub const VAL_0x06: u32 = 0x6;
   /// External clock source on T0 pin, clock on rising edge.
   pub const VAL_0x07: u32 = 0x7;
}

/// `TC0_COM0A_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc0_com0a_bitf {
   /// Normal port operation, OC0A disconnected.
   pub const VAL_0: u32 = 0x0;
   /// Toggle OC0A on Compare Match.
   pub const VAL_1: u32 = 0x1;
   /// Clear OC0A on Compare Match.
   pub const VAL_2: u32 = 0x2;
   /// Set OC0A on Compare Match.
   pub const VAL_3: u32 = 0x3;
}

/// `TC0_COM0B_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc0_com0b_bitf {
   /// Normal port operation, OC0B disconnected.
   pub const VAL_0: u32 = 0x0;
   /// Toggle OC0B on Compare Match.
   pub const VAL_1: u32 = 0x1;
   /// Clear OC0B on Compare Match.
   pub const VAL_2: u32 = 0x2;
   /// Set OC0B on Compare Match.
   pub const VAL_3: u32 = 0x3;
}

/// `TC0_WGM_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc0_wgm_bitf {
   /// Normal mode of operation.
   pub const VAL_0x0: u32 = 0x0;
   /// PWM, phase correct, TOP=0xFF.
   pub const VAL_0x1: u32 = 0x1;
   /// CTC, TOP = OCRA.
   pub const VAL_0x2: u32 = 0x2;
   /// Fast PWM, TOP=0xFF.
   pub const VAL_0x3: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x4: u32 = 0x4;
   /// PWM, Phase correct, TOP = OCRA.
   pub const VAL_0x5: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x6: u32 = 0x6;
   /// Fast PWM, TOP=OCRA.
   pub const VAL_0x7: u32 = 0x7;
}

/// `TC1_COMNX_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc1_comnx_bitf {
   /// Normal port operation, OCnA/OCnB/OCnC disconnected.
   pub const VAL_0: u32 = 0x0;
   /// Toggle OCnA/OCnB/OCnC on Compare Match.
   pub const VAL_1: u32 = 0x1;
   /// Clear OCnA/OCnB/OCnC on Compare Match (set output to low level).
   pub const VAL_2: u32 = 0x2;
   /// Set OCnA/OCnB/OCnC on Compare Match (set output to high level).
   pub const VAL_3: u32 = 0x3;
}

/// `TC1_WGMX_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc1_wgmx_bitf {
   /// Normal mode of operation.
   pub const VAL_0x0: u32 = 0x0;
   /// PWM, phase correct, 8-bit.
   pub const VAL_0x1: u32 = 0x1;
   /// PWM, phase correct, 9-bit.
   pub const VAL_0x2: u32 = 0x2;
   /// PWM, phase correct, 10-bit.
   pub const VAL_0x3: u32 = 0x3;
   /// CTC, TOP = OCRnA.
   pub const VAL_0x4: u32 = 0x4;
   /// Fast PWM, 8-bit.
   pub const VAL_0x5: u32 = 0x5;
   /// Fast PWM, 9-bit.
   pub const VAL_0x6: u32 = 0x6;
   /// Fast PWM, 10-bit.
   pub const VAL_0x7: u32 = 0x7;
   /// PWM, Phase and frequency correct, TOP = ICRn.
   pub const VAL_0x8: u32 = 0x8;
   /// PWM, Phase and frequency correct, TOP = OCRnA.
   pub const VAL_0x9: u32 = 0x9;
   /// PWM, Phase correct, TOP = ICRn.
   pub const VAL_0xA: u32 = 0xA;
   /// PWM, Phase correct, TOP = OCRnA.
   pub const VAL_0xB: u32 = 0xB;
   /// CTC, TOP = OCRnA.
   pub const VAL_0xC: u32 = 0xC;
   /// Reserved.
   pub const VAL_0xD: u32 = 0xD;
   /// Fast PWM, TOP = ICRn.
   pub const VAL_0xE: u32 = 0xE;
   /// Fast PWM, TOP = OCRnA.
   pub const VAL_0xF: u32 = 0xF;
}

/// `TC2_CLK_SEL_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod tc2_clk_sel_3bit {
   /// No clock source (Timer/Counter2 stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T2S/1 (no prescaling).
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T2S/8 (from prescaler).
   pub const VAL_0x02: u32 = 0x2;
   /// clk_T2S/32 (from prescaler).
   pub const VAL_0x03: u32 = 0x3;
   /// clk_T2S/64 (from prescaler).
   pub const VAL_0x04: u32 = 0x4;
   /// clk_T2S/128 (from prescaler).
   pub const VAL_0x05: u32 = 0x5;
   /// clk_T2S/256 (from prescaler).
   pub const VAL_0x06: u32 = 0x6;
   /// clk_T2S/1024 (from prescaler).
   pub const VAL_0x07: u32 = 0x7;
}

/// `TC2_COM2A_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc2_com2a_bitf {
   /// Normal port operation, OC2A disconnected.
   pub const VAL_0: u32 = 0x0;
   /// Toggle OC2A on Compare Match.
   pub const VAL_1: u32 = 0x1;
   /// Clear OC2A on Compare Match.
   pub const VAL_2: u32 = 0x2;
   /// Set OC2A on Compare Match.
   pub const VAL_3: u32 = 0x3;
}

/// `TC2_COM2B_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc2_com2b_bitf {
   /// Normal port operation, OC2B disconnected.
   pub const VAL_0: u32 = 0x0;
   /// Toggle OC2B on Compare Match.
   pub const VAL_1: u32 = 0x1;
   /// Clear OC2B on Compare Match.
   pub const VAL_2: u32 = 0x2;
   /// Set OC2B on Compare Match.
   pub const VAL_3: u32 = 0x3;
}

/// `TC4_COMNX_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tc4_comnx_bitf {
   /// Normal operation.
   pub const VAL_0: u32 = 0x0;
   /// Reserved.
   pub const VAL_1: u32 = 0x1;
   /// Reserved.
   pub const VAL_2: u32 = 0x2;
   /// Reserved.
   pub const VAL_3: u32 = 0x3;
}

/// `TRAC_STATUS_bitf` value group
#[allow(non_upper_case_globals)]
pub mod trac_status_bitf {
   /// SUCCESS (RX_AACK, TX_ARET).
   pub const TRAC_SUCCESS: u32 = 0x0;
   /// SUCCESS_DATA_PENDING (TX_ARET).
   pub const TRAC_SUCCESS_DATA_PENDING: u32 = 0x1;
   /// SUCCESS_WAIT_FOR_ACK (RX_AACK).
   pub const TRAC_SUCCESS_WAIT_FOR_ACK: u32 = 0x2;
   /// CHANNEL_ACCESS_FAILURE (TX_ARET).
   pub const TRAC_CHANNEL_ACCESS_FAILURE: u32 = 0x3;
   /// NO_ACK (TX_ARET).
   pub const TRAC_NO_ACK: u32 = 0x5;
   /// INVALID (RX_AACK, TX_ARET).
   pub const TRAC_INVALID: u32 = 0x7;
}

/// `TRX_CMD_bitf` value group
#[allow(non_upper_case_globals)]
pub mod trx_cmd_bitf {
   /// NOP.
   pub const CMD_NOP: u32 = 0x0;
   /// TX_START.
   pub const CMD_TX_START: u32 = 0x2;
   /// FORCE_TRX_OFF.
   pub const CMD_FORCE_TRX_OFF: u32 = 0x3;
   /// FORCE_PLL_ON.
   pub const CMD_FORCE_PLL_ON: u32 = 0x4;
   /// RX_ON.
   pub const CMD_RX_ON: u32 = 0x6;
   /// TRX_OFF.
   pub const CMD_TRX_OFF: u32 = 0x8;
   /// PLL_ON (TX_ON).
   pub const CMD_PLL_ON: u32 = 0x9;
   /// RX_AACK_ON.
   pub const CMD_RX_AACK_ON: u32 = 0x16;
   /// TX_ARET_ON.
   pub const CMD_TX_ARET_ON: u32 = 0x19;
}

/// `TRX_STATUS_bitf` value group
#[allow(non_upper_case_globals)]
pub mod trx_status_bitf {
   /// P_ON.
   pub const P_ON: u32 = 0x0;
   /// BUSY_RX.
   pub const BUSY_RX: u32 = 0x1;
   /// BUSY_TX.
   pub const BUSY_TX: u32 = 0x2;
   /// RX_ON.
   pub const RX_ON: u32 = 0x6;
   /// TRX_OFF.
   pub const TRX_OFF: u32 = 0x8;
   /// PLL_ON.
   pub const PLL_ON: u32 = 0x9;
   /// SLEEP.
   pub const SLEEP: u32 = 0xF;
   /// BUSY_RX_AACK.
   pub const BUSY_RX_AACK: u32 = 0x11;
   /// BUSY_TX_ARET.
   pub const BUSY_TX_ARET: u32 = 0x12;
   /// RX_AACK_ON.
   pub const RX_AACK_ON: u32 = 0x16;
   /// TX_ARET_ON.
   pub const TX_ARET_ON: u32 = 0x19;
   /// STATE_TRANSITION_IN_PROGRESS.
   pub const STATE_TRANSITION_IN_PROGRESS: u32 = 0x1F;
}

/// `TST_CTRL_DIG_BITF` value group
#[allow(non_upper_case_globals)]
pub mod tst_ctrl_dig_bitf {
   /// NORMAL (no test is active).
   pub const VAL_0: u32 = 0x0;
   /// TST_CONT_TX (continuous transmit).
   pub const VAL_15: u32 = 0xF;
}

/// `TST_STATUS_bitf` value group
#[allow(non_upper_case_globals)]
pub mod tst_status_bitf {
   /// Test mode is disabled.
   pub const TST_DISABLED: u32 = 0x0;
   /// Test mode is active.
   pub const TST_ENABLED: u32 = 0x1;
}

/// `TWI_STATUS_BITF` value group
#[allow(non_upper_case_globals)]
pub mod twi_status_bitf {
   /// Bus error due to illegal START or STOP condition.
   pub const VAL_0x00: u32 = 0x0;
   /// A START condition has been transmitted.
   pub const VAL_0x08: u32 = 0x8;
   /// A repeated START condition has been transmitted.
   pub const VAL_0x10: u32 = 0x10;
   /// SLA+W has been transmitted; ACK has been received.
   pub const VAL_0x18: u32 = 0x18;
   /// SLA+W has been transmitted; NOT ACK has been received.
   pub const VAL_0x20: u32 = 0x20;
   /// Data byte has been transmitted; ACK has been received.
   pub const VAL_0x28: u32 = 0x28;
   /// Data byte has been transmitted; NOT ACK has been received.
   pub const VAL_0x30: u32 = 0x30;
   /// Arbitration lost in SLA+W or data bytes (Transmitter); Arbitration lost in SLA+R or NOT ACK bit (Receiver).
   pub const VAL_0x38: u32 = 0x38;
   /// SLA+R has been transmitted; ACK has been received.
   pub const VAL_0x40: u32 = 0x40;
   /// SLA+R has been transmitted; NOT ACK has been received.
   pub const VAL_0x48: u32 = 0x48;
   /// Data byte has been received; ACK has been returned.
   pub const VAL_0x50: u32 = 0x50;
   /// Data byte has been received; NOT ACK has been returned.
   pub const VAL_0x58: u32 = 0x58;
   /// Own SLA+W has been received; ACK has been returned.
   pub const VAL_0x60: u32 = 0x60;
   /// Arbitration lost in SLA+R/W as Master; own SLA+W has been received; ACK has been returned.
   pub const VAL_0x68: u32 = 0x68;
   /// General call address has been received; ACK has been returned.
   pub const VAL_0x70: u32 = 0x70;
   /// Arbitration lost in SLA+R/W as Master; general call address has been received; ACK has been returned.
   pub const VAL_0x78: u32 = 0x78;
   /// Previously addressed with own SLA+W; data has been received; ACK has been returned.
   pub const VAL_0x80: u32 = 0x80;
   /// Previously addressed with own SLA+W; data has been received; NOT ACK has been returned.
   pub const VAL_0x88: u32 = 0x88;
   /// Previously addressed with general call; data has been received; ACK has been returned.
   pub const VAL_0x90: u32 = 0x90;
   /// Previously addressed with general call; data has been received; NOT ACK has been returned.
   pub const VAL_0x98: u32 = 0x98;
   /// A STOP condition or repeated START condition has been received while still addressed as Slave.
   pub const VAL_0xA0: u32 = 0xA0;
   /// Own SLA+R has been received; ACK has been returned.
   pub const VAL_0xA8: u32 = 0xA8;
   /// Arbitration lost in SLA+R/W as Master; own SLA+R has been received; ACK has been returned.
   pub const VAL_0xB0: u32 = 0xB0;
   /// Data byte in TWDR has been transmitted; ACK has been received.
   pub const VAL_0xB8: u32 = 0xB8;
   /// Data byte in TWDR has been transmitted; NO ACK has been received.
   pub const VAL_0xC0: u32 = 0xC0;
   /// Last data byte in TWDR has been transmitted (TWEA = 0); ACK has been received.
   pub const VAL_0xC8: u32 = 0xC8;
   /// No relevant state information available; TWINT = 0.
   pub const VAL_0xF8: u32 = 0xF8;
}

/// `TX_PWR_bitf` value group
#[allow(non_upper_case_globals)]
pub mod tx_pwr_bitf {
   /// 3.5 dBm.
   pub const VAL_0: u32 = 0x0;
   /// 3.3 dBm.
   pub const VAL_1: u32 = 0x1;
   /// 2.8 dBm.
   pub const VAL_2: u32 = 0x2;
   /// 2.3 dBm.
   pub const VAL_3: u32 = 0x3;
   /// 1.8 dBm.
   pub const VAL_4: u32 = 0x4;
   /// 1.2 dBm.
   pub const VAL_5: u32 = 0x5;
   /// 0.5 dBm.
   pub const VAL_6: u32 = 0x6;
   /// -0.5 dBm.
   pub const VAL_7: u32 = 0x7;
   /// -1.5 dBm.
   pub const VAL_8: u32 = 0x8;
   /// -2.5 dBm.
   pub const VAL_9: u32 = 0x9;
   /// -3.5 dBm.
   pub const VAL_10: u32 = 0xA;
   /// -4.5 dBm.
   pub const VAL_11: u32 = 0xB;
   /// -6.5 dBm.
   pub const VAL_12: u32 = 0xC;
   /// -8.5 dBm.
   pub const VAL_13: u32 = 0xD;
   /// -11.5 dBm.
   pub const VAL_14: u32 = 0xE;
   /// -16.5 dBm.
   pub const VAL_15: u32 = 0xF;
}

/// `USART_CHAR_SIZE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod usart_char_size_bitf {
   /// 5-bit.
   pub const VAL_0: u32 = 0x0;
   /// 6-bit.
   pub const VAL_1: u32 = 0x1;
   /// 7-bit.
   pub const VAL_2: u32 = 0x2;
   /// 8-bit.
   pub const VAL_3: u32 = 0x3;
   /// Reserved.
   pub const VAL_4: u32 = 0x4;
   /// Reserved.
   pub const VAL_5: u32 = 0x5;
   /// Reserved.
   pub const VAL_6: u32 = 0x6;
   /// 9-bit.
   pub const VAL_7: u32 = 0x7;
}

/// `USART_CLK_POLARITY_BITF` value group
#[allow(non_upper_case_globals)]
pub mod usart_clk_polarity_bitf {
   /// Rising XCKn Edge (Transmitted Data Changed), Falling XCKn Edge (Received Data Sampled).
   pub const VAL_0: u32 = 0x0;
   /// Falling XCKn Edge (Transmitted Data Changed), Rising XCKn Edge (Received Data Sampled).
   pub const VAL_1: u32 = 0x1;
}

/// `VERSION_NUM_2_BITF` value group
#[allow(non_upper_case_globals)]
pub mod version_num_2_bitf {
   /// Revision A.
   pub const REV_A: u32 = 0xC;
   /// Revision B.
   pub const REV_B: u32 = 0x1;
   /// Revision C.
   pub const REV_C: u32 = 0x3;
   /// Revision D.
   pub const REV_D: u32 = 0x4;
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

/// `XTAL_MODE_BITF` value group
#[allow(non_upper_case_globals)]
pub mod xtal_mode_bitf {
   /// Internal crystal oscillator disabled; use external reference frequency.
   pub const VAL_0x4: u32 = 0x4;
   /// Internal crystal oscillator enabled; amplitude regulation of oscillation enabled.
   pub const VAL_0xF: u32 = 0xF;
}

/// `XTAL_TRIM_bitf` value group
#[allow(non_upper_case_globals)]
pub mod xtal_trim_bitf {
   /// 0.0 pF, trimming capacitors disconnected.
   pub const XTAL_TRIM_MIN: u32 = 0x0;
   /// 0.3 pF, trimming capacitor switched on.
   pub const VAL_0x1: u32 = 0x1;
   /// ...
   pub const VAL_0x2: u32 = 0x2;
   /// 4.5 pF, trimming capacitor switched on.
   pub const XTAL_TRIM_MAX: u32 = 0xF;
}

