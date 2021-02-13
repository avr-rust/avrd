//! The AVR ATA5787 microcontroller
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
/// | BOOTRST | 100 |
/// | DWEN | 1000000 |
/// | CKDIV8 | 10000000 |
/// | EESAVE | 1000 |
/// | RSTDISBL | 10 |
/// | SPIEN | 100000 |
/// | EXTCLKEN | 1 |
/// | WDTON | 10000 |
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

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRCO | 100000 |
/// | PRRXDC | 10 |
/// | PRTXDC | 100 |
/// | PRTRC | 10000000 |
/// | PRCRC | 1000 |
/// | PRVM | 10000 |
/// | PRSPI | 1 |
/// | PRUART | 1000000 |
pub const PRR0: *mut u8 = 0x21 as *mut u8;

/// Power Reduction Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRT2 | 10 |
/// | PRT1 | 1 |
/// | PRT5 | 10000 |
/// | PRT4 | 1000 |
/// | PRT3 | 100 |
pub const PRR1: *mut u8 = 0x22 as *mut u8;

/// Power Reduction Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRSSM | 10000000 |
/// | PRXB | 1 |
/// | PRSF | 100 |
/// | PRRS | 100000 |
/// | PRIDS | 10000 |
/// | PRXA | 10 |
/// | PRDF | 1000 |
pub const PRR2: *mut u8 = 0x23 as *mut u8;

/// Rx DSP Power Reduction.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDPRF | 10000000 |
/// | APRPTB | 10000 |
/// | ARDPRF | 1000000 |
/// | PRFLT | 100 |
/// | PRPTA | 10 |
/// | PRPTB | 1 |
/// | PRTMP | 1000 |
/// | APRPTA | 100000 |
pub const RDPR: *mut u8 = 0x24 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x25 as *mut u8;

/// Port B Data Direction.
pub const DDRB: *mut u8 = 0x26 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x27 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x28 as *mut u8;

/// Port C Data Direction.
pub const DDRC: *mut u8 = 0x29 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x2A as *mut u8;

/// Rx DSP Status Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTA | 100 |
/// | EOTB | 1000 |
/// | WCOB | 10000000 |
/// | NBITB | 10 |
/// | SOTA | 10000 |
/// | SOTB | 100000 |
/// | WCOA | 1000000 |
/// | NBITA | 1 |
pub const RDSIFR: *mut u8 = 0x2D as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUD | 10000 |
/// | PB7HS | 10000000 |
/// | PB4HS | 100000 |
/// | PB7LS | 1000000 |
/// | IVCE | 1 |
/// | ENPS | 1000 |
/// | IVSEL | 10 |
/// | SPIIO | 100 |
pub const MCUCR: *mut u8 = 0x2E as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
/// | PCIF1 | 10 |
pub const PCIFR: *mut u8 = 0x2F as *mut u8;

/// Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T0PR | 10000 |
/// | T0PS | 111 |
/// | T0IE | 1000 |
pub const T0CR: *mut u8 = 0x30 as *mut u8;

/// Timer1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T1TOS | 1000000 |
/// | T1CTM | 10 |
/// | T1TOP | 10000 |
/// | T1CRM | 100 |
/// | T1OTM | 1 |
/// | T1ENA | 10000000 |
/// | T1RES | 100000 |
pub const T1CR: *mut u8 = 0x31 as *mut u8;

/// Timer2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T2CTM | 10 |
/// | T2ENA | 10000000 |
/// | T2RES | 100000 |
/// | T2TOP | 10000 |
/// | T2TOS | 1000000 |
/// | T2OTM | 1 |
/// | T2CRM | 100 |
pub const T2CR: *mut u8 = 0x32 as *mut u8;

/// Timer3 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3RES | 100000 |
/// | T3CPRM | 1000 |
/// | T3OTM | 1 |
/// | T3CTM | 10 |
/// | T3ENA | 10000000 |
/// | T3CRM | 100 |
/// | T3TOP | 10000 |
/// | T3TOS | 1000000 |
pub const T3CR: *mut u8 = 0x33 as *mut u8;

/// Timer4 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CRM | 100 |
/// | T4OTM | 1 |
/// | T4CTM | 10 |
/// | T4RES | 100000 |
/// | T4TOP | 10000 |
/// | T4ENA | 10000000 |
/// | T4TOS | 1000000 |
/// | T4CPRM | 1000 |
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

/// Timer3 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3ICF | 100 |
/// | T3OFF | 1 |
/// | T3COF | 10 |
pub const T3IFR: *mut u8 = 0x37 as *mut u8;

/// Timer4 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4ICF | 100 |
/// | T4OFF | 1 |
/// | T4COF | 10 |
pub const T4IFR: *mut u8 = 0x38 as *mut u8;

/// Timer5 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OFF | 1 |
/// | T5COF | 10 |
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
/// | EEPM | 110000 |
/// | EEWE | 10 |
/// | EERE | 1 |
/// | EEMWE | 100 |
/// | NVMBSY | 10000000 |
/// | EEPAGE | 1000000 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Address Register.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Address Register low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Address Register high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// EEPROM Protect Register.
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

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE1 | 10 |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x46 as *mut u8;

/// External Interrupt Mask.
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
/// | INTF0 | 1 |
/// | INTF1 | 10 |
pub const EIFR: *mut u8 = 0x48 as *mut u8;

/// CRC Data Input Register.
pub const CRCDIR: *mut u8 = 0x49 as *mut u8;

/// Voltage Monitor Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VMIM | 10000 |
/// | VMLS | 1111 |
/// | VMF | 100000 |
pub const VMCSR: *mut u8 = 0x4A as *mut u8;

/// MCU Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | DWRF | 10000 |
/// | PORF | 1 |
/// | WDRF | 1000 |
pub const MCUSR: *mut u8 = 0x4B as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DORD | 100000 |
/// | SPE | 1000000 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
/// | SPR | 11 |
/// | CPHA | 100 |
/// | CPOL | 1000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXIF | 100000 |
/// | SPIF | 10000000 |
/// | RXIF | 10000 |
/// | SPI2X | 1 |
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

/// Debug Wire Data Register.
pub const DWDR: *mut u8 = 0x51 as *mut u8;

/// Rx DSP Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDPU | 1 |
/// | ADIVEN | 10 |
/// | RDEN | 100 |
pub const RDCR: *mut u8 = 0x52 as *mut u8;

/// End Of Telegram Status Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTFA | 100 |
/// | TELRA | 100000 |
/// | TMOFA | 10000 |
/// | MANFA | 1000 |
/// | CARFA | 1 |
/// | AMPFA | 10 |
/// | RRFA | 1000000 |
/// | EOTBF | 10000000 |
pub const EOTSA: *mut u8 = 0x53 as *mut u8;

/// End Of Telegram Conditions Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TELREA | 100000 |
/// | CARFEA | 1 |
/// | RRFEA | 1000000 |
/// | TMOFEA | 10000 |
/// | AMPFEA | 10 |
/// | EOTBFE | 10000000 |
/// | SYTFEA | 100 |
/// | MANFEA | 1000 |
pub const EOTCA: *mut u8 = 0x54 as *mut u8;

/// End Of Telegram Status Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TELRB | 100000 |
/// | MANFB | 1000 |
/// | RRFB | 1000000 |
/// | TMOFB | 10000 |
/// | AMPFB | 10 |
/// | CARFB | 1 |
/// | SYTFB | 100 |
/// | EOTAF | 10000000 |
pub const EOTSB: *mut u8 = 0x55 as *mut u8;

/// End Of Telegram Conditions Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTAFE | 10000000 |
/// | AMPFEB | 10 |
/// | TMOFEB | 10000 |
/// | TELREB | 100000 |
/// | RRFEB | 1000000 |
/// | MANFEB | 1000 |
/// | CARFEB | 1 |
/// | SYTFEB | 100 |
pub const EOTCB: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | PGERS | 10 |
/// | SELFPRGEN | 1 |
/// | SPMIE | 10000000 |
/// | BLBSET | 1000 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 110 |
pub const SMCR: *mut u8 = 0x59 as *mut u8;

/// Clock Management Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CMONEN | 1000000 |
/// | CCS | 1000 |
/// | CMM | 111 |
/// | CMCCE | 10000000 |
pub const CMCR: *mut u8 = 0x5A as *mut u8;

/// Clock Management Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECIE | 1 |
pub const CMIMR: *mut u8 = 0x5B as *mut u8;

/// Clock Prescaler.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 111 |
/// | CLPCE | 10000000 |
/// | CLTPS | 111000 |
pub const CLPR: *mut u8 = 0x5C as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x5D as *mut u16;

/// Stack Pointer low byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Stack Pointer high byte.
pub const SPH: *mut u8 = 0x5E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | H | 100000 |
/// | Z | 10 |
/// | I | 10000000 |
/// | S | 10000 |
/// | N | 100 |
/// | C | 1 |
/// | T | 1000000 |
/// | V | 1000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Frequency Synthesizer Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SDEN | 10 |
/// | SDPU | 1 |
pub const FSEN: *mut u8 = 0x60 as *mut u8;

/// Fractional Frequency 1 Low Byte.
pub const FFREQ1L: *mut u8 = 0x64 as *mut u8;

/// Fractional Frequency 1 Middle Byte.
pub const FFREQ1M: *mut u8 = 0x65 as *mut u8;

