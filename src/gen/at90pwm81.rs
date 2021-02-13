//! The AVR AT90PWM81 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.7V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
/// | BLB0 | 1100 |
/// | BLB1 | 110000 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKDIV8 | 10000000 |
/// | SUT_CKSEL | 111111 |
/// | CKOUT | 1000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIEN | 100000 |
/// | EESAVE | 1000 |
/// | DWEN | 1000000 |
/// | BOOTSZ | 110 |
/// | BOOTRST | 1 |
/// | RSTDISBL | 10000000 |
/// | WDTON | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSCINRB | 1000 |
/// | PSC2RB | 10000000 |
/// | PSC0RB | 100000 |
/// | BODLEVEL | 111 |
/// | PSC2RBA | 1000000 |
/// | PSCRV | 10000 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Analog Comparator Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC3O | 1000 |
/// | AC2IF | 1000000 |
/// | AC1O | 10 |
/// | AC1IF | 100000 |
/// | AC2O | 100 |
/// | AC3IF | 10000000 |
pub const ACSR: *mut u8 = 0x20 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 1 |
/// | ICIE1 | 100000 |
pub const TIMSK1: *mut u8 = 0x21 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF1 | 100000 |
/// | TOV1 | 1 |
pub const TIFR1: *mut u8 = 0x22 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADPS | 111 |
/// | ADIE | 1000 |
/// | ADEN | 10000000 |
/// | ADIF | 10000 |
/// | ADSC | 1000000 |
/// | ADATE | 100000 |
pub const ADCSRA: *mut u8 = 0x26 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADSSEN | 10000 |
/// | ADHSM | 10000000 |
/// | ADNCDIS | 1000000 |
/// | ADTS | 1111 |
pub const ADCSRB: *mut u8 = 0x27 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADLAR | 100000 |
/// | REFS | 11000000 |
/// | MUX | 1111 |
pub const ADMUX: *mut u8 = 0x28 as *mut u8;

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

/// PSC0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEVE0A | 1000 |
/// | PEVE0B | 10000 |
/// | PEOPE0 | 1 |
/// | PEOEPE0 | 10 |
pub const PIM0: *mut u8 = 0x2F as *mut u8;

/// PSC0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEOP0 | 1 |
/// | POAC0A | 1000000 |
/// | PRN0 | 110 |
/// | PEV0A | 1000 |
/// | POAC0B | 10000000 |
/// | PEV0B | 10000 |
pub const PIFR0: *mut u8 = 0x30 as *mut u8;

/// PSC 0 Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCLKSEL0 | 10 |
/// | PMODE0 | 11000 |
/// | PALOCK0 | 1000000 |
/// | POP0 | 100 |
/// | PFIFTY0 | 10000000 |
/// | PLOCK0 | 100000 |
pub const PCNF0: *mut u8 = 0x31 as *mut u8;

/// PSC 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PAOC0B | 10000 |
/// | PCCYC0 | 10 |
/// | PAOC0A | 1000 |
/// | PPRE0 | 11000000 |
/// | PRUN0 | 1 |
/// | PBFM0 | 100100 |
pub const PCTL0: *mut u8 = 0x32 as *mut u8;

/// PSC2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEVE2B | 10000 |
/// | PEOPE2 | 1 |
/// | PEVE2A | 1000 |
/// | PSEIE2 | 100000 |
/// | PEOEPE2 | 10 |
pub const PIM2: *mut u8 = 0x33 as *mut u8;

/// PSC2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEOP2 | 1 |
/// | PRN2 | 110 |
/// | POAC2A | 1000000 |
/// | PEV2A | 1000 |
/// | POAC2B | 10000000 |
/// | PEV2B | 10000 |
/// | PSEI2 | 100000 |
pub const PIFR2: *mut u8 = 0x34 as *mut u8;

/// PSC 2 Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POP2 | 100 |
/// | PCLKSEL2 | 10 |
/// | PALOCK2 | 1000000 |
/// | PLOCK2 | 100000 |
/// | PFIFTY2 | 10000000 |
/// | PMODE2 | 11000 |
/// | POME2 | 1 |
pub const PCNF2: *mut u8 = 0x35 as *mut u8;

