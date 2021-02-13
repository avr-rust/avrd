//! The AVR ATmega64HVE2 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 3V - 3.6V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKDIV8 | 1000 |
/// | SPIEN | 100000 |
/// | BODEN | 10000 |
/// | SUT | 110 |
/// | WDTON | 10000000 |
/// | EESAVE | 1000000 |
/// | OSCSEL0 | 1 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLB1 | 110000 |
/// | BLB0 | 1100 |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BOOTRST | 1 |
/// | DWEN | 1000 |
/// | BOOTSZ | 110 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

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

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0B | 100 |
/// | ICF0 | 1000 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICF1 | 1000 |
/// | OCF1B | 100 |
/// | TOV1 | 1 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 11 |
pub const PCIFR: *mut u8 = 0x3B as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEMPE | 100 |
/// | EEPM | 110000 |
/// | EERE | 1 |
/// | EERIE | 1000 |
/// | EEPE | 10 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Read/Write Access low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Read/Write Access high byte.
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

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCW0 | 10000000 |
/// | ICEN0 | 1000000 |
/// | ICES0 | 10000 |
/// | WGM00 | 1 |
/// | ICNC0 | 100000 |
/// | ICS0 | 1000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS00 | 1 |
/// | CS02 | 100 |
/// | CS01 | 10 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer Counter 0  Bytes.
pub const TCNT0: *mut u16 = 0x46 as *mut u16;

/// Timer Counter 0  Bytes low byte.
pub const TCNT0L: *mut u8 = 0x46 as *mut u8;

/// Timer Counter 0  Bytes high byte.
pub const TCNT0H: *mut u8 = 0x47 as *mut u8;

/// Output Compare Register 0A.
pub const OCR0A: *mut u8 = 0x48 as *mut u8;

/// Output Compare Register B.
pub const OCR0B: *mut u8 = 0x49 as *mut u8;

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
/// | CPOL | 1000 |
/// | SPR | 11 |
/// | DORD | 100000 |
/// | SPIE | 10000000 |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | CPHA | 100 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOL | 1000000 |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

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
/// | OCDRF | 10000 |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | BODRF | 100 |
/// | WDRF | 1000 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IVCE | 1 |
/// | IVSEL | 10 |
/// | PUD | 10000 |
/// | CKOE | 100000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSRE | 10000 |
/// | SPMEN | 1 |
/// | SPMIE | 10000000 |
/// | PGWRT | 100 |
/// | PGERS | 10 |
/// | RWWSB | 1000000 |
/// | SIGRD | 100000 |
/// | LBSET | 1000 |
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
/// | T | 1000000 |
/// | H | 100000 |
/// | Z | 10 |
/// | S | 10000 |
/// | C | 1 |
/// | N | 100 |
/// | I | 10000000 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDIE | 1000000 |
/// | WDIF | 10000000 |
/// | WDP | 100111 |
/// | WDCE | 10000 |
pub const WDTCSR: *mut u8 = 0x60 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPCE | 10000000 |
/// | CLKPS | 11 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Wake-up Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUTE | 1000 |
/// | WUTP | 111 |
/// | WUTIE | 1000000 |
/// | WUTIF | 10000000 |
/// | WUTR | 10000 |
pub const WUTCSR: *mut u8 = 0x62 as *mut u8;

/// Watchdog Timer Configuration Lock Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCL | 110 |
/// | WDCLE | 1 |
pub const WDTCLR: *mut u8 = 0x63 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSPI | 100 |
/// | PRTIM1 | 10 |
/// | PRLIN | 1000 |
/// | PRTIM0 | 1 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Slow Oscillator Calibration Register A.
pub const SOSCCALA: *mut u8 = 0x66 as *mut u8;

/// Oscillator Calibration Register B.
pub const SOSCCALB: *mut u8 = 0x67 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 11 |
pub const PCICR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC01 | 10 |
/// | ISC00 | 1 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Enable Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Enable Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE0 | 1000 |
/// | OCIE0A | 10 |
/// | TOIE0 | 1 |
/// | OCIE0B | 100 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICIE1 | 1000 |
/// | TOIE1 | 1 |
/// | OCIE1A | 10 |
/// | OCIE1B | 100 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Digital Input Disable Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PA1DID | 10 |
/// | PA0DID | 1 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Timer/Counter 1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES1 | 10000 |
/// | WGM10 | 1 |
/// | ICEN1 | 1000000 |
/// | ICS1 | 1000 |
/// | ICNC1 | 100000 |
/// | TCW1 | 10000000 |
pub const TCCR1A: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS | 111 |
pub const TCCR1B: *mut u8 = 0x81 as *mut u8;

