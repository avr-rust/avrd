//! The AVR ATmega128A microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega128A-AU | TQFPQFN64 | TQFP64 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
//! | ATmega128A-MU | TQFPQFN64 | QFN64 | -40°C - 85°C | 2.7V - 5.5V | 16 MHz |
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
/// | BODEN | 1000000 |
/// | SUT_CKSEL | 111111 |
/// | BODLEVEL | 10000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BOOTRST | 1 |
/// | CKOPT | 10000 |
/// | EESAVE | 1000 |
/// | BOOTSZ | 110 |
/// | SPIEN | 100000 |
/// | JTAGEN | 1000000 |
/// | OCDEN | 10000000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | M103C | 10 |
/// | WDTON | 1 |
pub const EXTENDED: *mut u8 = 0x2 as *mut u8;

/// Input Pins, Port F.
pub const PINF: *mut u8 = 0x20 as *mut u8;

/// Input Pins, Port E.
pub const PINE: *mut u8 = 0x21 as *mut u8;

/// Data Direction Register, Port E.
pub const DDRE: *mut u8 = 0x22 as *mut u8;

/// Data Register, Port E.
pub const PORTE: *mut u8 = 0x23 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x24 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x24 as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x25 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADIF | 10000 |
/// | ADPS | 111 |
/// | ADSC | 1000000 |
/// | ADEN | 10000000 |
/// | ADFR | 100000 |
/// | ADIE | 1000 |
pub const ADCSRA: *mut u8 = 0x26 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
/// | MUX | 11111 |
pub const ADMUX: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACD | 10000000 |
/// | ACI | 10000 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
/// | ACIC | 100 |
/// | ACO | 100000 |
/// | ACIE | 1000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// USART Baud Rate Register Low Byte.
pub const UBRR0L: *mut u8 = 0x29 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXCIE0 | 1000000 |
/// | UCSZ02 | 100 |
/// | TXB80 | 1 |
/// | RXB80 | 10 |
/// | RXCIE0 | 10000000 |
/// | TXEN0 | 1000 |
/// | RXEN0 | 10000 |
/// | UDRIE0 | 100000 |
pub const UCSR0B: *mut u8 = 0x2A as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FE0 | 10000 |
/// | MPCM0 | 1 |
/// | UPE0 | 100 |
/// | UDRE0 | 100000 |
/// | TXC0 | 1000000 |
/// | U2X0 | 10 |
/// | RXC0 | 10000000 |
/// | DOR0 | 1000 |
pub const UCSR0A: *mut u8 = 0x2B as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0x2C as *mut u8;

/// SPI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DORD | 100000 |
/// | SPE | 1000000 |
/// | SPR | 11 |
/// | SPIE | 10000000 |
/// | MSTR | 10000 |
/// | CPOL | 1000 |
/// | CPHA | 100 |
pub const SPCR: *mut u8 = 0x2D as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
pub const SPSR: *mut u8 = 0x2E as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x2F as *mut u8;

/// Port D Input Pins.
pub const PIND: *mut u8 = 0x30 as *mut u8;

/// Port D Data Direction Register.
pub const DDRD: *mut u8 = 0x31 as *mut u8;

/// Port D Data Register.
pub const PORTD: *mut u8 = 0x32 as *mut u8;

/// Port C Input Pins.
pub const PINC: *mut u8 = 0x33 as *mut u8;

/// Port C Data Direction Register.
pub const DDRC: *mut u8 = 0x34 as *mut u8;

/// Port C Data Register.
pub const PORTC: *mut u8 = 0x35 as *mut u8;

/// Port B Input Pins.
pub const PINB: *mut u8 = 0x36 as *mut u8;

/// Port B Data Direction Register.
pub const DDRB: *mut u8 = 0x37 as *mut u8;

/// Port B Data Register.
pub const PORTB: *mut u8 = 0x38 as *mut u8;

/// Port A Input Pins.
pub const PINA: *mut u8 = 0x39 as *mut u8;

/// Port A Data Direction Register.
pub const DDRA: *mut u8 = 0x3A as *mut u8;

/// Port A Data Register.
pub const PORTA: *mut u8 = 0x3B as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEMWE | 100 |
/// | EEWE | 10 |
/// | EERE | 1 |
/// | EERIE | 1000 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access  Bytes low byte.
pub const EEARL: *mut u8 = 0x3E as *mut u8;

/// EEPROM Read/Write Access  Bytes.
pub const EEAR: *mut u16 = 0x3E as *mut u16;