/// Fractional Frequency 1 High Byte.
pub const FFREQ1H: *mut u8 = 0x66 as *mut u8;

/// Fractional Frequency 2 Low Byte.
pub const FFREQ2L: *mut u8 = 0x67 as *mut u8;

/// Fractional Frequency 2 Middle Byte.
pub const FFREQ2M: *mut u8 = 0x68 as *mut u8;

/// Fractional Frequency 2 High Byte.
pub const FFREQ2H: *mut u8 = 0x69 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
pub const EICRA: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6C as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6D as *mut u8;

/// Watchdog Timer0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDCE | 10000 |
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
/// | T1DC | 11000000 |
/// | T1CS | 11 |
/// | T1PS | 111100 |
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
/// | T2CS | 11 |
/// | T2DC | 11000000 |
/// | T2PS | 111100 |
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

/// Timer3 Counter.
pub const T3CNT: *mut u16 = 0x77 as *mut u16;

/// Timer3 Counter low byte.
pub const T3CNTL: *mut u8 = 0x77 as *mut u8;

/// Timer3 Counter high byte.
pub const T3CNTH: *mut u8 = 0x78 as *mut u8;

/// Timer3 Compare.
pub const T3COR: *mut u16 = 0x79 as *mut u16;

/// Timer3 Compare low byte.
pub const T3CORL: *mut u8 = 0x79 as *mut u8;

/// Timer3 Compare high byte.
pub const T3CORH: *mut u8 = 0x7A as *mut u8;

/// Timer3 Input Capture low byte.
pub const T3ICRL: *mut u8 = 0x7B as *mut u8;

/// Timer3 Input Capture.
pub const T3ICR: *mut u16 = 0x7B as *mut u16;

/// Timer3 Input Capture high byte.
pub const T3ICRH: *mut u8 = 0x7C as *mut u8;

/// Timer3 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3PS | 11100 |
/// | T3CS | 11 |
pub const T3MRA: *mut u8 = 0x7D as *mut u8;

/// Timer3 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3CNC | 100 |
/// | T3CE | 11000 |
/// | T3SCE | 10 |
/// | T3ICS | 11100000 |
pub const T3MRB: *mut u8 = 0x7E as *mut u8;

/// Timer3 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T3OIM | 1 |
/// | T3CIM | 10 |
/// | T3CPIM | 100 |
pub const T3IMR: *mut u8 = 0x7F as *mut u8;

/// Timer4 Counter low byte.
pub const T4CNTL: *mut u8 = 0x80 as *mut u8;

/// Timer4 Counter.
pub const T4CNT: *mut u16 = 0x80 as *mut u16;

/// Timer4 Counter high byte.
pub const T4CNTH: *mut u8 = 0x81 as *mut u8;

/// Timer4 Compare.
pub const T4COR: *mut u16 = 0x82 as *mut u16;

/// Timer4 Compare low byte.
pub const T4CORL: *mut u8 = 0x82 as *mut u8;

/// Timer4 Compare high byte.
pub const T4CORH: *mut u8 = 0x83 as *mut u8;

/// Timer4 Input Capture.
pub const T4ICR: *mut u16 = 0x84 as *mut u16;

/// Timer4 Input Capture low byte.
pub const T4ICRL: *mut u8 = 0x84 as *mut u8;

/// Timer4 Input Capture high byte.
pub const T4ICRH: *mut u8 = 0x85 as *mut u8;

/// Timer4 Mode Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CS | 11 |
/// | T4PS | 11100 |
pub const T4MRA: *mut u8 = 0x86 as *mut u8;

/// Timer4 Mode Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4SCE | 10 |
/// | T4ICS | 11100000 |
/// | T4CE | 11000 |
/// | T4CNC | 100 |
pub const T4MRB: *mut u8 = 0x87 as *mut u8;

/// Timer4 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T4CIM | 10 |
/// | T4CPIM | 100 |
/// | T4OIM | 1 |
pub const T4IMR: *mut u8 = 0x88 as *mut u8;

/// Timer5 Output Compare low byte.
pub const T5OCRL: *mut u8 = 0x8A as *mut u8;

/// Timer5 Output Compare.
pub const T5OCR: *mut u16 = 0x8A as *mut u16;

/// Timer5 Output Compare high byte.
pub const T5OCRH: *mut u8 = 0x8B as *mut u8;

/// Timer5 Configuration and Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5CS | 111 |
/// | T5CTC | 1000 |
pub const T5CCR: *mut u8 = 0x8C as *mut u8;

/// Timer5 Counter.
pub const T5CNT: *mut u16 = 0x8D as *mut u16;

/// Timer5 Counter low byte.
pub const T5CNTL: *mut u8 = 0x8D as *mut u8;

/// Timer5 Counter high byte.
pub const T5CNTH: *mut u8 = 0x8E as *mut u8;

/// Timer5 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | T5OIM | 1 |
/// | T5CIM | 10 |
pub const T5IMR: *mut u8 = 0x8F as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR10 | 1 |
pub const GTCCR: *mut u8 = 0x90 as *mut u8;

/// Start Of Telegram Status Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WCOAO | 10000000 |
/// | RROB | 1000000 |
/// | SFIDOB | 100000 |
/// | CAROB | 1 |
/// | WUPOB | 10000 |
/// | AMPOB | 10 |
/// | SYTOB | 100 |
/// | MANOB | 1000 |
pub const SOTSB: *mut u8 = 0x91 as *mut u8;

/// Start Of Telegram Status Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MANOA | 1000 |
/// | WUPOA | 10000 |
/// | CAROA | 1 |
/// | SYTOA | 100 |
/// | WCOBO | 10000000 |
/// | SFIDOA | 100000 |
/// | RROA | 1000000 |
/// | AMPOA | 10 |
pub const SOTSA: *mut u8 = 0x92 as *mut u8;

/// Start Of Telegram Conditions Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RROEB | 1000000 |
/// | WUPEB | 10000 |
/// | AMPOEB | 10 |
/// | MANOEB | 1000 |
/// | SFIDEB | 100000 |
/// | WCOAOE | 10000000 |
/// | SYTOEB | 100 |
/// | CAROEB | 1 |
pub const SOTCB: *mut u8 = 0x93 as *mut u8;

/// Start Of Telegram Conditions Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTOEA | 100 |
/// | CAROEA | 1 |
/// | AMPOEA | 10 |
/// | WCOBOE | 10000000 |
/// | SFIDEA | 100000 |
/// | RROEA | 1000000 |
/// | WUPEA | 10000 |
/// | MANOEA | 1000 |
pub const SOTCA: *mut u8 = 0x94 as *mut u8;

/// Telegram Status Register Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTLB | 110 |
/// | CRCOB | 1 |
pub const TESRB: *mut u8 = 0x95 as *mut u8;

/// Telegram Status Register Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCOA | 1 |
/// | EOTLA | 110 |
pub const TESRA: *mut u8 = 0x96 as *mut u8;

/// Rx DSP Status Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NBITAM | 1 |
/// | EOTAM | 100 |
/// | SOTBM | 100000 |
/// | NBITBM | 10 |
/// | EOTBM | 1000 |
/// | SOTAM | 10000 |
/// | WCOBM | 10000000 |
/// | WCOAM | 1000000 |
pub const RDSIMR: *mut u8 = 0x98 as *mut u8;

/// Rx DSP Output Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RDSIDA | 100000 |
/// | RDSIDB | 1000000 |
/// | ETRPB | 10000 |
/// | ETRPA | 1000 |
/// | TMDS | 110 |
pub const RDOCR: *mut u8 = 0x99 as *mut u8;

/// Temperature.
pub const TEMP: *mut u16 = 0x9B as *mut u16;

/// Temperature low byte.
pub const TEMPL: *mut u8 = 0x9B as *mut u8;

/// Temperature high byte.
pub const TEMPH: *mut u8 = 0x9C as *mut u8;

/// Symbol Check Configuration Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTLB | 11110000 |
/// | SYCSB | 1111 |
pub const SYCB: *mut u8 = 0x9D as *mut u8;

/// Symbol Check Configuration Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTLA | 11110000 |
/// | SYCSA | 1111 |
pub const SYCA: *mut u8 = 0x9E as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency Path B.
pub const RXFOB: *mut u8 = 0x9F as *mut u8;

/// Received Frequency Offset vs Intermediate Frequency Path A.
pub const RXFOA: *mut u8 = 0xA0 as *mut u8;

/// Demodulator Signal Check Pattern Path B.
pub const DMPATB: *mut u8 = 0xA1 as *mut u8;

/// Demodulator Signal Check Pattern Path A.
pub const DMPATA: *mut u8 = 0xA2 as *mut u8;

/// Demodulator Pattern Check Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCSIGA | 100 |
/// | PCIALA | 10 |
/// | PCIALB | 100000 |
/// | PCFTDB | 10000 |
/// | PCSIGB | 1000000 |
/// | PCENA | 1000 |
/// | PCFTDA | 1 |
/// | PCENB | 10000000 |
pub const DMPC: *mut u8 = 0xA3 as *mut u8;

/// Demodulator Pattern Check Control Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSELB | 11000000 |
/// | PCSEVB | 111 |
/// | PCLENB | 111000 |
pub const DMPCB: *mut u8 = 0xA4 as *mut u8;

/// Demodulator Pattern Check Control Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCSEVA | 111 |
/// | PSELA | 11000000 |
/// | PCLENA | 111000 |
pub const DMPCA: *mut u8 = 0xA5 as *mut u8;

