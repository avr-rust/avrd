//! The AVR ATA5833 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 2.4V - 5.5V | 0 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 1000000 |
/// | EXTCLKEN | 1 |
/// | RSTDISBL | 10 |
/// | EESAVE | 1000 |
/// | CKDIV8 | 10000000 |
/// | BOOTRST | 100 |
/// | WDTON | 10000 |
/// | SPIEN | 100000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BLP | 110000 |
/// | LB | 11 |
/// | AP | 1100 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSPI | 1 |
/// | PRCRC | 1000 |
/// | PRCO | 100000 |
/// | PRRXDC | 10 |
/// | PRTXDC | 100 |
/// | PRVM | 10000 |
pub const PRR0: *mut u8 = 0x21 as *mut u8;

/// Power Reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRT5 | 10000 |
/// | PRT1 | 1 |
/// | PRT2 | 10 |
/// | PRT3 | 100 |
/// | PRT4 | 1000 |
pub const PRR1: *mut u8 = 0x22 as *mut u8;

/// Power Reduction Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSF | 100 |
/// | PRXA | 10 |
/// | PRDF | 1000 |
/// | PRTM | 1000000 |
/// | PRXB | 1 |
/// | PRRS | 100000 |
/// | PRSSM | 10000000 |
/// | PRIDS | 10000 |
pub const PRR2: *mut u8 = 0x23 as *mut u8;

/// Rx DSP power reduction register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTMP | 1000 |
/// | APRPTB | 10000 |
/// | RDPRF | 10000000 |
/// | APRPTA | 100000 |
/// | ARDPRF | 1000000 |
/// | PRPTA | 10 |
/// | PRFLT | 100 |
/// | PRPTB | 1 |
pub const RDPR: *mut u8 = 0x24 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x25 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x26 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x27 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x28 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x29 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x2A as *mut u8;

/// Frequency Synthesizer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PAOER | 10000 |
/// | SFM | 10 |
/// | TXMOD | 1 |
/// | PAON | 10000000 |
/// | TXMS | 1100 |
pub const FSCR: *mut u8 = 0x2B as *mut u8;

/// Rx DSP status interrupt flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOB | 10000000 |
/// | EOTA | 100 |
/// | NBITB | 10 |
/// | EOTB | 1000 |
/// | NBITA | 1 |
/// | WCOA | 1000000 |
/// | SOTA | 10000 |
/// | SOTB | 100000 |
pub const RDSIFR: *mut u8 = 0x2D as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIIO | 100 |
/// | PB4HS | 100000 |
/// | PB7LS | 1000000 |
/// | IVSEL | 10 |
/// | IVCE | 1 |
/// | PB7HS | 10000000 |
/// | PUD | 10000 |
/// | ENPS | 1000 |
pub const MCUCR: *mut u8 = 0x2E as *mut u8;

/// Pin change Interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF1 | 10 |
/// | PCIF0 | 1 |
pub const PCIFR: *mut u8 = 0x2F as *mut u8;

/// Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PS | 111 |
/// | T0IE | 1000 |
/// | T0PR | 10000 |
pub const T0CR: *mut u8 = 0x30 as *mut u8;

/// Timer1 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1OTM | 1 |
/// | T1TOP | 10000 |
/// | T1CTM | 10 |
/// | T1CRM | 100 |
/// | T1ENA | 10000000 |
/// | T1RES | 100000 |
/// | T1TOS | 1000000 |
pub const T1CR: *mut u8 = 0x31 as *mut u8;

/// Timer2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2TOP | 10000 |
/// | T2RES | 100000 |
/// | T2TOS | 1000000 |
/// | T2CRM | 100 |
/// | T2OTM | 1 |
/// | T2CTM | 10 |
/// | T2ENA | 10000000 |
pub const T2CR: *mut u8 = 0x32 as *mut u8;

/// Timer3 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CRM | 100 |
/// | T3RES | 100000 |
/// | T3CPRM | 1000 |
/// | T3TOS | 1000000 |
/// | T3ENA | 10000000 |
/// | T3CTM | 10 |
/// | T3TOP | 10000 |
/// | T3OTM | 1 |
pub const T3CR: *mut u8 = 0x33 as *mut u8;

/// Timer4 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4ENA | 10000000 |
/// | T4CRM | 100 |
/// | T4TOS | 1000000 |
/// | T4RES | 100000 |
/// | T4CPRM | 1000 |
/// | T4OTM | 1 |
/// | T4TOP | 10000 |
/// | T4CTM | 10 |
pub const T4CR: *mut u8 = 0x34 as *mut u8;

/// Timer1 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1COF | 10 |
/// | T1OFF | 1 |
pub const T1IFR: *mut u8 = 0x35 as *mut u8;

/// Timer2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2COF | 10 |
/// | T2OFF | 1 |
pub const T2IFR: *mut u8 = 0x36 as *mut u8;

/// Timer3 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3OFF | 1 |
/// | T3COF | 10 |
/// | T3ICF | 100 |
pub const T3IFR: *mut u8 = 0x37 as *mut u8;

/// Timer4 interrupt flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4COF | 10 |
/// | T4ICF | 100 |
/// | T4OFF | 1 |
pub const T4IFR: *mut u8 = 0x38 as *mut u8;

/// Timer5 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5COF | 10 |
/// | T5OFF | 1 |
pub const T5IFR: *mut u8 = 0x39 as *mut u8;

/// General Purpose I/O Register 0.
pub const GPIOR0: *mut u8 = 0x3A as *mut u8;

/// General Purpose I/O Register 3.
pub const GPIOR3: *mut u8 = 0x3B as *mut u8;

/// General Purpose I/O Register 4.
pub const GPIOR4: *mut u8 = 0x3C as *mut u8;

/// General Purpose I/O Register 5.
pub const GPIOR5: *mut u8 = 0x3D as *mut u8;

/// General Purpose I/O Register 6.
pub const GPIOR6: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEWE | 10 |
/// | EEPM | 110000 |
/// | EEPAGE | 1000000 |
/// | EERIE | 1000 |
/// | EERE | 1 |
/// | NVMBSY | 10000000 |
/// | EEMWE | 100 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// EEPROM Protection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEAP | 1111 |
pub const EEPR: *mut u8 = 0x43 as *mut u8;

/// General Purpose I/O Register 1.
pub const GPIOR1: *mut u8 = 0x44 as *mut u8;

/// General Purpose I/O Register 2.
pub const GPIOR2: *mut u8 = 0x45 as *mut u8;

/// Pin change Interrupt control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE0 | 1 |
/// | PCIE1 | 10 |
pub const PCICR: *mut u8 = 0x46 as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT1 | 10 |
/// | INT0 | 1 |
pub const EIMSK: *mut u8 = 0x47 as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF1 | 10 |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x48 as *mut u8;

/// CRC Data Input Register.
pub const CRCDIR: *mut u8 = 0x49 as *mut u8;

/// Voltage Monitor Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMLS | 1111 |
/// | VMF | 100000 |
/// | VMIM | 10000 |
pub const VMCSR: *mut u8 = 0x4A as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PORF | 1 |
/// | WDRF | 1000 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DORD | 100000 |
/// | SPIE | 10000000 |
/// | CPHA | 100 |
/// | CPOL | 1000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXIF | 100000 |
/// | RXIF | 10000 |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Timer0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0F | 1 |
pub const T0IFR: *mut u8 = 0x4F as *mut u8;

/// debugWire communication Register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Rx DSP control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDEN | 100 |
/// | ADIVEN | 10 |
/// | RDPU | 1 |
pub const RDCR: *mut u8 = 0x53 as *mut u8;

/// End Of Telegram Status on path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTFA | 100 |
/// | AMPFA | 10 |
/// | TELRA | 100000 |
/// | EOTBF | 10000000 |
/// | RRFA | 1000000 |
/// | TMOFA | 10000 |
/// | MANFA | 1000 |
/// | CARFA | 1 |
pub const EOTSA: *mut u8 = 0x54 as *mut u8;

/// End Of Telegram Conditions for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFEA | 1000000 |
/// | SYTFEA | 100 |
/// | TMOFEA | 10000 |
/// | MANFEA | 1000 |
/// | EOTBFE | 10000000 |
/// | AMPFEA | 10 |
/// | CARFEA | 1 |
/// | TELREA | 100000 |
pub const EOTCA: *mut u8 = 0x55 as *mut u8;

/// End Of Telegram Status on path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFB | 1000000 |
/// | TELRB | 100000 |
/// | CARFB | 1 |
/// | EOTAF | 10000000 |
/// | AMPFB | 10 |
/// | TMOFB | 10000 |
/// | MANFB | 1000 |
/// | SYTFB | 100 |
pub const EOTSB: *mut u8 = 0x56 as *mut u8;

/// End Of Telegram Conditions for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFEB | 1000000 |
/// | MANFEB | 1000 |
/// | CARFEB | 1 |
/// | AMPFEB | 10 |
/// | EOTAFE | 10000000 |
/// | SYTFEB | 100 |
/// | TELREB | 100000 |
/// | TMOFEB | 10000 |
pub const EOTCB: *mut u8 = 0x57 as *mut u8;

/// Sleep mode control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 1110 |
/// | SE | 1 |
pub const SMCR: *mut u8 = 0x58 as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCD | 10000 |
/// | CMM | 111 |
/// | CMCCE | 10000000 |
/// | CMONEN | 1000000 |
/// | CCS | 1000 |
pub const CMCR: *mut u8 = 0x59 as *mut u8;

/// Clock Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x5A as *mut u8;

