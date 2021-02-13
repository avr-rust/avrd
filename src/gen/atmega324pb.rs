//! The AVR ATmega324PB microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega324PB-AU | TQFP44 | TQFP44 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-AUR | TQFP44 | TQFP44 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-MU | VQFN44 | VQFN44 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-MUR | VQFN44 | VQFN44 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-AN | TQFP44 | TQFP44 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-ANR | TQFP44 | TQFP44 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-MN | VQFN44 | VQFN44 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega324PB-MNR | VQFN44 | VQFN44 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

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
/// | SPIEN | 100000 |
/// | BOOTRST | 1 |
/// | WDTON | 10000 |
/// | EESAVE | 1000 |
/// | OCDEN | 10000000 |
/// | BOOTSZ | 110 |
/// | JTAGEN | 1000000 |
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
/// | OCF1B | 100 |
/// | TOV1 | 1 |
/// | ICF1 | 100000 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV2 | 1 |
/// | OCF2B | 100 |
/// | OCF2A | 10 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF3A | 10 |
/// | ICF3 | 100000 |
/// | TOV3 | 1 |
/// | OCF3B | 100 |
pub const TIFR3: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF4 | 100000 |
/// | OCF4B | 100 |
/// | OCF4A | 10 |
/// | TOV4 | 1 |
pub const TIFR4: *mut u8 = 0x39 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 11111 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 111 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 111 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPM | 110000 |
/// | EEMPE | 100 |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | EEPE | 10 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register Low Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register Low Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register Low Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSRSYNC | 1 |
/// | PSRASY | 10 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 11000000 |
/// | WGM0 | 11 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0A | 10000000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
/// | WGM02 | 1000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x4A as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
pub const SPCR0: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
pub const SPSR0: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR0: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register B.
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
/// | ACD | 10000000 |
/// | ACO | 100000 |
/// | ACI | 10000 |
/// | ACIS | 11 |
/// | ACBG | 1000000 |
/// | ACIC | 100 |
/// | ACIE | 1000 |
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
/// | EXTRF | 10 |
/// | PORF | 1 |
/// | BORF | 100 |
/// | WDRF | 1000 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODS | 1000000 |
/// | IVCE | 1 |
/// | IVSEL | 10 |
/// | JTD | 10000000 |
/// | BODSE | 100000 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | SIGRD | 100000 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
/// | PGWRT | 100 |
/// | RWWSRE | 10000 |
/// | SPMIE | 10000000 |
/// | BLBSET | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

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
/// | N | 100 |
/// | S | 10000 |
/// | V | 1000 |
/// | H | 100000 |
/// | Z | 10 |
/// | C | 1 |
/// | T | 1000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
/// | WDP | 100111 |
/// | WDCE | 10000 |
/// | WDIF | 10000000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// `CLKPR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
/// | CLKPCE | 10000000 |
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

/// Power Reduction Register2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTWI1 | 1 |
/// | PRUSART2 | 100 |
/// | PRSPI1 | 10 |
/// | PRPTC | 1000 |
pub const PRR2: *mut u8 = 0x63 as *mut u8;

/// Power Reduction Register0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 100000 |
/// | PRUSART0 | 10 |
/// | PRUSART1 | 10000 |
/// | PRTWI0 | 10000000 |
/// | PRTIM1 | 1000 |
/// | PRSPI0 | 100 |
/// | PRADC | 1 |
/// | PRTIM2 | 1000000 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Power Reduction Register1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM4 | 10 |
/// | PRTIM3 | 1 |
pub const PRR1: *mut u8 = 0x65 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 11111 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC2 | 110000 |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
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
/// | OCIE1A | 10 |
/// | OCIE1B | 100 |
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
/// | OCIE3A | 10 |
/// | OCIE3B | 100 |
/// | TOIE3 | 1 |
/// | ICIE3 | 100000 |
pub const TIMSK3: *mut u8 = 0x71 as *mut u8;

/// Timer/Counter4 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE4A | 10 |
/// | OCIE4B | 100 |
/// | TOIE4 | 1 |
/// | ICIE4 | 100000 |
pub const TIMSK4: *mut u8 = 0x72 as *mut u8;

/// Pin Change Mask Register 3.
pub const PCMSK3: *mut u8 = 0x73 as *mut u8;

