//! The AVR ATmega32HVBrevB microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | standard |  |  | 0°C - 0°C | 3V - 4.5V | 0 MHz |
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
/// | SUT | 11100 |
/// | EESAVE | 1000000 |
/// | WDTON | 10000000 |
/// | SPIEN | 100000 |
/// | OSCSEL | 11 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 1000 |
/// | BOOTRST | 1 |
/// | BOOTSZ | 110 |
/// | DUVRDINIT | 10000 |
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

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x26 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x28 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0A | 10 |
/// | TOV0 | 1 |
/// | ICF0 | 1000 |
/// | OCF0B | 100 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV1 | 1 |
/// | ICF1 | 1000 |
/// | OCF1B | 100 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// Oscillator Sampling Interface Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OSISEL0 | 10000 |
/// | OSIEN | 1 |
/// | OSIST | 10 |
pub const OSICSR: *mut u8 = 0x37 as *mut u8;

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
/// | INTF | 1111 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT | 1111 |
pub const EIMSK: *mut u8 = 0x3D as *mut u8;

/// General Purpose IO Register 0.
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERE | 1 |
/// | EEPM | 110000 |
/// | EEMPE | 100 |
/// | EEPE | 10 |
/// | EERIE | 1000 |
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
/// | PSRSYNC | 1 |
/// | TSM | 10000000 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICS0 | 1000 |
/// | ICES0 | 10000 |
/// | ICNC0 | 100000 |
/// | WGM00 | 1 |
/// | TCW0 | 10000000 |
/// | ICEN0 | 1000000 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS01 | 10 |
/// | CS02 | 100 |
/// | CS00 | 1 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer Counter 0  Bytes low byte.
pub const TCNT0L: *mut u8 = 0x46 as *mut u8;

/// Timer Counter 0  Bytes.
pub const TCNT0: *mut u16 = 0x46 as *mut u16;

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
/// | MSTR | 10000 |
/// | SPE | 1000000 |
/// | SPIE | 10000000 |
/// | CPOL | 1000 |
/// | CPHA | 100 |
/// | SPR | 11 |
/// | DORD | 100000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
/// | SPI2X | 1 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

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
/// | BODRF | 100 |
/// | OCDRF | 10000 |
/// | PORF | 1 |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PUD | 10000 |
/// | CKOE | 100000 |
/// | IVCE | 1 |
/// | IVSEL | 10 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | SPMIE | 10000000 |
/// | RWWSB | 1000000 |
/// | SIGRD | 100000 |
/// | SPMEN | 1 |
/// | RWWSRE | 10000 |
/// | LBSET | 1000 |
/// | PGERS | 10 |
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
/// | Z | 10 |
/// | N | 100 |
/// | I | 10000000 |
/// | T | 1000000 |
/// | V | 1000 |
/// | C | 1 |
/// | S | 10000 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDIE | 1000000 |
/// | WDIF | 10000000 |
/// | WDE | 1000 |
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

/// Power Reduction Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTWI | 1000000 |
/// | PRVRM | 100000 |
/// | PRTIM0 | 10 |
/// | PRTIM1 | 100 |
/// | PRVADC | 1 |
/// | PRSPI | 1000 |
pub const PRR0: *mut u8 = 0x64 as *mut u8;

/// Fast Oscillator Calibration Value.
pub const FOSCCAL: *mut u8 = 0x66 as *mut u8;

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
/// | ISC0 | 11 |
/// | ISC2 | 110000 |
/// | ISC1 | 1100 |
/// | ISC3 | 11000000 |
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
/// | ICIE1 | 1000 |
/// | TOIE1 | 1 |
/// | OCIE1B | 100 |
/// | OCIE1A | 10 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// VADC Data Register  Bytes low byte.
pub const VADCL: *mut u8 = 0x78 as *mut u8;

/// VADC Data Register  Bytes.
pub const VADC: *mut u16 = 0x78 as *mut u16;

