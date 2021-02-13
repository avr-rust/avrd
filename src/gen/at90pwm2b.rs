//! The AVR AT90PWM2B microcontroller
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
/// | SUT_CKSEL | 111111 |
/// | CKDIV8 | 10000000 |
/// | CKOUT | 1000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 10000000 |
/// | EESAVE | 1000 |
/// | WDTON | 10000 |
/// | DWEN | 1000000 |
/// | SPIEN | 100000 |
/// | BODLEVEL | 111 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSC2RB | 10000000 |
/// | PSC1RB | 1000000 |
/// | PSC0RB | 100000 |
/// | PSCRV | 10000 |
/// | BOOTRST | 1 |
/// | BOOTSZ | 110 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x23 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x24 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x25 as *mut u8;

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
/// | OCF0B | 100 |
/// | OCF0A | 10 |
/// | TOV0 | 1 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV1 | 1 |
/// | OCF1B | 100 |
/// | ICF1 | 100000 |
/// | OCF1A | 10 |
pub const TIFR1: *mut u8 = 0x36 as *mut u8;

/// General Purpose IO Register 1.
pub const GPIOR1: *mut u8 = 0x39 as *mut u8;

/// General Purpose IO Register 2.
pub const GPIOR2: *mut u8 = 0x3A as *mut u8;

/// General Purpose IO Register 3.
pub const GPIOR3: *mut u8 = 0x3B as *mut u8;

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
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | GPIOR02 | 100 |
/// | GPIOR04 | 10000 |
/// | GPIOR00 | 1 |
/// | GPIOR06 | 1000000 |
/// | GPIOR01 | 10 |
/// | GPIOR03 | 1000 |
/// | GPIOR05 | 100000 |
/// | GPIOR07 | 10000000 |
pub const GPIOR0: *mut u8 = 0x3E as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEWE | 10 |
/// | EEMWE | 100 |
/// | EERE | 1 |
/// | EERIE | 1000 |
/// | EEPM | 110000 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Read/Write Access Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Read/Write Access Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Read/Write Access Bytes high byte.
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
/// | COM0B | 110000 |
/// | WGM0 | 11 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS0 | 111 |
/// | WGM02 | 1000 |
/// | FOC0B | 1000000 |
/// | FOC0A | 10000000 |
pub const TCCR0B: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x48 as *mut u8;

/// PLL Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLOCK | 1 |
/// | PLLF | 100 |
/// | PLLE | 10 |
pub const PLLCSR: *mut u8 = 0x49 as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIE | 10000000 |
/// | CPOL | 1000 |
/// | MSTR | 10000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | DORD | 100000 |
/// | CPHA | 100 |
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

/// Analog Comparator Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC1O | 10 |
/// | AC1IF | 100000 |
/// | AC2O | 100 |
/// | AC0IF | 10000 |
/// | AC0O | 1 |
/// | AC2IF | 1000000 |
/// | ACCKDIV | 10000000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

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
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | BORF | 100 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPIPS | 10000000 |
/// | PUD | 10000 |
/// | IVSEL | 10 |
/// | IVCE | 1 |
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
/// | SPMEN | 1 |
/// | SPMIE | 10000000 |
/// | RWWSRE | 10000 |
/// | BLBSET | 1000 |
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
/// | N | 100 |
/// | C | 1 |
/// | I | 10000000 |
/// | T | 1000000 |
/// | H | 100000 |
/// | Z | 10 |
/// | V | 1000 |
/// | S | 10000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDIF | 10000000 |
/// | WDP | 100111 |
/// | WDIE | 1000000 |
/// | WDCE | 10000 |
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

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 1000 |
/// | PRUSART0 | 10 |
/// | PRSPI | 100 |
/// | PRPSC1 | 1000000 |
/// | PRADC | 1 |
/// | PRPSC2 | 10000000 |
/// | PRTIM1 | 10000 |
/// | PRPSC0 | 100000 |
pub const PRR: *mut u8 = 0x64 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC2 | 110000 |
/// | ISC0 | 11 |
/// | ISC1 | 1100 |
/// | ISC3 | 11000000 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0B | 100 |
/// | TOIE0 | 1 |
/// | OCIE0A | 10 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1A | 10 |
/// | ICIE1 | 100000 |
/// | TOIE1 | 1 |
/// | OCIE1B | 100 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// `AMP0CSR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMP0IS | 1000000 |
/// | AMP0G | 110000 |
/// | AMP0TS | 11 |
/// | AMP0EN | 10000000 |
pub const AMP0CSR: *mut u8 = 0x76 as *mut u8;