/// PSC 2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PPRE2 | 11000000 |
/// | PBFM2 | 100000 |
/// | PAOC2B | 10000 |
/// | PRUN2 | 1 |
/// | PAOC2A | 1000 |
/// | PCCYC2 | 10 |
/// | PARUN2 | 100 |
pub const PCTL2: *mut u8 = 0x36 as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSTR | 10000 |
/// | SPIE | 10000000 |
/// | SPR | 11 |
/// | DORD | 100000 |
/// | CPOL | 1000 |
/// | SPE | 1000000 |
/// | CPHA | 100 |
pub const SPCR: *mut u8 = 0x37 as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
pub const SPSR: *mut u8 = 0x38 as *mut u8;

/// General Purpose IO Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GPIOR05 | 100000 |
/// | GPIOR06 | 1000000 |
/// | GPIOR03 | 1000 |
/// | GPIOR00 | 1 |
/// | GPIOR01 | 10 |
/// | GPIOR07 | 10000000 |
/// | GPIOR04 | 10000 |
/// | GPIOR02 | 100 |
pub const GPIOR0: *mut u8 = 0x39 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x3A as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x3B as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEPAGE | 1000000 |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | NVMBSY | 10000000 |
/// | EEWE | 10 |
/// | EEMWE | 100 |
/// | EEPM | 110000 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Read/Write Access Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Read/Write Access Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF | 111 |
pub const EIFR: *mut u8 = 0x40 as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 111 |
pub const EIMSK: *mut u8 = 0x41 as *mut u8;

/// Output Compare SB Register.
pub const OCR0SB: *mut u16 = 0x42 as *mut u16;

/// Output Compare SB Register  low byte.
pub const OCR0SBL: *mut u8 = 0x42 as *mut u8;

/// Output Compare SB Register  high byte.
pub const OCR0SBH: *mut u8 = 0x43 as *mut u8;

/// Output Compare RB Register.
pub const OCR0RB: *mut u16 = 0x44 as *mut u16;

/// Output Compare RB Register  low byte.
pub const OCR0RBL: *mut u8 = 0x44 as *mut u8;

/// Output Compare RB Register  high byte.
pub const OCR0RBH: *mut u8 = 0x45 as *mut u8;

/// Output Compare SB Register.
pub const OCR2SB: *mut u16 = 0x46 as *mut u16;

/// Output Compare SB Register  low byte.
pub const OCR2SBL: *mut u8 = 0x46 as *mut u8;

/// Output Compare SB Register  high byte.
pub const OCR2SBH: *mut u8 = 0x47 as *mut u8;

/// Output Compare RB Register  low byte.
pub const OCR2RBL: *mut u8 = 0x48 as *mut u8;

/// Output Compare RB Register.
pub const OCR2RB: *mut u16 = 0x48 as *mut u16;

/// Output Compare RB Register  high byte.
pub const OCR2RBH: *mut u8 = 0x49 as *mut u8;

/// Output Compare RA Register.
pub const OCR0RA: *mut u16 = 0x4A as *mut u16;

/// Output Compare RA Register  low byte.
pub const OCR0RAL: *mut u8 = 0x4A as *mut u8;

/// Output Compare RA Register  high byte.
pub const OCR0RAH: *mut u8 = 0x4B as *mut u8;

/// ADC Data Register Bytes.
pub const ADC: *mut u16 = 0x4C as *mut u16;

/// ADC Data Register Bytes low byte.
pub const ADCL: *mut u8 = 0x4C as *mut u8;

/// ADC Data Register Bytes high byte.
pub const ADCH: *mut u8 = 0x4D as *mut u8;

/// Output Compare RA Register  low byte.
pub const OCR2RAL: *mut u8 = 0x4E as *mut u8;

/// Output Compare RA Register.
pub const OCR2RA: *mut u16 = 0x4E as *mut u16;

/// Output Compare RA Register  high byte.
pub const OCR2RAH: *mut u8 = 0x4F as *mut u8;

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
/// | WDRF | 1000 |
/// | EXTRF | 10 |
/// | BORF | 100 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVSEL | 10 |
/// | IVCE | 1 |
/// | PUD | 10000 |
/// | RSTDIS | 1000 |
/// | CKRC81 | 100 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGERS | 10 |
/// | BLBSET | 1000 |
/// | SPMIE | 10000000 |
/// | RWWSRE | 10000 |
/// | SIGRD | 100000 |
/// | SPMEN | 1 |
/// | RWWSB | 1000000 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// DAC Data Register low byte.
pub const DACL: *mut u8 = 0x58 as *mut u8;