/// VADC Data Register  Bytes high byte.
pub const VADCH: *mut u8 = 0x79 as *mut u8;

/// The VADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VADEN | 1000 |
/// | VADCCIF | 10 |
/// | VADCCIE | 1 |
/// | VADSC | 100 |
pub const VADCSR: *mut u8 = 0x7A as *mut u8;

/// The VADC multiplexer Selection Register.
pub const VADMUX: *mut u8 = 0x7C as *mut u8;

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
/// | ICEN1 | 1000000 |
/// | WGM10 | 1 |
/// | ICES1 | 10000 |
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

/// Timer Counter 1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer Counter 1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer Counter 1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Output Compare Register 1A.
pub const OCR1A: *mut u8 = 0x88 as *mut u8;

/// Output Compare Register B.
pub const OCR1B: *mut u8 = 0x89 as *mut u8;

/// TWI Bit Rate register.
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

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWGCE | 1 |
/// | TWA | 11111110 |
pub const TWAR: *mut u8 = 0xBA as *mut u8;

/// TWI Data register.
pub const TWDR: *mut u8 = 0xBB as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWSTA | 100000 |
/// | TWEA | 1000000 |
/// | TWEN | 100 |
/// | TWIE | 1 |
/// | TWWC | 1000 |
/// | TWINT | 10000000 |
/// | TWSTO | 10000 |
pub const TWCR: *mut u8 = 0xBC as *mut u8;

/// TWI (Slave) Address Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWAM | 11111110 |
pub const TWAMR: *mut u8 = 0xBD as *mut u8;

/// TWI Bus Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWBCIE | 1000000 |
/// | TWBCIP | 1 |
/// | TWBDT | 110 |
/// | TWBCIF | 10000000 |
pub const TWBCSR: *mut u8 = 0xBE as *mut u8;

/// Regulator Operating Condition Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ROCS | 10000000 |
/// | ROCD | 10000 |
/// | ROCWIE | 1 |
/// | ROCWIF | 10 |
pub const ROCR: *mut u8 = 0xC8 as *mut u8;

/// Bandgap Calibration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGCC | 111111 |
pub const BGCCR: *mut u8 = 0xD0 as *mut u8;

/// Bandgap Calibration of Resistor Ladder.
pub const BGCRR: *mut u8 = 0xD1 as *mut u8;

/// Bandgap Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BGSCDIE | 1 |
/// | BGSCDE | 10000 |
/// | BGD | 100000 |
/// | BGSCDIF | 10 |
pub const BGCSR: *mut u8 = 0xD2 as *mut u8;

/// Charger Detect Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CHGDIE | 1 |
/// | CHGDIF | 10 |
/// | CHGDISC | 1100 |
/// | BATTPVL | 10000 |
pub const CHGDCSR: *mut u8 = 0xD4 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC0: *mut u8 = 0xE0 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC1: *mut u8 = 0xE1 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC2: *mut u8 = 0xE2 as *mut u8;

/// ADC Accumulate Current.
pub const CADAC3: *mut u8 = 0xE3 as *mut u8;

/// CC-ADC Instantaneous Current low byte.
pub const CADICL: *mut u8 = 0xE4 as *mut u8;

/// CC-ADC Instantaneous Current.
pub const CADIC: *mut u16 = 0xE4 as *mut u16;

/// CC-ADC Instantaneous Current high byte.
pub const CADICH: *mut u8 = 0xE5 as *mut u8;

/// CC-ADC Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADAS | 11000 |
/// | CADUB | 100000 |
/// | CADEN | 10000000 |
/// | CADSE | 1 |
/// | CADSI | 110 |
/// | CADPOL | 1000000 |
pub const CADCSRA: *mut u8 = 0xE6 as *mut u8;