/// `AMP1CSR` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AMP1IS | 1000000 |
/// | AMP1G | 110000 |
/// | AMP1TS | 11 |
/// | AMP1EN | 10000000 |
pub const AMP1CSR: *mut u8 = 0x77 as *mut u8;

/// ADC Data Register Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000000 |
/// | ADATE | 100000 |
/// | ADIF | 10000 |
/// | ADSC | 1000000 |
/// | ADIE | 1000 |
/// | ADPS | 111 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADTS | 1111 |
/// | ADHSM | 10000000 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | MUX | 1111 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC1D | 10 |
/// | ADC7D | 10000000 |
/// | ADC5D | 100000 |
/// | ADC3D | 1000 |
/// | ADC0D | 1 |
/// | ADC4D | 10000 |
/// | ADC6D | 1000000 |
/// | ADC2D | 100 |
pub const DIDR0: *mut u8 = 0x7E as *mut u8;

/// Digital Input Disable Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC10D | 100 |
/// | ADC8D | 1 |
/// | ADC9D | 10 |
/// | ACMP0D | 100000 |
/// | AMP0PD | 10000 |
/// | AMP0ND | 1000 |
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
/// | CS1 | 111 |
/// | ICNC1 | 10000000 |
/// | ICES1 | 1000000 |
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

/// Timer/Counter1 Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1 Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

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

/// Timer/Counter1 Output Compare Register Bytes low byte.
pub const OCR1BL: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter1 Output Compare Register Bytes.
pub const OCR1B: *mut u16 = 0x8A as *mut u16;

/// Timer/Counter1 Output Compare Register Bytes high byte.
pub const OCR1BH: *mut u8 = 0x8B as *mut u8;

/// PSC0 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEV0A | 1000 |
/// | PEOP0 | 1 |
/// | POAC0B | 10000000 |
/// | POAC0A | 1000000 |
/// | PSEI0 | 100000 |
/// | PEV0B | 10000 |
/// | PRN0 | 110 |
pub const PIFR0: *mut u8 = 0xA0 as *mut u8;

/// PSC0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEVE0A | 1000 |
/// | PEVE0B | 10000 |
/// | PSEIE0 | 100000 |
/// | PEOPE0 | 1 |
pub const PIM0: *mut u8 = 0xA1 as *mut u8;

/// PSC2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEOP2 | 1 |
/// | PEV2B | 10000 |
/// | PRN2 | 110 |
/// | PEV2A | 1000 |
/// | POAC2A | 1000000 |
/// | PSEI2 | 100000 |
/// | POAC2B | 10000000 |
pub const PIFR2: *mut u8 = 0xA4 as *mut u8;

/// PSC2 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PEVE2A | 1000 |
/// | PEOPE2 | 1 |
/// | PSEIE2 | 100000 |
/// | PEVE2B | 10000 |
pub const PIM2: *mut u8 = 0xA5 as *mut u8;

/// DAC Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DALA | 100 |
/// | DAATE | 10000000 |
/// | DATS | 1110000 |
/// | DAEN | 1 |
/// | DAOE | 10 |
pub const DACON: *mut u8 = 0xAA as *mut u8;

/// DAC Data Register.
pub const DAC: *mut u16 = 0xAB as *mut u16;

/// DAC Data Register low byte.
pub const DACL: *mut u8 = 0xAB as *mut u8;

/// DAC Data Register high byte.
pub const DACH: *mut u8 = 0xAC as *mut u8;