/// DAC Data Register.
pub const DAC: *mut u16 = 0x58 as *mut u16;

/// DAC Data Register high byte.
pub const DACH: *mut u8 = 0x59 as *mut u8;

/// Timer/Counter1 Bytes.
pub const TCNT1: *mut u16 = 0x5A as *mut u16;

/// Timer/Counter1 Bytes low byte.
pub const TCNT1L: *mut u8 = 0x5A as *mut u8;

/// Timer/Counter1 Bytes high byte.
pub const TCNT1H: *mut u8 = 0x5B as *mut u8;

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
/// | C | 1 |
/// | T | 1000000 |
/// | S | 10000 |
/// | N | 100 |
/// | H | 100000 |
/// | Z | 10 |
/// | V | 1000 |
/// | I | 10000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Output Compare SA Register.
pub const OCR0SA: *mut u16 = 0x60 as *mut u16;

/// Output Compare SA Register  low byte.
pub const OCR0SAL: *mut u8 = 0x60 as *mut u8;

/// Output Compare SA Register  high byte.
pub const OCR0SAH: *mut u8 = 0x61 as *mut u8;

/// PSC 0 Input A Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRFM0A | 1111 |
/// | PFLTE0A | 10000 |
/// | PELEV0A | 100000 |
/// | PCAE0A | 10000000 |
/// | PISEL0A | 1000000 |
pub const PFRC0A: *mut u8 = 0x62 as *mut u8;

/// PSC 0 Input B Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRFM0B | 1111 |
/// | PCAE0B | 10000000 |
/// | PELEV0B | 100000 |
/// | PFLTE0B | 10000 |
/// | PISEL0B | 1000000 |
pub const PFRC0B: *mut u8 = 0x63 as *mut u8;

/// Output Compare SA Register.
pub const OCR2SA: *mut u16 = 0x64 as *mut u16;

/// Output Compare SA Register  low byte.
pub const OCR2SAL: *mut u8 = 0x64 as *mut u8;

/// Output Compare SA Register  high byte.
pub const OCR2SAH: *mut u8 = 0x65 as *mut u8;

/// PSC 2 Input B Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PELEV2A | 100000 |
/// | PISEL2A | 1000000 |
/// | PRFM2A | 1111 |
/// | PFLTE2A | 10000 |
/// | PCAE2A | 10000000 |
pub const PFRC2A: *mut u8 = 0x66 as *mut u8;

/// PSC 2 Input B Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PFLTE2B | 10000 |
/// | PRFM2B | 1111 |
/// | PCAE2B | 10000000 |
/// | PELEV2B | 100000 |
/// | PISEL2B | 1000000 |
pub const PFRC2B: *mut u8 = 0x67 as *mut u8;

/// PSC 0 Input Capture Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCST0 | 1000000000000000 |
pub const PICR0: *mut u16 = 0x68 as *mut u16;

/// PSC 0 Input Capture Register  low byte.
pub const PICR0L: *mut u8 = 0x68 as *mut u8;

/// PSC 0 Input Capture Register  high byte.
pub const PICR0H: *mut u8 = 0x69 as *mut u8;

/// PSC0 Synchro and Output Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PISEL0B1 | 1000000 |
/// | PISEL0A1 | 10000000 |
/// | PSYNC0 | 110000 |
/// | POEN0A | 1 |
/// | POEN0B | 100 |
pub const PSOC0: *mut u8 = 0x6A as *mut u8;

/// PSC 2 Input Capture Register low byte.
pub const PICR2L: *mut u8 = 0x6C as *mut u8;

/// PSC 2 Input Capture Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCST2 | 1000000000000000 |
pub const PICR2: *mut u16 = 0x6C as *mut u16;

/// PSC 2 Input Capture Register high byte.
pub const PICR2H: *mut u8 = 0x6D as *mut u8;

/// PSC2 Synchro and Output Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POS2 | 11000000 |
/// | POEN2A | 1 |
/// | POEN2B | 100 |
/// | POEN2C | 10 |
/// | PSYNC2 | 110000 |
/// | POEN2D | 1000 |
pub const PSOC2: *mut u8 = 0x6E as *mut u8;

/// PSC 2 Output Matrix.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POMV2A | 1111 |
/// | POMV2B | 11110000 |
pub const POM2: *mut u8 = 0x6F as *mut u8;

