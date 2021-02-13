//! The AVR ATmega329A microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATmega329A-AU | TQFP64 | TQFP64 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega329A-MU | QFN64 | QFN64 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega329A-AN | TQFP64 | TQFP64  | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATmega329A-MN | QFN64 | QFN64 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CKOUT | 1000000 |
/// | SUT_CKSEL | 111111 |
/// | CKDIV8 | 10000000 |
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
/// | SPIEN | 100000 |
/// | BOOTSZ | 110 |
/// | WDTON | 10000 |
/// | EESAVE | 1000 |
/// | JTAGEN | 1000000 |
/// | OCDEN | 10000000 |
/// | BOOTRST | 1 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// `EXTENDED` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 1 |
/// | BODLEVEL | 110 |
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

/// Input Pins, Port E.
pub const PINE: *mut u8 = 0x2C as *mut u8;

/// Data Direction Register, Port E.
pub const DDRE: *mut u8 = 0x2D as *mut u8;

/// Data Register, Port E.
pub const PORTE: *mut u8 = 0x2E as *mut u8;

/// Input Pins, Port F.
pub const PINF: *mut u8 = 0x2F as *mut u8;

/// Data Direction Register, Port F.
pub const DDRF: *mut u8 = 0x30 as *mut u8;

/// Data Register, Port F.
pub const PORTF: *mut u8 = 0x31 as *mut u8;

/// Port G Input Pins.
pub const PING: *mut u8 = 0x32 as *mut u8;

/// Port G Data Direction Register.
pub const DDRG: *mut u8 = 0x33 as *mut u8;

/// Port G Data Register.
pub const PORTG: *mut u8 = 0x34 as *mut u8;

/// Timer/Counter0 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | OCF0A | 10 |
pub const TIFR0: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter1 Interrupt Flag register.
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

/// Timer/Counter2 Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV2 | 1 |
/// | OCF2A | 10 |
pub const TIFR2: *mut u8 = 0x37 as *mut u8;

/// External Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
/// | PCIF | 11110000 |
pub const EIFR: *mut u8 = 0x3C as *mut u8;

/// External Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE | 11110000 |
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
/// | EEWE | 10 |
/// | EERE | 1 |
/// | EERIE | 1000 |
/// | EEMWE | 100 |
pub const EECR: *mut u8 = 0x3F as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x40 as *mut u8;

/// EEPROM Read/Write Access  Bytes.
pub const EEAR: *mut u16 = 0x41 as *mut u16;

/// EEPROM Read/Write Access  Bytes low byte.
pub const EEARL: *mut u8 = 0x41 as *mut u8;

/// EEPROM Read/Write Access  Bytes high byte.
pub const EEARH: *mut u8 = 0x42 as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR2 | 10 |
pub const GTCCR: *mut u8 = 0x43 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0A | 110000 |
/// | WGM01 | 1000 |
/// | WGM00 | 1000000 |
/// | FOC0A | 10000000 |
/// | CS0 | 111 |
pub const TCCR0A: *mut u8 = 0x44 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x46 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x47 as *mut u8;

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
/// | DORD | 100000 |
/// | SPR | 11 |
/// | SPE | 1000000 |
/// | MSTR | 10000 |
/// | CPHA | 100 |
/// | CPOL | 1000 |
/// | SPIE | 10000000 |
pub const SPCR: *mut u8 = 0x4C as *mut u8;

/// SPI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SPI2X | 1 |
/// | SPIF | 10000000 |
/// | WCOL | 1000000 |
pub const SPSR: *mut u8 = 0x4D as *mut u8;

/// SPI Data Register.
pub const SPDR: *mut u8 = 0x4E as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACIE | 1000 |
/// | ACIC | 100 |
/// | ACI | 10000 |
/// | ACBG | 1000000 |
/// | ACIS | 11 |
/// | ACD | 10000000 |
/// | ACO | 100000 |
pub const ACSR: *mut u8 = 0x50 as *mut u8;