/// Clock Prescaler Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 111 |
/// | CLPCE | 10000000 |
/// | CLTPS | 111000 |
pub const CLPR: *mut u8 = 0x5B as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SELFPRGEN | 1 |
/// | PGERS | 10 |
/// | BLBSET | 1000 |
/// | PGWRT | 100 |
/// | SPMIE | 10000000 |
pub const SPMCSR: *mut u8 = 0x5C as *mut u8;

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
/// | T | 1000000 |
/// | V | 1000 |
/// | Z | 10 |
/// | H | 100000 |
/// | I | 10000000 |
/// | N | 100 |
/// | C | 1 |
/// | S | 10000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Frequency Synthesizer Enable register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ANTT | 100000 |
/// | SDEN | 10 |
/// | SDPU | 1 |
/// | PEEN | 1000 |
/// | ASEN | 10000 |
/// | GAEN | 100 |
pub const FSEN: *mut u8 = 0x60 as *mut u8;

/// Frequency Synthesizer Filter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ASDIV | 11110000 |
/// | BTSEL | 11 |
pub const FSFCR: *mut u8 = 0x61 as *mut u8;

/// Gauss Clock Divider.
pub const GACDIV: *mut u16 = 0x62 as *mut u16;

/// Gauss Clock Divider low byte.
pub const GACDIVL: *mut u8 = 0x62 as *mut u8;

/// Gauss Clock Divider high byte.
pub const GACDIVH: *mut u8 = 0x63 as *mut u8;

/// Fractional Frequency 1 Setting, Low Byte.
pub const FFREQ1L: *mut u8 = 0x64 as *mut u8;

/// Fractional Frequency 1 Setting, Middle Byte.
pub const FFREQ1M: *mut u8 = 0x65 as *mut u8;

/// Fractional Frequency 1 Setting, High Byte.
pub const FFREQ1H: *mut u8 = 0x66 as *mut u8;

/// Fractional Frequency 2 Setting, Low Byte.
pub const FFREQ2L: *mut u8 = 0x67 as *mut u8;

/// Fractional Frequency 2 Setting, Middle Byte.
pub const FFREQ2M: *mut u8 = 0x68 as *mut u8;

/// Fractional Frequency 2 Setting, High Byte.
pub const FFREQ2H: *mut u8 = 0x69 as *mut u8;

/// External Interrupt control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x6B as *mut u8;

/// Pin change Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT7 | 10000000 |
/// | PCINT6 | 1000000 |
/// | PCINT4 | 10000 |
/// | PCINT0 | 1 |
/// | PCINT5 | 100000 |
/// | PCINT3 | 1000 |
/// | PCINT2 | 100 |
/// | PCINT1 | 10 |
pub const PCMSK0: *mut u8 = 0x6C as *mut u8;

/// Pin change Mask Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT10 | 100 |
/// | PCINT9 | 10 |
/// | PCINT12 | 10000 |
/// | PCINT8 | 1 |
/// | PCINT11 | 1000 |
/// | PCINT13 | 100000 |
pub const PCMSK1: *mut u8 = 0x6D as *mut u8;

/// Watchdog Timer0 control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDE | 1000 |
/// | WDPS | 111 |
pub const WDTCR: *mut u8 = 0x6E as *mut u8;

/// Timer1 Counter Register.
pub const T1CNT: *mut u8 = 0x6F as *mut u8;

/// Timer1 Compare Register.
pub const T1COR: *mut u8 = 0x70 as *mut u8;

/// Timer1 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1PS | 111100 |
/// | T1DC | 11000000 |
/// | T1CS | 11 |
pub const T1MR: *mut u8 = 0x71 as *mut u8;

/// Timer1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1CIM | 10 |
/// | T1OIM | 1 |
pub const T1IMR: *mut u8 = 0x72 as *mut u8;

/// Timer2 Counter Register.
pub const T2CNT: *mut u8 = 0x73 as *mut u8;

/// Timer2 Compare Register.
pub const T2COR: *mut u8 = 0x74 as *mut u8;

/// Timer2 Mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2PS | 111100 |
/// | T2CS | 11 |
/// | T2DC | 11000000 |
pub const T2MR: *mut u8 = 0x75 as *mut u8;

/// Timer2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2OIM | 1 |
/// | T2CIM | 10 |
pub const T2IMR: *mut u8 = 0x76 as *mut u8;

/// Timer3 counter Register.
pub const T3CNT: *mut u16 = 0x77 as *mut u16;

/// Timer3 counter Register low byte.
pub const T3CNTL: *mut u8 = 0x77 as *mut u8;

/// Timer3 counter Register high byte.
pub const T3CNTH: *mut u8 = 0x78 as *mut u8;

/// Timer3 compare Register.
pub const T3COR: *mut u16 = 0x79 as *mut u16;

/// Timer3 compare Register low byte.
pub const T3CORL: *mut u8 = 0x79 as *mut u8;

/// Timer3 compare Register high byte.
pub const T3CORH: *mut u8 = 0x7A as *mut u8;

/// Timer3 input capture Register.
pub const T3ICR: *mut u16 = 0x7B as *mut u16;

/// Timer3 input capture Register low byte.
pub const T3ICRL: *mut u8 = 0x7B as *mut u8;

/// Timer3 input capture Register high byte.
pub const T3ICRH: *mut u8 = 0x7C as *mut u8;

/// Timer3 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CS | 11 |
/// | T3PS | 11100 |
pub const T3MRA: *mut u8 = 0x7D as *mut u8;

/// Timer3 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CE | 11000 |
/// | T3SCE | 10 |
/// | T3ICS | 11100000 |
/// | T3CNC | 100 |
pub const T3MRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CPIM | 100 |
/// | T3CIM | 10 |
/// | T3OIM | 1 |
pub const T3IMR: *mut u8 = 0x7F as *mut u8;

/// Timer4 counter Register.
pub const T4CNT: *mut u16 = 0x80 as *mut u16;

/// Timer4 counter Register low byte.
pub const T4CNTL: *mut u8 = 0x80 as *mut u8;

/// Timer4 counter Register high byte.
pub const T4CNTH: *mut u8 = 0x81 as *mut u8;

/// Timer4 compare Register.
pub const T4COR: *mut u16 = 0x82 as *mut u16;

/// Timer4 compare Register low byte.
pub const T4CORL: *mut u8 = 0x82 as *mut u8;

/// Timer4 compare Register high byte.
pub const T4CORH: *mut u8 = 0x83 as *mut u8;

/// Timer4 input capture Register.
pub const T4ICR: *mut u16 = 0x84 as *mut u16;

/// Timer4 input capture Register low byte.
pub const T4ICRL: *mut u8 = 0x84 as *mut u8;

/// Timer4 input capture Register high byte.
pub const T4ICRH: *mut u8 = 0x85 as *mut u8;

/// Timer4 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CS | 11 |
/// | T4PS | 11100 |
pub const T4MRA: *mut u8 = 0x86 as *mut u8;

/// Timer4 mode Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4ICS | 11100000 |
/// | T4CNC | 100 |
/// | T4CE | 11000 |
/// | T4SCE | 10 |
pub const T4MRB: *mut u8 = 0x87 as *mut u8;

/// Timer4 interrupt mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4OIM | 1 |
/// | T4CPIM | 100 |
/// | T4CIM | 10 |
pub const T4IMR: *mut u8 = 0x88 as *mut u8;

/// Timer5 Output Compare Register.
pub const T5OCR: *mut u16 = 0x8A as *mut u16;

/// Timer5 Output Compare Register low byte.
pub const T5OCRL: *mut u8 = 0x8A as *mut u8;

/// Timer5 Output Compare Register high byte.
pub const T5OCRH: *mut u8 = 0x8B as *mut u8;

/// Timer5 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CS | 111 |
/// | T5CTC | 1000 |
pub const T5CCR: *mut u8 = 0x8C as *mut u8;

/// Timer5 Counter low byte.
pub const T5CNTL: *mut u8 = 0x8D as *mut u8;

/// Timer5 Counter.
pub const T5CNT: *mut u16 = 0x8D as *mut u16;

/// Timer5 Counter high byte.
pub const T5CNTH: *mut u8 = 0x8E as *mut u8;

/// Timer5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CIM | 10 |
/// | T5OIM | 1 |
pub const T5IMR: *mut u8 = 0x8F as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR10 | 1 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0x90 as *mut u8;

/// Start Of Telegram Status for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUPOB | 10000 |
/// | SFIDOB | 100000 |
/// | RROB | 1000000 |
/// | SYTOB | 100 |
/// | AMPOB | 10 |
/// | MANOB | 1000 |
/// | WCOAO | 10000000 |
/// | CAROB | 1 |
pub const SOTSB: *mut u8 = 0x91 as *mut u8;

/// Start Of Telegram Status for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROA | 1000000 |
/// | MANOA | 1000 |
/// | WUPOA | 10000 |
/// | SFIDOA | 100000 |
/// | SYTOA | 100 |
/// | WCOBO | 10000000 |
/// | AMPOA | 10 |
/// | CAROA | 1 |
pub const SOTSA: *mut u8 = 0x92 as *mut u8;

/// Start Of Telegram Conditions for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CAROEB | 1 |
/// | WCOAOE | 10000000 |
/// | RROEB | 1000000 |
/// | SFIDEB | 100000 |
/// | AMPOEB | 10 |
/// | SYTOEB | 100 |
/// | WUPEB | 10000 |
/// | MANOEB | 1000 |
pub const SOTCB: *mut u8 = 0x93 as *mut u8;

/// Start Of Telegram Conditions for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOBOE | 10000000 |
/// | RROEA | 1000000 |
/// | AMPOEA | 10 |
/// | SYTOEA | 100 |
/// | CAROEA | 1 |
/// | MANOEA | 1000 |
/// | WUPEA | 10000 |
/// | SFIDEA | 100000 |
pub const SOTCA: *mut u8 = 0x94 as *mut u8;

/// Telegram Status Register on Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTLB | 110 |
/// | CRCOB | 1 |
pub const TESRB: *mut u8 = 0x95 as *mut u8;

/// Telegram Status Register on Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTLA | 110 |
/// | CRCOA | 1 |
pub const TESRA: *mut u8 = 0x96 as *mut u8;

