//! The AVR ATmega328PB microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega328PB-AU | TQFP32 | TQFP32 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega328PB-MU | VQFN32 | VQFN32 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega328PB-AN | TQFP32 | TQFP32 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega328PB-MN | VQFN32 | VQFN32 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

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

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESAVE | 1000 |
/// | BOOTSZ | 110 |
/// | DWEN | 1000000 |
/// | BOOTRST | 1 |
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
/// | BODLEVEL | 111 |
/// | CFD | 1000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

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

/// Port E Input Pins.
pub const PINE: *mut u8 = 0x2C as *mut u8;

/// Port E Data Direction Register.
pub const DDRE: *mut u8 = 0x2D as *mut u8;

/// Port E Data Register.
pub const PORTE: *mut u8 = 0x2E as *mut u8;

/// Timer/Counter0 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0A | 10 |
/// | TOV0 | 1 |
/// | OCF0B | 100 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF1 | 100000 |
/// | OCF1B | 100 |
/// | OCF1A | 10 |
/// | TOV1 | 1 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF2A | 10 |
/// | TOV2 | 1 |
/// | OCF2B | 100 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter3 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF3B | 100 |
/// | ICF3 | 100000 |
/// | OCF3A | 10 |
/// | TOV3 | 1 |
pub const TIFR3: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter4 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV4 | 1 |
/// | ICF4 | 100000 |
/// | OCF4B | 100 |
/// | OCF4A | 10 |
pub const TIFR4: *mut u8 = 0x39 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 1111 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 11 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 11 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPE | 10 |
/// | EEMPE | 100 |
/// | EEPM | 110000 |
/// | EERE | 1 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register Bytes high byte.
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

/// Timer/Counter Control Register A.
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
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
/// | CS0 | 111 |
/// | WGM02 | 1000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSTR | 10000 |
/// | CPOL | 1000 |
/// | SPE | 1000000 |
/// | DORD | 100000 |
/// | SPR | 11 |
/// | SPIE | 10000000 |
/// | CPHA | 100 |
pub const SPCR0: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
pub const SPSR0: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR0: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register-B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACOE | 1 |
pub const ACSRB: *mut u8 = 0x4F as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACIS | 11 |
/// | ACBG | 1000000 |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACIC | 100 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// Analog Comparator Control And Status Register-A.
pub const ACSRA: *mut u8 = 0x50 as *mut u8;

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
/// | BODS | 1000000 |
/// | IVCE | 1 |
/// | BODSE | 100000 |
/// | IVSEL | 10 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSRE | 10000 |
/// | SPMIE | 10000000 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
/// | SIGRD | 100000 |
/// | BLBSET | 1000 |
/// | PGWRT | 100 |
/// | PGERS | 10 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Stack Pointer low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | Z | 10 |
/// | H | 100000 |
/// | S | 10000 |
/// | T | 1000000 |
/// | C | 1 |
/// | V | 1000 |
/// | I | 10000000 |
/// | N | 100 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIF | 10000000 |
/// | WDCE | 10000 |
/// | WDIE | 1000000 |
/// | WDP | 100111 |
/// | WDE | 1000 |
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

/// XOSC Failure Detection Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XFDIF | 10 |
/// | XFDIE | 1 |
pub const XFDCSR: *mut u8 = 0x62 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUSART1 | 10000 |
/// | PRUSART0 | 10 |
/// | PRTIM0 | 100000 |
/// | PRADC | 1 |
/// | PRSPI0 | 100 |
/// | PRTWI0 | 10000000 |
/// | PRTIM1 | 1000 |
/// | PRTIM2 | 1000000 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Power Reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRPTC | 10000 |
/// | PRTIM4 | 1000 |
/// | PRTWI1 | 100000 |
/// | PRTIM3 | 1 |
/// | PRSPI1 | 100 |
pub const PRR1: *mut u8 = 0x65 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 1111 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC0 | 11 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

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
/// | TOIE0 | 1 |
/// | OCIE0A | 10 |
/// | OCIE0B | 100 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE1 | 100000 |
/// | OCIE1B | 100 |
/// | OCIE1A | 10 |
/// | TOIE1 | 1 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE2A | 10 |
/// | OCIE2B | 100 |
/// | TOIE2 | 1 |
pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE3B | 100 |
/// | ICIE3 | 100000 |
/// | OCIE3A | 10 |
/// | TOIE3 | 1 |
pub const TIMSK3: *mut u8 = 0x71 as *mut u8;