/// Timer Counter 1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer Counter 1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer Counter 1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Output Compare Register 1A.
pub const OCR1A: *mut u8 = 0x88 as *mut u8;

/// Output Compare Register B.
pub const OCR1B: *mut u8 = 0x89 as *mut u8;

/// LIN Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LIN13 | 1000000 |
/// | LSWRES | 10000000 |
/// | LENA | 1000 |
/// | LCONF | 110000 |
/// | LCMD | 111 |
pub const LINCR: *mut u8 = 0xC0 as *mut u8;

/// LIN Status and Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LRXOK | 1 |
/// | LTXOK | 10 |
/// | LIDST | 11100000 |
/// | LBUSY | 10000 |
/// | LIDOK | 100 |
/// | LERR | 1000 |
pub const LINSIR: *mut u8 = 0xC1 as *mut u8;

/// LIN Enable Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LENERR | 1000 |
/// | LENTXOK | 10 |
/// | LENRXOK | 1 |
/// | LENIDOK | 100 |
pub const LINENIR: *mut u8 = 0xC2 as *mut u8;

/// LIN Error Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LFERR | 10000 |
/// | LOVERR | 100000 |
/// | LTOERR | 1000000 |
/// | LABORT | 10000000 |
/// | LCERR | 10 |
/// | LBERR | 1 |
/// | LPERR | 100 |
/// | LSERR | 1000 |
pub const LINERR: *mut u8 = 0xC3 as *mut u8;

/// LIN Bit Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LDISR | 10000000 |
/// | LBT | 111111 |
pub const LINBTR: *mut u8 = 0xC4 as *mut u8;

/// LIN Baud Rate Low Register.
pub const LINBRRL: *mut u8 = 0xC5 as *mut u8;

/// LIN Baud Rate High Register.
pub const LINBRRH: *mut u8 = 0xC6 as *mut u8;

/// LIN Data Length Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LTXDL | 11110000 |
/// | LRXDL | 1111 |
pub const LINDLR: *mut u8 = 0xC7 as *mut u8;

/// LIN Identifier Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LP | 11000000 |
/// | LID | 111111 |
pub const LINIDR: *mut u8 = 0xC8 as *mut u8;

/// LIN Data Buffer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LAINC | 1000 |
/// | LINDX | 111 |
pub const LINSEL: *mut u8 = 0xC9 as *mut u8;

/// LIN Data Register.
pub const LINDAT: *mut u8 = 0xCA as *mut u8;

/// Bandgap Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGSC | 111 |
pub const BGCSRA: *mut u8 = 0xD1 as *mut u8;

/// Band Gap Calibration Register B.
pub const BGCRB: *mut u8 = 0xD2 as *mut u8;

/// Band Gap Calibration Register A.
pub const BGCRA: *mut u8 = 0xD3 as *mut u8;

/// Band Gap Lock Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGPLE | 10 |
/// | BGPL | 1 |
pub const BGLR: *mut u8 = 0xD4 as *mut u8;

/// PLL Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LOCK | 10000 |
/// | SWEN | 100000 |
/// | PLLCIF | 10 |
/// | PLLCIE | 1 |
pub const PLLCSR: *mut u8 = 0xD8 as *mut u8;

/// Port B Override.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PBOE0 | 1 |
/// | PBOVCE | 10000000 |
/// | PBOE3 | 1000 |
pub const PBOV: *mut u8 = 0xDC as *mut u8;

/// ADC Synchronization Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SCMD | 11 |
/// | SBSY | 100 |
pub const ADSCSRA: *mut u8 = 0xE0 as *mut u8;

/// ADC Synchronization Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADACRB | 10 |
/// | VADACRB | 100000 |
/// | VADICRB | 10000 |
/// | VADICPS | 1000000 |
/// | CADICPS | 100 |
/// | CADICRB | 1 |
pub const ADSCSRB: *mut u8 = 0xE1 as *mut u8;

/// ADC Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADPSEL | 1000 |
/// | ADCMS | 110 |
/// | CKSEL | 1 |
pub const ADCRA: *mut u8 = 0xE2 as *mut u8;

/// ADC Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADIDES | 11000 |
/// | ADADES | 111 |
pub const ADCRB: *mut u8 = 0xE3 as *mut u8;

/// ADC Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADRCM | 110000 |
/// | CADRCT | 1111 |
/// | CADEN | 10000000 |
pub const ADCRC: *mut u8 = 0xE4 as *mut u8;