/// Rx DSP status interrupt mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOAM | 1000000 |
/// | EOTBM | 1000 |
/// | SOTBM | 100000 |
/// | SOTAM | 10000 |
/// | NBITBM | 10 |
/// | NBITAM | 1 |
/// | EOTAM | 100 |
/// | WCOBM | 10000000 |
pub const RDSIMR: *mut u8 = 0x98 as *mut u8;

/// Rx DSP output control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDSIDB | 1000000 |
/// | ETRPA | 1000 |
/// | RDSIDA | 100000 |
/// | TMDS | 110 |
/// | ETRPB | 10000 |
pub const RDOCR: *mut u8 = 0x99 as *mut u8;

/// Temperature Low byte.
pub const TEMPL: *mut u8 = 0x9B as *mut u8;

/// Temperature High byte.
pub const TEMPH: *mut u8 = 0x9C as *mut u8;

/// Symbol check configuration for data path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYCSB | 1111 |
/// | SYTLB | 11110000 |
pub const SYCB: *mut u8 = 0x9D as *mut u8;

/// Symbol check configuration for data path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTLA | 11110000 |
/// | SYCSA | 1111 |
pub const SYCA: *mut u8 = 0x9E as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency on path B.
pub const RXFOB: *mut u8 = 0x9F as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency on path A.
pub const RXFOA: *mut u8 = 0xA0 as *mut u8;

/// Demodulator Mode for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMATB | 11111 |
/// | DMHB | 1000000 |
/// | DMPB | 100000 |
/// | DMNEB | 10000000 |
pub const DMMB: *mut u8 = 0xA1 as *mut u8;

/// Demodulator Mode for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMATA | 11111 |
/// | DMNEA | 10000000 |
/// | DMHA | 1000000 |
/// | DMPA | 100000 |
pub const DMMA: *mut u8 = 0xA2 as *mut u8;

/// Demodulator Carrier Detect for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCTB | 11100000 |
/// | DMCLB | 11111 |
pub const DMCDB: *mut u8 = 0xA3 as *mut u8;

/// Demodulator Carrier Detect for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCTA | 11100000 |
/// | DMCLA | 11111 |
pub const DMCDA: *mut u8 = 0xA4 as *mut u8;

/// Demodulator Control Register for path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMPGB | 11111 |
/// | DMARB | 10000000 |
/// | SY1TB | 1000000 |
/// | SASKB | 100000 |
pub const DMCRB: *mut u8 = 0xA5 as *mut u8;

/// Demodulator Control Register for path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SY1TA | 1000000 |
/// | SASKA | 100000 |
/// | DMPGA | 11111 |
/// | DMARA | 10000000 |
pub const DMCRA: *mut u8 = 0xA6 as *mut u8;

/// Demodulator Data Rate on path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMDNB | 11110000 |
/// | DMAB | 1111 |
pub const DMDRB: *mut u8 = 0xA7 as *mut u8;

/// Demodulator Data Rate on path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMDNA | 11110000 |
/// | DMAA | 1111 |
pub const DMDRA: *mut u8 = 0xA8 as *mut u8;

/// Channel Filter Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BWM | 1111 |
pub const CHCR: *mut u8 = 0xA9 as *mut u8;

/// Channel Filter Down Sampling Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBDN | 11111 |
/// | ADCDN | 100000 |
pub const CHDN: *mut u8 = 0xAA as *mut u8;

/// Start-Frame ID Control for data path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEMEB | 10000000 |
/// | SFIDTB | 11111 |
pub const SFIDCB: *mut u8 = 0xAB as *mut u8;

/// Start-Frame ID Length for data path B.
pub const SFIDLB: *mut u8 = 0xAC as *mut u8;

/// Wake-Up Pattern Threshold for data path B.
pub const WUPTB: *mut u8 = 0xAD as *mut u8;

/// Wake-Up Pattern Length for data path B.
pub const WUPLB: *mut u8 = 0xAE as *mut u8;

/// Start-Frame ID byte 1 for data path B.
pub const SFID1B: *mut u8 = 0xAF as *mut u8;

/// Start-Frame ID byte 2 for data path B.
pub const SFID2B: *mut u8 = 0xB0 as *mut u8;

/// Start-Frame ID byte 3 for data path B.
pub const SFID3B: *mut u8 = 0xB1 as *mut u8;

/// Start-Frame ID byte 4 for data path B.
pub const SFID4B: *mut u8 = 0xB2 as *mut u8;

/// Wake-Up Pattern byte 1 for data path B.
pub const WUP1B: *mut u8 = 0xB3 as *mut u8;

/// Wake-Up Pattern byte 2 for data path B.
pub const WUP2B: *mut u8 = 0xB4 as *mut u8;

/// Wake-Up Pattern byte 3 for data path B.
pub const WUP3B: *mut u8 = 0xB5 as *mut u8;

/// Wake-Up Pattern byte 4 for data path B.
pub const WUP4B: *mut u8 = 0xB6 as *mut u8;

/// Start-Frame ID Control for data path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFIDTA | 11111 |
/// | SEMEA | 10000000 |
pub const SFIDCA: *mut u8 = 0xB7 as *mut u8;

/// Start-Frame ID Length for data path A.
pub const SFIDLA: *mut u8 = 0xB8 as *mut u8;

/// Wake-Up Pattern Threshold for data path A.
pub const WUPTA: *mut u8 = 0xB9 as *mut u8;

/// Wake-Up Pattern Length for data path A.
pub const WUPLA: *mut u8 = 0xBA as *mut u8;

/// Start-Frame ID byte 1 for data path A.
pub const SFID1A: *mut u8 = 0xBB as *mut u8;

/// Start-Frame ID byte 2 for data path A.
pub const SFID2A: *mut u8 = 0xBC as *mut u8;

/// Start-Frame ID byte 3 for data path A.
pub const SFID3A: *mut u8 = 0xBD as *mut u8;

/// Start-Frame ID byte 4 for data path A.
pub const SFID4A: *mut u8 = 0xBE as *mut u8;

/// Wake-Up Pattern byte 1 for data path A.
pub const WUP1A: *mut u8 = 0xBF as *mut u8;

/// Wake-Up Pattern byte 2 for data path A.
pub const WUP2A: *mut u8 = 0xC0 as *mut u8;

/// Wake-Up Pattern byte 3 for data path A.
pub const WUP3A: *mut u8 = 0xC1 as *mut u8;

/// Wake-Up Pattern byte 4 for data path A.
pub const WUP4A: *mut u8 = 0xC2 as *mut u8;

/// Clock output divider settings Register.
pub const CLKOD: *mut u8 = 0xC3 as *mut u8;

/// Clock output control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOS | 11 |
/// | CLKOEN | 100 |
pub const CLKOCR: *mut u8 = 0xC4 as *mut u8;

/// `XFUSE` register
pub const XFUSE: *mut u8 = 0xC5 as *mut u8;

/// Slow RC oscillator calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCTC | 11000000 |
pub const SRCCAL: *mut u8 = 0xC6 as *mut u8;

/// Fast RC oscillator calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCTC | 100000 |
pub const FRCCAL: *mut u8 = 0xC7 as *mut u8;

/// Clock management status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0xC8 as *mut u8;

/// Clock management override control register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCACT | 1000 |
/// | SRCAO | 10 |
/// | FRCAO | 1 |
/// | FRCACT | 100 |
pub const CMOCR: *mut u8 = 0xC9 as *mut u8;

/// Supply Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVCCLF | 10 |
/// | AVCCRF | 1 |
pub const SUPFR: *mut u8 = 0xCA as *mut u8;

/// Supply Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DVDIS | 10000 |
/// | AVCCRM | 1 |
/// | AVEN | 100000 |
/// | PVEN | 100 |
/// | AVCCLM | 10 |
/// | AVDIC | 1000000 |
pub const SUPCR: *mut u8 = 0xCB as *mut u8;

/// Supply calibration register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PV22 | 100 |
/// | PVDIC | 1000 |
/// | PVCAL | 11110000 |
pub const SUPCA1: *mut u8 = 0xCC as *mut u8;

/// Supply calibration register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL | 1111 |
pub const SUPCA2: *mut u8 = 0xCD as *mut u8;

/// Supply calibration register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACAL5 | 10 |
/// | DCAL6 | 1000000 |
/// | ACAL7 | 1000 |
/// | DCAL5 | 100000 |
/// | ACAL6 | 100 |
/// | ACAL4 | 1 |
/// | DCAL4 | 10000 |
pub const SUPCA3: *mut u8 = 0xCE as *mut u8;

/// Supply calibration register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCAL3 | 10000000 |
/// | DCAL0 | 10000 |
/// | DCAL2 | 1000000 |
/// | ACAL3 | 1000 |
/// | DCAL1 | 100000 |
/// | ACAL2 | 100 |
/// | ACAL0 | 1 |
/// | ACAL1 | 10 |
pub const SUPCA4: *mut u8 = 0xCF as *mut u8;

/// Calibration ready signature.
pub const CALRDY: *mut u8 = 0xD0 as *mut u8;

/// Voltage Monitor Calibration register.
pub const VMCAL: *mut u8 = 0xD1 as *mut u8;

/// Data FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFOFL | 100 |
/// | DFUFL | 10 |
/// | DFFLRF | 1 |
pub const DFS: *mut u8 = 0xD2 as *mut u8;

/// Data FIFO Telegram Length.
pub const DFTL: *mut u16 = 0xD3 as *mut u16;

/// Data FIFO Telegram Length low byte.
pub const DFTLL: *mut u8 = 0xD3 as *mut u8;

/// Data FIFO Telegram Length high byte.
pub const DFTLH: *mut u8 = 0xD4 as *mut u8;

/// Data FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLS | 111111 |
/// | DFCLR | 10000000 |
pub const DFL: *mut u8 = 0xD5 as *mut u8;

/// Data FIFO Write Pointer.
pub const DFWP: *mut u8 = 0xD6 as *mut u8;