/// Timer/Counter4 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE4 | 1 |
/// | ICIE4 | 100000 |
/// | OCIE4B | 100 |
/// | OCIE4A | 10 |
pub const TIMSK4: *mut u8 = 0x72 as *mut u8;

/// Pin Change Mask Register 3.
pub const PCMSK3: *mut u8 = 0x73 as *mut u8;

/// ADC Data Register Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADPS | 111 |
/// | ADATE | 100000 |
/// | ADIF | 10000 |
/// | ADIE | 1000 |
/// | ADEN | 10000000 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// The ADC Control and Status register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADTS | 111 |
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
/// | MUX | 1111 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC4D | 10000 |
/// | ADC3D | 1000 |
/// | ADC5D | 100000 |
/// | ADC0D | 1 |
/// | ADC1D | 10 |
/// | ADC6D | 1000000 |
/// | ADC2D | 100 |
/// | ADC7D | 10000000 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 1.
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
/// | COM1A | 11000000 |
/// | COM1B | 110000 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC1 | 10000000 |
/// | ICES1 | 1000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1 Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1 Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1 Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter1 Input Capture Register Bytes high byte.
pub const ICR1H: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter1 Output Compare Register Bytes.
pub const OCR1A: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter1 Output Compare Register Bytes low byte.
pub const OCR1AL: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter1 Output Compare Register Bytes high byte.
pub const OCR1AH: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Output Compare Register Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter3 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM3 | 11 |
/// | COM3B | 110000 |
/// | COM3A | 11000000 |
pub const TCCR3A: *mut u8 = 0x90 as *mut u8;

/// Timer/Counter3 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES3 | 1000000 |
/// | WGM33 | 10000 |
/// | WGM32 | 1000 |
/// | CS3 | 111 |
/// | ICNC3 | 10000000 |
pub const TCCR3B: *mut u8 = 0x91 as *mut u8;

/// Timer/Counter3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3B | 1000000 |
/// | FOC3A | 10000000 |
pub const TCCR3C: *mut u8 = 0x92 as *mut u8;

/// Timer/Counter3 Bytes low byte.
pub const TCNT3L: *mut u8 = 0x94 as *mut u8;

/// Timer/Counter3 Bytes.
pub const TCNT3: *mut u16 = 0x94 as *mut u16;

/// Timer/Counter3 Bytes high byte.
pub const TCNT3H: *mut u8 = 0x95 as *mut u8;

/// Timer/Counter3 Input Capture Register Bytes.
pub const ICR3: *mut u16 = 0x96 as *mut u16;

/// Timer/Counter3 Input Capture Register Bytes low byte.
pub const ICR3L: *mut u8 = 0x96 as *mut u8;

/// Timer/Counter3 Input Capture Register Bytes high byte.
pub const ICR3H: *mut u8 = 0x97 as *mut u8;

/// Timer/Counter3 Output Compare Register Bytes low byte.
pub const OCR3AL: *mut u8 = 0x98 as *mut u8;

/// Timer/Counter3 Output Compare Register Bytes.
pub const OCR3A: *mut u16 = 0x98 as *mut u16;

/// Timer/Counter3 Output Compare Register Bytes high byte.
pub const OCR3AH: *mut u8 = 0x99 as *mut u8;

/// Timer/Counter3 Output Compare Register Bytes.
pub const OCR3B: *mut u16 = 0x9A as *mut u16;

/// Timer/Counter3 Output Compare Register Bytes low byte.
pub const OCR3BL: *mut u8 = 0x9A as *mut u8;

/// Timer/Counter3 Output Compare Register Bytes high byte.
pub const OCR3BH: *mut u8 = 0x9B as *mut u8;

/// Timer/Counter4 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM4A | 11000000 |
/// | WGM4 | 11 |
/// | COM4B | 110000 |
pub const TCCR4A: *mut u8 = 0xA0 as *mut u8;