/// Pin Change Mask Register 4.
pub const PCMSK4: *mut u8 = 0x75 as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// ADC Control and Status register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000000 |
/// | ADIE | 1000 |
/// | ADATE | 100000 |
/// | ADIF | 10000 |
/// | ADPS | 111 |
/// | ADSC | 1000000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GPIOEN | 10000000 |
/// | ADTS | 111 |
/// | ACME | 1000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 100000 |
/// | MUX | 11111 |
/// | REFS | 11000000 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC0D | 1 |
/// | ADC6D | 1000000 |
/// | ADC3D | 1000 |
/// | ADC4D | 10000 |
/// | ADC7D | 10000000 |
/// | ADC2D | 100 |
/// | ADC5D | 100000 |
/// | ADC1D | 10 |
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
/// | COM1A | 11000000 |
/// | COM1B | 110000 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 1000000 |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1A | 10000000 |
/// | FOC1B | 1000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

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

/// Timer/Counter1 Output Compare Register B  Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register B  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register B  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter3 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM3A | 11000000 |
/// | COM3B | 110000 |
pub const TCCR3A: *mut u8 = 0x90 as *mut u8;

/// Timer/Counter3 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES3 | 1000000 |
/// | CS3 | 111 |
/// | ICNC3 | 10000000 |
pub const TCCR3B: *mut u8 = 0x91 as *mut u8;

/// Timer/Counter3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3A | 10000000 |
/// | FOC3B | 1000000 |
pub const TCCR3C: *mut u8 = 0x92 as *mut u8;

/// Timer/Counter3 Bytes.
pub const TCNT3: *mut u16 = 0x94 as *mut u16;

/// Timer/Counter3 Bytes low byte.
pub const TCNT3L: *mut u8 = 0x94 as *mut u8;

/// Timer/Counter3 Bytes high byte.
pub const TCNT3H: *mut u8 = 0x95 as *mut u8;

/// Timer/Counter3 Input Capture Register Bytes.
pub const ICR3: *mut u16 = 0x96 as *mut u16;

/// Timer/Counter3 Input Capture Register Bytes low byte.
pub const ICR3L: *mut u8 = 0x96 as *mut u8;

/// Timer/Counter3 Input Capture Register Bytes high byte.
pub const ICR3H: *mut u8 = 0x97 as *mut u8;

/// Timer/Counter3 Output Compare Register A Bytes.
pub const OCR3A: *mut u16 = 0x98 as *mut u16;

/// Timer/Counter3 Output Compare Register A Bytes low byte.
pub const OCR3AL: *mut u8 = 0x98 as *mut u8;

/// Timer/Counter3 Output Compare Register A Bytes high byte.
pub const OCR3AH: *mut u8 = 0x99 as *mut u8;

/// Timer/Counter3 Output Compare Register B Bytes.
pub const OCR3B: *mut u16 = 0x9A as *mut u16;

/// Timer/Counter3 Output Compare Register B Bytes low byte.
pub const OCR3BL: *mut u8 = 0x9A as *mut u8;

/// Timer/Counter3 Output Compare Register B Bytes high byte.
pub const OCR3BH: *mut u8 = 0x9B as *mut u8;

/// Timer/Counter4 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM4A | 11000000 |
/// | COM4B | 110000 |
pub const TCCR4A: *mut u8 = 0xA0 as *mut u8;

/// Timer/Counter4 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES4 | 1000000 |
/// | CS4 | 111 |
/// | ICNC4 | 10000000 |
pub const TCCR4B: *mut u8 = 0xA1 as *mut u8;

/// Timer/Counter4 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC4B | 1000000 |
/// | FOC4A | 10000000 |
pub const TCCR4C: *mut u8 = 0xA2 as *mut u8;

/// Timer/Counter4 Bytes low byte.
pub const TCNT4L: *mut u8 = 0xA4 as *mut u8;

/// Timer/Counter4 Bytes.
pub const TCNT4: *mut u16 = 0xA4 as *mut u16;

/// Timer/Counter4 Bytes high byte.
pub const TCNT4H: *mut u8 = 0xA5 as *mut u8;

/// Timer/Counter4 Input Capture Register Bytes.
pub const ICR4: *mut u16 = 0xA6 as *mut u16;

/// Timer/Counter4 Input Capture Register Bytes low byte.
pub const ICR4L: *mut u8 = 0xA6 as *mut u8;

/// Timer/Counter4 Input Capture Register Bytes high byte.
pub const ICR4H: *mut u8 = 0xA7 as *mut u8;

/// Timer/Counter4 Output Compare Register A Bytes.
pub const OCR4A: *mut u16 = 0xA8 as *mut u16;

/// Timer/Counter4 Output Compare Register A Bytes low byte.
pub const OCR4AL: *mut u8 = 0xA8 as *mut u8;