/// Data FIFO Read Pointer.
pub const DFRP: *mut u8 = 0xD7 as *mut u8;

/// Data FIFO Data Register.
pub const DFD: *mut u8 = 0xD8 as *mut u8;

/// Data FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFERIM | 10 |
/// | DFFLIM | 1 |
pub const DFI: *mut u8 = 0xD9 as *mut u8;

/// Data FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLC | 111111 |
/// | DFDRA | 10000000 |
pub const DFC: *mut u8 = 0xDA as *mut u8;

/// Support FIFO Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLRF | 1 |
/// | SFOFL | 100 |
/// | SFUFL | 10 |
pub const SFS: *mut u8 = 0xDB as *mut u8;

/// Support FIFO Fill Level Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLS | 11111 |
/// | SFCLR | 10000000 |
pub const SFL: *mut u8 = 0xDC as *mut u8;

/// Support FIFO Write Pointer.
pub const SFWP: *mut u8 = 0xDD as *mut u8;

/// Support FIFO Read Pointer.
pub const SFRP: *mut u8 = 0xDE as *mut u8;

/// Support FIFO Data Register.
pub const SFD: *mut u8 = 0xDF as *mut u8;

/// Support FIFO Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLIM | 1 |
/// | SFERIM | 10 |
pub const SFI: *mut u8 = 0xE0 as *mut u8;

/// Support FIFO Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDRA | 10000000 |
/// | SFFLC | 11111 |
pub const SFC: *mut u8 = 0xE1 as *mut u8;

/// SSM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SETRPA | 1000000 |
/// | SSMTAE | 100000 |
/// | SSMTM | 10 |
/// | SSMTX | 1 |
/// | SETRPB | 10000000 |
/// | SSMTPE | 1000 |
/// | SSMTGE | 100 |
/// | SSMPVE | 10000 |
pub const SSMCR: *mut u8 = 0xE2 as *mut u8;

/// SSM Rx Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMADA | 100 |
/// | SSMPB | 10 |
/// | SSMIDSE | 1000000 |
/// | SSMPVS | 10000 |
/// | SSMADB | 1000 |
/// | SSMIFA | 100000 |
/// | SSMTMOE | 10000000 |
/// | SSMPA | 1 |
pub const SSMRCR: *mut u8 = 0xE3 as *mut u8;

/// SSM Filter Bandwidth Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMDFDT | 1000 |
/// | SSMHADT | 10000 |
/// | SSMPLDT | 100000 |
/// | SSMFID | 111 |
pub const SSMFBR: *mut u8 = 0xE4 as *mut u8;

/// SSM Run Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMR | 1 |
/// | SSMST | 10 |
pub const SSMRR: *mut u8 = 0xE5 as *mut u8;

/// SSM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMESM | 1111 |
/// | SSMERR | 10000000 |
pub const SSMSR: *mut u8 = 0xE6 as *mut u8;

/// SSM Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIF | 1 |
pub const SSMIFR: *mut u8 = 0xE7 as *mut u8;

/// SSM interrupt mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIM | 1 |
pub const SSMIMR: *mut u8 = 0xE8 as *mut u8;

/// Master State Machine state register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMMST | 11111 |
pub const MSMSTR: *mut u8 = 0xE9 as *mut u8;

/// SSM State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTA | 111111 |
pub const SSMSTR: *mut u8 = 0xEA as *mut u8;

/// SSM extended State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTB | 111111 |
pub const SSMXSR: *mut u8 = 0xEB as *mut u8;

/// Master State Machine Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM0 | 1111 |
/// | MSMSM1 | 11110000 |
pub const MSMCR1: *mut u8 = 0xEC as *mut u8;

/// Master State Machine Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM2 | 1111 |
/// | MSMSM3 | 11110000 |
pub const MSMCR2: *mut u8 = 0xED as *mut u8;

/// Master State Machine Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM4 | 1111 |
/// | MSMSM5 | 11110000 |
pub const MSMCR3: *mut u8 = 0xEE as *mut u8;

/// Master State Machine Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM7 | 11110000 |
/// | MSMSM6 | 1111 |
pub const MSMCR4: *mut u8 = 0xEF as *mut u8;

/// Get Telegram Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTEHA | 1 |
/// | GAPMA | 10 |
/// | GAPMB | 100000 |
/// | IWUPB | 10000000 |
/// | IWUPA | 1000 |
/// | RXTEHB | 10000 |
/// | DARB | 1000000 |
/// | DARA | 100 |
pub const GTCR: *mut u8 = 0xF0 as *mut u8;

/// Start Of Telegram Conditions 1 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MANOEA1 | 1000 |
/// | CAROEA1 | 1 |
/// | SYTOEA1 | 100 |
/// | WCOBOE1 | 10000000 |
/// | SFIDEA1 | 100000 |
/// | RROEA1 | 1000000 |
/// | WUPEA1 | 10000 |
/// | AMPOEA1 | 10 |
pub const SOTC1A: *mut u8 = 0xF1 as *mut u8;

/// Start Of Telegram Conditions 2 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPOEA2 | 10 |
/// | SFIDEA2 | 100000 |
/// | CAROEA2 | 1 |
/// | SYTOEA2 | 100 |
/// | WUPEA2 | 10000 |
/// | MANOEA2 | 1000 |
/// | RROEA2 | 1000000 |
/// | WCOBOE2 | 10000000 |
pub const SOTC2A: *mut u8 = 0xF2 as *mut u8;

/// Start Of Telegram Conditions 1 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUPEB1 | 10000 |
/// | MANOEB1 | 1000 |
/// | SYTOEB1 | 100 |
/// | SFIDEB1 | 100000 |
/// | WCOAOE1 | 10000000 |
/// | CAROEB1 | 1 |
/// | RROEB1 | 1000000 |
/// | AMPOEB1 | 10 |
pub const SOTC1B: *mut u8 = 0xF3 as *mut u8;

/// Start Of Telegram Conditions 2 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WUPEB2 | 10000 |
/// | MANOEB2 | 1000 |
/// | SFIDEB2 | 100000 |
/// | RROEB2 | 1000000 |
/// | SYTOEB2 | 100 |
/// | WCOAOE2 | 10000000 |
/// | CAROEB2 | 1 |
/// | AMPOEB2 | 10 |
pub const SOTC2B: *mut u8 = 0xF4 as *mut u8;

/// End Of Telegram Conditions 1 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFEA1 | 10 |
/// | TELREA1 | 100000 |
/// | RRFEA1 | 1000000 |
/// | EOTBFE1 | 10000000 |
/// | TMOFEA1 | 10000 |
/// | CARFEA1 | 1 |
/// | MANFEA1 | 1000 |
/// | SYTFEA1 | 100 |
pub const EOTC1A: *mut u8 = 0xF5 as *mut u8;

/// End Of Telegram Conditions 2 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CARFEA2 | 1 |
/// | EOTBFE2 | 10000000 |
/// | TELREA2 | 100000 |
/// | RRFEA2 | 1000000 |
/// | AMPFEA2 | 10 |
/// | SYTFEA2 | 100 |
/// | MANFEA2 | 1000 |
/// | TMOFEA2 | 10000 |
pub const EOTC2A: *mut u8 = 0xF6 as *mut u8;

/// End Of Telegram Conditions 3 for Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTBFE3 | 10000000 |
/// | SYTFEA3 | 100 |
/// | MANFEA3 | 1000 |
/// | AMPFEA3 | 10 |
/// | RRFEA3 | 1000000 |
/// | TMOFEA3 | 10000 |
/// | CARFEA3 | 1 |
/// | TELREA3 | 100000 |
pub const EOTC3A: *mut u8 = 0xF7 as *mut u8;

/// End Of Telegram Conditions 1 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFEB1 | 1000000 |
/// | CARFEB1 | 1 |
/// | TELREB1 | 100000 |
/// | MANFEB1 | 1000 |
/// | TMOFEB1 | 10000 |
/// | AMPFEB1 | 10 |
/// | EOTAFE1 | 10000000 |
/// | SYTFEB1 | 100 |
pub const EOTC1B: *mut u8 = 0xF8 as *mut u8;

/// End Of Telegram Conditions 2 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMOFEB2 | 10000 |
/// | EOTAFE2 | 10000000 |
/// | TELREB2 | 100000 |
/// | MANFEB2 | 1000 |
/// | AMPFEB2 | 10 |
/// | CARFEB2 | 1 |
/// | RRFEB2 | 1000000 |
/// | SYTFEB2 | 100 |
pub const EOTC2B: *mut u8 = 0xF9 as *mut u8;

/// End Of Telegram Conditions 3 for Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RRFEB3 | 1000000 |
/// | EOTAFE3 | 10000000 |
/// | AMPFEB3 | 10 |
/// | SYTFEB3 | 100 |
/// | TELREB3 | 100000 |
/// | CARFEB3 | 1 |
/// | TMOFEB3 | 10000 |
/// | MANFEB3 | 1000 |
pub const EOTC3B: *mut u8 = 0xFA as *mut u8;

/// Wait check ok time out for path A.
pub const WCOTOA: *mut u8 = 0xFB as *mut u8;

/// Wait check ok time out for path B.
pub const WCOTOB: *mut u8 = 0xFC as *mut u8;

/// Start Of Telegram Time Out for path A.
pub const SOTTOA: *mut u8 = 0xFD as *mut u8;

/// Start Of Telegram Time Out for path B.
pub const SOTTOB: *mut u8 = 0xFE as *mut u8;

/// SSM Flow Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIDSO | 1 |
/// | SSMIDSF | 10 |
pub const SSMFCR: *mut u8 = 0xFF as *mut u8;

/// Front-End Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLCK | 1000 |
/// | ANTS | 10000 |
/// | XRDY | 100 |
/// | HBSAT | 10 |
/// | LBSAT | 1 |
pub const FESR: *mut u8 = 0x100 as *mut u8;