/// Timer/Counter4 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC4 | 10000000 |
/// | WGM42 | 1000 |
/// | ICES4 | 1000000 |
/// | WGM43 | 10000 |
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
pub const TCCR4C: *mut u8 = 0xA2 as *mut u8;

/// Timer/Counter4 Bytes.
pub const TCNT4: *mut u16 = 0xA4 as *mut u16;

/// Timer/Counter4 Bytes low byte.
pub const TCNT4L: *mut u8 = 0xA4 as *mut u8;

/// Timer/Counter4 Bytes high byte.
pub const TCNT4H: *mut u8 = 0xA5 as *mut u8;

/// Timer/Counter4 Input Capture Register Bytes.
pub const ICR4: *mut u16 = 0xA6 as *mut u16;

/// Timer/Counter4 Input Capture Register Bytes low byte.
pub const ICR4L: *mut u8 = 0xA6 as *mut u8;

/// Timer/Counter4 Input Capture Register Bytes high byte.
pub const ICR4H: *mut u8 = 0xA7 as *mut u8;

/// Timer/Counter4 Output Compare Register Bytes low byte.
pub const OCR4AL: *mut u8 = 0xA8 as *mut u8;

/// Timer/Counter4 Output Compare Register Bytes.
pub const OCR4A: *mut u16 = 0xA8 as *mut u16;

/// Timer/Counter4 Output Compare Register Bytes high byte.
pub const OCR4AH: *mut u8 = 0xA9 as *mut u8;

/// Timer/Counter4 Output Compare Register Bytes low byte.
pub const OCR4BL: *mut u8 = 0xAA as *mut u8;

/// Timer/Counter4 Output Compare Register Bytes.
pub const OCR4B: *mut u16 = 0xAA as *mut u16;

/// Timer/Counter4 Output Compare Register Bytes high byte.
pub const OCR4BH: *mut u8 = 0xAB as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPHA1 | 100 |
/// | SPE1 | 1000000 |
/// | DORD1 | 100000 |
/// | CPOL1 | 1000 |
/// | SPR1 | 11 |
/// | MSTR1 | 10000 |
/// | SPIE1 | 10000000 |
pub const SPCR1: *mut u8 = 0xAC as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIF1 | 10000000 |
/// | WCOL1 | 1000000 |
/// | SPI2X1 | 1 |
pub const SPSR1: *mut u8 = 0xAD as *mut u8;

/// SPI Data Register.
pub const SPDR1: *mut u8 = 0xAE as *mut u8;

/// Timer/Counter2 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM2A | 11000000 |
/// | COM2B | 110000 |
/// | WGM2 | 11 |
pub const TCCR2A: *mut u8 = 0xB0 as *mut u8;

/// Timer/Counter2 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC2B | 1000000 |
/// | FOC2A | 10000000 |
/// | WGM22 | 1000 |
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
/// | OCR2AUB | 1000 |
/// | EXCLK | 1000000 |
/// | TCR2AUB | 10 |
/// | TCR2BUB | 1 |
/// | TCN2UB | 10000 |
/// | AS2 | 100000 |
/// | OCR2BUB | 100 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR0: *mut u8 = 0xB8 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWS | 11111000 |
/// | TWPS | 11 |
pub const TWSR0: *mut u8 = 0xB9 as *mut u8;

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWGCE | 1 |
/// | TWA | 11111110 |
pub const TWAR0: *mut u8 = 0xBA as *mut u8;

/// TWI Data register.
pub const TWDR0: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWEN | 100 |
/// | TWINT | 10000000 |
/// | TWWC | 1000 |
/// | TWEA | 1000000 |
/// | TWSTO | 10000 |
/// | TWIE | 1 |
/// | TWSTA | 100000 |
pub const TWCR0: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM | 11111110 |
pub const TWAMR0: *mut u8 = 0xBD as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXC0 | 1000000 |
/// | UPE0 | 100 |
/// | UDRE0 | 100000 |
/// | DOR0 | 1000 |
/// | FE0 | 10000 |
/// | MPCM0 | 1 |
/// | RXC0 | 10000000 |
/// | U2X0 | 10 |
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXCIE0 | 1000000 |
/// | RXEN0 | 10000 |
/// | UCSZ02 | 100 |
/// | TXB80 | 1 |
/// | RXCIE0 | 10000000 |
/// | TXEN0 | 1000 |
/// | UDRIE0 | 100000 |
/// | RXB80 | 10 |
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS0 | 1000 |
/// | UCPOL0 | 1 |
/// | UMSEL0 | 11000000 |
/// | UPM0 | 110000 |
/// | UCSZ0 | 110 |
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXS | 1000000 |
/// | RXSIE | 10000000 |
/// | SFDE | 100000 |
pub const UCSR0D: *mut u8 = 0xC3 as *mut u8;