/// EEPROM Read/Write Access  Bytes high byte.
pub const EEARH: *mut u8 = 0x3F as *mut u8;

/// Special Function IO Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR321 | 1 |
pub const SFIOR: *mut u8 = 0x40 as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDP | 111 |
/// | WDE | 1000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// On-Chip Debug Related Register in I/O Memory.
pub const OCDR: *mut u8 = 0x42 as *mut u8;

/// Output Compare Register.
pub const OCR2: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter Register.
pub const TCNT2: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM20 | 1000000 |
/// | WGM21 | 1000 |
/// | COM2 | 110000 |
/// | CS2 | 111 |
/// | FOC2 | 10000000 |
pub const TCCR2: *mut u8 = 0x45 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x46 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes high byte.
pub const ICR1H: *mut u8 = 0x47 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1BL: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1B: *mut u16 = 0x48 as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1BH: *mut u8 = 0x49 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1AL: *mut u8 = 0x4A as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1A: *mut u16 = 0x4A as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1AH: *mut u8 = 0x4B as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x4C as *mut u16;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x4C as *mut u8;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x4D as *mut u8;

/// Timer/Counter1 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS1 | 111 |
/// | ICNC1 | 10000000 |
/// | ICES1 | 1000000 |
pub const TCCR1B: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter1 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM1B | 110000 |
/// | COM1C | 1100 |
/// | COM1A | 11000000 |
pub const TCCR1A: *mut u8 = 0x4F as *mut u8;

/// Asynchronus Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TCR0UB | 1 |
/// | AS0 | 1000 |
/// | OCR0UB | 10 |
/// | TCN0UB | 100 |
pub const ASSR: *mut u8 = 0x50 as *mut u8;

/// Output Compare Register.
pub const OCR0: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter Register.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM00 | 1000000 |
/// | COM0 | 110000 |
/// | FOC0 | 10000000 |
/// | WGM01 | 1000 |
/// | CS0 | 111 |
pub const TCCR0: *mut u8 = 0x53 as *mut u8;

/// MCU Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTRF | 10000 |
/// | JTD | 10000000 |
pub const MCUCSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRW10 | 1000000 |
/// | SM2 | 100 |
/// | IVSEL | 10 |
/// | SE | 100000 |
/// | IVCE | 1 |
/// | SM | 11000 |
/// | SRE | 10000000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF2 | 10000000 |
/// | TOV2 | 1000000 |
pub const TIFR: *mut u8 = 0x56 as *mut u8;

/// `TIMSK` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE2 | 1000000 |
/// | OCIE2 | 10000000 |
pub const TIMSK: *mut u8 = 0x57 as *mut u8;

/// External Interrupt Flag Register.
pub const EIFR: *mut u8 = 0x58 as *mut u8;

/// External Interrupt Mask Register.
pub const EIMSK: *mut u8 = 0x59 as *mut u8;

/// External Interrupt Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC5 | 1100 |
/// | ISC6 | 110000 |
/// | ISC7 | 11000000 |
/// | ISC4 | 11 |
pub const EICRB: *mut u8 = 0x5A as *mut u8;

/// RAM Page Z Select Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RAMPZ0 | 1 |
pub const RAMPZ: *mut u8 = 0x5B as *mut u8;

/// XTAL Divide Control Register.
pub const XDIV: *mut u8 = 0x5C as *mut u8;

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
/// | H | 100000 |
/// | N | 100 |
/// | I | 10000000 |
/// | T | 1000000 |
/// | Z | 10 |
/// | V | 1000 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Data Direction Register, Port F.
pub const DDRF: *mut u8 = 0x61 as *mut u8;

/// Data Register, Port F.
pub const PORTF: *mut u8 = 0x62 as *mut u8;

/// Input Pins, Port G.
pub const PING: *mut u8 = 0x63 as *mut u8;

/// Data Direction Register, Port G.
pub const DDRG: *mut u8 = 0x64 as *mut u8;

/// Data Register, Port G.
pub const PORTG: *mut u8 = 0x65 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RWWSB | 1000000 |
/// | SPMIE | 10000000 |
/// | SPMEN | 1 |
/// | PGERS | 10 |
/// | BLBSET | 1000 |
/// | RWWSRE | 10000 |
/// | PGWRT | 100 |
pub const SPMCSR: *mut u8 = 0x68 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC1 | 1100 |
/// | ISC2 | 110000 |
/// | ISC0 | 11 |
/// | ISC3 | 11000000 |
pub const EICRA: *mut u8 = 0x6A as *mut u8;