/// Front-End Enable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ATEN | 10000000 |
/// | PLEN | 1 |
/// | LNAEN | 1000 |
/// | PLSP1 | 1000000 |
/// | ADCLK | 100000 |
/// | XTOEN | 100 |
/// | PLCAL | 10 |
/// | ADEN | 10000 |
pub const FEEN1: *mut u8 = 0x101 as *mut u8;

/// Front-End Enable Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMPM | 1000 |
/// | PAEN | 100 |
/// | CPBIA | 1000000 |
/// | SDRX | 1 |
/// | PLPEN | 10000 |
/// | XTPEN | 100000 |
/// | SDTX | 10 |
pub const FEEN2: *mut u8 = 0x102 as *mut u8;

/// Front-End LNA Bias Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBH | 1111 |
/// | LBL | 11110000 |
pub const FELNA: *mut u8 = 0x103 as *mut u8;

/// Front-End Antenna Tuning.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ANTN | 1111 |
pub const FEAT: *mut u8 = 0x104 as *mut u8;

/// Front-End Power Amplifier Control Register Pout/dBm LowBand HighBand.
pub const FEPAC: *mut u8 = 0x105 as *mut u8;

/// Front-End VCO Tuning Register.
pub const FEVCT: *mut u8 = 0x106 as *mut u8;

/// Front-End RC Tuning Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTN2 | 1100 |
/// | CTN2 | 11 |
pub const FEBT: *mut u8 = 0x107 as *mut u8;

/// Front-End Main and Swallow Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLM | 11110000 |
/// | PLLS | 1111 |
pub const FEMS: *mut u8 = 0x108 as *mut u8;

/// Front-End RC Tuning 4bit Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTN4 | 11110000 |
/// | CTN4 | 1111 |
pub const FETN4: *mut u8 = 0x109 as *mut u8;

/// Front-End Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | S4N3 | 10 |
/// | ADHS | 1000 |
/// | PLCKG | 10000 |
/// | LBNHB | 1 |
/// | ANPS | 100000 |
/// | ANDP | 100 |
pub const FECR: *mut u8 = 0x10A as *mut u8;

/// Front-End VCO and PLL control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPCC | 1111 |
/// | VCOB | 11110000 |
pub const FEVCO: *mut u8 = 0x10B as *mut u8;

/// Front-End Antenna Level Detector Range.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RNGE | 11 |
pub const FEALR: *mut u8 = 0x10C as *mut u8;

/// Front-End ANTenna.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LVLC | 1111 |
pub const FEANT: *mut u8 = 0x10D as *mut u8;

/// Front-End IF Amplifier BIAS.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IFAEN | 10000000 |
pub const FEBIA: *mut u8 = 0x10E as *mut u8;

/// Tx Modulator Finite State Machine.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMSSM | 1111 |
/// | TMMSM | 1110000 |
pub const TMFSM: *mut u8 = 0x120 as *mut u8;

/// Tx Modulator CRC Result.
pub const TMCR: *mut u16 = 0x121 as *mut u16;

/// Tx Modulator CRC Result low byte.
pub const TMCRL: *mut u8 = 0x121 as *mut u8;

/// Tx Modulator CRC Result high byte.
pub const TMCRH: *mut u8 = 0x122 as *mut u8;

/// Tx Modulator CRC Skip Bit Number.
pub const TMCSB: *mut u8 = 0x123 as *mut u8;

/// Tx Modulator CRC Init Value.
pub const TMCI: *mut u16 = 0x124 as *mut u16;

/// Tx Modulator CRC Init Value low byte.
pub const TMCIL: *mut u8 = 0x124 as *mut u8;

/// Tx Modulator CRC Init Value high byte.
pub const TMCIH: *mut u8 = 0x125 as *mut u8;

/// Tx Modulator CRC Polynomial low byte.
pub const TMCPL: *mut u8 = 0x126 as *mut u8;

/// Tx Modulator CRC Polynomial.
pub const TMCP: *mut u16 = 0x126 as *mut u16;

/// Tx Modulator CRC Polynomial high byte.
pub const TMCPH: *mut u8 = 0x127 as *mut u8;

/// Tx Modulator Shift Register.
pub const TMSHR: *mut u8 = 0x128 as *mut u8;

/// Tx Modulator Telegram Length Register low byte.
pub const TMTLL: *mut u8 = 0x129 as *mut u8;

/// Tx Modulator Telegram Length Register.
pub const TMTL: *mut u16 = 0x129 as *mut u16;

/// Tx Modulator Telegram Length Register high byte.
pub const TMTLH: *mut u8 = 0x12A as *mut u8;

/// Tx Modulator Stop Sequence Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMSSL | 1110000 |
/// | TMSSH | 10000000 |
/// | TMSSP | 1111 |
pub const TMSSC: *mut u8 = 0x12B as *mut u8;

/// Tx Modulator Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMTCF | 1 |
pub const TMSR: *mut u8 = 0x12C as *mut u8;

/// Tx Modulator Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMCRCE | 1 |
/// | TMPOL | 10000 |
/// | TMNRZE | 1000 |
/// | TMCRCL | 110 |
/// | TMMSB | 1000000 |
/// | TMSSE | 100000 |
pub const TMCR2: *mut u8 = 0x12D as *mut u8;

/// Tx Modulator Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMCIM | 10000 |
/// | TMPIS | 111 |
/// | TMSCS | 1000 |
pub const TMCR1: *mut u8 = 0x12E as *mut u8;

/// Rx Buffer configuration register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXCBLB | 1100000 |
/// | RXCEB | 10000 |
/// | RXCEA | 1 |
/// | RXMSBA | 1000 |
/// | RXMSBB | 10000000 |
/// | RXCBLA | 110 |
pub const RXBC1: *mut u8 = 0x12F as *mut u8;

/// Rx Buffer configuration register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXBF | 10 |
/// | RXBCLR | 100 |
/// | RXBPB | 1 |
pub const RXBC2: *mut u8 = 0x130 as *mut u8;

/// Rx data telegram length register low byte for data path B.
pub const RXTLLB: *mut u8 = 0x131 as *mut u8;

/// Rx data telegram length register high byte for data path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTLHB1 | 10 |
/// | RXTLHB3 | 1000 |
/// | RXTLHB0 | 1 |
/// | RXTLHB2 | 100 |
pub const RXTLHB: *mut u8 = 0x132 as *mut u8;

/// Rx CRC result register low byte for data path B.
pub const RXCRLB: *mut u8 = 0x133 as *mut u8;

/// Rx CRC result register high byte for data path B.
pub const RXCRHB: *mut u8 = 0x134 as *mut u8;

/// Rx CRC skip bit number for data path B.
pub const RXCSBB: *mut u8 = 0x135 as *mut u8;

/// Rx CRC Init value (16-bit RXCI) low byte for data path B.
pub const RXCILB: *mut u8 = 0x136 as *mut u8;

/// Rx CRC Init value (16-bit RXCI) high byte for data path B.
pub const RXCIHB: *mut u8 = 0x137 as *mut u8;

/// Rx CRC polynomial low byte for data path B.
pub const RXCPLB: *mut u8 = 0x138 as *mut u8;

/// Rx CRC polynomial (15 bit RXCPB) high byte for data path B.
pub const RXCPHB: *mut u8 = 0x139 as *mut u8;

/// Rx data shift register for data path B.
pub const RXDSB: *mut u8 = 0x13A as *mut u8;

/// Rx data telegram length register low byte for data path A.
pub const RXTLLA: *mut u8 = 0x13B as *mut u8;

/// Rx data telegram length register high byte for data path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTLHA0 | 1 |
/// | RXTLHA1 | 10 |
/// | RXTLHA2 | 100 |
/// | RXTLHA3 | 1000 |
pub const RXTLHA: *mut u8 = 0x13C as *mut u8;

/// Rx CRC result register low byte for data path A.
pub const RXCRLA: *mut u8 = 0x13D as *mut u8;

/// Rx CRC result register high byte for data path A.
pub const RXCRHA: *mut u8 = 0x13E as *mut u8;

/// Rx CRC skip bit number for data path A.
pub const RXCSBA: *mut u8 = 0x13F as *mut u8;

/// Rx CRC Init value (16-bit RXCI) low byte for data path A.
pub const RXCILA: *mut u8 = 0x140 as *mut u8;

/// Rx CRC Init value (16-bit RXCI) high byte for data path A.
pub const RXCIHA: *mut u8 = 0x141 as *mut u8;

/// Rx CRC polynomial low byte for data path A.
pub const RXCPLA: *mut u8 = 0x142 as *mut u8;

/// Rx CRC polynomial (15 bit RXCPA) high byte for data path A.
pub const RXCPHA: *mut u8 = 0x143 as *mut u8;

/// Rx data shift register for data path A.
pub const RXDSA: *mut u8 = 0x144 as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCRS | 1 |
/// | REFLI | 10 |
/// | REFLO | 100 |
pub const CRCCR: *mut u8 = 0x145 as *mut u8;

/// CRC Data Output Register.
pub const CRCDOR: *mut u8 = 0x146 as *mut u8;

/// ID Byte 0.
pub const IDB0: *mut u8 = 0x147 as *mut u8;

/// ID Byte 1.
pub const IDB1: *mut u8 = 0x148 as *mut u8;

/// ID Byte 2.
pub const IDB2: *mut u8 = 0x149 as *mut u8;

/// ID Byte 3.
pub const IDB3: *mut u8 = 0x14A as *mut u8;

/// ID Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDCE | 10000000 |
/// | IDBO | 1100 |
/// | IDFIM | 100000 |
/// | IDL | 11 |
/// | IDCLR | 1000000 |
pub const IDC: *mut u8 = 0x14B as *mut u8;

/// ID Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDFULL | 10 |
/// | IDOK | 1 |
pub const IDS: *mut u8 = 0x14C as *mut u8;

/// RSSI Average Value.
pub const RSSAV: *mut u8 = 0x14D as *mut u8;

/// RSSI Peak Value.
pub const RSSPK: *mut u8 = 0x14E as *mut u8;