/// PSC 2 Enhanced Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PISEL2B1 | 1 |
/// | PASDLK2 | 11100000 |
/// | PISEL2A1 | 10 |
/// | PELEV2B1 | 100 |
/// | PBFM21 | 10000 |
/// | PELEV2A1 | 1000 |
pub const PCNFE2: *mut u8 = 0x70 as *mut u8;

/// Analog Synchronization Delay Register.
pub const PASDLY2: *mut u8 = 0x71 as *mut u8;

/// DAC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DALA | 100 |
/// | DAEN | 1 |
/// | DATS | 1110000 |
/// | DAATE | 10000000 |
pub const DACON: *mut u8 = 0x76 as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC2D | 100 |
/// | ADC4D | 10000 |
/// | ADC0D | 1 |
/// | ADC8D | 10000000 |
/// | ADC7D | 1000000 |
/// | ADC5D | 100000 |
/// | ADC3D | 1000 |
/// | ADC1D | 10 |
pub const DIDR0: *mut u8 = 0x77 as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMP0PD | 100 |
/// | ADC10D | 10 |
/// | ADC9D | 1 |
/// | ACMP1MD | 1000 |
pub const DIDR1: *mut u8 = 0x78 as *mut u8;

/// `AMP0CSR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMP0IS | 1000000 |
/// | AMP0G | 110000 |
/// | AMP0GS | 1000 |
/// | AMP0TS | 11 |
/// | AMP0EN | 10000000 |
pub const AMP0CSR: *mut u8 = 0x79 as *mut u8;

/// `AC1ECON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC1ICE | 1000 |
/// | AC1OE | 10000 |
/// | AC1OI | 100000 |
/// | AC1H | 111 |
pub const AC1ECON: *mut u8 = 0x7A as *mut u8;

/// `AC2ECON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC2OE | 10000 |
/// | AC2OI | 100000 |
/// | AC2H | 111 |
pub const AC2ECON: *mut u8 = 0x7B as *mut u8;

/// `AC3ECON` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC3OI | 100000 |
/// | AC3OE | 10000 |
/// | AC3H | 111 |
pub const AC3ECON: *mut u8 = 0x7C as *mut u8;

/// Analog Comparator 1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC1M | 111 |
/// | AC1EN | 10000000 |
/// | AC1IS | 110000 |
/// | AC1IE | 1000000 |
pub const AC1CON: *mut u8 = 0x7D as *mut u8;

/// Analog Comparator 2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC2IE | 1000000 |
/// | AC2EN | 10000000 |
/// | AC2IS | 110000 |
/// | AC2M | 111 |
pub const AC2CON: *mut u8 = 0x7E as *mut u8;

/// Analog Comparator3 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC3IS | 110000 |
/// | AC3M | 111 |
/// | AC3OEA | 1000 |
/// | AC3EN | 10000000 |
/// | AC3IE | 1000000 |
pub const AC3CON: *mut u8 = 0x7F as *mut u8;

/// BandGap Resistor Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCR | 1111 |
pub const BGCRR: *mut u8 = 0x80 as *mut u8;

/// BandGap Current Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCC | 1111 |
pub const BGCCR: *mut u8 = 0x81 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIF | 10000000 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
/// | WDCE | 10000 |
/// | WDP | 100111 |
pub const WDTCSR: *mut u8 = 0x82 as *mut u8;

/// `CLKPR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 1111 |
pub const CLKPR: *mut u8 = 0x83 as *mut u8;

/// `CLKCSR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKCCE | 10000000 |
/// | CLKRDY | 10000 |
/// | CLKC | 1111 |
pub const CLKCSR: *mut u8 = 0x84 as *mut u8;

/// `CLKSELR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CSUT | 110000 |
/// | CKSEL | 1111 |
/// | COUT | 1000000 |
pub const CLKSELR: *mut u8 = 0x85 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRADC | 1 |
/// | PRPSC2 | 10000000 |
/// | PRTIM1 | 10000 |
/// | PRPSCR | 100000 |
/// | PRSPI | 100 |
pub const PRR: *mut u8 = 0x86 as *mut u8;

/// PLL Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLF | 111100 |
/// | PLLE | 10 |
/// | PLOCK | 1 |
pub const PLLCSR: *mut u8 = 0x87 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x88 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC2 | 110000 |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 1000000 |
/// | WGM13 | 10000 |
/// | ICNC1 | 10000000 |
/// | CS1 | 111 |
pub const TCCR1B: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Input Capture Register Bytes low byte.
pub const ICR1L: *mut u8 = 0x8C as *mut u8;