/// On-Chip Debug Related Register in I/O Memory.
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
/// | BORF | 100 |
/// | WDRF | 1000 |
/// | PORF | 1 |
/// | JTRF | 10000 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | JTD | 10000000 |
/// | IVSEL | 10 |
/// | IVCE | 1 |
/// | PUD | 10000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Store Program Memory Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | SPMIE | 10000000 |
/// | PGERS | 10 |
/// | RWWSB | 1000000 |
/// | SPMEN | 1 |
/// | BLBSET | 1000 |
/// | RWWSRE | 10000 |
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
/// | N | 100 |
/// | S | 10000 |
/// | T | 1000000 |
/// | I | 10000000 |
/// | V | 1000 |
/// | Z | 10 |
/// | C | 1 |
/// | H | 100000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDCE | 10000 |
/// | WDE | 1000 |
/// | WDP | 111 |
pub const WDTCR: *mut u8 = 0x60 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
/// | CLKPCE | 10000000 |
pub const CLKPR: *mut u8 = 0x61 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRADC | 1 |
/// | PRTIM1 | 1000 |
/// | PRSPI | 100 |
/// | PRUSART0 | 10 |
/// | PRLCD | 10000 |
pub const PRR: *mut u8 = 0x64 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x66 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC01 | 10 |
/// | ISC00 | 1 |
pub const EICRA: *mut u8 = 0x69 as *mut u8;

/// Pin Change Mask Register 0.
pub const PCMSK0: *mut u8 = 0x6B as *mut u8;

/// Pin Change Mask Register 1.
pub const PCMSK1: *mut u8 = 0x6C as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 10 |
/// | TOIE0 | 1 |
pub const TIMSK0: *mut u8 = 0x6E as *mut u8;

/// Timer/Counter1 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE1B | 100 |
/// | OCIE1A | 10 |
/// | TOIE1 | 1 |
/// | ICIE1 | 100000 |
pub const TIMSK1: *mut u8 = 0x6F as *mut u8;

/// Timer/Counter2 Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE2 | 1 |
/// | OCIE2A | 10 |
pub const TIMSK2: *mut u8 = 0x70 as *mut u8;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x78 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x78 as *mut u16;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x79 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADEN | 10000000 |
/// | ADPS | 111 |
/// | ADIF | 10000 |
/// | ADIE | 1000 |
/// | ADSC | 1000000 |
/// | ADATE | 100000 |
pub const ADCSRA: *mut u8 = 0x7A as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADTS | 111 |
pub const ADCSRB: *mut u8 = 0x7B as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 11111 |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x7C as *mut u8;

/// Digital Input Disable Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADC5D | 100000 |
/// | ADC7D | 10000000 |
/// | ADC0D | 1 |
/// | ADC6D | 1000000 |
/// | ADC3D | 1000 |
/// | ADC2D | 100 |
/// | ADC4D | 10000 |
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

/// Timer/Counter 1 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC1B | 1000000 |
/// | FOC1A | 10000000 |
pub const TCCR1C: *mut u8 = 0x82 as *mut u8;

/// Timer/Counter1  Bytes low byte.
pub const TCNT1L: *mut u8 = 0x84 as *mut u8;

/// Timer/Counter1  Bytes.
pub const TCNT1: *mut u16 = 0x84 as *mut u16;

/// Timer/Counter1  Bytes high byte.
pub const TCNT1H: *mut u8 = 0x85 as *mut u8;

/// Timer/Counter1 Input Capture Register  Bytes.
pub const ICR1: *mut u16 = 0x86 as *mut u16;

/// Timer/Counter1 Input Capture Register  Bytes low byte.
pub const ICR1L: *mut u8 = 0x86 as *mut u8;

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

/// Timer/Counter2 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC2A | 10000000 |
/// | CS2 | 111 |
/// | WGM21 | 1000 |
/// | COM2A | 110000 |
/// | WGM20 | 1000000 |
pub const TCCR2A: *mut u8 = 0xB0 as *mut u8;

/// Timer/Counter2.
pub const TCNT2: *mut u8 = 0xB2 as *mut u8;

/// Timer/Counter2 Output Compare Register.
pub const OCR2A: *mut u8 = 0xB3 as *mut u8;

/// Asynchronous Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AS2 | 1000 |
/// | EXCLK | 10000 |
/// | TCR2UB | 1 |
/// | OCR2UB | 10 |
/// | TCN2UB | 100 |
pub const ASSR: *mut u8 = 0xB6 as *mut u8;

/// USI Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USITC | 1 |
/// | USICS | 1100 |
/// | USISIE | 10000000 |
/// | USIOIE | 1000000 |
/// | USIWM | 110000 |
/// | USICLK | 10 |
pub const USICR: *mut u8 = 0xB8 as *mut u8;