/// ADC Control Register D.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADG | 111000 |
/// | CADDSEL | 1 |
/// | CADPDM | 110 |
pub const ADCRD: *mut u8 = 0xE5 as *mut u8;

/// ADC Control Register E.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADPDM | 11000 |
/// | VADEN | 10000000 |
/// | VADREFS | 100000 |
/// | VADMUX | 111 |
pub const ADCRE: *mut u8 = 0xE6 as *mut u8;

/// ADC Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADICIF | 10000 |
/// | CADICIF | 1 |
/// | VADACIF | 100000 |
/// | CADRCIF | 100 |
/// | CADACIF | 10 |
pub const ADIFR: *mut u8 = 0xE7 as *mut u8;

/// ADC Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADICIE | 10000 |
/// | VADACIE | 100000 |
/// | CADRCIE | 100 |
/// | CADACIE | 10 |
/// | CADICIE | 1 |
pub const ADIMR: *mut u8 = 0xE8 as *mut u8;

/// CC-ADC Regulator Current Comparator Threshold Level.
pub const CADRCL: *mut u16 = 0xE9 as *mut u16;

/// CC-ADC Regulator Current Comparator Threshold Level low byte.
pub const CADRCLL: *mut u8 = 0xE9 as *mut u8;

/// CC-ADC Regulator Current Comparator Threshold Level high byte.
pub const CADRCLH: *mut u8 = 0xEA as *mut u8;

/// C-ADC Instantaneous Conversion Result low byte.
pub const CADICL: *mut u8 = 0xEB as *mut u8;

/// C-ADC Instantaneous Conversion Result.
pub const CADIC: *mut u16 = 0xEB as *mut u16;

/// C-ADC Instantaneous Conversion Result high byte.
pub const CADICH: *mut u8 = 0xEC as *mut u8;

/// C-ADC Accumulated Conversion Result.
pub const CADAC0: *mut u8 = 0xED as *mut u8;

/// C-ADC Accumulated Conversion Result.
pub const CADAC1: *mut u8 = 0xEE as *mut u8;

/// C-ADC Accumulated Conversion Result.
pub const CADAC2: *mut u8 = 0xEF as *mut u8;

/// C-ADC Accumulated Conversion Result.
pub const CADAC3: *mut u8 = 0xF0 as *mut u8;

/// V-ADC Instantaneous Conversion Result low byte.
pub const VADICL: *mut u8 = 0xF1 as *mut u8;

/// V-ADC Instantaneous Conversion Result.
pub const VADIC: *mut u16 = 0xF1 as *mut u16;

/// V-ADC Instantaneous Conversion Result high byte.
pub const VADICH: *mut u8 = 0xF2 as *mut u8;

/// V-ADC Accumulated Conversion Result.
pub const VADAC0: *mut u8 = 0xF3 as *mut u8;

/// V-ADC Accumulated Conversion Result.
pub const VADAC1: *mut u8 = 0xF4 as *mut u8;

/// V-ADC Accumulated Conversion Result.
pub const VADAC2: *mut u8 = 0xF5 as *mut u8;

/// V-ADC Accumulated Conversion Result.
pub const VADAC3: *mut u8 = 0xF6 as *mut u8;