/// Timer/Counter4 Output Compare Register A Bytes high byte.
pub const OCR4AH: *mut u8 = 0xA9 as *mut u8;

/// Timer/Counter4 Output Compare Register B Bytes.
pub const OCR4B: *mut u16 = 0xAA as *mut u16;

/// Timer/Counter4 Output Compare Register B Bytes low byte.
pub const OCR4BL: *mut u8 = 0xAA as *mut u8;

/// Timer/Counter4 Output Compare Register B Bytes high byte.
pub const OCR4BH: *mut u8 = 0xAB as *mut u8;

/// SPI Control Register.
pub const SPCR1: *mut u8 = 0xAC as *mut u8;

/// SPI Status Register.
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
/// | FOC2A | 10000000 |
/// | CS2 | 111 |
/// | WGM22 | 1000 |
/// | FOC2B | 1000000 |
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
/// | EXCLK | 1000000 |
/// | OCR2AUB | 1000 |
/// | OCR2BUB | 100 |
/// | AS2 | 100000 |
/// | TCR2AUB | 10 |
/// | TCR2BUB | 1 |
/// | TCN2UB | 10000 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR0: *mut u8 = 0xB8 as *mut u8;

/// TWI Status Register.
pub const TWSR0: *mut u8 = 0xB9 as *mut u8;

/// TWI (Slave) Address register.
pub const TWAR0: *mut u8 = 0xBA as *mut u8;

/// TWI Data register.
pub const TWDR0: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
pub const TWCR0: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM0 | 11111110 |
pub const TWAMR0: *mut u8 = 0xBD as *mut u8;

/// USART Control and Status Register A.
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

/// USART Control and Status Register B.
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

/// USART Control and Status Register C.
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

/// USART Control and Status Register D.
pub const UCSR0D: *mut u8 = 0xC3 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR0: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

/// USART Control and Status Register A.
pub const UCSR1A: *mut u8 = 0xC8 as *mut u8;

/// USART Control and Status Register B.
pub const UCSR1B: *mut u8 = 0xC9 as *mut u8;

/// USART Control and Status Register C.
pub const UCSR1C: *mut u8 = 0xCA as *mut u8;

/// USART Control and Status Register D.
pub const UCSR1D: *mut u8 = 0xCB as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR1L: *mut u8 = 0xCC as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR1: *mut u16 = 0xCC as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR1H: *mut u8 = 0xCD as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0xCE as *mut u8;

/// USART Control and Status Register A.
pub const UCSR2A: *mut u8 = 0xD0 as *mut u8;

/// USART Control and Status Register B.
pub const UCSR2B: *mut u8 = 0xD1 as *mut u8;

/// USART Control and Status Register C.
pub const UCSR2C: *mut u8 = 0xD2 as *mut u8;

/// USART Control and Status Register D.
pub const UCSR2D: *mut u8 = 0xD3 as *mut u8;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR2L: *mut u8 = 0xD4 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR2: *mut u16 = 0xD4 as *mut u16;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR2H: *mut u8 = 0xD5 as *mut u8;

/// USART I/O Data Register.
pub const UDR2: *mut u8 = 0xD6 as *mut u8;

/// TWI Bit Rate register.
pub const TWBR1: *mut u8 = 0xD8 as *mut u8;

/// TWI Status Register.
pub const TWSR1: *mut u8 = 0xD9 as *mut u8;

/// TWI (Slave) Address register.
pub const TWAR1: *mut u8 = 0xDA as *mut u8;

/// TWI Data register.
pub const TWDR1: *mut u8 = 0xDB as *mut u8;