/// USI Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | USIPF | 100000 |
/// | USISIF | 10000000 |
/// | USIDC | 10000 |
/// | USICNT | 1111 |
/// | USIOIF | 1000000 |
pub const USISR: *mut u8 = 0xB9 as *mut u8;

/// USI Data Register.
pub const USIDR: *mut u8 = 0xBA as *mut u8;

/// USART Control and Status Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UDRE0 | 100000 |
/// | RXC0 | 10000000 |
/// | MPCM0 | 1 |
/// | UPE0 | 100 |
/// | TXC0 | 1000000 |
/// | DOR0 | 1000 |
/// | FE0 | 10000 |
/// | U2X0 | 10 |
pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;

/// USART Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RXB80 | 10 |
/// | RXEN0 | 10000 |
/// | TXEN0 | 1000 |
/// | TXCIE0 | 1000000 |
/// | UCSZ02 | 100 |
/// | TXB80 | 1 |
/// | RXCIE0 | 10000000 |
/// | UDRIE0 | 100000 |
pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;

/// USART Control and Status Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | UPM0 | 110000 |
/// | UCSZ0 | 110 |
/// | UCPOL0 | 1 |
/// | USBS0 | 1000 |
/// | UMSEL0 | 1000000 |
pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

/// USART Baud Rate Register  Bytes.
pub const UBRR0: *mut u16 = 0xC4 as *mut u16;

/// USART Baud Rate Register  Bytes low byte.
pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;

/// USART Baud Rate Register  Bytes high byte.
pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;

/// USART I/O Data Register.
pub const UDR0: *mut u8 = 0xC6 as *mut u8;

/// LCD Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCDAB | 1000000 |
/// | LCDEN | 10000000 |
/// | LCDIF | 10000 |
/// | LCDBL | 1 |
/// | LCDBD | 100 |
/// | LCDIE | 1000 |
/// | LCDCCD | 10 |
pub const LCDCRA: *mut u8 = 0xE4 as *mut u8;

/// LCD Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCD2B | 1000000 |
/// | LCDCS | 10000000 |
/// | LCDMUX | 110000 |
/// | LCDPM | 1111 |
pub const LCDCRB: *mut u8 = 0xE5 as *mut u8;

/// LCD Frame Rate Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCDPS | 1110000 |
/// | LCDCD | 111 |
pub const LCDFRR: *mut u8 = 0xE6 as *mut u8;

/// LCD Contrast Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LCDMDT | 10000 |
/// | LCDCC | 1111 |
/// | LCDDC | 11100000 |
pub const LCDCCR: *mut u8 = 0xE7 as *mut u8;

/// LCD Data Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG002 | 100 |
/// | SEG000 | 1 |
/// | SEG004 | 10000 |
/// | SEG003 | 1000 |
/// | SEG001 | 10 |
/// | SEG006 | 1000000 |
/// | SEG007 | 10000000 |
/// | SEG005 | 100000 |
pub const LCDDR00: *mut u8 = 0xEC as *mut u8;

/// LCD Data Register 1.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG015 | 10000000 |
/// | SEG008 | 1 |
/// | SEG014 | 1000000 |
/// | SEG012 | 10000 |
/// | SEG009 | 10 |
/// | SEG010 | 100 |
/// | SEG011 | 1000 |
/// | SEG013 | 100000 |
pub const LCDDR01: *mut u8 = 0xED as *mut u8;

/// LCD Data Register 2.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG023 | 10000000 |
/// | SEG021 | 100000 |
/// | SEG017 | 10 |
/// | SEG016 | 1 |
/// | SEG022 | 1000000 |
/// | SEG020 | 10000 |
/// | SEG018 | 100 |
/// | SEG019 | 1000 |
pub const LCDDR02: *mut u8 = 0xEE as *mut u8;

/// LCD Data Register 3.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG026 | 100 |
/// | SEG027 | 1000 |
/// | SEG030 | 1000000 |
/// | SEG029 | 100000 |
/// | SEG025 | 10 |
/// | SEG024 | 1 |
/// | SEG028 | 10000 |
/// | SEG031 | 10000000 |
pub const LCDDR03: *mut u8 = 0xEF as *mut u8;