/// Demodulator Symbol Rate Path B.
pub const DMSRB: *mut u8 = 0xA6 as *mut u8;

/// Demodulator Symbol Rate Path A.
pub const DMSRA: *mut u8 = 0xA7 as *mut u8;

/// Demodulator Mode Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMHB | 1000000 |
/// | DMNEB | 10000000 |
/// | DMPB | 100000 |
/// | DMATB | 11111 |
pub const DMMB: *mut u8 = 0xA8 as *mut u8;

/// Demodulator Mode Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMHA | 1000000 |
/// | DMATA | 11111 |
/// | DMPA | 100000 |
/// | DMNEA | 10000000 |
pub const DMMA: *mut u8 = 0xA9 as *mut u8;

/// Demodulator Carrier Detect Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCLB | 11111 |
/// | DMCTB | 11100000 |
pub const DMCDB: *mut u8 = 0xAA as *mut u8;

/// Demodulator Carrier Detect Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMCTA | 11100000 |
/// | DMCLA | 11111 |
pub const DMCDA: *mut u8 = 0xAB as *mut u8;

/// Demodulator Control Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SY1TB | 1000000 |
/// | SASKB | 100000 |
/// | DMARB | 10000000 |
/// | DMPGB | 11111 |
pub const DMCRB: *mut u8 = 0xAC as *mut u8;

/// Demodulator Control Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMPGA | 11111 |
/// | SY1TA | 1000000 |
/// | DMARA | 10000000 |
/// | SASKA | 100000 |
pub const DMCRA: *mut u8 = 0xAD as *mut u8;

/// Demodulator Down Sampling.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DMDNA | 1111 |
/// | DMDNB | 11110000 |
pub const DMDN: *mut u8 = 0xAE as *mut u8;

/// Channel Filter Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BWM | 1111 |
pub const CHCR: *mut u8 = 0xAF as *mut u8;

/// Channel Filter Down Sampling Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADCDN | 100000 |
/// | BBDN | 11111 |
pub const CHDN: *mut u8 = 0xB0 as *mut u8;

/// Start Frame ID Configuration Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFIDTB | 11111 |
/// | SEMEB | 10000000 |
pub const SFIDCB: *mut u8 = 0xB1 as *mut u8;

/// Start Frame ID Length Path B.
pub const SFIDLB: *mut u8 = 0xB2 as *mut u8;

/// Wake-Up Pattern Threshold Path B.
pub const WUPTB: *mut u8 = 0xB3 as *mut u8;

/// Wake-Up Pattern Length Path B.
pub const WUPLB: *mut u8 = 0xB4 as *mut u8;

/// Start Frame ID Byte 1 Path B.
pub const SFID1B: *mut u8 = 0xB5 as *mut u8;

/// Start Frame ID Byte 2 Path B.
pub const SFID2B: *mut u8 = 0xB6 as *mut u8;

/// Start Frame ID Byte 3 Path B.
pub const SFID3B: *mut u8 = 0xB7 as *mut u8;

/// Start Frame ID Byte 4 Path B.
pub const SFID4B: *mut u8 = 0xB8 as *mut u8;

/// Wake-Up Pattern Byte 1 Path B.
pub const WUP1B: *mut u8 = 0xB9 as *mut u8;

/// Wake-Up Pattern Byte 2 Path B.
pub const WUP2B: *mut u8 = 0xBA as *mut u8;

/// Wake-Up Pattern Byte 3 Path B.
pub const WUP3B: *mut u8 = 0xBB as *mut u8;

/// Wake-Up Pattern Byte 4 Path B.
pub const WUP4B: *mut u8 = 0xBC as *mut u8;

/// Start Frame ID Configuration Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEMEA | 10000000 |
/// | SFIDTA | 11111 |
pub const SFIDCA: *mut u8 = 0xBD as *mut u8;

/// Start Frame ID Length Path A.
pub const SFIDLA: *mut u8 = 0xBE as *mut u8;

/// Wake-Up Pattern Threshold Path A.
pub const WUPTA: *mut u8 = 0xBF as *mut u8;

/// Wake-Up Pattern Length Path A.
pub const WUPLA: *mut u8 = 0xC0 as *mut u8;

/// Start Frame ID Byte 1 Path A.
pub const SFID1A: *mut u8 = 0xC1 as *mut u8;

/// Start Frame ID Byte 2 Path A.
pub const SFID2A: *mut u8 = 0xC2 as *mut u8;

/// Start Frame ID Byte 3 Path A.
pub const SFID3A: *mut u8 = 0xC3 as *mut u8;

/// Start Frame ID Byte 4 Path A.
pub const SFID4A: *mut u8 = 0xC4 as *mut u8;

/// Wake-Up Pattern Byte 1 Path A.
pub const WUP1A: *mut u8 = 0xC5 as *mut u8;

/// Wake-Up Pattern Byte 2 Path A.
pub const WUP2A: *mut u8 = 0xC6 as *mut u8;

/// Wake-Up Pattern Byte 3 Path A.
pub const WUP3A: *mut u8 = 0xC7 as *mut u8;

/// Wake-Up Pattern Byte 4 Path A.
pub const WUP4A: *mut u8 = 0xC8 as *mut u8;

/// Clock Output Divider.
pub const CLKOD: *mut u8 = 0xC9 as *mut u8;

/// Clock output control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKOS | 11 |
/// | CLKOEN | 100 |
pub const CLKOCR: *mut u8 = 0xCA as *mut u8;

/// XROW Fuse.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NVPTE | 10000 |
/// | E2PT | 1100 |
/// | FLPT | 11 |
/// | CKOUT | 1000000 |
pub const XFUSE: *mut u8 = 0xCB as *mut u8;

/// Slow RC Oscillator Calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRCTC | 11000000 |
pub const SRCCAL: *mut u8 = 0xCC as *mut u8;

/// Fast RC Oscillator Calibration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCTC | 100000 |
pub const FRCCAL: *mut u8 = 0xCD as *mut u8;

/// Clock Management Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ECF | 1 |
pub const CMSR: *mut u8 = 0xCE as *mut u8;

/// Clock Management Override Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FRCACT | 100 |
/// | FRCAO | 1 |
/// | SRCACT | 1000 |
/// | SRCAO | 10 |
pub const CMOCR: *mut u8 = 0xCF as *mut u8;

/// Supply Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCRDYF | 100 |
/// | AVCCRF | 1 |
/// | DCERF | 1000 |
/// | AVCCLF | 10 |
pub const SUPFR: *mut u8 = 0xD0 as *mut u8;

/// Supply Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AVCCLM | 10 |
/// | AVEN | 100000 |
/// | AVCCRM | 1 |
/// | AVDIC | 1000000 |
pub const SUPCR: *mut u8 = 0xD1 as *mut u8;

/// Supply Calibration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VVCAL | 1100 |
/// | AVCAL | 110000 |
/// | DVCAL | 11 |
pub const SUPCA1: *mut u8 = 0xD2 as *mut u8;

/// Supply Calibration 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCAL | 111111 |
pub const SUPCA2: *mut u8 = 0xD3 as *mut u8;

/// Supply Calibration 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMCAL | 110000 |
/// | VMCAL | 1100 |
/// | VMOCAL | 11000000 |
/// | DMCAL | 11 |
pub const SUPCA3: *mut u8 = 0xD4 as *mut u8;

/// Supply Calibration 4.
pub const SUPCA4: *mut u8 = 0xD5 as *mut u8;

/// DCDC Converter Calibration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ZCCAL | 11110000 |
/// | CCAL | 1111 |
pub const DCCAL1: *mut u8 = 0xD6 as *mut u8;

/// DCDC Converter Calibration 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DTCAL | 1111 |
/// | OFCAL | 11110000 |
pub const DCCAL2: *mut u8 = 0xD7 as *mut u8;

/// DCDC Converter Calibration 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SAWCAL | 11 |
pub const DCCAL3: *mut u8 = 0xD8 as *mut u8;

/// DCDC Converter Test Mode.
pub const DCTST: *mut u8 = 0xD9 as *mut u8;

/// Calibration Ready Signature.
pub const CALRDY: *mut u8 = 0xDA as *mut u8;

/// Resistor Capacitor 4 Bit Tuning.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTUNE4 | 11110000 |
/// | CTUNE4 | 1111 |
pub const RCTUNE4: *mut u8 = 0xDC as *mut u8;

/// DCDC Converter Control 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCHSSW | 10 |
/// | DCCDIV | 110000 |
/// | DCEN | 1 |
pub const DCC1: *mut u8 = 0xDD as *mut u8;

/// DCDC Converter Control 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DCDRV | 110000 |
/// | DCVOUT | 11 |
/// | DCCLIM | 1100 |
pub const DCC2: *mut u8 = 0xDE as *mut u8;

/// Data FIFO Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFUFL | 10 |
/// | DFFLRF | 1 |
/// | DFOFL | 100 |
pub const DFS: *mut u8 = 0xDF as *mut u8;

/// Data FIFO Telegram Length.
pub const DFTL: *mut u16 = 0xE0 as *mut u16;

/// Data FIFO Telegram Length low byte.
pub const DFTLL: *mut u8 = 0xE0 as *mut u8;

/// Data FIFO Telegram Length high byte.
pub const DFTLH: *mut u8 = 0xE1 as *mut u8;