/// Analog Comparator 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC0M | 111 |
/// | AC0IE | 1000000 |
/// | AC0IS | 110000 |
/// | AC0EN | 10000000 |
pub const AC0CON: *mut u8 = 0xAD as *mut u8;

/// Analog Comparator 1 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC1EN | 10000000 |
/// | AC1IE | 1000000 |
/// | AC1IS | 110000 |
/// | AC1M | 111 |
/// | AC1ICE | 1000 |
pub const AC1CON: *mut u8 = 0xAE as *mut u8;

/// Analog Comparator 2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AC2M | 111 |
/// | AC2IS | 110000 |
/// | AC2EN | 10000000 |
/// | AC2IE | 1000000 |
pub const AC2CON: *mut u8 = 0xAF as *mut u8;

/// USART Control and Status register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FE | 10000 |
/// | MPCM | 1 |
/// | UPE | 100 |
/// | TXC | 1000000 |
/// | RXC | 10000000 |
/// | U2X | 10 |
/// | DOR | 1000 |
/// | UDRE | 100000 |
pub const UCSRA: *mut u8 = 0xC0 as *mut u8;

/// USART Control an Status register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXB8 | 1 |
/// | TXCIE | 1000000 |
/// | RXEN | 10000 |
/// | UDRIE | 100000 |
/// | RXCIE | 10000000 |
/// | UCSZ2 | 100 |
/// | TXEN | 1000 |
/// | RXB8 | 10 |
pub const UCSRB: *mut u8 = 0xC1 as *mut u8;

/// USART Control an Status register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS | 1000 |
/// | UMSEL0 | 1000000 |
/// | UCSZ | 110 |
/// | UCPOL | 1 |
/// | UPM | 110000 |
pub const UCSRC: *mut u8 = 0xC2 as *mut u8;

/// USART Baud Rate Register.
pub const UBRR: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register low byte.
pub const UBRRL: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register high byte.
pub const UBRRH: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register.
pub const UDR: *mut u8 = 0xC6 as *mut u8;

/// EUSART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | URxS | 1111 |
/// | UTxS | 11110000 |
pub const EUCSRA: *mut u8 = 0xC8 as *mut u8;

/// EUSART Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EUSART | 10000 |
/// | EMCH | 10 |
/// | EUSBS | 1000 |
/// | BODR | 1 |
pub const EUCSRB: *mut u8 = 0xC9 as *mut u8;

/// EUSART Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | STP | 11 |
/// | F1617 | 100 |
/// | FEM | 1000 |
pub const EUCSRC: *mut u8 = 0xCA as *mut u8;

/// Manchester Receiver Baud Rate Register.
pub const MUBRR: *mut u16 = 0xCC as *mut u16;

/// Manchester Receiver Baud Rate Register low byte.
pub const MUBRRL: *mut u8 = 0xCC as *mut u8;

/// Manchester Receiver Baud Rate Register high byte.
pub const MUBRRH: *mut u8 = 0xCD as *mut u8;

/// EUSART I/O Data Register.
pub const EUDR: *mut u8 = 0xCE as *mut u8;

/// PSC0 Synchro and Output Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSYNC0 | 110000 |
/// | POEN0A | 1 |
/// | POEN0B | 100 |
pub const PSOC0: *mut u8 = 0xD0 as *mut u8;

/// Output Compare 0 SA Register.
pub const OCR0SA: *mut u16 = 0xD2 as *mut u16;

/// Output Compare 0 SA Register low byte.
pub const OCR0SAL: *mut u8 = 0xD2 as *mut u8;

/// Output Compare 0 SA Register high byte.
pub const OCR0SAH: *mut u8 = 0xD3 as *mut u8;

/// Output Compare 0 RA Register.
pub const OCR0RA: *mut u16 = 0xD4 as *mut u16;

/// Output Compare 0 RA Register low byte.
pub const OCR0RAL: *mut u8 = 0xD4 as *mut u8;

/// Output Compare 0 RA Register high byte.
pub const OCR0RAH: *mut u8 = 0xD5 as *mut u8;