/// LCD Data Register 5.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG107 | 10000000 |
/// | SEG105 | 100000 |
/// | SEG106 | 1000000 |
/// | SEG102 | 100 |
/// | SEG101 | 10 |
/// | SEG100 | 1 |
/// | SEG103 | 1000 |
/// | SEG104 | 10000 |
pub const LCDDR05: *mut u8 = 0xF1 as *mut u8;

/// LCD Data Register 6.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG109 | 10 |
/// | SEG114 | 1000000 |
/// | SEG113 | 100000 |
/// | SEG110 | 100 |
/// | SEG112 | 10000 |
/// | SEG108 | 1 |
/// | SEG111 | 1000 |
/// | SEG115 | 10000000 |
pub const LCDDR06: *mut u8 = 0xF2 as *mut u8;

/// LCD Data Register 7.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG121 | 100000 |
/// | SEG123 | 10000000 |
/// | SEG119 | 1000 |
/// | SEG117 | 10 |
/// | SEG118 | 100 |
/// | SEG120 | 10000 |
/// | SEG116 | 1 |
/// | SEG122 | 1000000 |
pub const LCDDR07: *mut u8 = 0xF3 as *mut u8;

/// LCD Data Register 8.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG127 | 1000 |
/// | SEG124 | 1 |
/// | SEG130 | 1000000 |
/// | SEG129 | 100000 |
/// | SEG125 | 10 |
/// | SEG128 | 10000 |
/// | SEG131 | 10000000 |
/// | SEG126 | 100 |
pub const LCDDR08: *mut u8 = 0xF4 as *mut u8;

/// LCD Data Register 10.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG202 | 100 |
/// | SEG205 | 100000 |
/// | SEG207 | 10000000 |
/// | SEG203 | 1000 |
/// | SEG206 | 1000000 |
/// | SEG201 | 10 |
/// | SEG200 | 1 |
/// | SEG204 | 10000 |
pub const LCDDR10: *mut u8 = 0xF6 as *mut u8;

/// LCD Data Register 11.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG209 | 10 |
/// | SEG212 | 10000 |
/// | SEG211 | 1000 |
/// | SEG215 | 10000000 |
/// | SEG214 | 1000000 |
/// | SEG208 | 1 |
/// | SEG210 | 100 |
/// | SEG213 | 100000 |
pub const LCDDR11: *mut u8 = 0xF7 as *mut u8;

/// LCD Data Register 12.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG221 | 100000 |
/// | SEG217 | 10 |
/// | SEG218 | 100 |
/// | SEG216 | 1 |
/// | SEG222 | 1000000 |
/// | SEG219 | 1000 |
/// | SEG223 | 10000000 |
/// | SEG220 | 10000 |
pub const LCDDR12: *mut u8 = 0xF8 as *mut u8;

/// LCD Data Register 13.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG231 | 10000000 |
/// | SEG224 | 1 |
/// | SEG229 | 100000 |
/// | SEG226 | 100 |
/// | SEG227 | 1000 |
/// | SEG228 | 10000 |
/// | SEG230 | 1000000 |
/// | SEG225 | 10 |
pub const LCDDR13: *mut u8 = 0xF9 as *mut u8;

/// LCD Data Register 15.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG303 | 1000 |
/// | SEG307 | 10000000 |
/// | SEG306 | 1000000 |
/// | SEG304 | 10000 |
/// | SEG300 | 1 |
/// | SEG305 | 100000 |
/// | SEG301 | 10 |
/// | SEG302 | 100 |
pub const LCDDR15: *mut u8 = 0xFB as *mut u8;

/// LCD Data Register 16.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG310 | 100 |
/// | SEG308 | 1 |
/// | SEG313 | 100000 |
/// | SEG312 | 10000 |
/// | SEG314 | 1000000 |
/// | SEG315 | 10000000 |
/// | SEG309 | 10 |
/// | SEG311 | 1000 |
pub const LCDDR16: *mut u8 = 0xFC as *mut u8;

/// LCD Data Register 17.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG316 | 1 |
/// | SEG319 | 1000 |
/// | SEG323 | 10000000 |
/// | SEG320 | 10000 |
/// | SEG317 | 10 |
/// | SEG318 | 100 |
/// | SEG321 | 100000 |
/// | SEG322 | 1000000 |
pub const LCDDR17: *mut u8 = 0xFD as *mut u8;