/// RSSI Low Threshold for Signal Check.
pub const RSSL: *mut u8 = 0x14F as *mut u8;

/// RSSI High Threshold for Signal Check.
pub const RSSH: *mut u8 = 0x150 as *mut u8;

/// RSSI Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSWLH | 10000 |
/// | RSUP | 1111 |
/// | RSPKF | 1000000 |
/// | RSHRX | 100000 |
pub const RSSC: *mut u8 = 0x151 as *mut u8;

/// DeBounce Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBCS | 10 |
/// | DBMD | 1 |
/// | DBTMS | 100 |
/// | DBHA | 1000 |
pub const DBCR: *mut u8 = 0x152 as *mut u8;

/// Debounce Timer Compare Register.
pub const DBTC: *mut u8 = 0x153 as *mut u8;

/// DeBounce Enable Port B.
pub const DBENB: *mut u8 = 0x154 as *mut u8;

/// DeBounce Enable Port C.
pub const DBENC: *mut u8 = 0x155 as *mut u8;

/// Debugging Support Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBGGS | 1111 |
/// | CPBFOS | 110000 |
/// | CPBF | 1000000 |
/// | DBGSE | 10000000 |
pub const DBGSW: *mut u8 = 0x156 as *mut u8;

/// SPI FIFO Fill Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RFC | 1000 |
/// | TFL | 1110000 |
/// | RFL | 111 |
/// | TFC | 10000000 |
pub const SFFR: *mut u8 = 0x157 as *mut u8;

/// SPI FIFO Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RIL | 111 |
/// | TIL | 1110000 |
/// | SRIE | 1000 |
/// | STIE | 10000000 |
pub const SFIR: *mut u8 = 0x158 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEBRE | 1 |
pub const EECR2: *mut u8 = 0x159 as *mut u8;

/// Program Memory Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGMSYN | 11111 |
pub const PGMST: *mut u8 = 0x15A as *mut u8;

/// EEPROM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESYN | 1111 |
pub const EEST: *mut u8 = 0x15B as *mut u8;

/// RSSI High IF Amplifier Gain.
pub const RSIFG: *mut u8 = 0x15C as *mut u8;

/// RSSI Low Band Damping Value.
pub const RSLDV: *mut u8 = 0x15D as *mut u8;

/// RSSI High Band Damping Value.
pub const RSHDV: *mut u8 = 0x15E as *mut u8;

/// RSSI Compensation Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSIFC | 10 |
/// | RSDC | 1 |
pub const RSCOM: *mut u8 = 0x15F as *mut u8;

/// Bitfield on register `CHCR`
pub const BWM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CHDN`
pub const BBDN: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `CHDN`
pub const ADCDN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CLKOCR`
pub const CLKOS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKOCR`
pub const CLKOEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CLPR`
pub const CLKPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CLPR`
pub const CLPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLPR`
pub const CLTPS: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `CMCR`
pub const SRCD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMCCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMONEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CMCR`
pub const CCS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CMIMR`
pub const ECIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMOCR`
pub const SRCACT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CMOCR`
pub const SRCAO: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CMOCR`
pub const FRCAO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMOCR`
pub const FRCACT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CMSR`
pub const ECF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CRCCR`
pub const CRCRS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CRCCR`
pub const REFLI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CRCCR`
pub const REFLO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBCS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBMD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBTMS: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBHA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DBGSW`
pub const DBGGS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DBGSW`
pub const CPBFOS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DBGSW`
pub const CPBF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DBGSW`
pub const DBGSE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFC`
pub const DFFLC: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `DFC`
pub const DFDRA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFI`
pub const DFERIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DFI`
pub const DFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DFL`
pub const DFFLS: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `DFL`
pub const DFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFS`
pub const DFOFL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DFS`
pub const DFUFL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DFS`
pub const DFFLRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DMCDA`
pub const DMCTA: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `DMCDA`
pub const DMCLA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCDB`
pub const DMCTB: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `DMCDB`
pub const DMCLB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCRA`
pub const SY1TA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMCRA`
pub const SASKA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMCRA`
pub const DMPGA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCRA`
pub const DMARA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMCRB`
pub const DMPGB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCRB`
pub const DMARB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMCRB`
pub const SY1TB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMCRB`
pub const SASKB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMDRA`
pub const DMDNA: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `DMDRA`
pub const DMAA: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DMDRB`
pub const DMDNB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `DMDRB`
pub const DMAB: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DMMA`
pub const DMATA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMMA`
pub const DMNEA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMMA`
pub const DMHA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMMA`
pub const DMPA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMATB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMMB`
pub const DMHB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMPB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMNEB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPAGE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR2`
pub const EEBRE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EEPR`
pub const EEAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EEST`
pub const EESYN: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const AMPFEA1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const TELREA1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const RRFEA1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const EOTBFE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const TMOFEA1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const CARFEA1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const MANFEA1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const SYTFEA1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const RRFEB1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const CARFEB1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const TELREB1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const MANFEB1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const TMOFEB1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const AMPFEB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const EOTAFE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const SYTFEB1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const CARFEA2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const EOTBFE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const TELREA2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const RRFEA2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const AMPFEA2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const SYTFEA2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const MANFEA2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const TMOFEA2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const TMOFEB2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const EOTAFE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const TELREB2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const MANFEB2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const AMPFEB2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const CARFEB2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const RRFEB2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const SYTFEB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const EOTBFE3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const SYTFEA3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const MANFEA3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const AMPFEA3: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const RRFEA3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const TMOFEA3: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const CARFEA3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const TELREA3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const RRFEB3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const EOTAFE3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const AMPFEB3: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const SYTFEB3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const TELREB3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const CARFEB3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const TMOFEB3: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const MANFEB3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTCA`
pub const RRFEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTCA`
pub const SYTFEA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTCA`
pub const TMOFEA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTCA`
pub const MANFEA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTCA`
pub const EOTBFE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTCA`
pub const AMPFEA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTCA`
pub const CARFEA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTCA`
pub const TELREA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTCB`
pub const RRFEB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTCB`
pub const MANFEB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTCB`
pub const CARFEB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTCB`
pub const AMPFEB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTCB`
pub const EOTAFE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTCB`
pub const SYTFEB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTCB`
pub const TELREB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTCB`
pub const TMOFEB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTSA`
pub const SYTFA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTSA`
pub const AMPFA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTSA`
pub const TELRA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTSA`
pub const EOTBF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTSA`
pub const RRFA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTSA`
pub const TMOFA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTSA`
pub const MANFA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTSA`
pub const CARFA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTSB`
pub const RRFB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTSB`
pub const TELRB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTSB`
pub const CARFB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTSB`
pub const EOTAF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTSB`
pub const AMPFB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTSB`
pub const TMOFB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTSB`
pub const MANFB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTSB`
pub const SYTFB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEALR`
pub const RNGE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FEANT`
pub const LVLC: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FEAT`
pub const ANTN: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FEBIA`
pub const IFAEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FEBT`
pub const RTN2: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `FEBT`
pub const CTN2: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FECR`
pub const S4N3: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FECR`
pub const ADHS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FECR`
pub const PLCKG: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FECR`
pub const LBNHB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FECR`
pub const ANPS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FECR`
pub const ANDP: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEEN1`
pub const ATEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FEEN1`
pub const PLEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FEEN1`
pub const LNAEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FEEN1`
pub const PLSP1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FEEN1`
pub const ADCLK: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FEEN1`
pub const XTOEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEEN1`
pub const PLCAL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FEEN1`
pub const ADEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FEEN2`
pub const TMPM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FEEN2`
pub const PAEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEEN2`
pub const CPBIA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FEEN2`
pub const SDRX: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FEEN2`
pub const PLPEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FEEN2`
pub const XTPEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FEEN2`
pub const SDTX: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FELNA`
pub const LBH: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FELNA`
pub const LBL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FEMS`
pub const PLLM: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FEMS`
pub const PLLS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FESR`
pub const PLCK: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FESR`
pub const ANTS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FESR`
pub const XRDY: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FESR`
pub const HBSAT: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FESR`
pub const LBSAT: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FETN4`
pub const RTN4: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FETN4`
pub const CTN4: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FEVCO`
pub const CPCC: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FEVCO`
pub const VCOB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FRCCAL`
pub const FRCTC: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FSCR`
pub const PAOER: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FSCR`
pub const SFM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FSCR`
pub const TXMOD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FSCR`
pub const PAON: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FSCR`
pub const TXMS: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `FSEN`
pub const ANTT: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FSEN`
pub const SDEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FSEN`
pub const SDPU: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FSEN`
pub const PEEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FSEN`
pub const ASEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FSEN`
pub const GAEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FSFCR`
pub const ASDIV: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FSFCR`
pub const BTSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCR`
pub const RXTEHA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCR`
pub const GAPMA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GTCR`
pub const GAPMB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GTCR`
pub const IWUPB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCR`
pub const IWUPA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GTCR`
pub const RXTEHB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GTCR`
pub const DARB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GTCR`
pub const DARA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `IDC`
pub const IDCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `IDC`
pub const IDBO: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `IDC`
pub const IDFIM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `IDC`
pub const IDL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `IDC`
pub const IDCLR: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `IDS`
pub const IDFULL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `IDS`
pub const IDOK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLP: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const AP: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOW`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const EXTCLKEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOW`
pub const RSTDISBL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const BOOTRST: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SPIIO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PB4HS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PB7LS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PB7HS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ENPS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MSMCR1`
pub const MSMSM0: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR1`
pub const MSMSM1: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR2`
pub const MSMSM2: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR2`
pub const MSMSM3: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR3`
pub const MSMSM4: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR3`
pub const MSMSM5: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR4`
pub const MSMSM7: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMCR4`
pub const MSMSM6: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMSTR`
pub const SSMMST: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT7: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK0`
pub const PCINT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT10: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT9: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT12: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT11: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCMSK1`
pub const PCINT13: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PGMST`
pub const PGMSYN: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCRC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRRXDC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTXDC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRVM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT5: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT4: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRXA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRDF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRTM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRXB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRRS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRIDS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDCR`
pub const RDEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDCR`
pub const ADIVEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDCR`
pub const RDPU: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDOCR`
pub const RDSIDB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDOCR`
pub const ETRPA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDOCR`
pub const RDSIDA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDOCR`
pub const TMDS: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `RDOCR`
pub const ETRPB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRTMP: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDPR`
pub const APRPTB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDPR`
pub const RDPRF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RDPR`
pub const APRPTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDPR`
pub const ARDPRF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRPTA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRFLT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRPTB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const WCOB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const EOTA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const NBITB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const EOTB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const NBITA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const WCOA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const SOTA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const SOTB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const WCOAM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const EOTBM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const SOTBM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const SOTAM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const NBITBM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const NBITAM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const EOTAM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const WCOBM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RSCOM`
pub const RSIFC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSCOM`
pub const RSDC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSSC`
pub const RSWLH: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSSC`
pub const RSUP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `RSSC`
pub const RSPKF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSSC`
pub const RSHRX: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCBLB: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCEB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCEA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXMSBA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXMSBB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCBLA: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `RXBC2`
pub const RXBF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXBC2`
pub const RXBCLR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RXBC2`
pub const RXBPB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RXTLHA`
pub const RXTLHA0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RXTLHA`
pub const RXTLHA1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXTLHA`
pub const RXTLHA2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RXTLHA`
pub const RXTLHA3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RXTLHB`
pub const RXTLHB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXTLHB`
pub const RXTLHB3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RXTLHB`
pub const RXTLHB0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RXTLHB`
pub const RXTLHB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SFC`
pub const SFDRA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFC`
pub const SFFLC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFFR`
pub const RFC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SFFR`
pub const TFL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `SFFR`
pub const RFL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SFFR`
pub const TFC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFI`
pub const SFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFI`
pub const SFERIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFIDCA`
pub const SFIDTA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFIDCA`
pub const SEMEA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIDCB`
pub const SEMEB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIDCB`
pub const SFIDTB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFIR`
pub const RIL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SFIR`
pub const TIL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `SFIR`
pub const SRIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SFIR`
pub const STIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFL`
pub const SFFLS: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFL`
pub const SFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFS`
pub const SFFLRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFS`
pub const SFOFL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SFS`
pub const SFUFL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const MANOEA1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const CAROEA1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const SYTOEA1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const WCOBOE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const SFIDEA1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const RROEA1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const WUPEA1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const AMPOEA1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const WUPEB1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const MANOEB1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const SYTOEB1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const SFIDEB1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const WCOAOE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const CAROEB1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const RROEB1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const AMPOEB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const AMPOEA2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const SFIDEA2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const CAROEA2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const SYTOEA2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const WUPEA2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const MANOEA2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const RROEA2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const WCOBOE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const WUPEB2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const MANOEB2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const SFIDEB2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const RROEB2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const SYTOEB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const WCOAOE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const CAROEB2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const AMPOEB2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTCA`
pub const WCOBOE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTCA`
pub const RROEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTCA`
pub const AMPOEA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTCA`
pub const SYTOEA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTCA`
pub const CAROEA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTCA`
pub const MANOEA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTCA`
pub const WUPEA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTCA`
pub const SFIDEA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTCB`
pub const CAROEB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTCB`
pub const WCOAOE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTCB`
pub const RROEB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTCB`
pub const SFIDEB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTCB`
pub const AMPOEB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTCB`
pub const SYTOEB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTCB`
pub const WUPEB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTCB`
pub const MANOEB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTSA`
pub const RROA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTSA`
pub const MANOA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTSA`
pub const WUPOA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTSA`
pub const SFIDOA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTSA`
pub const SYTOA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTSA`
pub const WCOBO: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTSA`
pub const AMPOA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTSA`
pub const CAROA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTSB`
pub const WUPOB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTSB`
pub const SFIDOB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTSB`
pub const RROB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTSB`
pub const SYTOB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTSB`
pub const AMPOB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTSB`
pub const MANOB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTSB`
pub const WCOAO: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTSB`
pub const CAROB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const TXIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPSR`
pub const RXIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCTC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SETRPA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTAE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTX: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SETRPB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTPE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTGE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMPVE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMDFDT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMHADT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMPLDT: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMFID: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SSMFCR`
pub const SSMIDSO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMFCR`
pub const SSMIDSF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMIFR`
pub const SSMIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMIMR`
pub const SSMIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMADA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMPB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMIDSE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMPVS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMADB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMIFA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMTMOE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMPA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMRR`
pub const SSMR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMRR`
pub const SSMST: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMSR`
pub const SSMESM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SSMSR`
pub const SSMERR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMSTR`
pub const SSMSTA: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SSMXSR`
pub const SSMSTB: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SUPCA1`
pub const PV22: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPCA1`
pub const PVDIC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SUPCA1`
pub const PVCAL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SUPCA2`
pub const BGCAL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SUPCA3`
pub const ACAL5: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const DCAL6: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const ACAL7: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const DCAL5: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const ACAL6: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const ACAL4: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const DCAL4: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const DCAL3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const DCAL0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const DCAL2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const ACAL3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const DCAL1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const ACAL2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const ACAL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SUPCA4`
pub const ACAL1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPCR`
pub const DVDIS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVCCRM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SUPCR`
pub const PVEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVCCLM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVDIC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SUPFR`
pub const AVCCLF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPFR`
pub const AVCCRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SYCA`
pub const SYTLA: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SYCA`
pub const SYCSA: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SYCB`
pub const SYCSB: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SYCB`
pub const SYTLB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0IE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T0IFR`
pub const T0F: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T1IFR`
pub const T1COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1IFR`
pub const T1OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1IMR`
pub const T1CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1IMR`
pub const T1OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1PS: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `T1MR`
pub const T1DC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2PS: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `T2MR`
pub const T2CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2DC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CPRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3PS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3ICS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3CNC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CPRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4MRA`
pub const T4CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T4MRA`
pub const T4PS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4ICS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4CNC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T5CCR`
pub const T5CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T5CCR`
pub const T5CTC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T5IFR`
pub const T5COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T5IFR`
pub const T5OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T5IMR`
pub const T5CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T5IMR`
pub const T5OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TESRA`
pub const EOTLA: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TESRA`
pub const CRCOA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TESRB`
pub const EOTLB: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TESRB`
pub const CRCOB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TMCR1`
pub const TMCIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TMCR1`
pub const TMPIS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TMCR1`
pub const TMSCS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TMCR2`
pub const TMCRCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TMCR2`
pub const TMPOL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TMCR2`
pub const TMNRZE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TMCR2`
pub const TMCRCL: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TMCR2`
pub const TMMSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TMCR2`
pub const TMSSE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TMFSM`
pub const TMSSM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `TMFSM`
pub const TMMSM: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `TMSR`
pub const TMTCF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TMSSC`
pub const TMSSL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `TMSSC`
pub const TMSSH: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TMSSC`
pub const TMSSP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMLS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDPS: *mut u8 = 0x7 as *mut u8;

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