/// CC-ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADRCIE | 100000 |
/// | CADICIE | 10000 |
/// | CADACIF | 100 |
/// | CADRCIF | 10 |
/// | CADICIF | 1 |
/// | CADACIE | 1000000 |
pub const CADCSRB: *mut u8 = 0xE7 as *mut u8;

/// CC-ADC Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CADVSE | 1 |
pub const CADCSRC: *mut u8 = 0xE8 as *mut u8;

/// CC-ADC Regular Charge Current.
pub const CADRCC: *mut u8 = 0xE9 as *mut u8;

/// CC-ADC Regular Discharge Current.
pub const CADRDC: *mut u8 = 0xEA as *mut u8;

/// FET Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CPS | 100 |
/// | DFE | 10 |
/// | DUVRD | 1000 |
/// | CFE | 1 |
pub const FCSR: *mut u8 = 0xF0 as *mut u8;

/// Cell Balancing Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CBE | 1111 |
pub const CBCR: *mut u8 = 0xF1 as *mut u8;

/// Battery Protection Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COCIE | 100 |
/// | DHCIE | 10 |
/// | CHCIE | 1 |
/// | DOCIE | 1000 |
/// | SCIE | 10000 |
pub const BPIMSK: *mut u8 = 0xF2 as *mut u8;

/// Battery Protection Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DHCIF | 10 |
/// | SCIF | 10000 |
/// | DOCIF | 1000 |
/// | CHCIF | 1 |
/// | COCIF | 100 |
pub const BPIFR: *mut u8 = 0xF3 as *mut u8;

/// Battery Protection Short-Circuit Detection Level Register.
pub const BPSCD: *mut u8 = 0xF5 as *mut u8;

/// Battery Protection Discharge-Over-current Detection Level Register.
pub const BPDOCD: *mut u8 = 0xF6 as *mut u8;

/// Battery Protection Charge-Over-current Detection Level Register.
pub const BPCOCD: *mut u8 = 0xF7 as *mut u8;

/// Battery Protection Discharge-High-current Detection Level Register.
pub const BPDHCD: *mut u8 = 0xF8 as *mut u8;

/// Battery Protection Charge-High-current Detection Level Register.
pub const BPCHCD: *mut u8 = 0xF9 as *mut u8;

/// Battery Protection Short-current Timing Register.
pub const BPSCTR: *mut u8 = 0xFA as *mut u8;

/// Battery Protection Over-current Timing Register.
pub const BPOCTR: *mut u8 = 0xFB as *mut u8;

/// Battery Protection Short-current Timing Register.
pub const BPHCTR: *mut u8 = 0xFC as *mut u8;

/// Battery Protection Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EPID | 100000 |
/// | COCD | 100 |
/// | SCD | 10000 |
/// | CHCD | 1 |
/// | DHCD | 10 |
/// | DOCD | 1000 |
pub const BPCR: *mut u8 = 0xFD as *mut u8;