/// Bitfield on register `ADCRA`
pub const ADPSEL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCRA`
pub const ADCMS: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `ADCRA`
pub const CKSEL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ADCRB`
pub const ADIDES: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `ADCRB`
pub const ADADES: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCRC`
pub const CADRCM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `ADCRC`
pub const CADRCT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ADCRC`
pub const CADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCRD`
pub const CADG: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `ADCRD`
pub const CADDSEL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ADCRD`
pub const CADPDM: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `ADCRE`
pub const VADPDM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `ADCRE`
pub const VADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCRE`
pub const VADREFS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCRE`
pub const VADMUX: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADIFR`
pub const VADICIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADIFR`
pub const CADICIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ADIFR`
pub const VADACIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADIFR`
pub const CADRCIF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ADIFR`
pub const CADACIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ADIMR`
pub const VADICIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADIMR`
pub const VADACIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADIMR`
pub const CADRCIE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ADIMR`
pub const CADACIE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ADIMR`
pub const CADICIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ADSCSRA`
pub const SCMD: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ADSCSRA`
pub const SBSY: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ADSCSRB`
pub const CADACRB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ADSCSRB`
pub const VADACRB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADSCSRB`
pub const VADICRB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADSCSRB`
pub const VADICPS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADSCSRB`
pub const CADICPS: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ADSCSRB`
pub const CADICRB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BGCSRA`
pub const BGSC: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `BGLR`
pub const BGPLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BGLR`
pub const BGPL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DIDR0`
pub const PA1DID: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const PA0DID: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

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
pub const PSRSYNC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `LINBTR`
pub const LDISR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LINBTR`
pub const LBT: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LINCR`
pub const LIN13: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LINCR`
pub const LSWRES: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LINCR`
pub const LENA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINCR`
pub const LCONF: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LINCR`
pub const LCMD: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LINDLR`
pub const LTXDL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `LINDLR`
pub const LRXDL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENERR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENTXOK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENRXOK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENIDOK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LINERR`
pub const LFERR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LINERR`
pub const LOVERR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LINERR`
pub const LTOERR: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LINERR`
pub const LABORT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LINERR`
pub const LCERR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LINERR`
pub const LBERR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINERR`
pub const LPERR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LINERR`
pub const LSERR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINIDR`
pub const LP: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LINIDR`
pub const LID: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LINSEL`
pub const LAINC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINSEL`
pub const LINDX: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LRXOK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LTXOK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LIDST: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LBUSY: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LIDOK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LERR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOW`
pub const BODEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const OSCSEL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const CKOE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUSR`
pub const OCDRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BODRF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PBOV`
pub const PBOE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PBOV`
pub const PBOVCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PBOV`
pub const PBOE3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const LOCK: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const SWEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLCIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLCIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRLIN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const LBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const TCW0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICEN0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICES0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICNC0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICES1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const WGM10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICEN1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICS1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICNC1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const TCW1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR0`
pub const ICF0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const ICIE0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `WDTCLR`
pub const WDCL: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `WDTCLR`
pub const WDCLE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WUTCSR`
pub const WUTR: *mut u8 = 0x10 as *mut u8;

/// `ACCUMULATED_DECIMATION_RATIO_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod accumulated_decimation_ratio_select {
   /// 512.
   pub const VAL_0x00: u32 = 0x0;
   /// 256.
   pub const VAL_0x01: u32 = 0x1;
   /// 128.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
   /// 32.
   pub const VAL_0x04: u32 = 0x4;
   /// 16.
   pub const VAL_0x05: u32 = 0x5;
   /// 8.
   pub const VAL_0x06: u32 = 0x6;
   /// 4.
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

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC.
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

/// `C_ADC_CHOPPER_MODE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod c_adc_chopper_mode_select {
   /// Chopping Disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Automatic Fast Chopping.
   pub const VAL_0x01: u32 = 0x1;
   /// Automatic Slow Chopping.
   pub const VAL_0x02: u32 = 0x2;
   /// Software Polarity Control.
   pub const VAL_0x03: u32 = 0x3;
}

/// `C_ADC_INPUT_GAIN` value group
#[allow(non_upper_case_globals)]
pub mod c_adc_input_gain {
   /// 4x.
   pub const VAL_0x00: u32 = 0x0;
   /// 8x.
   pub const VAL_0x01: u32 = 0x1;
   /// 16x.
   pub const VAL_0x02: u32 = 0x2;
   /// 32x.
   pub const VAL_0x03: u32 = 0x3;
   /// 64x.
   pub const VAL_0x04: u32 = 0x4;
   /// 128x.
   pub const VAL_0x05: u32 = 0x5;
   /// 256x.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
}

/// `C_ADC_PIN_DIAGNOSIS_MODE` value group
#[allow(non_upper_case_globals)]
pub mod c_adc_pin_diagnosis_mode {
   /// No current source is enabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Current source on PI pin enabled.
   pub const VAL_0x01: u32 = 0x1;
   /// Current source on NI pin enabled.
   pub const VAL_0x02: u32 = 0x2;
   /// Current source on both PI/NI pins enabled.
   pub const VAL_0x03: u32 = 0x3;
}