/// Output Compare 0 SB Register low byte.
pub const OCR0SBL: *mut u8 = 0xD6 as *mut u8;

/// Output Compare 0 SB Register.
pub const OCR0SB: *mut u16 = 0xD6 as *mut u16;

/// Output Compare 0 SB Register high byte.
pub const OCR0SBH: *mut u8 = 0xD7 as *mut u8;

/// Output Compare 0 RB Register low byte.
pub const OCR0RBL: *mut u8 = 0xD8 as *mut u8;

/// Output Compare 0 RB Register.
pub const OCR0RB: *mut u16 = 0xD8 as *mut u16;

/// Output Compare 0 RB Register high byte.
pub const OCR0RBH: *mut u8 = 0xD9 as *mut u8;

/// PSC 0 Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCLKSEL0 | 10 |
/// | PMODE0 | 11000 |
/// | PFIFTY0 | 10000000 |
/// | PALOCK0 | 1000000 |
/// | POP0 | 100 |
/// | PLOCK0 | 100000 |
pub const PCNF0: *mut u8 = 0xDA as *mut u8;

/// PSC 0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRUN0 | 1 |
/// | PAOC0A | 1000 |
/// | PARUN0 | 100 |
/// | PCCYC0 | 10 |
/// | PPRE0 | 11000000 |
/// | PAOC0B | 10000 |
/// | PBFM0 | 100000 |
pub const PCTL0: *mut u8 = 0xDB as *mut u8;

/// PSC 0 Input A Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRFM0A | 1111 |
/// | PFLTE0A | 10000 |
/// | PISEL0A | 1000000 |
/// | PELEV0A | 100000 |
/// | PCAE0A | 10000000 |
pub const PFRC0A: *mut u8 = 0xDC as *mut u8;

/// PSC 0 Input B Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PFLTE0B | 10000 |
/// | PISEL0B | 1000000 |
/// | PELEV0B | 100000 |
/// | PRFM0B | 1111 |
/// | PCAE0B | 10000000 |
pub const PFRC0B: *mut u8 = 0xDD as *mut u8;

/// PSC 0 Input Capture Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCST0 | 1000000000000000 |
pub const PICR0: *mut u16 = 0xDE as *mut u16;

/// PSC 0 Input Capture Register  low byte.
pub const PICR0L: *mut u8 = 0xDE as *mut u8;

/// PSC 0 Input Capture Register  high byte.
pub const PICR0H: *mut u8 = 0xDF as *mut u8;

/// PSC2 Synchro and Output Configuration.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POEN2A | 1 |
/// | POEN2D | 1000 |
/// | POEN2C | 10 |
/// | POEN2B | 100 |
/// | POS2 | 11000000 |
/// | PSYNC2_ | 110000 |
pub const PSOC2: *mut u8 = 0xF0 as *mut u8;

/// PSC 2 Output Matrix.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | POMV2A | 1111 |
/// | POMV2B | 11110000 |
pub const POM2: *mut u8 = 0xF1 as *mut u8;

/// Output Compare 2 SA Register.
pub const OCR2SA: *mut u16 = 0xF2 as *mut u16;

/// Output Compare 2 SA Register low byte.
pub const OCR2SAL: *mut u8 = 0xF2 as *mut u8;

/// Output Compare 2 SA Register high byte.
pub const OCR2SAH: *mut u8 = 0xF3 as *mut u8;

/// Output Compare 2 RA Register low byte.
pub const OCR2RAL: *mut u8 = 0xF4 as *mut u8;

/// Output Compare 2 RA Register.
pub const OCR2RA: *mut u16 = 0xF4 as *mut u16;

/// Output Compare 2 RA Register high byte.
pub const OCR2RAH: *mut u8 = 0xF5 as *mut u8;

/// Output Compare 2 SB Register.
pub const OCR2SB: *mut u16 = 0xF6 as *mut u16;

/// Output Compare 2 SB Register low byte.
pub const OCR2SBL: *mut u8 = 0xF6 as *mut u8;