/// Data FIFO Fill Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFFLS | 111111 |
/// | DFCLR | 10000000 |
pub const DFL: *mut u8 = 0xE2 as *mut u8;

/// Data FIFO Write Pointer.
pub const DFWP: *mut u8 = 0xE3 as *mut u8;

/// Data FIFO Read Pointer.
pub const DFRP: *mut u8 = 0xE4 as *mut u8;

/// Data FIFO Data.
pub const DFD: *mut u8 = 0xE5 as *mut u8;

/// Data FIFO Interrupt Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFERIM | 10 |
/// | DFFLIM | 1 |
pub const DFI: *mut u8 = 0xE6 as *mut u8;

/// Data FIFO Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DFDRA | 10000000 |
/// | DFFLC | 111111 |
pub const DFC: *mut u8 = 0xE7 as *mut u8;

/// Support FIFO Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLRF | 1 |
/// | SFUFL | 10 |
/// | SFOFL | 100 |
pub const SFS: *mut u8 = 0xE8 as *mut u8;

/// Support FIFO Fill Level.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFCLR | 10000000 |
/// | SFFLS | 11111 |
pub const SFL: *mut u8 = 0xE9 as *mut u8;

/// Support FIFO Write Pointer.
pub const SFWP: *mut u8 = 0xEA as *mut u8;

/// Support FIFO Read Pointer.
pub const SFRP: *mut u8 = 0xEB as *mut u8;

/// Support FIFO Data.
pub const SFD: *mut u8 = 0xEC as *mut u8;

/// Support FIFO Interrupt Mask.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFFLIM | 1 |
/// | SFERIM | 10 |
pub const SFI: *mut u8 = 0xED as *mut u8;

/// Support FIFO Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SFDRA | 10000000 |
/// | SFFLC | 11111 |
pub const SFC: *mut u8 = 0xEE as *mut u8;

/// Sequencer State Machine Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMTX | 1 |
/// | SETRPB | 10000000 |
/// | SETRPA | 1000000 |
/// | SSMTM | 10 |
pub const SSMCR: *mut u8 = 0xEF as *mut u8;

/// Sequencer State Machine Rx Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMPB | 10 |
/// | SSMHIS | 1000 |
/// | SSMAD | 100 |
/// | SSMIDSE | 1000000 |
/// | SSMPA | 1 |
/// | SSMPVS | 10000 |
/// | SSMIFA | 100000 |
/// | SSMTMOE | 10000000 |
pub const SSMRCR: *mut u8 = 0xF0 as *mut u8;

/// Sequencer State Machine Filter Bandwidth Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMFID | 111 |
/// | SSMDFDT | 1000 |
/// | SSMPLDT | 100000 |
/// | SSMHADT | 10000 |
pub const SSMFBR: *mut u8 = 0xF1 as *mut u8;

/// Sequencer State Machine Run Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMST | 10 |
/// | SSMR | 1 |
pub const SSMRR: *mut u8 = 0xF2 as *mut u8;

/// Sequencer State Machine Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMERR | 10000000 |
/// | SSMESM | 1111 |
pub const SSMSR: *mut u8 = 0xF3 as *mut u8;

/// Sequencer State Machine Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIF | 1 |
pub const SSMIFR: *mut u8 = 0xF4 as *mut u8;

/// Sequencer State Machine Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIM | 1 |
pub const SSMIMR: *mut u8 = 0xF5 as *mut u8;

/// Master State Machine State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMMST | 11111 |
pub const MSMSTR: *mut u8 = 0xF6 as *mut u8;

/// Sequencer State Machine State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTA | 111111 |
pub const SSMSTR: *mut u8 = 0xF7 as *mut u8;

/// Sequencer State Machine Extended State Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMSTB | 111111 |
pub const SSMXSR: *mut u8 = 0xF8 as *mut u8;

/// Master State Machine Control Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM0 | 1111 |
/// | MSMSM1 | 11110000 |
pub const MSMCR1: *mut u8 = 0xF9 as *mut u8;

/// Master State Machine Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM2 | 1111 |
/// | MSMSM3 | 11110000 |
pub const MSMCR2: *mut u8 = 0xFA as *mut u8;

/// Master State Machine Control Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM4 | 1111 |
/// | MSMSM5 | 11110000 |
pub const MSMCR3: *mut u8 = 0xFB as *mut u8;

/// Master State Machine Control Register 4.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MSMSM6 | 1111 |
/// | MSMSM7 | 11110000 |
pub const MSMCR4: *mut u8 = 0xFC as *mut u8;

/// Get Telegram Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXTEHB | 10000 |
/// | DARB | 1000000 |
/// | IWUPB | 10000000 |
/// | GAPMA | 10 |
/// | DARA | 100 |
/// | IWUPA | 1000 |
/// | GAPMB | 100000 |
/// | RXTEHA | 1 |
pub const GTCR: *mut u8 = 0xFD as *mut u8;

/// RF Front End Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XRDY | 100 |
/// | SAT | 1 |
/// | PLCK | 1000 |
pub const FESR: *mut u8 = 0x100 as *mut u8;

/// RF Front End Enable 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLCAL | 10 |
/// | ADCLK | 100000 |
/// | XTOEN | 100 |
/// | PLEN | 1 |
/// | ADEN | 10000 |
/// | LNAEN | 1000 |
/// | PLSP1 | 1000000 |
pub const FEEN1: *mut u8 = 0x101 as *mut u8;

/// RF Front End Enable 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TMPM | 1000 |
/// | XTPEN | 100000 |
/// | PAEN | 100 |
/// | XTOEXT | 10000000 |
/// | PLPEN | 10000 |
pub const FEEN2: *mut u8 = 0x102 as *mut u8;

/// RF Front End LNA Bias.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LNABN | 1111 |
/// | LNABH | 11110000 |
pub const FELNA: *mut u8 = 0x103 as *mut u8;

/// RF Front End Antenna Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SDRX2 | 100 |
/// | SDRX3 | 10000 |
/// | SDRX1 | 1 |
/// | SDTX3 | 100000 |
/// | SDTX2 | 1000 |
/// | SDTX1 | 10 |
pub const FEAS: *mut u8 = 0x104 as *mut u8;

/// RF Front End VCO Tuning.
pub const FEVCT: *mut u8 = 0x106 as *mut u8;

/// RF Front End RC Tuning.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTN2 | 11 |
/// | RTN2 | 1100 |
pub const FEBT: *mut u8 = 0x107 as *mut u8;

/// RF Front End Main and Swallow Counter.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLLS | 1111 |
/// | PLLM | 11110000 |
pub const FEMS: *mut u8 = 0x108 as *mut u8;

/// RF Front End RC Tuning 4bit Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RTN4 | 11110000 |
/// | CTN4 | 1111 |
pub const FETN4: *mut u8 = 0x109 as *mut u8;

/// RF Front End Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLCKG | 10000 |
/// | ADHS | 1000 |
/// | ANPS | 100000 |
/// | ANDP | 100 |
/// | S4N3 | 10 |
/// | LBNHB | 1 |
pub const FECR: *mut u8 = 0x10A as *mut u8;

/// RF Front End VCO and PLL Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VCOB | 11110000 |
/// | CPCC | 1111 |
pub const FEVCO: *mut u8 = 0x10B as *mut u8;

/// RF Front End Amplifier Bias.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | HISEN | 1000000 |
/// | IFAEN | 10000000 |
pub const FEBIA: *mut u8 = 0x10C as *mut u8;

/// RF Front End Spare Register 1.
pub const SPARE1: *mut u8 = 0x10D as *mut u8;

/// Start Of Telegram Conditions 1 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTOEA1 | 100 |
/// | MANOEA1 | 1000 |
/// | RROEA1 | 1000000 |
/// | WUPEA1 | 10000 |
/// | SFIDEA1 | 100000 |
/// | CAROEA1 | 1 |
/// | AMPOEA1 | 10 |
/// | WCOBOE1 | 10000000 |
pub const SOTC1A: *mut u8 = 0x120 as *mut u8;

/// Start Of Telegram Conditions 2 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPOEA2 | 10 |
/// | SFIDEA2 | 100000 |
/// | WCOBOE2 | 10000000 |
/// | SYTOEA2 | 100 |
/// | MANOEA2 | 1000 |
/// | CAROEA2 | 1 |
/// | WUPEA2 | 10000 |
/// | RROEA2 | 1000000 |
pub const SOTC2A: *mut u8 = 0x121 as *mut u8;

/// Start Of Telegram Conditions 1 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTOEB1 | 100 |
/// | WCOAOE1 | 10000000 |
/// | RROEB1 | 1000000 |
/// | AMPOEB1 | 10 |
/// | SFIDEB1 | 100000 |
/// | MANOEB1 | 1000 |
/// | WUPEB1 | 10000 |
/// | CAROEB1 | 1 |
pub const SOTC1B: *mut u8 = 0x122 as *mut u8;

/// Start Of Telegram Conditions 2 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPOEB2 | 10 |
/// | RROEB2 | 1000000 |
/// | WCOAOE2 | 10000000 |
/// | WUPEB2 | 10000 |
/// | MANOEB2 | 1000 |
/// | SFIDEB2 | 100000 |
/// | SYTOEB2 | 100 |
/// | CAROEB2 | 1 |
pub const SOTC2B: *mut u8 = 0x123 as *mut u8;