/// External Memory Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | XMM | 111 |
/// | XMBK | 10000000 |
pub const XMCRB: *mut u8 = 0x6C as *mut u8;

/// External Memory Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SRW11 | 10 |
/// | SRW0 | 1100 |
/// | SRL | 1110000 |
pub const XMCRA: *mut u8 = 0x6D as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x6F as *mut u8;

/// TWI Bit Rate register.
pub const TWBR: *mut u8 = 0x70 as *mut u8;

/// TWI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWPS | 11 |
/// | TWS | 11111000 |
pub const TWSR: *mut u8 = 0x71 as *mut u8;

/// TWI (Slave) Address register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWA | 11111110 |
/// | TWGCE | 1 |
pub const TWAR: *mut u8 = 0x72 as *mut u8;

/// TWI Data register.
pub const TWDR: *mut u8 = 0x73 as *mut u8;

/// TWI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TWSTA | 100000 |
/// | TWIE | 1 |
/// | TWWC | 1000 |
/// | TWSTO | 10000 |
/// | TWEA | 1000000 |
/// | TWEN | 100 |
/// | TWINT | 10000000 |
pub const TWCR: *mut u8 = 0x74 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes.
pub const OCR1C: *mut u16 = 0x78 as *mut u16;

/// Timer/Counter1 Output Compare Register  Bytes low byte.
pub const OCR1CL: *mut u8 = 0x78 as *mut u8;

/// Timer/Counter1 Output Compare Register  Bytes high byte.
pub const OCR1CH: *mut u8 = 0x79 as *mut u8;

/// Timer/Counter1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
/// | FOC1C | 100000 |
pub const TCCR1C: *mut u8 = 0x7A as *mut u8;

/// Extended Timer/Counter Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV3 | 100 |
/// | OCF3A | 10000 |
/// | ICF3 | 100000 |
/// | OCF3B | 1000 |
/// | OCF3C | 10 |
pub const ETIFR: *mut u8 = 0x7C as *mut u8;

/// Extended Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE3C | 10 |
/// | TOIE3 | 100 |
/// | OCIE3B | 1000 |
/// | TICIE3 | 100000 |
/// | OCIE3A | 10000 |
pub const ETIMSK: *mut u8 = 0x7D as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes low byte.
pub const ICR3L: *mut u8 = 0x80 as *mut u8;

/// Timer/Counter3 Input Capture Register  Bytes.
pub const ICR3: *mut u16 = 0x80 as *mut u16;

/// Timer/Counter3 Input Capture Register  Bytes high byte.
pub const ICR3H: *mut u8 = 0x81 as *mut u8;

/// Timer/Counter3 Output compare Register C  Bytes.
pub const OCR3C: *mut u16 = 0x82 as *mut u16;

/// Timer/Counter3 Output compare Register C  Bytes low byte.
pub const OCR3CL: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter3 Output compare Register C  Bytes high byte.
pub const OCR3CH: *mut u8 = 0x83 as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes.
pub const OCR3B: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter3 Output Compare Register B  Bytes low byte.
pub const OCR3BL: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter3 Output Compare Register B  Bytes high byte.
pub const OCR3BH: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes.
pub const OCR3A: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter3 Output Compare Register A  Bytes low byte.
pub const OCR3AL: *mut u8 = 0x86 as *mut u8;

/// Timer/Counter3 Output Compare Register A  Bytes high byte.
pub const OCR3AH: *mut u8 = 0x87 as *mut u8;

/// Timer/Counter3  Bytes low byte.
pub const TCNT3L: *mut u8 = 0x88 as *mut u8;

/// Timer/Counter3  Bytes.
pub const TCNT3: *mut u16 = 0x88 as *mut u16;

/// Timer/Counter3  Bytes high byte.
pub const TCNT3H: *mut u8 = 0x89 as *mut u8;

/// Timer/Counter3 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICES3 | 1000000 |
/// | ICNC3 | 10000000 |
/// | CS3 | 111 |
pub const TCCR3B: *mut u8 = 0x8A as *mut u8;

/// Timer/Counter3 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM3A | 11000000 |
/// | COM3C | 1100 |
/// | COM3B | 110000 |
pub const TCCR3A: *mut u8 = 0x8B as *mut u8;