/// `C_ADC_REGULAR_CURRENT_COMPARATOR_MODE` value group
#[allow(non_upper_case_globals)]
pub mod c_adc_regular_current_comparator_mode {
   /// Comparator Disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Comparator Enabled. Regular Current Counter counts up if Accumulated Current is above threshold and is reset if Accumulated Current is below threshold.
   pub const VAL_0x01: u32 = 0x1;
   /// Comparator Enabled. Regular Current Counter counts up if Accumulated Current is above threshold and down if Accumulated Current is below threshold.
   pub const VAL_0x02: u32 = 0x2;
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

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=512 words Boot address=$7E00.
   pub const _512W_7E00: u32 = 0x3;
   /// Boot Flash size=1024 words Boot address=$7C00.
   pub const _1024W_7C00: u32 = 0x2;
   /// Boot Flash size=2048 words Boot address=$7800.
   pub const _2048W_7800: u32 = 0x1;
   /// Boot Flash size=4096 words Boot address=$7000.
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

/// `ENUM_SUT` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut {
   /// Start-up time 14 CK + 0 ms.
   pub const _14CK_0MS: u32 = 0x0;
   /// Start-up time 14 CK + 16 ms.
   pub const _14CK_16MS: u32 = 0x1;
   /// Start-up time 14 CK + 32 ms.
   pub const _14CK_32MS: u32 = 0x2;
   /// Start-up time 14 CK + 64 ms.
   pub const _14CK_64MS: u32 = 0x3;
}

/// `INSTANTANEOUS_DECIMATION_RATIO_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod instantaneous_decimation_ratio_select {
   /// 512.
   pub const VAL_0x00: u32 = 0x0;
   /// 256.
   pub const VAL_0x01: u32 = 0x1;
   /// 128.
   pub const VAL_0x02: u32 = 0x2;
   /// 64.
   pub const VAL_0x03: u32 = 0x3;
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

/// `MISC_4BIT_COUNT` value group
#[allow(non_upper_case_globals)]
pub mod misc_4bit_count {
   /// 1.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.
   pub const VAL_0x01: u32 = 0x1;
   /// 3.
   pub const VAL_0x02: u32 = 0x2;
   /// 4.
   pub const VAL_0x03: u32 = 0x3;
   /// 5.
   pub const VAL_0x04: u32 = 0x4;
   /// 6.
   pub const VAL_0x05: u32 = 0x5;
   /// 7.
   pub const VAL_0x06: u32 = 0x6;
   /// 8.
   pub const VAL_0x07: u32 = 0x7;
   /// 9.
   pub const VAL_0x08: u32 = 0x8;
   /// 10.
   pub const VAL_0x09: u32 = 0x9;
   /// 11.
   pub const VAL_0x0a: u32 = 0xA;
   /// 12.
   pub const VAL_0x0b: u32 = 0xB;
   /// 13.
   pub const VAL_0x0c: u32 = 0xC;
   /// 14.
   pub const VAL_0x0d: u32 = 0xD;
   /// 15.
   pub const VAL_0x0e: u32 = 0xE;
   /// 16.
   pub const VAL_0x0f: u32 = 0xF;
}

/// `SAMPLING_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod sampling_clock_select {
   /// PLL (512kHz output) as sampling clock.
   pub const VAL_0x00: u32 = 0x0;
   /// Slow RC Oscillator as sampling clock.
   pub const VAL_0x01: u32 = 0x1;
}

/// `SYNCHRONIZATION_COMMAND` value group
#[allow(non_upper_case_globals)]
pub mod synchronization_command {
   /// Reserved.
   pub const VAL_0x00: u32 = 0x0;
   /// Reset and Synchronize.
   pub const VAL_0x01: u32 = 0x1;
   /// Synchronize on next Instantaneous Conversion.
   pub const VAL_0x02: u32 = 0x2;
   /// Synchronize on next Accumulated Conversion.
   pub const VAL_0x03: u32 = 0x3;
}

/// `V_ADC_CHANNEL_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod v_adc_channel_select {
   /// PV - NV.
   pub const VAL_0x00: u32 = 0x0;
   /// ADC0 - SGND.
   pub const VAL_0x01: u32 = 0x1;
   /// ADC1 - SGND.
   pub const VAL_0x02: u32 = 0x2;
   /// VTEMP - GND.
   pub const VAL_0x03: u32 = 0x3;
   /// DIAGNOSIS - GND (VREF/TBD).
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
}

/// `V_ADC_PIN_DIAGNOSIS_MODE` value group
#[allow(non_upper_case_globals)]
pub mod v_adc_pin_diagnosis_mode {
   /// No current source is enabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Current source on PV pin enabled.
   pub const VAL_0x01: u32 = 0x1;
   /// Current source on NV pin enabled.
   pub const VAL_0x02: u32 = 0x2;
   /// Current source on both PV/NV pins enabled.
   pub const VAL_0x03: u32 = 0x3;
}

/// `V_ADC_REFERENCE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod v_adc_reference_select {
   /// VREF as reference.
   pub const VAL_0x00: u32 = 0x0;
   /// AVDD/3 as reference (for diagnosis purpose).
   pub const VAL_0x01: u32 = 0x1;
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