/// End Of Telegram Conditions 1 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTBFE1 | 10000000 |
/// | MANFEA1 | 1000 |
/// | RRFEA1 | 1000000 |
/// | TELREA1 | 100000 |
/// | SYTFEA1 | 100 |
/// | TMOFEA1 | 10000 |
/// | CARFEA1 | 1 |
/// | AMPFEA1 | 10 |
pub const EOTC1A: *mut u8 = 0x124 as *mut u8;

/// End Of Telegram Conditions 2 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TELREA2 | 100000 |
/// | MANFEA2 | 1000 |
/// | EOTBFE2 | 10000000 |
/// | SYTFEA2 | 100 |
/// | RRFEA2 | 1000000 |
/// | CARFEA2 | 1 |
/// | AMPFEA2 | 10 |
/// | TMOFEA2 | 10000 |
pub const EOTC2A: *mut u8 = 0x125 as *mut u8;

/// End Of Telegram Conditions 3 Path A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TELREA3 | 100000 |
/// | TMOFEA3 | 10000 |
/// | MANFEA3 | 1000 |
/// | SYTFEA3 | 100 |
/// | RRFEA3 | 1000000 |
/// | CARFEA3 | 1 |
/// | AMPFEA3 | 10 |
/// | EOTBFE3 | 10000000 |
pub const EOTC3A: *mut u8 = 0x126 as *mut u8;

/// End Of Telegram Conditions 1 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SYTFEB1 | 100 |
/// | EOTAFE1 | 10000000 |
/// | MANFEB1 | 1000 |
/// | AMPFEB1 | 10 |
/// | TMOFEB1 | 10000 |
/// | RRFEB1 | 1000000 |
/// | TELREB1 | 100000 |
/// | CARFEB1 | 1 |
pub const EOTC1B: *mut u8 = 0x127 as *mut u8;

/// End Of Telegram Conditions 2 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMPFEB2 | 10 |
/// | TMOFEB2 | 10000 |
/// | TELREB2 | 100000 |
/// | MANFEB2 | 1000 |
/// | SYTFEB2 | 100 |
/// | EOTAFE2 | 10000000 |
/// | RRFEB2 | 1000000 |
/// | CARFEB2 | 1 |
pub const EOTC2B: *mut u8 = 0x128 as *mut u8;

/// End Of Telegram Conditions 3 Path B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EOTAFE3 | 10000000 |
/// | SYTFEB3 | 100 |
/// | CARFEB3 | 1 |
/// | RRFEB3 | 1000000 |
/// | TELREB3 | 100000 |
/// | MANFEB3 | 1000 |
/// | AMPFEB3 | 10 |
/// | TMOFEB3 | 10000 |
pub const EOTC3B: *mut u8 = 0x129 as *mut u8;

/// Wake Check Ok Time-Out Path A.
pub const WCOTOA: *mut u8 = 0x12A as *mut u8;

/// Wake Check Ok Time-Out Path B.
pub const WCOTOB: *mut u8 = 0x12B as *mut u8;

/// Start Of Telegram Time Out Path A.
pub const SOTTOA: *mut u8 = 0x12C as *mut u8;

/// Start Of Telegram Time Out Path B.
pub const SOTTOB: *mut u8 = 0x12D as *mut u8;

/// Sequencer State Machine Flow Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SSMIDSF | 10 |
/// | SSMIDSO | 1 |
/// | SSMSEH | 100 |
/// | SSMCEH | 1000 |
pub const SSMFCR: *mut u8 = 0x12E as *mut u8;

/// Rx Buffer Configuration 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXCEB | 10000 |
/// | RXCBLB | 1100000 |
/// | RXLSBA | 1000 |
/// | RXLSBB | 10000000 |
/// | RXCBLA | 110 |
/// | RXCEA | 1 |
pub const RXBC1: *mut u8 = 0x13E as *mut u8;

/// Rx Buffer Configuration 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXBPB | 1 |
/// | RXBF | 10 |
/// | RXBCLR | 100 |
pub const RXBC2: *mut u8 = 0x13F as *mut u8;

/// Rx Telegram Length Low Byte Path B.
pub const RXTLLB: *mut u8 = 0x140 as *mut u8;

/// Rx Telegram Length High Byte Path B.
pub const RXTLHB: *mut u8 = 0x141 as *mut u8;

/// Rx CRC Result Low Byte Path B.
pub const RXCRLB: *mut u8 = 0x142 as *mut u8;

/// Rx CRC Result High Byte Path B.
pub const RXCRHB: *mut u8 = 0x143 as *mut u8;

/// Rx CRC Skip Bit Number Path B.
pub const RXCSBB: *mut u8 = 0x144 as *mut u8;

/// Rx CRC Init Value Low Byte Path B.
pub const RXCILB: *mut u8 = 0x145 as *mut u8;

/// Rx CRC Init Value High Byte Path B.
pub const RXCIHB: *mut u8 = 0x146 as *mut u8;

/// Rx CRC Polynomial Low Byte Path B.
pub const RXCPLB: *mut u8 = 0x147 as *mut u8;

/// Rx CRC Polynomial High Byte Path B.
pub const RXCPHB: *mut u8 = 0x148 as *mut u8;

/// Receive Data Shift Register Path B.
pub const RXDSB: *mut u8 = 0x149 as *mut u8;

/// Rx Telegram Length Low Byte Path A.
pub const RXTLLA: *mut u8 = 0x14A as *mut u8;

/// Rx Telegram Length High Byte Path A.
pub const RXTLHA: *mut u8 = 0x14B as *mut u8;

/// Rx CRC Result Low Byte Path A.
pub const RXCRLA: *mut u8 = 0x14C as *mut u8;

/// Rx CRC Result High Byte Path A.
pub const RXCRHA: *mut u8 = 0x14D as *mut u8;

/// Rx CRC Skip Bit Number Path A.
pub const RXCSBA: *mut u8 = 0x14E as *mut u8;

/// Rx CRC Init Value Low Byte Path A.
pub const RXCILA: *mut u8 = 0x14F as *mut u8;

/// Rx CRC Init Value High Byte Path A.
pub const RXCIHA: *mut u8 = 0x150 as *mut u8;

/// Rx CRC Polynomial Low Byte Path A.
pub const RXCPLA: *mut u8 = 0x151 as *mut u8;

/// Rx CRC Polynomial High Byte Path A.
pub const RXCPHA: *mut u8 = 0x152 as *mut u8;

/// Receive Data Shift Register Path A.
pub const RXDSA: *mut u8 = 0x153 as *mut u8;

/// CRC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CRCRS | 1 |
/// | REFLO | 100 |
/// | REFLI | 10 |
pub const CRCCR: *mut u8 = 0x154 as *mut u8;

/// CRC Data Output Register.
pub const CRCDOR: *mut u8 = 0x155 as *mut u8;

/// ID Check Byte 0.
pub const IDB0: *mut u8 = 0x156 as *mut u8;

/// ID Check Byte 1.
pub const IDB1: *mut u8 = 0x157 as *mut u8;

/// ID Check Byte 2.
pub const IDB2: *mut u8 = 0x158 as *mut u8;

/// ID Check Byte 3.
pub const IDB3: *mut u8 = 0x159 as *mut u8;

/// ID Check Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDFIM | 100000 |
/// | IDL | 11 |
/// | IDCLR | 1000000 |
/// | IDCE | 10000000 |
/// | IDBO | 1100 |
pub const IDC: *mut u8 = 0x15A as *mut u8;

/// ID Check Status.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | IDFULL | 10 |
/// | IDOK | 1 |
pub const IDS: *mut u8 = 0x15B as *mut u8;

/// RSSI Average Value.
pub const RSSAV: *mut u8 = 0x15C as *mut u8;

/// RSSI Peak Value.
pub const RSSPK: *mut u8 = 0x15D as *mut u8;

/// RSSI Low Threshold.
pub const RSSL: *mut u8 = 0x15E as *mut u8;

/// RSSI High Threshold.
pub const RSSH: *mut u8 = 0x15F as *mut u8;

/// RSSI Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSPKF | 1000000 |
/// | RSUP | 1111 |
/// | RSWLH | 10000 |
/// | RSHRX | 100000 |
pub const RSSC: *mut u8 = 0x160 as *mut u8;

/// Debounce Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBMD | 1 |
/// | DBTMS | 100 |
/// | DBCS | 10 |
pub const DBCR: *mut u8 = 0x161 as *mut u8;

/// Debounce Timer Compare.
pub const DBTC: *mut u8 = 0x162 as *mut u8;

/// Debounce Enable Port B.
pub const DBENB: *mut u8 = 0x163 as *mut u8;

/// Debounce Enable Port C.
pub const DBENC: *mut u8 = 0x164 as *mut u8;

/// Debug Support Switch.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DBGSE | 10000000 |
/// | DBGGS | 1111 |
/// | CPBF | 1000000 |
/// | CPBFOS | 110000 |
pub const DBGSW: *mut u8 = 0x165 as *mut u8;

/// SPI FIFO Fill Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TFL | 1110000 |
/// | RFC | 1000 |
/// | RFL | 111 |
/// | TFC | 10000000 |
pub const SFFR: *mut u8 = 0x166 as *mut u8;

/// SPI FIFO Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TIL | 1110000 |
/// | SRIE | 1000 |
/// | STIE | 10000000 |
/// | RIL | 111 |
pub const SFIR: *mut u8 = 0x167 as *mut u8;