/// LCD Data Register 18.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SEG331 | 10000000 |
/// | SEG330 | 1000000 |
/// | SEG324 | 1 |
/// | SEG327 | 1000 |
/// | SEG329 | 100000 |
/// | SEG328 | 10000 |
/// | SEG325 | 10 |
/// | SEG326 | 100 |
pub const LCDDR18: *mut u8 = 0xFE as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ADTS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x1F as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ASSR`
pub const AS2: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ASSR`
pub const EXCLK: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCR2UB: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `ASSR`
pub const OCR2UB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `ASSR`
pub const TCN2UB: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC5D: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC7D: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC6D: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC3D: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC2D: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC4D: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `DIDR0`
pub const ADC1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR1`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIFR`
pub const PCIF: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `EIMSK`
pub const PCIE: *mut u8 = 0xF0 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const RSTDISBL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EXTENDED`
pub const BODLEVEL: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR2: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `HIGH`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTSZ: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const WDTON: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `HIGH`
pub const EESAVE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const JTAGEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `HIGH`
pub const OCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `HIGH`
pub const BOOTRST: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDCCR`
pub const LCDMDT: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDCCR`
pub const LCDCC: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LCDCCR`
pub const LCDDC: *mut u8 = 0xE0 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDAB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDBL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDBD: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDCRA`
pub const LCDCCD: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDCRB`
pub const LCD2B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDCRB`
pub const LCDCS: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDCRB`
pub const LCDMUX: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LCDCRB`
pub const LCDPM: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG002: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG000: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG004: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG003: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG001: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG006: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG007: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR00`
pub const SEG005: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG015: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG008: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG014: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG012: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG009: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG010: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG011: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR01`
pub const SEG013: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG023: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG021: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG017: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG016: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG022: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG020: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG018: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR02`
pub const SEG019: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG026: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG027: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG030: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG029: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG025: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG024: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG028: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR03`
pub const SEG031: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG107: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG105: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG106: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG102: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG101: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG100: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG103: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR05`
pub const SEG104: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG109: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG114: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG113: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG110: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG112: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG108: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG111: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR06`
pub const SEG115: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG121: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG123: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG119: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG117: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG118: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG120: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG116: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR07`
pub const SEG122: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG127: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG124: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG130: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG129: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG125: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG128: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG131: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR08`
pub const SEG126: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG202: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG205: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG207: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG203: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG206: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG201: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG200: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR10`
pub const SEG204: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG209: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG212: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG211: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG215: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG214: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG208: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG210: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR11`
pub const SEG213: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG221: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG217: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG218: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG216: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG222: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG219: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG223: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR12`
pub const SEG220: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG231: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG224: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG229: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG226: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG227: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG228: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG230: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR13`
pub const SEG225: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG303: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG307: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG306: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG304: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG300: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG305: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG301: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR15`
pub const SEG302: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG310: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG308: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG313: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG312: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG314: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG315: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG309: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR16`
pub const SEG311: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG316: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG319: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG323: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG320: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG317: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG318: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG321: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR17`
pub const SEG322: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG331: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG330: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG324: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG327: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG329: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG328: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG325: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `LCDDR18`
pub const SEG326: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `LCDFRR`
pub const LCDPS: *mut u8 = 0x70 as *mut u8;

/// Bitfield on register `LCDFRR`
pub const LCDCD: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB1: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const BLB0: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const CKOUT: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const JTD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVSEL: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUCR`
pub const IVCE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const JTRF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM1: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `PRR`
pub const PRSPI: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `PRR`
pub const PRUSART0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRLCD: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SPCR`
pub const DORD: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPR: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPCR`
pub const MSTR: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPHA: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPCR`
pub const CPOL: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPCR`
pub const SPIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSB: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const BLBSET: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RWWSRE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPI2X: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPSR`
pub const SPIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SPSR`
pub const WCOL: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM01: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM00: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const CS0: *mut u8 = 0x7 as *mut u8;

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
pub const FOC1B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1C`
pub const FOC1A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const FOC2A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const CS2: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const WGM21: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const COM2A: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR2A`
pub const WGM20: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const ICF1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIFR1`
pub const OCF1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR1`
pub const TOV1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const TOV2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR2`
pub const OCF2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const OCIE1A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const TOIE1: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK1`
pub const ICIE1: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const TOIE2: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK2`
pub const OCIE2A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UDRE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const RXC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const MPCM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const UPE0: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const TXC0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const DOR0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const FE0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0A`
pub const U2X0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXB80: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXEN0: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXEN0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXCIE0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UCSZ02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const TXB80: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const RXCIE0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `UCSR0B`
pub const UDRIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UPM0: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCSZ0: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UCPOL0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const USBS0: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `UCSR0C`
pub const UMSEL0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USICR`
pub const USITC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `USICR`
pub const USICS: *mut u8 = 0xC as *mut u8;