/// Timer/Counter3 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC3A | 10000000 |
/// | FOC3B | 1000000 |
/// | FOC3C | 100000 |
pub const TCCR3C: *mut u8 = 0x8C as *mut u8;

/// USART Baud Rate Register Hight Byte.
pub const UBRR0H: *mut u8 = 0x90 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UCSZ0 | 110 |
/// | UCPOL0 | 1 |
/// | UMSEL0 | 1000000 |
/// | UPM0 | 110000 |
/// | USBS0 | 1000 |
pub const UCSR0C: *mut u8 = 0x95 as *mut u8;

/// USART Baud Rate Register Hight Byte.
pub const UBRR1H: *mut u8 = 0x98 as *mut u8;

/// USART Baud Rate Register Low Byte.
pub const UBRR1L: *mut u8 = 0x99 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TXB81 | 1 |
/// | RXB81 | 10 |
/// | RXCIE1 | 10000000 |
/// | UCSZ12 | 100 |
/// | UDRIE1 | 100000 |
/// | RXEN1 | 10000 |
/// | TXEN1 | 1000 |
/// | TXCIE1 | 1000000 |
pub const UCSR1B: *mut u8 = 0x9A as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXC1 | 10000000 |
/// | TXC1 | 1000000 |
/// | UPE1 | 100 |
/// | UDRE1 | 100000 |
/// | DOR1 | 1000 |
/// | FE1 | 10000 |
/// | MPCM1 | 1 |
/// | U2X1 | 10 |
pub const UCSR1A: *mut u8 = 0x9B as *mut u8;