/// Battery Protection Parameter Lock Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BPPL | 1 |
/// | BPPLE | 10 |
pub const BPPLR: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `BGCCR`
pub const BGCC: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `BGCSR`
pub const BGSCDIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BGCSR`
pub const BGSCDE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BGCSR`
pub const BGD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `BGCSR`
pub const BGSCDIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPCR`
pub const EPID: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `BPCR`
pub const COCD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BPCR`
pub const SCD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BPCR`
pub const CHCD: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPCR`
pub const DHCD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPCR`
pub const DOCD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `BPIFR`
pub const DHCIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPIFR`
pub const SCIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BPIFR`
pub const DOCIF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `BPIFR`
pub const CHCIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPIFR`
pub const COCIF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BPIMSK`
pub const COCIE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `BPIMSK`
pub const DHCIE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BPIMSK`
pub const CHCIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPIMSK`
pub const DOCIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `BPIMSK`
pub const SCIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `BPPLR`
pub const BPPL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BPPLR`
pub const BPPLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADAS: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADUB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADSE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADSI: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `CADCSRA`
pub const CADPOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADRCIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADICIE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADACIF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADRCIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADICIF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CADCSRB`
pub const CADACIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `CADCSRC`
pub const CADVSE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CBCR`
pub const CBE: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CHGDCSR`
pub const CHGDIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CHGDCSR`
pub const CHGDIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `CHGDCSR`
pub const CHGDISC: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `CHGDCSR`
pub const BATTPVL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `DIDR0`
pub const PA1DID: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const PA0DID: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `FCSR`
pub const CPS: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `FCSR`
pub const DFE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `FCSR`
pub const DUVRD: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `FCSR`
pub const CFE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const DUVRDINIT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT: *mut u8 = 0x1C as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOW`
pub const OSCSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const CKOE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BODRF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const OCDRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `OSICSR`
pub const OSISEL0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `OSICSR`
pub const OSIEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `OSICSR`
pub const OSIST: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTWI: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRVRM: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRTIM1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRVADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR0`
pub const PRSPI: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ROCR`
pub const ROCS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ROCR`
pub const ROCD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ROCR`
pub const ROCWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ROCR`
pub const ROCWIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SIGRD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const LBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICES0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICNC0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const TCW0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const ICEN0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICEN1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const WGM10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICES1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICS1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const ICNC1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const TCW1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const ICF0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const ICIE0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TWAMR`
pub const TWAM: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWAR`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBCIP: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBDT: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `TWBCSR`
pub const TWBCIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADCCIF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADCCIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `VADCSR`
pub const VADSC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

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

/// `COMM_TW_BUS_TIMEOUT` value group
#[allow(non_upper_case_globals)]
pub mod comm_tw_bus_timeout {
   /// 250ms.
   pub const VAL_0x00: u32 = 0x0;
   /// 500ms.
   pub const VAL_0x01: u32 = 0x1;
   /// 1000ms.
   pub const VAL_0x02: u32 = 0x2;
   /// 2000ms.
   pub const VAL_0x03: u32 = 0x3;
}

/// `CPU_SLEEP_MODE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC.
   pub const ADC: u32 = 0x1;
   /// Reserved.
   pub const VAL_0x02: u32 = 0x2;
   /// Power Save.
   pub const PSAVE: u32 = 0x3;
   /// Power Off.
   pub const POFF: u32 = 0x4;
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
   /// Boot Flash size=256 words Boot address=$3F00.
   pub const _256W_3F00: u32 = 0x3;
   /// Boot Flash size=512 words Boot address=$3E00.
   pub const _512W_3E00: u32 = 0x2;
   /// Boot Flash size=1024 words Boot address=$3C00.
   pub const _1024W_3C00: u32 = 0x1;
   /// Boot Flash size=2048 words Boot address=$3800.
   pub const _2048W_3800: u32 = 0x0;
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

/// `ENUM_OSCSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_oscsel {
   /// Default.
   pub const DEFAULT: u32 = 0x1;
}

/// `ENUM_SUT` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut {
   /// Start-up time 14 CK + 4 ms.
   pub const _14CK_4MS: u32 = 0x0;
   /// Start-up time 14 CK + 8 ms.
   pub const _14CK_8MS: u32 = 0x1;
   /// Start-up time 14 CK + 16 ms.
   pub const _14CK_16MS: u32 = 0x2;
   /// Start-up time 14 CK + 32 ms.
   pub const _14CK_32MS: u32 = 0x3;
   /// Start-up time 14 CK + 64 ms.
   pub const _14CK_64MS: u32 = 0x4;
   /// Start-up time 14 CK + 128 ms.
   pub const _14CK_128MS: u32 = 0x5;
   /// Start-up time 14 CK + 256 ms.
   pub const _14CK_256MS: u32 = 0x6;
   /// Start-up time 14 CK + 512 ms.
   pub const _14CK_512MS: u32 = 0x7;
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