/// Output Compare 2 SB Register high byte.
pub const OCR2SBH: *mut u8 = 0xF7 as *mut u8;

/// Output Compare 2 RB Register low byte.
pub const OCR2RBL: *mut u8 = 0xF8 as *mut u8;

/// Output Compare 2 RB Register.
pub const OCR2RB: *mut u16 = 0xF8 as *mut u16;

/// Output Compare 2 RB Register high byte.
pub const OCR2RBH: *mut u8 = 0xF9 as *mut u8;

/// PSC 2 Configuration Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PLOCK2 | 100000 |
/// | POP2 | 100 |
/// | PALOCK2 | 1000000 |
/// | PCLKSEL2 | 10 |
/// | PMODE2 | 11000 |
/// | PFIFTY2 | 10000000 |
/// | POME2 | 1 |
pub const PCNF2: *mut u8 = 0xFA as *mut u8;

/// PSC 2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PPRE2 | 11000000 |
/// | PBFM2 | 100000 |
/// | PARUN2 | 100 |
/// | PAOC2A | 1000 |
/// | PRUN2 | 1 |
/// | PCCYC2 | 10 |
/// | PAOC2B | 10000 |
pub const PCTL2: *mut u8 = 0xFB as *mut u8;

/// PSC 2 Input B Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCAE2A | 10000000 |
/// | PISEL2A | 1000000 |
/// | PFLTE2A | 10000 |
/// | PRFM2A | 1111 |
/// | PELEV2A | 100000 |
pub const PFRC2A: *mut u8 = 0xFC as *mut u8;

/// PSC 2 Input B Control.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCAE2B | 10000000 |
/// | PELEV2B | 100000 |
/// | PISEL2B | 1000000 |
/// | PFLTE2B | 10000 |
/// | PRFM2B | 1111 |
pub const PFRC2B: *mut u8 = 0xFD as *mut u8;

/// PSC 2 Input Capture Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCST2 | 1000000000000000 |
pub const PICR2: *mut u16 = 0xFE as *mut u16;

/// PSC 2 Input Capture Register  low byte.
pub const PICR2L: *mut u8 = 0xFE as *mut u8;

/// PSC 2 Input Capture Register  high byte.
pub const PICR2H: *mut u8 = 0xFF as *mut u8;