/// USART I/O Data Register.
pub const UDR1: *mut u8 = 0x9C as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USBS1 | 1000 |
/// | UMSEL1 | 1000000 |
/// | UPM1 | 110000 |
/// | UCPOL1 | 1 |
/// | UCSZ1 | 110 |
pub const UCSR1C: *mut u8 = 0x9D as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADFR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR0UB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR0UB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN0UB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC1: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC3: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC5: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC6: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC7: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `EICRB`
pub const ISC4: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ETIFR`
pub const TOV3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ETIFR`
pub const OCF3A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ETIFR`
pub const ICF3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ETIFR`
pub const OCF3B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ETIFR`
pub const OCF3C: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ETIMSK`
pub const OCIE3C: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ETIMSK`
pub const TOIE3: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ETIMSK`
pub const OCIE3B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ETIMSK`
pub const TICIE3: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ETIMSK`
pub const OCIE3A: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const M103C: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const WDTON: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const CKOPT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const BODEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const BODLEVEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SRW10: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM2: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SRE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUCSR`
pub const JTD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `RAMPZ`
pub const RAMPZ0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFIOR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SFIOR`
pub const PSR321: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0`
pub const WGM00: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0`
pub const COM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0`
pub const FOC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0`
pub const WGM01: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0`
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
pub const ICNC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1B`
pub const ICES1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR2`
pub const WGM20: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR2`
pub const WGM21: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2`
pub const COM2: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR2`
pub const FOC2: *mut u8 = 0x80 as *mut u8;

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
pub const FOC3B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR3C`
pub const FOC3C: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE2: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE2: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWAR`
pub const TWA: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `TWAR`
pub const TWGCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTA: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWIE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWWC: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWSTO: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEA: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWEN: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TWCR`
pub const TWINT: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWPS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TWSR`
pub const TWS: *mut u8 = 0xF8 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const FE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const MPCM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UPE0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UDRE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const TXC0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const U2X0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const RXC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const DOR0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXB80: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXB80: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXEN0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXEN0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UPM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const USBS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const RXC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const TXC1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UPE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const UDRE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const DOR1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const FE1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const MPCM1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1A`
pub const U2X1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXB81: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXB81: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXCIE1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UCSZ12: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const UDRIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const RXEN1: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXEN1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1B`
pub const TXCIE1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const USBS1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UMSEL1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UPM1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCPOL1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR1C`
pub const UCSZ1: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRW11: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRW0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `XMCRA`
pub const SRL: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `XMCRB`
pub const XMM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `XMCRB`
pub const XMBK: *mut u8 = 0x80 as *mut u8;

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

/// `COMM_USART_MODE` value group
#[allow(non_upper_case_globals)]
pub mod comm_usart_mode {
   /// Asynchronous Operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Synchronous Operation.
   pub const VAL_0x01: u32 = 0x1;
}

/// `CPU_SECTOR_LIMITS` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sector_limits {
   /// LS = N/A, US = 0x1100 - 0xFFFF.
   pub const VAL_0x00: u32 = 0x0;
   /// LS = 0x1100 - 0x1FFF, US = 0x2000 - 0xFFFF.
   pub const VAL_0x01: u32 = 0x1;
   /// LS = 0x1100 - 0x3FFF, US = 0x4000 - 0xFFFF.
   pub const VAL_0x02: u32 = 0x2;
   /// LS = 0x1100 - 0x5FFF, US = 0x6000 - 0xFFFF.
   pub const VAL_0x03: u32 = 0x3;
   /// LS = 0x1100 - 0x7FFF, US = 0x8000 - 0xFFFF.
   pub const VAL_0x04: u32 = 0x4;
   /// LS = 0x1100 - 0x9FFF, US = 0xA000 - 0xFFFF.
   pub const VAL_0x05: u32 = 0x5;
   /// LS = 0x1100 - 0xBFFF, US = 0xC000 - 0xFFFF.
   pub const VAL_0x06: u32 = 0x6;
   /// LS = 0x1100 - 0xDFFF, US = 0xE000 - 0xFFFF.
   pub const VAL_0x07: u32 = 0x7;
}

/// `CPU_SLEEP_MODE_2BITS1` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_2bits1 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x2;
   /// Power Down.
   pub const PDOWN: u32 = 0x4;
   /// Power Save.
   pub const PSAVE: u32 = 0x6;
   /// Standby.
   pub const STDBY: u32 = 0x5;
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
   /// Brown-out detection level at VCC=4.0 V.
   pub const _4V0: u32 = 0x0;
   /// Brown-out detection level at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
}

/// `ENUM_BOOTSZ` value group
#[allow(non_upper_case_globals)]
pub mod enum_bootsz {
   /// Boot Flash size=512 words start address=$FE00.
   pub const _512W_FE00: u32 = 0x3;
   /// Boot Flash size=1024 words start address=$FC00.
   pub const _1024W_FC00: u32 = 0x2;
   /// Boot Flashsize=2048 words start address=$F800.
   pub const _2048W_F800: u32 = 0x1;
   /// Boot Flash size=4096 words start address=$F000.
   pub const _4096W_F000: u32 = 0x0;
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
   /// Ext. Clock; Start-up time: 6 CK + 4 ms.
   pub const EXTCLK_6CK_4MS: u32 = 0x10;
   /// Ext. Clock; Start-up time: 6 CK + 64 ms.
   pub const EXTCLK_6CK_64MS: u32 = 0x20;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_1MHZ_6CK_0MS: u32 = 0x1;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_1MHZ_6CK_4MS: u32 = 0x11;
   /// Int. RC Osc. 1 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_1MHZ_6CK_64MS: u32 = 0x21;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_2MHZ_6CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_2MHZ_6CK_4MS: u32 = 0x12;
   /// Int. RC Osc. 2 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_2MHZ_6CK_64MS: u32 = 0x22;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_4MHZ_6CK_0MS: u32 = 0x3;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_4MHZ_6CK_4MS: u32 = 0x13;
   /// Int. RC Osc. 4 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_4MHZ_6CK_64MS: u32 = 0x23;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 0 ms.
   pub const INTRCOSC_8MHZ_6CK_0MS: u32 = 0x4;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 4 ms.
   pub const INTRCOSC_8MHZ_6CK_4MS: u32 = 0x14;
   /// Int. RC Osc. 8 MHz; Start-up time: 6 CK + 64 ms.
   pub const INTRCOSC_8MHZ_6CK_64MS: u32 = 0x24;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_0MS: u32 = 0x5;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_4MS: u32 = 0x15;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_XX_0MHZ9_18CK_64MS: u32 = 0x25;
   /// Ext. RC Osc.         -  0.9 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_XX_0MHZ9_6CK_4MS: u32 = 0x35;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_0MS: u32 = 0x6;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_4MS: u32 = 0x16;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_18CK_64MS: u32 = 0x26;
   /// Ext. RC Osc. 0.9 MHz -  3.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_0MHZ9_3MHZ_6CK_4MS: u32 = 0x36;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_0MS: u32 = 0x7;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_4MS: u32 = 0x17;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_18CK_64MS: u32 = 0x27;
   /// Ext. RC Osc. 3.0 MHz -  8.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_3MHZ_8MHZ_6CK_4MS: u32 = 0x37;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 0 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_0MS: u32 = 0x8;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_4MS: u32 = 0x18;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 18 CK + 64 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_18CK_64MS: u32 = 0x28;
   /// Ext. RC Osc. 8.0 MHz - 12.0 MHz; Start-up time: 6 CK + 4 ms.
   pub const EXTRCOSC_8MHZ_12MHZ_6CK_4MS: u32 = 0x38;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTAL_1KCK_4MS: u32 = 0x9;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTAL_1KCK_64MS: u32 = 0x19;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 64 ms.
   pub const EXTLOFXTAL_32KCK_64MS: u32 = 0x29;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTLOFXTALRES_258CK_4MS: u32 = 0xA;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTLOFXTALRES_258CK_64MS: u32 = 0x1A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTALRES_1KCK_0MS: u32 = 0x2A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTLOFXTALRES_1KCK_4MS: u32 = 0x3A;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTLOFXTALRES_1KCK_64MS: u32 = 0xB;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTLOFXTALRES_16KCK_0MS: u32 = 0x1B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTLOFXTALRES_16KCK_4MS: u32 = 0x2B;
   /// Ext. Crystal/Resonator Low Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTLOFXTALRES_16KCK_64MS: u32 = 0x3B;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTMEDFXTALRES_258CK_4MS: u32 = 0xC;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTMEDFXTALRES_258CK_64MS: u32 = 0x1C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTMEDFXTALRES_1KCK_0MS: u32 = 0x2C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTMEDFXTALRES_1KCK_4MS: u32 = 0x3C;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTMEDFXTALRES_1KCK_64MS: u32 = 0xD;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTMEDFXTALRES_16KCK_0MS: u32 = 0x1D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTMEDFXTALRES_16KCK_4MS: u32 = 0x2D;
   /// Ext. Crystal/Resonator Medium Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTMEDFXTALRES_16KCK_64MS: u32 = 0x3D;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 4 ms.
   pub const EXTHIFXTALRES_258CK_4MS: u32 = 0xE;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 258 CK + 64 ms.
   pub const EXTHIFXTALRES_258CK_64MS: u32 = 0x1E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 0 ms.
   pub const EXTHIFXTALRES_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 4 ms.
   pub const EXTHIFXTALRES_1KCK_4MS: u32 = 0x3E;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 1K CK + 64 ms.
   pub const EXTHIFXTALRES_1KCK_64MS: u32 = 0xF;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 0 ms.
   pub const EXTHIFXTALRES_16KCK_0MS: u32 = 0x1F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 4 ms.
   pub const EXTHIFXTALRES_16KCK_4MS: u32 = 0x2F;
   /// Ext. Crystal/Resonator High Freq.; Start-up time: 16K CK + 64 ms.
   pub const EXTHIFXTALRES_16KCK_64MS: u32 = 0x3F;
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
   /// 1.0 MHz.
   pub const _1_0_MHz_: u32 = 0x0;
   /// 2.0 MHz.
   pub const _2_0_MHz_: u32 = 0x1;
   /// 4.0 MHz.
   pub const _4_0_MHz_: u32 = 0x2;
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x3;
}

/// `WAVEFORM_GEN_MODE` value group
#[allow(non_upper_case_globals)]
pub mod waveform_gen_mode {
   /// Normal.
   pub const VAL_0x00: u32 = 0x0;
   /// PWM, Phase Correct.
   pub const VAL_0x02: u32 = 0x2;
   /// CTC.
   pub const VAL_0x01: u32 = 0x1;
   /// Fast PWM.
   pub const VAL_0x03: u32 = 0x3;
}

/// `WDOG_TIMER_PRESCALE_3BITS` value group
#[allow(non_upper_case_globals)]
pub mod wdog_timer_prescale_3bits {
   /// Oscillator Cycles 16K.
   pub const VAL_0x00: u32 = 0x0;
   /// Oscillator Cycles 32K.
   pub const VAL_0x01: u32 = 0x1;
   /// Oscillator Cycles 64K.
   pub const VAL_0x02: u32 = 0x2;
   /// Oscillator Cycles 128K.
   pub const VAL_0x03: u32 = 0x3;
   /// Oscillator Cycles 256K.
   pub const VAL_0x04: u32 = 0x4;
   /// Oscillator Cycles 512K.
   pub const VAL_0x05: u32 = 0x5;
   /// Oscillator Cycles 1024K.
   pub const VAL_0x06: u32 = 0x6;
   /// Oscillator Cycles 2048K.
   pub const VAL_0x07: u32 = 0x7;
}