/// USART Baud Rate Register Bytes.
pub const UBRR0: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register Bytes low byte.
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register Bytes high byte.
pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register 0.
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPE1 | 100 |
/// | RXC1 | 10000000 |
/// | FE1 | 10000 |
/// | U2X1 | 10 |
/// | DOR1 | 1000 |
/// | TXC1 | 1000000 |
/// | UDRE1 | 100000 |
/// | MPCM1 | 1 |
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UDRIE1 | 100000 |
/// | TXB81 | 1 |
/// | RXB81 | 10 |
/// | TXEN1 | 1000 |
/// | TXCIE1 | 1000000 |
/// | UCSZ12 | 100 |
/// | RXCIE1 | 10000000 |
/// | RXEN1 | 10000 |
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UMSEL1 | 11000000 |
/// | UCSZ1 | 110 |
/// | USBS1 | 1000 |
/// | UPM1 | 110000 |
/// | UCPOL1 | 1 |
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART Control and Status Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXS1 | 1000000 |
/// | RXSIE1 | 10000000 |
/// | SFDE1 | 100000 |
pub const UCSR1D: *mut u8 = 0xCB as *mut u8;

/// USART Baud Rate Register Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART Baud Rate Register Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART Baud Rate Register Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// TWI Bit Rate register.
pub const TWBR1: *mut u8 = 0xD8 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWPS1 | 11 |
/// | TWS1 | 11111000 |
pub const TWSR1: *mut u8 = 0xD9 as *mut u8;

/// TWI (Slave) Address register.
pub const TWAR1: *mut u8 = 0xDA as *mut u8;