/// Bitfield on register `AC0CON`
pub const AC0M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC0CON`
pub const AC0IE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AC0CON`
pub const AC0IS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AC0CON`
pub const AC0EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1IE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1IS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC1CON`
pub const AC1ICE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2M: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2IS: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AC2CON`
pub const AC2IE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC1O: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC1IF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC2O: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC0IF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC0O: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ACSR`
pub const AC2IF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACCKDIV: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADHSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0IS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0G: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0TS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `AMP0CSR`
pub const AMP0EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `AMP1CSR`
pub const AMP1IS: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `AMP1CSR`
pub const AMP1G: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `AMP1CSR`
pub const AMP1TS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `AMP1CSR`
pub const AMP1EN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DACON`
pub const DALA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DACON`
pub const DAATE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DACON`
pub const DATS: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `DACON`
pub const DAEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DACON`
pub const DAOE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC10D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC8D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ADC9D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const ACMP0D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AMP0PD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AMP0ND: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EUCSRA`
pub const URxS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `EUCSRA`
pub const UTxS: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `EUCSRB`
pub const EUSART: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EUCSRB`
pub const EMCH: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EUCSRB`
pub const EUSBS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EUCSRB`
pub const BODR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EUCSRC`
pub const STP: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EUCSRC`
pub const F1617: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EUCSRC`
pub const FEM: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSC2RB: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSC1RB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSC0RB: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const PSCRV: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR04: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR06: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR03: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR05: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GPIOR0`
pub const GPIOR07: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSRSYNC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const BODLEVEL: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SPIPS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PCLKSEL0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PMODE0: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PFIFTY0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PALOCK0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCNF0`
pub const POP0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCNF0`
pub const PLOCK0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PLOCK2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCNF2`
pub const POP2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PALOCK2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PCLKSEL2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PMODE2: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `PCNF2`
pub const PFIFTY2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCNF2`
pub const POME2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PRUN0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PAOC0A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PARUN0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PCCYC0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PPRE0: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PAOC0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PCTL0`
pub const PBFM0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PPRE2: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PBFM2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PARUN2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PAOC2A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PRUN2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PCCYC2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PCTL2`
pub const PAOC2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PRFM0A: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PFLTE0A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PISEL0A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PELEV0A: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC0A`
pub const PCAE0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PFLTE0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PISEL0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PELEV0B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PRFM0B: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC0B`
pub const PCAE0B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PCAE2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PISEL2A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PFLTE2A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PRFM2A: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PFRC2A`
pub const PELEV2A: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PCAE2B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PELEV2B: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PISEL2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PFLTE2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PFRC2B`
pub const PRFM2B: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PICR0`
pub const PCST0: *mut u16 = 0x8000 as *mut u16;

/// Bitfield on register `PICR2`
pub const PCST2: *mut u16 = 0x8000 as *mut u16;

/// Bitfield on register `PIFR0`
pub const PEV0A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PEOP0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIFR0`
pub const POAC0B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PIFR0`
pub const POAC0A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PSEI0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PEV0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIFR0`
pub const PRN0: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PEOP2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PEV2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PRN2: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PEV2A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIFR2`
pub const POAC2A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PIFR2`
pub const PSEI2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PIFR2`
pub const POAC2B: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEVE0A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEVE0B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PIM0`
pub const PSEIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PIM0`
pub const PEOPE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEVE2A: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEOPE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PIM2`
pub const PSEIE2: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PIM2`
pub const PEVE2B: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLOCK: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PLLCSR`
pub const PLLE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `POM2`
pub const POMV2A: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `POM2`
pub const POMV2B: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSART0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRSPI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRPSC1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRPSC2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `PRR`
pub const PRPSC0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `PSOC0`
pub const PSYNC0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `PSOC0`
pub const POEN0A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PSOC0`
pub const POEN0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2A: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2C: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POEN2B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PSOC2`
pub const POS2: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `PSOC2`
pub const PSYNC2_: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const WGM02: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR1A`
pub const COM1B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const CS1: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRA`
pub const FE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRA`
pub const MPCM: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UPE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRA`
pub const TXC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRA`
pub const RXC: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRA`
pub const U2X: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRA`
pub const DOR: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRA`
pub const UDRE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXB8: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXCIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UDRIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXCIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSRB`
pub const UCSZ2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSRB`
pub const TXEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRB`
pub const RXB8: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSRC`
pub const USBS: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UMSEL0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UCPOL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSRC`
pub const UPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

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

/// `COMM_STOP_BIT_SEL` value group
#[allow(non_upper_case_globals)]
pub mod comm_stop_bit_sel {
   /// 1-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// 2-bit.
   pub const VAL_0x01: u32 = 0x1;
}

/// `COMM_TRANS_CHAR_SIZE` value group
#[allow(non_upper_case_globals)]
pub mod comm_trans_char_size {
   /// 5-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// 6-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// 7-bit.
   pub const VAL_0x02: u32 = 0x2;
   /// 8-bit.
   pub const VAL_0x03: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// 9-bit.
   pub const VAL_0x07: u32 = 0x7;
   /// 13-bit.
   pub const VAL_0x08: u32 = 0x8;
   /// 14-bit.
   pub const VAL_0x09: u32 = 0x9;
   /// 15-bit.
   pub const VAL_0x0A: u32 = 0xA;
   /// 16-bit.
   pub const VAL_0x0B: u32 = 0xB;
   /// Reserved.
   pub const VAL_0x0C: u32 = 0xC;
   /// Reserved.
   pub const VAL_0x0D: u32 = 0xD;
   /// Reserved.
   pub const VAL_0x0E: u32 = 0xE;
   /// 17-bit.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `COMM_TRANS_CHAR_SIZE2` value group
#[allow(non_upper_case_globals)]
pub mod comm_trans_char_size2 {
   /// 5-bit.
   pub const VAL_0x00: u32 = 0x0;
   /// 6-bit.
   pub const VAL_0x01: u32 = 0x1;
   /// 7-bit.
   pub const VAL_0x02: u32 = 0x2;
   /// 8-bit.
   pub const VAL_0x03: u32 = 0x3;
   /// Reserved.
   pub const VAL_0x04: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// 9-bit.
   pub const VAL_0x07: u32 = 0x7;
   /// 13-bit.
   pub const VAL_0x08: u32 = 0x8;
   /// 14-bit.
   pub const VAL_0x09: u32 = 0x9;
   /// 15-bit.
   pub const VAL_0x0A: u32 = 0xA;
   /// 16-bit.
   pub const VAL_0x0B: u32 = 0xB;
   /// Reserved.
   pub const VAL_0x0C: u32 = 0xC;
   /// Reserved.
   pub const VAL_0x0D: u32 = 0xD;
   /// 16 or 17.
   pub const VAL_0x0E: u32 = 0xE;
   /// 17-bit.
   pub const VAL_0x0F: u32 = 0xF;
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
   /// Brown-out detection at VCC=4.5 V.
   pub const _4V5: u32 = 0x6;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x5;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x4;
   /// Brown-out detection at VCC=4.4 V.
   pub const _4V4: u32 = 0x3;
   /// Brown-out detection at VCC=4.2 V.
   pub const _4V2: u32 = 0x2;
   /// Brown-out detection at VCC=2.8 V.
   pub const _2V8: u32 = 0x1;
   /// Brown-out detection at VCC=2.6 V.
   pub const _2V6: u32 = 0x0;
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
   /// PLL clock 16 MHz; Rc. Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const PLLCLK_16MHZ_1KCK_14CK_0MS: u32 = 0x3;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4.1 ms.
   pub const PLLCLK_16MHZ_1KCK_14CK_4MS1: u32 = 0x13;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 1K CK/14 CK + 65 ms.
   pub const PLLCLK_16MHZ_1KCK_14CK_65MS: u32 = 0x23;
   /// PLL clock 16 MHz; Start-up time PWRDWN/RESET: 16K CK/14 CK + 0 ms.
   pub const PLLCLK_16MHZ_16KCK_14CK_0MS: u32 = 0x33;
   /// PLL clock /4; PLL input: Ext. Clock; Start-up time PWRDWN/RESET: 6K CK/14 CK + 0 ms.
   pub const PLLCLK_PLLIN_EXTCLK_6KCK_14CK_0MS: u32 = 0x1;
   /// PLL clock /4; PLL input: Ext. Clock; Start-up time PWRDWN/RESET: 6K CK/14 CK + 4 ms.
   pub const PLLCLK_PLLIN_EXTCLK_6KCK_14CK_4MS: u32 = 0x11;
   /// PLL clock /4; PLL input: Ext. Clock; Start-up time PWRDWN/RESET: 6K CK/14 CK + 64 ms.
   pub const PLLCLK_PLLIN_EXTCLK_6KCK_14CK_64MS: u32 = 0x21;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_1KCK_14CK_0MS: u32 = 0x5;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_1KCK_14CK_4MS: u32 = 0x15;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_16KCK_14CK_4MS: u32 = 0x25;
   /// PLL clock /4; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 64 ms.
   pub const PLLCLK_PLLIN_EXTXOSC_16KCK_14CK_64MS: u32 = 0x35;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 0 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_1KCK_14CK_0MS: u32 = 0x4;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 1K CK/14 CK + 4 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_1KCK_14CK_4MS: u32 = 0x14;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 4 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_16KCK_14CK_4MS: u32 = 0x24;
   /// Ext. Crystal Osc.; PLL input: Ext. Crystal Osc.; Start-up time PWRDWN/RESET: 16K CK/14 CK + 64 ms.
   pub const EXTXOSC_PLLIN_EXTXOSC_16KCK_14CK_64MS: u32 = 0x34;
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