/// EEPROM Control Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEFF | 1000000 |
/// | EEBRE | 1 |
/// | EECF | 10000000 |
pub const EECR2: *mut u8 = 0x168 as *mut u8;

/// Program Memory Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGMSYN | 11111 |
pub const PGMST: *mut u8 = 0x169 as *mut u8;

/// EEPROM Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EESYN | 1111 |
pub const EEST: *mut u8 = 0x16A as *mut u8;

/// RSSI LNA High Sensitivity Gain.
pub const RSHSG: *mut u8 = 0x16B as *mut u8;

/// RSSI IF Amplifier Gain.
pub const RSIFG: *mut u8 = 0x16C as *mut u8;

/// RSSI Low Band Damping Value.
pub const RSLDV: *mut u8 = 0x16D as *mut u8;

/// RSSI High Band Damping Value.
pub const RSHDV: *mut u8 = 0x16E as *mut u8;

/// RSSI Compensation Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSDC | 1 |
/// | RSIFC | 10 |
/// | RSHISC | 100 |
pub const RSCOM: *mut u8 = 0x16F as *mut u8;

/// Oscillator Calibration Counter Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCSEL | 10 |
/// | OCEN | 1 |
pub const OCCR: *mut u8 = 0x170 as *mut u8;

/// Oscillator Calibration Counter Value.
pub const OCCNT: *mut u8 = 0x171 as *mut u8;

/// Oscillator Calibration Counter Gate.
pub const OCGATE: *mut u8 = 0x172 as *mut u8;

/// LIN/UART Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LSWRES | 10000000 |
/// | LIN13 | 1000000 |
/// | LENA | 1000 |
/// | LCONF | 110000 |
/// | LCMD | 111 |
pub const LINCR: *mut u8 = 0x173 as *mut u8;

/// LIN Status and Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LRXOK | 1 |
/// | LIDOK | 100 |
/// | LIDST | 11100000 |
/// | LTXOK | 10 |
/// | LERR | 1000 |
/// | LBUSY | 10000 |
pub const LINSIR: *mut u8 = 0x174 as *mut u8;

/// LIN/UART Enable Interrupt Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LENRXOK | 1 |
/// | LENERR | 1000 |
/// | LENTXOK | 10 |
/// | LENIDOK | 100 |
pub const LINENIR: *mut u8 = 0x175 as *mut u8;

/// LIN/UART Error Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LSERR | 1000 |
/// | LOVERR | 100000 |
/// | LBERR | 1 |
/// | LCERR | 10 |
/// | LTOERR | 1000000 |
/// | LFERR | 10000 |
/// | LABORT | 10000000 |
/// | LPERR | 100 |
pub const LINERR: *mut u8 = 0x176 as *mut u8;

/// LIN/UART Bit Timing Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LBT | 111111 |
/// | LDISR | 10000000 |
pub const LINBTR: *mut u8 = 0x177 as *mut u8;

/// LIN/UART Baud Rate Register Low Byte.
pub const LINBRRL: *mut u8 = 0x178 as *mut u8;

/// LIN/UART Baud Rate Register High Byte.
pub const LINBRRH: *mut u8 = 0x179 as *mut u8;

/// LIN/UART Data Length Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LRXDL | 1111 |
/// | LTXDL | 11110000 |
pub const LINDLR: *mut u8 = 0x17A as *mut u8;

/// LIN/UART Identifier Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LID | 111111 |
/// | LP | 11000000 |
pub const LINIDR: *mut u8 = 0x17B as *mut u8;

/// LIN/UART Data Buffer Selection.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LAINC | 1000 |
/// | LINDX | 111 |
pub const LINSEL: *mut u8 = 0x17C as *mut u8;

/// LIN/UART Data Register.
pub const LINDAT: *mut u8 = 0x17D as *mut u8;

/// Trace Unit Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TRCEN | 10 |
/// | TRCCE | 1 |
pub const TRCCR: *mut u8 = 0x17E as *mut u8;

/// Trace Unit Data Register.
pub const TRCDR: *mut u8 = 0x17F as *mut u8;

/// Trace Identifier.
pub const TRCID: *mut u16 = 0x180 as *mut u16;

/// Trace Identifier low byte.
pub const TRCIDL: *mut u8 = 0x180 as *mut u8;

/// Trace Identifier high byte.
pub const TRCIDH: *mut u8 = 0x181 as *mut u8;