/// TWI Data register.
pub const TWDR1: *mut u8 = 0xDB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWINT1 | 10000000 |
/// | TWEA1 | 1000000 |
/// | TWSTA1 | 100000 |
/// | TWEN1 | 100 |
/// | TWWC1 | 1000 |
/// | TWIE1 | 1 |
/// | TWSTO1 | 10000 |
pub const TWCR1: *mut u8 = 0xDC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM1 | 11111110 |
pub const TWAMR1: *mut u8 = 0xDD as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSRB`
pub const ACOE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ACME: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2AUB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const EXCLK: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2AUB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2BUB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN2UB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2BUB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const CFD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const BODS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const BODSE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PRR0`
pub const PRUSART1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRUSART0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRPTC: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM4: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTWI1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRSPI1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR0`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR0`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR0`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR0`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR0`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR0`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR0`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR1`
pub const CPHA1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR1`
pub const SPE1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR1`
pub const DORD1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR1`
pub const CPOL1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR1`
pub const SPR1: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR1`
pub const MSTR1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR1`
pub const SPIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPSR0`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR0`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR0`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR1`
pub const SPIF1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR1`
pub const WCOL1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR1`
pub const SPI2X1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const WGM2: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const WGM22: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const WGM3: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICES3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const WGM33: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const WGM32: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const CS3: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICNC3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const WGM4: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const ICNC4: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const WGM42: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const ICES4: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const WGM43: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const CS4: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR2`
pub const TOV2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR3`
pub const ICF3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR3`
pub const TOV3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR4`
pub const TOV4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR4`
pub const ICF4: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const TOIE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const ICIE3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const TOIE3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const TOIE4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const ICIE4: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TWAMR0`
pub const TWAM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAMR1`
pub const TWAM1: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR0`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWAR0`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR0`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWINT1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWEA1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWSTA1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWEN1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWWC1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR1`
pub const TWSTO1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWSR0`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TWSR0`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWSR1`
pub const TWPS1: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWSR1`
pub const TWS1: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const TXC0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UPE0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UDRE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const DOR0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const FE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const MPCM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const RXC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const U2X0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXEN0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXB80: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXEN0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXB80: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const USBS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UPM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR0D`
pub const RXS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0D`
pub const RXSIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0D`
pub const SFDE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UPE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const RXC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const FE1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const U2X1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const DOR1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const TXC1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UDRE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const MPCM1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXB81: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXB81: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXEN1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXEN1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const USBS1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UPM1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1D`
pub const RXS1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1D`
pub const RXSIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1D`
pub const SFDE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `XFDCSR`
pub const XFDIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `XFDCSR`
pub const XFDIE: *mut u8 = 0x1 as *mut u8;

/// Input Channel Selection
#[allow(non_upper_case_globals)]
pub mod adc_mux_single {
   /// ADC Single Ended Input pin 0.
   pub const ADC0: u32 = 0x0;
   /// ADC Single Ended Input pin 1.
   pub const ADC1: u32 = 0x1;
   /// ADC Single Ended Input pin 2.
   pub const ADC2: u32 = 0x2;
   /// ADC Single Ended Input pin 3.
   pub const ADC3: u32 = 0x3;
   /// ADC Single Ended Input pin 4.
   pub const ADC4: u32 = 0x4;
   /// ADC Single Ended Input pin 5.
   pub const ADC5: u32 = 0x5;
   /// ADC Single Ended Input pin 6.
   pub const ADC6: u32 = 0x6;
   /// ADC Single Ended Input pin 7.
   pub const ADC7: u32 = 0x7;
   /// Temperature sensor.
   pub const TEMPSENS: u32 = 0x8;
   /// Internal Reference (VBG).
   pub const ADC_VBG: u32 = 0xE;
   /// 0V (GND).
   pub const ADC_GND: u32 = 0xF;
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

/// ADC Voltage Reference Selection
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref3 {
   /// AREF, Internal Vref turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVCC with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal 1.1V Voltage Reference with external capacitor at AREF pin.
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
   /// fosc/2 or fosc/4.
   pub const VAL_0x00: u32 = 0x0;
   /// fosc/8 or fosc/16.
   pub const VAL_0x01: u32 = 0x1;
   /// fosc/32 or fosc/64.
   pub const VAL_0x02: u32 = 0x2;
   /// fosc/64 or fosc/128.
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

/// `CPU_SLEEP_MODE_3BITS2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits2 {
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
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
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

/// `ENUM_CFD` value group
#[allow(non_upper_case_globals)]
pub mod enum_cfd {
   /// Disabled.
   pub const CFD_DISABLED: u32 = 0x0;
   /// Enabled.
   pub const CFD_ENABLED: u32 = 0x1;
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
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_4MS1: u32 = 0x12;
   /// Int. RC Osc. 8 MHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms.
   pub const INTRCOSC_8MHZ_6CK_14CK_65MS: u32 = 0x22;
   /// Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 0 ms.
   pub const INTRCOSC_128KHZ_6CK_14CK_0MS: u32 = 0x3;
   /// Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 4.1 ms.
   pub const INTRCOSC_128KHZ_6CK_14CK_4MS1: u32 = 0x13;
   /// Int. RC Osc. 128kHz; Start-up time PWRDWN/RESET: 6 CK/14 CK + 65 ms.
   pub const INTRCOSC_128KHZ_6CK_14CK_65MS: u32 = 0x23;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const EXTLOFXTAL_1KCK_14CK_0MS: u32 = 0x4;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4.1 ms.
   pub const EXTLOFXTAL_1KCK_14CK_4MS1: u32 = 0x14;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 1K CK/14 CK + 65 ms.
   pub const EXTLOFXTAL_1KCK_14CK_65MS: u32 = 0x24;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/14 CK + 0 ms.
   pub const EXTLOFXTAL_32KCK_14CK_0MS: u32 = 0x5;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/14 CK + 4.1 ms.
   pub const EXTLOFXTAL_32KCK_14CK_4MS1: u32 = 0x15;
   /// Ext. Low-Freq. Crystal; Start-up time PWRDWN/RESET: 32K CK/14 CK + 65 ms.
   pub const EXTLOFXTAL_32KCK_14CK_65MS: u32 = 0x25;
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