/// TWI Control Register.
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
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSRB`
pub const ACOE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const GPIOEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ACME: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ASSR`
pub const EXCLK: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2AUB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2BUB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2AUB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2BUB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN2UB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const CFD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRASY: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x40 as *mut u8;

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
pub const BODS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const JTD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const BODSE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRUSART0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRUSART1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM4: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRTIM3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRTWI1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRUSART2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSPI1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRPTC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

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
pub const COM2A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const WGM2: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const WGM22: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2B`
pub const FOC2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR3A`
pub const COM3B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICES3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const CS3: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR3B`
pub const ICNC3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR4A`
pub const COM4B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const ICES4: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const CS4: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR4B`
pub const ICNC4: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR4C`
pub const FOC4A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR2`
pub const TOV2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR3`
pub const ICF3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR3`
pub const TOV3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR3`
pub const OCF3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR4`
pub const ICF4: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR4`
pub const OCF4A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR4`
pub const TOV4: *mut u8 = 0x1 as *mut u8;

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
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const TOIE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const OCIE3B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const TOIE3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK3`
pub const ICIE3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const OCIE4B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const TOIE4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK4`
pub const ICIE4: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWAMR0`
pub const TWAM0: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAMR1`
pub const TWAM1: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `XFDCSR`
pub const XFDIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `XFDCSR`
pub const XFDIE: *mut u8 = 0x1 as *mut u8;

/// `ADC_MUX_DIFF` value group
#[allow(non_upper_case_globals)]
pub mod adc_mux_diff {
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
   /// ADC Differential Inputs Postive pin 0 Negative pin 0 10x Gain.
   pub const ADC0_ADC0_10X: u32 = 0x8;
   /// ADC Differential Inputs Postive pin 1 Negative pin 0 10x Gain.
   pub const ADC1_ADC0_10X: u32 = 0x9;
   /// ADC Differential Inputs Postive pin 0 Negative pin 0 200x Gain.
   pub const ADC0_ADC0_200x: u32 = 0xA;
   /// ADC Differential Inputs Postive pin 1 Negative pin 0 200x Gain.
   pub const ADC1_ADC0_200X: u32 = 0xB;
   /// ADC Differential Inputs Postive pin 2 Negative pin 2 10x Gain.
   pub const ADC2_ADC2_10X: u32 = 0xC;
   /// ADC Differential Inputs Postive pin 3 Negative pin 2 10x Gain.
   pub const ADC3_ADC2_10X: u32 = 0xD;
   /// ADC Differential Inputs Postive pin 2 Negative pin 2 200x Gain.
   pub const ADC2_ADC2_200X: u32 = 0xE;
   /// ADC Differential Inputs Postive pin 3 Negative pin 2 200x Gain.
   pub const ADC3_ADC2_200X: u32 = 0xF;
   /// ADC Differential Inputs Postive pin 0 Negative pin 1 1x Gain.
   pub const ADC0_ADC1_1X: u32 = 0x10;
   /// ADC Differential Inputs Postive pin 1 Negative pin 1 1x Gain.
   pub const ADC1_ADC1_1X: u32 = 0x11;
   /// ADC Differential Inputs Postive pin 2 Negative pin 1 1x Gain.
   pub const ADC2_ADC1_1X: u32 = 0x12;
   /// ADC Differential Inputs Postive pin 3 Negative pin 1 1x Gain.
   pub const ADC3_ADC1_1X: u32 = 0x13;
   /// ADC Differential Inputs Postive pin 4 Negative pin 1 1x Gain.
   pub const ADC4_ADC1_1X: u32 = 0x14;
   /// ADC Differential Inputs Postive pin 5 Negative pin 1 1x Gain.
   pub const ADC5_ADC1_1X: u32 = 0x15;
   /// ADC Differential Inputs Postive pin 6 Negative pin 1 1x Gain.
   pub const ADC6_ADC1_1X: u32 = 0x16;
   /// ADC Differential Inputs Postive pin 7 Negative pin 1 1x Gain.
   pub const ADC7_ADC1_1X: u32 = 0x17;
   /// ADC Differential Inputs Postive pin 0 Negative pin 2 1x Gain.
   pub const ADC0_ADC2_1X: u32 = 0x18;
   /// ADC Differential Inputs Postive pin 1 Negative pin 2 1x Gain.
   pub const ADC1_ADC2_1X: u32 = 0x19;
   /// ADC Differential Inputs Postive pin 2 Negative pin 2 1x Gain.
   pub const ADC2_ADC2_1X: u32 = 0x1A;
   /// ADC Differential Inputs Postive pin 3 Negative pin 2 1x Gain.
   pub const ADC3_ADC2_1X: u32 = 0x1B;
   /// ADC Differential Inputs Postive pin 4 Negative pin 2 1x Gain.
   pub const ADC4_ADC2_1X: u32 = 0x1C;
   /// ADC Differential Inputs Postive pin 5 Negative pin 2 1x Gain.
   pub const ADC5_ADC2_1X: u32 = 0x1D;
   /// Internal Reference (VBG).
   pub const ADC_VBG: u32 = 0x1E;
   /// 0V (GND).
   pub const ADC_GND: u32 = 0x1F;
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

/// `ANALOG_ADC_V_REF6` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref6 {
   /// AREF, Internal Vref turned off.
   pub const VAL_0x00: u32 = 0x0;
   /// AVCC with external capacitor at AREF pin.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal 1.1V Voltage Reference with external capacitor at AREF pin.
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

/// `COMM_TWI_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod comm_twi_prescale {
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
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x7;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x6;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
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