/// Bitfield on register `CHCR`
pub const BWM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CHDN`
pub const ADCDN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CHDN`
pub const BBDN: *mut u8 = 0x1F as *mut u8;

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
pub const CMONEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CMCR`
pub const CCS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `CMCR`
pub const CMCCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CMIMR`
pub const ECIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMOCR`
pub const FRCACT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CMOCR`
pub const FRCAO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CMOCR`
pub const SRCACT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `CMOCR`
pub const SRCAO: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CMSR`
pub const ECF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CRCCR`
pub const CRCRS: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CRCCR`
pub const REFLO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CRCCR`
pub const REFLI: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBMD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBTMS: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DBCR`
pub const DBCS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DBGSW`
pub const DBGSE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DBGSW`
pub const DBGGS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DBGSW`
pub const CPBF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DBGSW`
pub const CPBFOS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DCC1`
pub const DCHSSW: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DCC1`
pub const DCCDIV: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DCC1`
pub const DCEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DCC2`
pub const DCDRV: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `DCC2`
pub const DCVOUT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DCC2`
pub const DCCLIM: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `DCCAL1`
pub const ZCCAL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `DCCAL1`
pub const CCAL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DCCAL2`
pub const DTCAL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DCCAL2`
pub const OFCAL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `DCCAL3`
pub const SAWCAL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DFC`
pub const DFDRA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFC`
pub const DFFLC: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `DFI`
pub const DFERIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DFI`
pub const DFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DFL`
pub const DFFLS: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `DFL`
pub const DFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DFS`
pub const DFUFL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DFS`
pub const DFFLRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DFS`
pub const DFOFL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DMCDA`
pub const DMCTA: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `DMCDA`
pub const DMCLA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCDB`
pub const DMCLB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCDB`
pub const DMCTB: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `DMCRA`
pub const DMPGA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMCRA`
pub const SY1TA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMCRA`
pub const DMARA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMCRA`
pub const SASKA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMCRB`
pub const SY1TB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMCRB`
pub const SASKB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMCRB`
pub const DMARB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMCRB`
pub const DMPGB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMDN`
pub const DMDNA: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DMDN`
pub const DMDNB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `DMMA`
pub const DMHA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMMA`
pub const DMATA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMMA`
pub const DMPA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMMA`
pub const DMNEA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMHB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMNEB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMPB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMMB`
pub const DMATB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `DMPC`
pub const PCSIGA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCIALA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCIALB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCFTDB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCSIGB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCENA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCFTDA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DMPC`
pub const PCENB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DMPCA`
pub const PCSEVA: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `DMPCA`
pub const PSELA: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `DMPCA`
pub const PCLENA: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `DMPCB`
pub const PSELB: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `DMPCB`
pub const PCSEVB: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `DMPCB`
pub const PCLENB: *mut u8 = 0x38 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPAGE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR2`
pub const EEFF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EECR2`
pub const EEBRE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR2`
pub const EECF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EEPR`
pub const EEAP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EEST`
pub const EESYN: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const EOTBFE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const MANFEA1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const RRFEA1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const TELREA1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const SYTFEA1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const TMOFEA1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const CARFEA1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC1A`
pub const AMPFEA1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const SYTFEB1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const EOTAFE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const MANFEB1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const AMPFEB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const TMOFEB1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const RRFEB1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const TELREB1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC1B`
pub const CARFEB1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const TELREA2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const MANFEA2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const EOTBFE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const SYTFEA2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const RRFEA2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const CARFEA2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const AMPFEA2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC2A`
pub const TMOFEA2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const AMPFEB2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const TMOFEB2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const TELREB2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const MANFEB2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const SYTFEB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const EOTAFE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const RRFEB2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC2B`
pub const CARFEB2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const TELREA3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const TMOFEA3: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const MANFEA3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const SYTFEA3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const RRFEA3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const CARFEA3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const AMPFEA3: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC3A`
pub const EOTBFE3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const EOTAFE3: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const SYTFEB3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const CARFEB3: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const RRFEB3: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const TELREB3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const MANFEB3: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const AMPFEB3: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTC3B`
pub const TMOFEB3: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTCA`
pub const TELREA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTCA`
pub const CARFEA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTCA`
pub const RRFEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTCA`
pub const TMOFEA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTCA`
pub const AMPFEA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTCA`
pub const EOTBFE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTCA`
pub const SYTFEA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTCA`
pub const MANFEA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTCB`
pub const EOTAFE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTCB`
pub const AMPFEB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTCB`
pub const TMOFEB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTCB`
pub const TELREB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTCB`
pub const RRFEB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTCB`
pub const MANFEB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTCB`
pub const CARFEB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTCB`
pub const SYTFEB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTSA`
pub const SYTFA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTSA`
pub const TELRA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTSA`
pub const TMOFA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTSA`
pub const MANFA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTSA`
pub const CARFA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTSA`
pub const AMPFA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTSA`
pub const RRFA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTSA`
pub const EOTBF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EOTSB`
pub const TELRB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EOTSB`
pub const MANFB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EOTSB`
pub const RRFB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EOTSB`
pub const TMOFB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EOTSB`
pub const AMPFB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EOTSB`
pub const CARFB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EOTSB`
pub const SYTFB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EOTSB`
pub const EOTAF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FEAS`
pub const SDRX2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEAS`
pub const SDRX3: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FEAS`
pub const SDRX1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FEAS`
pub const SDTX3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FEAS`
pub const SDTX2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FEAS`
pub const SDTX1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FEBIA`
pub const HISEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FEBIA`
pub const IFAEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FEBT`
pub const CTN2: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `FEBT`
pub const RTN2: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `FECR`
pub const PLCKG: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FECR`
pub const ADHS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FECR`
pub const ANPS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FECR`
pub const ANDP: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FECR`
pub const S4N3: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FECR`
pub const LBNHB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FEEN1`
pub const PLCAL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FEEN1`
pub const ADCLK: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FEEN1`
pub const XTOEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEEN1`
pub const PLEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FEEN1`
pub const ADEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FEEN1`
pub const LNAEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FEEN1`
pub const PLSP1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `FEEN2`
pub const TMPM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FEEN2`
pub const XTPEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FEEN2`
pub const PAEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FEEN2`
pub const XTOEXT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `FEEN2`
pub const PLPEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `FELNA`
pub const LNABN: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FELNA`
pub const LNABH: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FEMS`
pub const PLLS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FEMS`
pub const PLLM: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FESR`
pub const XRDY: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FESR`
pub const SAT: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `FESR`
pub const PLCK: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FETN4`
pub const RTN4: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FETN4`
pub const CTN4: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FEVCO`
pub const VCOB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `FEVCO`
pub const CPCC: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FRCCAL`
pub const FRCTC: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `FSEN`
pub const SDEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FSEN`
pub const SDPU: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCR`
pub const RXTEHB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GTCR`
pub const DARB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GTCR`
pub const IWUPB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCR`
pub const GAPMA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GTCR`
pub const DARA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GTCR`
pub const IWUPA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GTCR`
pub const GAPMB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GTCR`
pub const RXTEHA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `IDC`
pub const IDFIM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `IDC`
pub const IDL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `IDC`
pub const IDCLR: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `IDC`
pub const IDCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `IDC`
pub const IDBO: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `IDS`
pub const IDFULL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `IDS`
pub const IDOK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINBTR`
pub const LBT: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LINBTR`
pub const LDISR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LINCR`
pub const LSWRES: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LINCR`
pub const LIN13: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LINCR`
pub const LENA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINCR`
pub const LCONF: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LINCR`
pub const LCMD: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LINDLR`
pub const LRXDL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LINDLR`
pub const LTXDL: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENRXOK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENERR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENTXOK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LINENIR`
pub const LENIDOK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LINERR`
pub const LSERR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINERR`
pub const LOVERR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LINERR`
pub const LBERR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINERR`
pub const LCERR: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LINERR`
pub const LTOERR: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LINERR`
pub const LFERR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LINERR`
pub const LABORT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LINERR`
pub const LPERR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LINIDR`
pub const LID: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LINIDR`
pub const LP: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `LINSEL`
pub const LAINC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINSEL`
pub const LINDX: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LRXOK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LIDOK: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LIDST: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LTXOK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LERR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LINSIR`
pub const LBUSY: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const BOOTRST: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LOW`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LOW`
pub const RSTDISBL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOW`
pub const EXTCLKEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PB7HS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PB4HS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PB7LS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ENPS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SPIIO: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const DWRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

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
pub const MSMSM6: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `MSMCR4`
pub const MSMSM7: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `MSMSTR`
pub const SSMMST: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `OCCR`
pub const OCSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `OCCR`
pub const OCEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PGMST`
pub const PGMSYN: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRRXDC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTXDC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTRC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRCRC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRVM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRUART: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT5: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT4: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR1`
pub const PRT3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRXB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRSF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRRS: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRIDS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRXA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR2`
pub const PRDF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RCTUNE4`
pub const RTUNE4: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `RCTUNE4`
pub const CTUNE4: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `RDCR`
pub const RDPU: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDCR`
pub const ADIVEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDCR`
pub const RDEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDOCR`
pub const RDSIDA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDOCR`
pub const RDSIDB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDOCR`
pub const ETRPB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDOCR`
pub const ETRPA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDOCR`
pub const TMDS: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `RDPR`
pub const RDPRF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RDPR`
pub const APRPTB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDPR`
pub const ARDPRF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRFLT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRPTA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRPTB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDPR`
pub const PRTMP: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDPR`
pub const APRPTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const EOTA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const EOTB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const WCOB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const NBITB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const SOTA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const SOTB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const WCOA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RDSIFR`
pub const NBITA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const NBITAM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const EOTAM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const SOTBM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const NBITBM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const EOTBM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const SOTAM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const WCOBM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RDSIMR`
pub const WCOAM: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSCOM`
pub const RSDC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSCOM`
pub const RSIFC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSCOM`
pub const RSHISC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `RSSC`
pub const RSPKF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `RSSC`
pub const RSUP: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `RSSC`
pub const RSWLH: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RSSC`
pub const RSHRX: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCEB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCBLB: *mut u8 = 0x60 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXLSBA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXLSBB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCBLA: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `RXBC1`
pub const RXCEA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RXBC2`
pub const RXBPB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RXBC2`
pub const RXBF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RXBC2`
pub const RXBCLR: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SFC`
pub const SFDRA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFC`
pub const SFFLC: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFFR`
pub const TFL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `SFFR`
pub const RFC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SFFR`
pub const RFL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SFFR`
pub const TFC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFI`
pub const SFFLIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFI`
pub const SFERIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFIDCA`
pub const SEMEA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIDCA`
pub const SFIDTA: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFIDCB`
pub const SFIDTB: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFIDCB`
pub const SEMEB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIR`
pub const TIL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `SFIR`
pub const SRIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SFIR`
pub const STIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIR`
pub const RIL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SFL`
pub const SFCLR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFL`
pub const SFFLS: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `SFS`
pub const SFFLRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFS`
pub const SFUFL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFS`
pub const SFOFL: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const SYTOEA1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const MANOEA1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const RROEA1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const WUPEA1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const SFIDEA1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const CAROEA1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const AMPOEA1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC1A`
pub const WCOBOE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const SYTOEB1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const WCOAOE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const RROEB1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const AMPOEB1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const SFIDEB1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const MANOEB1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const WUPEB1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC1B`
pub const CAROEB1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const AMPOEA2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const SFIDEA2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const WCOBOE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const SYTOEA2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const MANOEA2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const CAROEA2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const WUPEA2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC2A`
pub const RROEA2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const AMPOEB2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const RROEB2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const WCOAOE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const WUPEB2: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const MANOEB2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const SFIDEB2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const SYTOEB2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTC2B`
pub const CAROEB2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTCA`
pub const SYTOEA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTCA`
pub const CAROEA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTCA`
pub const AMPOEA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTCA`
pub const WCOBOE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTCA`
pub const SFIDEA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTCA`
pub const RROEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTCA`
pub const WUPEA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTCA`
pub const MANOEA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTCB`
pub const RROEB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTCB`
pub const WUPEB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTCB`
pub const AMPOEB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTCB`
pub const MANOEB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTCB`
pub const SFIDEB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTCB`
pub const WCOAOE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTCB`
pub const SYTOEB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTCB`
pub const CAROEB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTSA`
pub const MANOA: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SOTSA`
pub const WUPOA: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTSA`
pub const CAROA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTSA`
pub const SYTOA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTSA`
pub const WCOBO: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTSA`
pub const SFIDOA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTSA`
pub const RROA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTSA`
pub const AMPOA: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTSB`
pub const WCOAO: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SOTSB`
pub const RROB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SOTSB`
pub const SFIDOB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SOTSB`
pub const CAROB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SOTSB`
pub const WUPOB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SOTSB`
pub const AMPOB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SOTSB`
pub const SYTOB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SOTSB`
pub const MANOB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SELFPRGEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPSR`
pub const TXIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const RXIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SRCCAL`
pub const SRCTC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTX: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SETRPB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SETRPA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSMCR`
pub const SSMTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMFID: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMDFDT: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMPLDT: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMFBR`
pub const SSMHADT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMFCR`
pub const SSMIDSF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMFCR`
pub const SSMIDSO: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMFCR`
pub const SSMSEH: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SSMFCR`
pub const SSMCEH: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMIFR`
pub const SSMIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMIMR`
pub const SSMIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMPB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMHIS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMAD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMIDSE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMPA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMPVS: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMIFA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SSMRCR`
pub const SSMTMOE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMRR`
pub const SSMST: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SSMRR`
pub const SSMR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SSMSR`
pub const SSMERR: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SSMSR`
pub const SSMESM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SSMSTR`
pub const SSMSTA: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SSMXSR`
pub const SSMSTB: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SUPCA1`
pub const VVCAL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `SUPCA1`
pub const AVCAL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `SUPCA1`
pub const DVCAL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SUPCA2`
pub const BGCAL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `SUPCA3`
pub const AMCAL: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const VMCAL: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `SUPCA3`
pub const VMOCAL: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `SUPCA3`
pub const DMCAL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVCCLM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVCCRM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SUPCR`
pub const AVDIC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SUPFR`
pub const DCRDYF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SUPFR`
pub const AVCCRF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SUPFR`
pub const DCERF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SUPFR`
pub const AVCCLF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SYCA`
pub const SYTLA: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SYCA`
pub const SYCSA: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `SYCB`
pub const SYTLB: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `SYCB`
pub const SYCSB: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0PS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T0CR`
pub const T0IE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T0IFR`
pub const T0F: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T1CR`
pub const T1RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T1IFR`
pub const T1COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1IFR`
pub const T1OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1IMR`
pub const T1CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T1IMR`
pub const T1OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1DC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T1MR`
pub const T1PS: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `T2CR`
pub const T2CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2CR`
pub const T2CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2IFR`
pub const T2OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T2IMR`
pub const T2CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2DC: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `T2MR`
pub const T2PS: *mut u8 = 0x3C as *mut u8;

/// Bitfield on register `T3CR`
pub const T3RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CPRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T3CR`
pub const T3TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IFR`
pub const T3COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3IMR`
pub const T3CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3PS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `T3MRA`
pub const T3CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3CNC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T3MRB`
pub const T3ICS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CRM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4OTM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CTM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4RES: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4TOP: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4ENA: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4TOS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `T4CR`
pub const T4CPRM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4ICF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4IFR`
pub const T4COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4CPIM: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T4IMR`
pub const T4OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T4MRA`
pub const T4CS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `T4MRA`
pub const T4PS: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4SCE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4ICS: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4CE: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `T4MRB`
pub const T4CNC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `T5CCR`
pub const T5CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `T5CCR`
pub const T5CTC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `T5IFR`
pub const T5OFF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T5IFR`
pub const T5COF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `T5IMR`
pub const T5OIM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `T5IMR`
pub const T5CIM: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TESRA`
pub const CRCOA: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TESRA`
pub const EOTLA: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TESRB`
pub const EOTLB: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TESRB`
pub const CRCOB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TRCCR`
pub const TRCEN: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TRCCR`
pub const TRCCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMIM: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMLS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `VMCSR`
pub const VMF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `XFUSE`
pub const NVPTE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `XFUSE`
pub const E2PT: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `XFUSE`
pub const FLPT: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `XFUSE`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

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

/// `CLOCK_MANAGEMENT_MODE` value group
#[allow(non_upper_case_globals)]
pub mod clock_management_mode {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_adiv.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_ext.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto6.
   pub const VAL_0x03: u32 = 0x3;
   /// clk_xto4.
   pub const VAL_0x04: u32 = 0x4;
}

/// `CLOCK_OUTPUT_SOURCE` value group
#[allow(non_upper_case_globals)]
pub mod clock_output_source {
   /// clk_src.
   pub const VAL_0x00: u32 = 0x0;
   /// clk_frc.
   pub const VAL_0x01: u32 = 0x1;
   /// clk_adiv.
   pub const VAL_0x02: u32 = 0x2;
   /// clk_xto.
   pub const VAL_0x03: u32 = 0x3;
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

/// CPU Busy Flag Output
#[allow(non_upper_case_globals)]
pub mod cpu_busy_flag_out {
   /// no output.
   pub const VAL_0x00: u32 = 0x0;
   /// PB0.
   pub const VAL_0x01: u32 = 0x1;
   /// PB4.
   pub const VAL_0x02: u32 = 0x2;
   /// PC1.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_CLK_PRESCALE_3BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clk_prescale_3bits_small {
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

/// `CPU_CLT_PRESCALE_3BITS_SMALL` value group
#[allow(non_upper_case_globals)]
pub mod cpu_clt_prescale_3bits_small {
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

/// Sleep Mode
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_2bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// Power Down.
   pub const PDOWN: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Power Off.
   pub const POFF: u32 = 0x3;
}

/// `DCDC_CLOCK_DIVIDER` value group
#[allow(non_upper_case_globals)]
pub mod dcdc_clock_divider {
   /// 16 (375kHz).
   pub const VAL_0x00: u32 = 0x0;
   /// 12 (500kHz).
   pub const VAL_0x01: u32 = 0x1;
   /// 8 (750kHz).
   pub const VAL_0x02: u32 = 0x2;
   /// 6 (1MHz).
   pub const VAL_0x03: u32 = 0x3;
}

/// `DCDC_OUTPUT_VOLTAGE` value group
#[allow(non_upper_case_globals)]
pub mod dcdc_output_voltage {
   /// 2.2V.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.5V.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.8V.
   pub const VAL_0x02: u32 = 0x2;
   /// 3.3V.
   pub const VAL_0x03: u32 = 0x3;
}

/// Pattern Select
#[allow(non_upper_case_globals)]
pub mod dm_pattern_select {
   /// 1T alternating (0x55 0xAA).
   pub const VAL_0x00: u32 = 0x0;
   /// 2T alternating (0x33 0xCC).
   pub const VAL_0x01: u32 = 0x1;
   /// DMPATx defined.
   pub const VAL_0x02: u32 = 0x2;
   /// Manchester conformal.
   pub const VAL_0x03: u32 = 0x3;
}

/// Pattern Severity
#[allow(non_upper_case_globals)]
pub mod dm_pattern_severity {
   /// 12.5%.
   pub const VAL_0x00: u32 = 0x0;
   /// 25%.
   pub const VAL_0x01: u32 = 0x1;
   /// 37.5%.
   pub const VAL_0x02: u32 = 0x2;
   /// 50%.
   pub const VAL_0x03: u32 = 0x3;
   /// 62.5%.
   pub const VAL_0x04: u32 = 0x4;
   /// 75%.
   pub const VAL_0x05: u32 = 0x5;
   /// 87.5%.
   pub const VAL_0x06: u32 = 0x6;
   /// 100%.
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

/// `ENUM_BLB0` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb0 {
   /// LPM and SPM prohibited in Application Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Application Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Application Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Application Section.
   pub const VAL_0x03: u32 = 0x3;
}

/// `ENUM_BLB1` value group
#[allow(non_upper_case_globals)]
pub mod enum_blb1 {
   /// LPM and SPM prohibited in Boot Loader Section.
   pub const VAL_0x00: u32 = 0x0;
   /// LPM prohibited in Boot Loader Section.
   pub const VAL_0x01: u32 = 0x1;
   /// SPM prohibited in Boot Loader Section.
   pub const VAL_0x02: u32 = 0x2;
   /// No lock on SPM and LPM in Boot Loader Section.
   pub const VAL_0x03: u32 = 0x3;
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

/// RX Buffer CRC Length
#[allow(non_upper_case_globals)]
pub mod rxbuf_crc_length {
   /// CRC 4-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// CRC 8-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// CRC 16-bit.
   pub const VAL_0x02: u32 = 0x2;
}

/// SSM EndOfTelegram Location
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

/// `SSM_FILTER_DELAY` value group
#[allow(non_upper_case_globals)]
pub mod ssm_filter_delay {
   /// 380us (25kHz).
   pub const VAL_0x00: u32 = 0x0;
   /// 202us (50kHz).
   pub const VAL_0x01: u32 = 0x1;
   /// 135us (80kHz).
   pub const VAL_0x02: u32 = 0x2;
   /// 75us (165kHz).
   pub const VAL_0x03: u32 = 0x3;
   /// 58.5us (235kHz).
   pub const VAL_0x04: u32 = 0x4;
   /// 45us (360kHz).
   pub const VAL_0x05: u32 = 0x5;
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
}

/// Timer1 Clock Select
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

/// Timer2 Clock Select
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

/// Timer3 Capture Edge Select
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

/// Timer3 Capture Select
#[allow(non_upper_case_globals)]
pub mod tim3_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// trpa.
   pub const VAL_0x01: u32 = 0x1;
   /// trpb.
   pub const VAL_0x02: u32 = 0x2;
   /// ticp.
   pub const VAL_0x03: u32 = 0x3;
   /// clk_src.
   pub const VAL_0x04: u32 = 0x4;
}

/// Timer3 Clock Select
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

/// Timer4 Capture Edge Select
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

/// Timer4 Capture Select
#[allow(non_upper_case_globals)]
pub mod tim4_capture_select {
   /// clk_T2.
   pub const VAL_0x00: u32 = 0x0;
   /// trpa.
   pub const VAL_0x01: u32 = 0x1;
   /// trpb.
   pub const VAL_0x02: u32 = 0x2;
   /// ticp.
   pub const VAL_0x03: u32 = 0x3;
   /// clk_src.
   pub const VAL_0x04: u32 = 0x4;
}

/// Timer4 Clock Select
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

/// `VMON_LEVEL_SELECT` value group
#[allow(non_upper_case_globals)]
pub mod vmon_level_select {
   /// disable.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.0V.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.1V.
   pub const VAL_0x02: u32 = 0x2;
   /// 2.2V.
   pub const VAL_0x03: u32 = 0x3;
   /// 2.3V.
   pub const VAL_0x04: u32 = 0x4;
   /// 2.4V.
   pub const VAL_0x05: u32 = 0x5;
   /// 2.5V.
   pub const VAL_0x06: u32 = 0x6;
   /// 2.6V.
   pub const VAL_0x07: u32 = 0x7;
   /// 2.7V.
   pub const VAL_0x08: u32 = 0x8;
   /// 2.8V.
   pub const VAL_0x09: u32 = 0x9;
   /// 2.9V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 3.0V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 3.1V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 3.2V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 3.3V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 3.4V.
   pub const VAL_0x0F: u32 = 0xF;
}