/// `COMM_SCK_RATE_3BIT` value group
#[allow(non_upper_case_globals)]
pub mod comm_sck_rate_3bit {
   /// clkio/4.
   pub const VAL_0x00: u32 = 0x0;
   /// clkio/16.
   pub const VAL_0x01: u32 = 0x1;
   /// clkio/64.
   pub const VAL_0x02: u32 = 0x2;
   /// clkio/128.
   pub const VAL_0x03: u32 = 0x3;
   /// clkio/2.
   pub const VAL_0x04: u32 = 0x4;
   /// clkio/8.
   pub const VAL_0x05: u32 = 0x5;
   /// clkio/32.
   pub const VAL_0x06: u32 = 0x6;
   /// clkio/64.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_CLK_PRESCALE_3_BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_3_bits_small {
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
}

/// `CPU_CLT_PRESCALE_3_BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clt_prescale_3_bits_small {
   /// disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// 1.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.
   pub const VAL_0x02: u32 = 0x2;
   /// 4.
   pub const VAL_0x03: u32 = 0x3;
   /// 8.
   pub const VAL_0x04: u32 = 0x4;
   /// 16.
   pub const VAL_0x05: u32 = 0x5;
   /// 32.
   pub const VAL_0x06: u32 = 0x6;
   /// 64.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_3BITS2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits2 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Extended power-save.
   pub const EPSAVE: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
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

/// `ENUM_AP` value group
#[allow(non_upper_case_globals)]
pub mod enum_ap {
   /// LPM and SPM prohibited in Application Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const VAL_0x04: u32 = 0x4;
   /// SPM prohibited in Application Section.
   pub const VAL_0x08: u32 = 0x8;
   /// No lock on SPM and LPM in Application Section.
   pub const VAL_0x0C: u32 = 0xC;
}

/// `ENUM_BLP` value group
#[allow(non_upper_case_globals)]
pub mod enum_blp {
   /// LPM and SPM prohibited in Boot Loader Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Boot Loader Section.
   pub const VAL_0x10: u32 = 0x10;
   /// SPM prohibited in Boot Loader Section.
   pub const VAL_0x20: u32 = 0x20;
   /// No lock on SPM and LPM in Boot Loader Section.
   pub const VAL_0x30: u32 = 0x30;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const VAL_0x00: u32 = 0x0;
   /// Further programming disabled.
   pub const VAL_0x02: u32 = 0x2;
   /// No memory lock features enable.
   pub const VAL_0x03: u32 = 0x3;
}

/// `FE_ALR_RANGE` value group
#[allow(non_upper_case_globals)]
pub mod fe_alr_range {
   /// 0..3 dBm.
   pub const VAL_0x00: u32 = 0x0;
   /// 4..7 dBm.
   pub const VAL_0x01: u32 = 0x1;
   /// 8..14 dBm.
   pub const VAL_0x02: u32 = 0x2;
   /// Secure Measurement.
   pub const VAL_0x03: u32 = 0x3;
}