/// Timer/Counter1 Input Capture Register Bytes.
pub const ICR1: *mut u16 = 0x8C as *mut u16;

/// Timer/Counter1 Input Capture Register Bytes high byte.
pub const ICR1H: *mut u8 = 0x8D as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1IS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1IE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AC1ECON`
pub const AC1ICE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AC1ECON`
pub const AC1OE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `AC1ECON`
pub const AC1OI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AC1ECON`
pub const AC1H: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2IE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2IS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC2ECON`
pub const AC2OE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `AC2ECON`
pub const AC2OI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AC2ECON`
pub const AC2H: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC3CON`
pub const AC3IS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AC3CON`
pub const AC3M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC3CON`
pub const AC3OEA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AC3CON`
pub const AC3EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AC3CON`
pub const AC3IE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AC3ECON`
pub const AC3OI: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AC3ECON`
pub const AC3OE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `AC3ECON`
pub const AC3H: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC3O: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC2IF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC1O: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC1IF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC2O: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC3IF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADSSEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADHSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADNCDIS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0IS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0G: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0GS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0TS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `BGCCR`
pub const BGCC: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `BGCRR`
pub const BGCR: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKCSR`
pub const CLKCCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKCSR`
pub const CLKRDY: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CLKCSR`
pub const CLKC: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKSELR`
pub const CSUT: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `CLKSELR`
pub const CKSEL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKSELR`
pub const COUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DACON`
pub const DALA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DACON`
pub const DAEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DACON`
pub const DATS: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `DACON`
pub const DAATE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC8D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AMP0PD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC10D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC9D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ACMP1MD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPAGE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

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
pub const PSCINRB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSC2RB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSC0RB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSC2RBA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSCRV: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const RSTDIS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUCR`
pub const CKRC81: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PCLKSEL0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PMODE0: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PALOCK0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCNF0`
pub const POP0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PFIFTY0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PLOCK0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCNF2`
pub const POP2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PCLKSEL2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PALOCK2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PLOCK2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PFIFTY2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PMODE2: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `PCNF2`
pub const POME2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCNFE2`
pub const PISEL2B1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCNFE2`
pub const PASDLK2: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `PCNFE2`
pub const PISEL2A1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCNFE2`
pub const PELEV2B1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCNFE2`
pub const PBFM21: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCNFE2`
pub const PELEV2A1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PAOC0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PCCYC0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PAOC0A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PPRE0: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PRUN0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PBFM0: *mut u8 = 0x24 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PPRE2: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PBFM2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PAOC2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PRUN2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PAOC2A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PCCYC2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PARUN2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PRFM0A: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PFLTE0A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PELEV0A: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PCAE0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PISEL0A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PRFM0B: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PCAE0B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PELEV0B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PFLTE0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PISEL0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PELEV2A: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PISEL2A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PRFM2A: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PFLTE2A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PCAE2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PFLTE2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PRFM2B: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PCAE2B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PELEV2B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PISEL2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PICR0`
pub const PCST0: *mut u16 = 0x8000 as *mut u16;

/// Bitfield on register `PICR2`
pub const PCST2: *mut u16 = 0x8000 as *mut u16;

/// Bitfield on register `PIFR0`
pub const PEOP0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIFR0`
pub const POAC0A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PRN0: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PEV0A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIFR0`
pub const POAC0B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PEV0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PEOP2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PRN2: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `PIFR2`
pub const POAC2A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PEV2A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIFR2`
pub const POAC2B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PEV2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PSEI2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEVE0A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEVE0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEOPE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEOEPE0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEVE2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEOPE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEVE2A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIM2`
pub const PSEIE2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEOEPE2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLF: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `POM2`
pub const POMV2A: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `POM2`
pub const POMV2B: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRPSC2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR`
pub const PRPSCR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR`
pub const PRSPI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PSOC0`
pub const PISEL0B1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PSOC0`
pub const PISEL0A1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PSOC0`
pub const PSYNC0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `PSOC0`
pub const POEN0A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PSOC0`
pub const POEN0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POS2: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2C: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PSOC2`
pub const PSYNC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const WGM13: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// `ANALOG_ADC_AUTO_TRIGGER_4BITS` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger_4bits {
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
   /// PSC0ASY Event.
   pub const VAL_0x08: u32 = 0x8;
   /// PSC1ASY Event.
   pub const VAL_0x09: u32 = 0x9;
   /// PSC2ASY Event.
   pub const VAL_0x0A: u32 = 0xA;
   /// Analog comparator 1.
   pub const VAL_0x0B: u32 = 0xB;
   /// Analog comparator 2.
   pub const VAL_0x0C: u32 = 0xC;
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