/// Bitfield on register `USICR`
pub const USISIE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USICR`
pub const USIOIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `USICR`
pub const USIWM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `USICR`
pub const USICLK: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `USISR`
pub const USIPF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `USISR`
pub const USISIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `USISR`
pub const USIDC: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `USISR`
pub const USICNT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `USISR`
pub const USIOIF: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x7 as *mut u8;

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

/// `ANALOG_ADC_V_REF3` value group
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

/// `COMM_USI_OP` value group
#[allow(non_upper_case_globals)]
pub mod comm_usi_op {
   /// Normal Operation.
   pub const VAL_0x00: u32 = 0x0;
   /// Three-Wire Mode.
   pub const VAL_0x01: u32 = 0x1;
   /// Two-Wire Mode.
   pub const VAL_0x02: u32 = 0x2;
   /// Two-Wire Mode Held Low.
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
   pub const DISABLED: u32 = 0x3;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x2;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x0;
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
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 0 ms.
   pub const EXTLOFXTAL_32KCK_0MS: u32 = 0x7;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 4.1 ms.
   pub const EXTLOFXTAL_32KCK_4MS1: u32 = 0x17;
   /// Ext. Low-Freq. Crystal; Start-up time: 32K CK + 65 ms.
   pub const EXTLOFXTAL_32KCK_65MS: u32 = 0x27;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 0 ms.
   pub const EXTLOFXTAL_1KCK_0MS: u32 = 0x6;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 4.1 ms.
   pub const EXTLOFXTAL_1KCK_4MS1: u32 = 0x16;
   /// Ext. Low-Freq. Crystal; Start-up time: 1K CK + 65 ms.
   pub const EXTLOFXTAL_1KCK_65MS: u32 = 0x26;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_4MS1: u32 = 0x8;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_258CK_65MS: u32 = 0x18;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_0MS: u32 = 0x28;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_4MS1: u32 = 0x38;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_1KCK_65MS: u32 = 0x9;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_0MS: u32 = 0x19;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_4MS1: u32 = 0x29;
   /// Ext. Crystal Osc. 0.4-0.9 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_0MHZ4_0MHZ9_16KCK_65MS: u32 = 0x39;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_4MS1: u32 = 0xA;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_258CK_65MS: u32 = 0x1A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_0MS: u32 = 0x2A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_4MS1: u32 = 0x3A;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_1KCK_65MS: u32 = 0xB;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_0MS: u32 = 0x1B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_4MS1: u32 = 0x2B;
   /// Ext. Crystal Osc. 0.9-3.0 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_0MHZ9_3MHZ_16KCK_65MS: u32 = 0x3B;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_4MS1: u32 = 0xC;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_258CK_65MS: u32 = 0x1C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_0MS: u32 = 0x2C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_4MS1: u32 = 0x3C;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_1KCK_65MS: u32 = 0xD;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_0MS: u32 = 0x1D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_4MS1: u32 = 0x2D;
   /// Ext. Crystal Osc. 3.0-8.0 MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_3MHZ_8MHZ_16KCK_65MS: u32 = 0x3D;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 258 CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_4MS1: u32 = 0xE;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 258 CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_258CK_65MS: u32 = 0x1E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_0MS: u32 = 0x2E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_4MS1: u32 = 0x3E;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 1K CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_1KCK_65MS: u32 = 0xF;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 0 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_0MS: u32 = 0x1F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 4.1 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_4MS1: u32 = 0x2F;
   /// Ext. Crystal Osc. 8.0-    MHz; Start-up time: 16K CK + 65 ms.
   pub const EXTXOSC_8MHZ_XX_16KCK_65MS: u32 = 0x3F;
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

/// `LCD_CONTRAST` value group
#[allow(non_upper_case_globals)]
pub mod lcd_contrast {
   /// 2.60V.
   pub const VAL_0x00: u32 = 0x0;
   /// 2.65V.
   pub const VAL_0x01: u32 = 0x1;
   /// 2.70V.
   pub const VAL_0x02: u32 = 0x2;
   /// 2.75V.
   pub const VAL_0x03: u32 = 0x3;
   /// 2.80V.
   pub const VAL_0x04: u32 = 0x4;
   /// 2.85V.
   pub const VAL_0x05: u32 = 0x5;
   /// 2.90V.
   pub const VAL_0x06: u32 = 0x6;
   /// 2.95V.
   pub const VAL_0x07: u32 = 0x7;
   /// 3.00V.
   pub const VAL_0x08: u32 = 0x8;
   /// 3.05V.
   pub const VAL_0x09: u32 = 0x9;
   /// 3.10V.
   pub const VAL_0x0A: u32 = 0xA;
   /// 3.15V.
   pub const VAL_0x0B: u32 = 0xB;
   /// 3.20V.
   pub const VAL_0x0C: u32 = 0xC;
   /// 3.25V.
   pub const VAL_0x0D: u32 = 0xD;
   /// 3.30V.
   pub const VAL_0x0E: u32 = 0xE;
   /// 3.35V.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `LCD_DISP_CONF_DRIVE` value group
#[allow(non_upper_case_globals)]
pub mod lcd_disp_conf_drive {
   /// 300us.
   pub const VAL_0x00: u32 = 0x0;
   /// 70us.
   pub const VAL_0x01: u32 = 0x1;
   /// 150us.
   pub const VAL_0x02: u32 = 0x2;
   /// 450us.
   pub const VAL_0x03: u32 = 0x3;
   /// 575us.
   pub const VAL_0x04: u32 = 0x4;
   /// 850us.
   pub const VAL_0x05: u32 = 0x5;
   /// 1150us.
   pub const VAL_0x06: u32 = 0x6;
   /// 50% of LCD clock.
   pub const VAL_0x07: u32 = 0x7;
}

/// `LCD_PORT_MASK_4BIT` value group
#[allow(non_upper_case_globals)]
pub mod lcd_port_mask_4bit {
   /// SEG0:12.
   pub const VAL_0x00: u32 = 0x0;
   /// SEG0:14.
   pub const VAL_0x01: u32 = 0x1;
   /// SEG0:15.
   pub const VAL_0x02: u32 = 0x2;
   /// SEG0:18.
   pub const VAL_0x03: u32 = 0x3;
   /// SEG0:20.
   pub const VAL_0x04: u32 = 0x4;
   /// SEG0:22.
   pub const VAL_0x05: u32 = 0x5;
   /// SEG0:23.
   pub const VAL_0x06: u32 = 0x6;
   /// SEG0:24.
   pub const VAL_0x07: u32 = 0x7;
   /// SEG0:26.
   pub const VAL_0x08: u32 = 0x8;
   /// SEG0:28.
   pub const VAL_0x09: u32 = 0x9;
   /// SEG0:30.
   pub const VAL_0x0A: u32 = 0xA;
   /// SEG0:32.
   pub const VAL_0x0B: u32 = 0xB;
   /// SEG0:34.
   pub const VAL_0x0C: u32 = 0xC;
   /// SEG0:36.
   pub const VAL_0x0D: u32 = 0xD;
   /// SEG0:38.
   pub const VAL_0x0E: u32 = 0xE;
   /// SEG0:39.
   pub const VAL_0x0F: u32 = 0xF;
}

/// `LCD_PRESCALE` value group
#[allow(non_upper_case_globals)]
pub mod lcd_prescale {
   /// ClkLCD/16.
   pub const VAL_0x00: u32 = 0x0;
   /// ClkLCD/64.
   pub const VAL_0x01: u32 = 0x1;
   /// ClkLCD/128.
   pub const VAL_0x02: u32 = 0x2;
   /// ClkLCD/256.
   pub const VAL_0x03: u32 = 0x3;
   /// ClkLCD/512.
   pub const VAL_0x04: u32 = 0x4;
   /// ClkLCD/1024.
   pub const VAL_0x05: u32 = 0x5;
   /// ClkLCD/2048.
   pub const VAL_0x06: u32 = 0x6;
   /// ClkLCD/4096.
   pub const VAL_0x07: u32 = 0x7;
}

/// `MISC_3BIT_COUNT` value group
#[allow(non_upper_case_globals)]
pub mod misc_3bit_count {
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
}

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 8.0 MHz.
   pub const _8_0_MHz: u32 = 0x0;
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