/// `FE_POWER_AMPLIFIER_CONTROL` value group
#[allow(non_upper_case_globals)]
pub mod fe_power_amplifier_control {
   /// -11.80  -12.90.
   pub const VAL_0x00: u32 = 0x0;
   /// -11.30  -12.33.
   pub const VAL_0x01: u32 = 0x1;
   /// -10.70  -11.76.
   pub const VAL_0x02: u32 = 0x2;
   /// -10.20  -11.10.
   pub const VAL_0x03: u32 = 0x3;
   /// -9.70  -10.60.
   pub const VAL_0x04: u32 = 0x4;
   /// -9.20  -10.00.
   pub const VAL_0x05: u32 = 0x5;
   /// -8.60   -9.50.
   pub const VAL_0x06: u32 = 0x6;
   /// -8.00   -9.00.
   pub const VAL_0x07: u32 = 0x7;
   /// -7.50   -8.50.
   pub const VAL_0x08: u32 = 0x8;
   /// -7.00   -7.90.
   pub const VAL_0x09: u32 = 0x9;
   /// -6.40   -7.30.
   pub const VAL_0x0A: u32 = 0xA;
   /// -5.90   -6.80.
   pub const VAL_0x0B: u32 = 0xB;
   /// -5.30   -6.30.
   pub const VAL_0x0C: u32 = 0xC;
   /// -4.77   -5.70.
   pub const VAL_0x0D: u32 = 0xD;
   /// -4.17   -5.20.
   pub const VAL_0x0E: u32 = 0xE;
   /// -3.67   -4.60.
   pub const VAL_0x0F: u32 = 0xF;
   /// -3.12   -4.07.
   pub const VAL_0x10: u32 = 0x10;
   /// -2.56   -3.47.
   pub const VAL_0x11: u32 = 0x11;
   /// -2.10   -2.97.
   pub const VAL_0x12: u32 = 0x12;
   /// -1.58   -2.42.
   pub const VAL_0x13: u32 = 0x13;
   /// -1.08   -1.86.
   pub const VAL_0x14: u32 = 0x14;
   /// -0.50   -1.40.
   pub const VAL_0x15: u32 = 0x15;
   /// 0.00   -0.88.
   pub const VAL_0x16: u32 = 0x16;
   /// 0.41   -0.38.
   pub const VAL_0x17: u32 = 0x17;
   /// 1.00    0.20.
   pub const VAL_0x18: u32 = 0x18;
   /// 1.42    0.70.
   pub const VAL_0x19: u32 = 0x19;
   /// 1.83    1.11.
   pub const VAL_0x1A: u32 = 0x1A;
   /// 2.42    1.70.
   pub const VAL_0x1B: u32 = 0x1B;
   /// 2.88    2.12.
   pub const VAL_0x1C: u32 = 0x1C;
   /// 3.38    2.53.
   pub const VAL_0x1D: u32 = 0x1D;
   /// 3.81    3.12.
   pub const VAL_0x1E: u32 = 0x1E;
   /// 4.31    3.58.
   pub const VAL_0x1F: u32 = 0x1F;
   /// 4.72    4.08.
   pub const VAL_0x20: u32 = 0x20;
   /// 5.09    4.51.
   pub const VAL_0x21: u32 = 0x21;
   /// 5.57    5.01.
   pub const VAL_0x22: u32 = 0x22;
   /// 6.00    5.42.
   pub const VAL_0x23: u32 = 0x23;
   /// 6.41    5.79.
   pub const VAL_0x24: u32 = 0x24;
   /// 6.77    6.27.
   pub const VAL_0x25: u32 = 0x25;
   /// 7.19    6.70.
   pub const VAL_0x26: u32 = 0x26;
   /// 7.55    7.11.
   pub const VAL_0x27: u32 = 0x27;
   /// 7.98    7.47.
   pub const VAL_0x28: u32 = 0x28;
   /// 8.40    7.89.
   pub const VAL_0x29: u32 = 0x29;
   /// 8.79    8.25.
   pub const VAL_0x2A: u32 = 0x2A;
   /// 9.11    8.68.
   pub const VAL_0x2B: u32 = 0x2B;
   /// 9.46    9.10.
   pub const VAL_0x2C: u32 = 0x2C;
   /// 9.82    9.49.
   pub const VAL_0x2D: u32 = 0x2D;
   /// 10.18    9.81.
   pub const VAL_0x2E: u32 = 0x2E;
   /// 10.60   10.16.
   pub const VAL_0x2F: u32 = 0x2F;
   /// 10.89   10.52.
   pub const VAL_0x30: u32 = 0x30;
   /// 11.30   10.88.
   pub const VAL_0x31: u32 = 0x31;
   /// 11.62   11.30.
   pub const VAL_0x32: u32 = 0x32;
   /// 12.06   11.59.
   pub const VAL_0x33: u32 = 0x33;
   /// 12.39   12.00.
   pub const VAL_0x34: u32 = 0x34;
   /// 12.82   12.32.
   pub const VAL_0x35: u32 = 0x35;
   /// 13.22   12.76.
   pub const VAL_0x36: u32 = 0x36;
   /// 13.58   13.09.
   pub const VAL_0x37: u32 = 0x37;
   /// 13.95   13.52.
   pub const VAL_0x38: u32 = 0x38;
   /// 14.22   13.92.
   pub const VAL_0x39: u32 = 0x39;
   /// 14.41   14.28.
   pub const VAL_0x3A: u32 = 0x3A;
   /// 14.49   14.65.
   pub const VAL_0x3B: u32 = 0x3B;
   /// 14.60   14.65.
   pub const VAL_0x3C: u32 = 0x3C;
   /// 14.60   14.65.
   pub const VAL_0x3D: u32 = 0x3D;
   /// 14.60   14.65.
   pub const VAL_0x3E: u32 = 0x3E;
   /// 14.60   14.65.
   pub const VAL_0x3F: u32 = 0x3F;
}

/// Interrupt Sense Control
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Logical Change of INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// `RXBUF_CRC_LENGTH` value group
#[allow(non_upper_case_globals)]
pub mod rxbuf_crc_length {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x02: u32 = 0x2;
}

/// `SSM_EOT_LOCATION` value group
#[allow(non_upper_case_globals)]
pub mod ssm_eot_location {
   /// No EOT.
   pub const VAL_0x00: u32 = 0x0;
   /// Before WCO.
   pub const VAL_0x01: u32 = 0x1;
   /// Between WCO and SOT.
   pub const VAL_0x02: u32 = 0x2;
   /// After SOT.
   pub const VAL_0x03: u32 = 0x3;
}

/// `SSM_SUB_STATE_MACHINE` value group
#[allow(non_upper_case_globals)]
pub mod ssm_sub_state_machine {
   /// None/Stop.
   pub const VAL_0x00: u32 = 0x0;
   /// PLL en.
   pub const VAL_0x01: u32 = 0x1;
   /// PLL lock.
   pub const VAL_0x02: u32 = 0x2;
   /// RX DSP enable.
   pub const VAL_0x03: u32 = 0x3;
   /// RX DSP disable.
   pub const VAL_0x04: u32 = 0x4;
   /// TX DSP enable.
   pub const VAL_0x05: u32 = 0x5;
   /// TX DSP disable.
   pub const VAL_0x06: u32 = 0x6;
   /// RX to TX.
   pub const VAL_0x07: u32 = 0x7;
   /// TX to RX.
   pub const VAL_0x08: u32 = 0x8;
   /// Get telegram.
   pub const VAL_0x09: u32 = 0x9;
   /// Send telegram.
   pub const VAL_0x0A: u32 = 0xA;
   /// Shut down.
   pub const VAL_0x0B: u32 = 0xB;
   /// VCO Tuning.
   pub const VAL_0x0C: u32 = 0xC;
   /// Antenna Tuning.
   pub const VAL_0x0D: u32 = 0xD;
}

/// `TIM1_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim1_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM2_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim2_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_vdiv.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_T.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto4.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM3_CAPTURE_EDGE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim3_capture_edge_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// rising edge.
   pub const VAL_0x01: u32 = 0x1;
   /// falling edge.
   pub const VAL_0x02: u32 = 0x2;
   /// both edges.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM3_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim3_clock_select {
   /// clk_frc.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_xto4.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto2.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM4_CAPTURE_EDGE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_capture_edge_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// rising edge.
   pub const VAL_0x01: u32 = 0x1;
   /// falling edge.
   pub const VAL_0x02: u32 = 0x2;
   /// both edges.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TIM4_CLOCK_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod tim4_clock_select {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_T.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_xto6.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_frc.
   pub const VAL_0x03: u32 = 0x3;
}

/// `TXM_CRC_LENGTH` value group
#[allow(non_upper_case_globals)]
pub mod txm_crc_length {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x02: u32 = 0x2;
}

/// `TXM_PINTERFACE_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod txm_pinterface_select {
   /// Port C3.
   pub const VAL_0x00: u32 = 0x0;
   /// M2 - Toggle Register Timer2.
   pub const VAL_0x01: u32 = 0x1;
   /// M3 - Toggle Register Timer3.
   pub const VAL_0x02: u32 = 0x2;
   /// M4 - Toggle Register Timer4.
   pub const VAL_0x03: u32 = 0x3;
   /// SO Tx Modulator Serial Output.
   pub const VAL_0x04: u32 = 0x4;
}

/// `TX_MODULATION_SOURCE` value group
#[allow(non_upper_case_globals)]
pub mod tx_modulation_source {
   /// TXMOD Register.
   pub const VAL_0x00: u32 = 0x0;
   /// TMDI Input.
   pub const VAL_0x01: u32 = 0x1;
   /// Tx Modulator Serial Out.
   pub const VAL_0x02: u32 = 0x2;
}