/// `ANALOG_DAC_AUTO_TRIGGER` value group
#[allow(non_upper_case_globals)]
pub mod analog_dac_auto_trigger {
   /// Analog Comparator 0.
   pub const VAL_0x00: u32 = 0x0;
   /// Analog Comparator 1.
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

/// `CPU_SLEEP_MODE_3BITS4` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits4 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Standby.
   pub const STDBY: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
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
   /// Brown-out detection at VCC=2.3 V.
   pub const _2V3: u32 = 0x3;
   /// Brown-out detection at VCC=2.2 V.
   pub const _2V2: u32 = 0x2;
   /// Brown-out detection at VCC=1.9 V.
   pub const _1V9: u32 = 0x1;
   /// Brown-out detection at VCC=2.0 V.
   pub const _2V0: u32 = 0x0;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=128 words Boot address=$0F80.
   pub const _128W_0F80: u32 = 0x3;
   /// Boot Flash size=256 words Boot address=$0F00.
   pub const _256W_0F00: u32 = 0x2;
   /// Boot Flash size=512 words Boot address=$0E00.
   pub const _512W_0E00: u32 = 0x1;
   /// Boot Flash size=1024 words Boot address=$0C00.
   pub const _1024W_0C00: u32 = 0x0;
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
   /// Ext. CK; PLLin: RC8; SUT: 6CK/14CK+0ms; \[CKSEL=0000 SUT=00\].
   pub const EXTCLK_PLLIN_RC_8MHZ_6CK_14CK_0MS: u32 = 0x0;
   /// Ext. CK; PLLin: RC8 MHz; SUT:6 CK/14CK+4.1 ms; \[CKSEL=0000 SUT=01\].
   pub const EXTCLK_PLLIN_RC_8MHZ_6CK_14CK_4MS1: u32 = 0x10;
   /// Ext. CK; PLLin: RC8 MHz; SUT:6 CK/14CK+65 ms; \[CKSEL=0000 SUT=10\].
   pub const EXTCLK_PLLIN_RC_8MHZ_6CK_14CK_65MS: u32 = 0x20;
   /// RC8 MHz; PLLin: RC8; SUT: 6CK/14CK+0ms; \[CKSEL=0010 SUT=00\].
   pub const RC_8MHZ_PLLIN_RC_8MHZ_6CK_14CK_0MS: u32 = 0x2;
   /// RC8 MHz; PLLin: RC8; SUT: 6CK/14CK+4.1 ms; \[CKSEL=0010 SUT=01\].
   pub const RC_8MHZ_PLLIN_RC_8MHZ_6CK_14CK_4MS1: u32 = 0x12;
   /// RC8 MHz; PLLin: RC8; SUT: 6CK/14CK+65 ms;  \[CKSEL=0010 SUT=10\].
   pub const RC_8MHZ_PLLIN_RC_8MHZ_6CK_14CK_65MS: u32 = 0x22;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 258CK/14CK+4.1 ms; \[CKSEL=1000 SUT=00\].
   pub const XOSC_PLLIN_RC_8MHZ_258CK_14CK_4MS1: u32 = 0x8;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 258CK/14CK+65 ms; \[CKSEL=1000 SUT=01\].
   pub const XOSC_PLLIN_RC_8MHZ_258CK_14CK_65MS: u32 = 0x18;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 1KCK/14CK+0 ms; \[CKSEL=1000 SUT=10\].
   pub const XOSC_PLLIN_RC_8MHZ_1KCK_14CK_0MS: u32 = 0x28;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 1KCK/14CK+4.1 ms; \[CKSEL=1000 SUT=11\].
   pub const XOSC_PLLIN_RC_8MHZ_1KCK_14CK_4MS1: u32 = 0x38;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 1KCK/14CK+65 ms; \[CKSEL=1001 SUT=00\].
   pub const XOSC_PLLIN_RC_8MHZ_1KCK_14CK_65MS: u32 = 0x9;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 16KCK/14CK+0 ms; \[CKSEL=1001 SUT=01\].
   pub const XOSC_PLLIN_RC_8MHZ_16KCK_14CK_0MS: u32 = 0x19;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 16KCK/14CK+4.1 ms; \[CKSEL=1001 SUT=10\].
   pub const XOSC_PLLIN_RC_8MHZ_16KCK_14CK_4MS1: u32 = 0x29;
   /// XOSC.9-3MHz; PLLin: RC8; SUT: 16KCK/14CK+65 ms;  \[CKSEL=1001 SUT=11\].
   pub const XOSC_PLLIN_RC_8MHZ_16KCK_14CK_65MS: u32 = 0x39;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 258CK/14CK+4.1 ms; \[CKSEL=1010 SUT=00\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_258CK_14CK_4MS1: u32 = 0xA;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 258CK/14CK+65 ms; \[CKSEL=1010 SUT=01\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_258CK_14CK_65MS: u32 = 0x1A;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 1KCK/14CK+0 ms; \[CKSEL=1010 SUT=10\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_1KCK_14CK_0MS: u32 = 0x2A;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 1KCK/14CK+4.1 ms; \[CKSEL=1010 SUT=11\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_1KCK_14CK_4MS1: u32 = 0x3A;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 1KCK/14CK+65 ms;  \[CKSEL=1011 SUT=00\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_1KCK_14CK_65MS: u32 = 0xB;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 16KCK/14CK+0 ms; \[CKSEL=1011 SUT=01\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_16KCK_14CK_0MS: u32 = 0x1B;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 16KCK/14CK+4.1 ms; \[CKSEL=1011 SUT=10\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_16KCK_14CK_4MS1: u32 = 0x2B;
   /// XOSC3-8MHz; PLLin: RC8; SUT: 16KCK/14CK+65 ms;  \[CKSEL=1011 SUT=11\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_RC_8MHZ_16KCK_14CK_65MS: u32 = 0x3B;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 258CK/14CK+4.1 ms; \[CKSEL=1100 SUT=00\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_258CK_14CK_4MS1: u32 = 0xC;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 258CK/14CK+65 ms;  \[CKSEL=1100 SUT=01\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_258CK_14CK_65MS: u32 = 0x1C;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 1KCK/14CK+0 ms; \[CKSEL=1100 SUT=10\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_1KCK_14CK_0MS: u32 = 0x2C;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 1KCK/14CK+4.1 ms; \[CKSEL=1100 SUT=11\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_1KCK_14CK_4MS1: u32 = 0x3C;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 1KCK/14CK+65 ms;  \[CKSEL=1101 SUT=00.
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_1KCK_14CK_65MS: u32 = 0xD;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 16KCK/14CK+0 ms; \[CKSEL=1101 SUT=01\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_16KCK_14CK_0MS: u32 = 0x1D;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 16KCK/14CK+4.1 ms; \[CKSEL=1101 SUT=10\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_16KCK_14CK_4MS1: u32 = 0x2D;
   /// XOSC3-8MHz; PLLin: XOSC; SUT: 16KCK/14CK+65 ms;  \[CKSEL=1101 SUT=11\].
   pub const XOSC_3MHZ_8MHZ_PLLIN_XOSC_16KCK_14CK_65MS: u32 = 0x3D;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 258CK/14CK+4.1 ms; \[CKSEL=1110 SUT=00\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_258CK_14CK_4MS1: u32 = 0xE;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 258CK/14CK+65 ms;  \[CKSEL=1110 SUT=01\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_258CK_14CK_65MS: u32 = 0x1E;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 1KCK/14CK+0 ms; \[CKSEL=1110 SUT=10\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_1KCK_14CK_0MS: u32 = 0x2E;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 1KCK/14CK+4.1 ms; \[CKSEL=1110 SUT=11\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_1KCK_14CK_4MS1: u32 = 0x3E;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 1KCK/14CK+65 ms; \[CKSEL=1111 SUT=00\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_1KCK_14CK_65MS: u32 = 0xF;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 16KCK/14CK+0 ms; \[CKSEL=1111 SUT=01\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_16KCK_14CK_0MS: u32 = 0x1F;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 16KCK/14CK+4.1 ms; \[CKSEL=1111 SUT=10\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_16KCK_14CK_4MS1: u32 = 0x2F;
   /// XOSC8-16MHz; PLLin: RC8; SUT: 16KCK/14CK+65 ms;  \[CKSEL=1111 SUT=11\].
   pub const XOSC_8MHZ_16MHZ_PLLIN_RC_8MHZ_16KCK_14CK_65MS: u32 = 0x3F;
   /// WD128 KHz; SUT: 1KCK/14CK+0 ms; \[CKSEL=0011 SUT=00\].
   pub const WDOSC_128KHZ_1KCK_14CK_0MS: u32 = 0x3;
   /// WD128 KHz; SUT: 1KCK/14CK+4.1 ms; \[CKSEL=0011 SUT=01\].
   pub const WDOSC_128KHZ_1KCK_14CK_4MS1: u32 = 0x13;
   /// WD128 KHz; SUT: 1KCK/14CK+65 ms; \[CKSEL=0011 SUT=10\].
   pub const WDOSC_128KHZ_1KCK_14CK_65MS: u32 = 0x23;
   /// WD128 KHz; SUT: 16KCK/14CK+0 ms; \[CKSEL=0011 SUT=11\].
   pub const WDOSC_128KHZ_16KCK_14CK_0MS: u32 = 0x33;
   /// PLL/4; PLLin: RC8; SUT: 1KCK/14CK+0 ms; \[CKSEL=0001 SUT=00\].
   pub const PLLCLK_DIV4_PLLIN_RC_8MHZ_1KCK_14CK_0MS: u32 = 0x1;
   /// PLL/4; PLLin: RC8; SUT: 1KCK/14CK+4 ms;   \[CKSEL=0001 SUT=01\].
   pub const PLLCLK_DIV4_PLLIN_RC_8MHZ_1KCK_14CK_4MS: u32 = 0x11;
   /// PLL/4; PLLin: RC8; SUT: 1KCK/14CK+64 ms;   \[CKSEL=0001 SUT=10\].
   pub const PLLCLK_DIV4_PLLIN_RC_8MHZ_1KCK_14CK_64MS: u32 = 0x21;
   /// PLL/4; PLLin: Ext. CK; SUT: 16KCK/14CK+0 ms; \[CKSEL=0101 SUT=00\].
   pub const PLLCLK_DIV4_PLLIN_EXTCLK_16KCK_14CK_0MS: u32 = 0x5;
   /// PLL/4; PLLin: Ext. CK; SUT: 16KCK/14CK+64 ms; \[CKSEL=0101 SUT=11\].
   pub const PLLCLK_DIV4_PLLIN_EXTCLK_16KCK_14CK_64MS: u32 = 0x35;
   /// PLL/4; PLLin: XOSC; SUT: 1KCK/14CK+0 ms; \[CKSEL=0100 SUT=00\].
   pub const PLLCLK_DIV4_PLLIN_XOSC_1KCK_14CK_0MS: u32 = 0x4;
   /// PLL/4; PLLin: XOSC; SUT: 1KCK/14CK+4 ms; \[CKSEL=0100 SUT=01\].
   pub const PLLCLK_DIV4_PLLIN_XOSC_1KCK_14CK_4MS: u32 = 0x14;
   /// PLL/4; PLLin: XOSC; SUT: 1KCK/14CK+64 ms; \[CKSEL=0100 SUT=10\].
   pub const PLLCLK_DIV4_PLLIN_XOSC_1KCK_14CK_64MS: u32 = 0x24;
   /// PLL/4; PLLin: XOSC; SUT: 16KCK/14CK+0 ms; \[CKSEL=0100 SUT=11\].
   pub const PLLCLK_DIV4_PLLIN_XOSC_16KCK_14CK_0MS: u32 = 0x34;
   /// RC 1 MHz; SUT: 1KCK/14CK+0 ms; \[CKSEL=0110 SUT=00\].
   pub const RC_1MHZ_1KCK_14CK_0MS: u32 = 0x6;
   /// RC 1 MHz; SUT: 1KCK/14CK+4.1 ms; \[CKSEL=0110 SUT=01\].
   pub const RC_1MHZ_1KCK_14CK_4MS1: u32 = 0x16;
   /// RC 1 MHz; SUT: 1KCK/14CK+65 ms;  \[CKSEL=0110 SUT=10\].
   pub const RC_1MHZ_1KCK_14CK_65MS: u32 = 0x26;
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

