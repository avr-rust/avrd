//! The AVR ATtiny13A microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny13A-PU | PDIP8-SOIC8 | PDIP8 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SH | PDIP8-SOIC8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SSH | PDIP8-SOIC8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SSU | PDIP8-SOIC8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SU | PDIP8-SOIC8 | SOIC8 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-MMU | MLF10 | MLF10 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-MU | MLF20 | MLF20 | -40°C - 85°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SN | PDIP8-SOIC8 | SOIC8 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SS7 | PDIP8-SOIC8 | SOIC8 | -40°C - 105°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-SF | PDIP8-SOIC8 | SOIC8 | -40°C - 125°C | 1.8V - 5.5V | 20 MHz |
//! | ATtiny13A-MMF | MLF10 | MLF10 | -40°C - 125°C | 1.8V - 5.5V | 20 MHz |
//!

#![allow(non_upper_case_globals)]

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDTON | 100000 |
/// | CKDIV8 | 10000 |
/// | SPIEN | 10000000 |
/// | SUT_CKSEL | 1111 |
/// | EESAVE | 1000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// `HIGH` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | DWEN | 1000 |
/// | RSTDISBL | 1 |
/// | BODLEVEL | 110 |
/// | SELFPRGEN | 10000 |
pub const HIGH: *mut u8 = 0x1 as *mut u8;

/// ADC Control and Status Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACME | 1000000 |
pub const ADCSRB: *mut u8 = 0x23 as *mut u8;

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
/// | ADIE | 1000 |
/// | ADSC | 1000000 |
/// | ADEN | 10000000 |
/// | ADPS | 111 |
/// | ADATE | 100000 |
pub const ADCSRA: *mut u8 = 0x26 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | MUX | 11 |
/// | REFS0 | 1000000 |
/// | ADLAR | 100000 |
pub const ADMUX: *mut u8 = 0x27 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACI | 10000 |
/// | ACIS | 11 |
/// | ACD | 10000000 |
/// | ACBG | 1000000 |
pub const ACSR: *mut u8 = 0x28 as *mut u8;

/// `DIDR0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN1D | 10 |
/// | AIN0D | 1 |
pub const DIDR0: *mut u8 = 0x34 as *mut u8;

/// Pin Change Enable Mask.
pub const PCMSK: *mut u8 = 0x35 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x36 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x37 as *mut u8;

/// Data Register, Port B.
pub const PORTB: *mut u8 = 0x38 as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EEWE | 10 |
/// | EEPM | 110000 |
/// | EERE | 1 |
/// | EERIE | 1000 |
/// | EEMWE | 100 |
pub const EECR: *mut u8 = 0x3C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x3D as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u8 = 0x3E as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDTIF | 10000000 |
/// | WDCE | 10000 |
/// | WDP | 100111 |
/// | WDE | 1000 |
/// | WDTIE | 1000000 |
pub const WDTCR: *mut u8 = 0x41 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 10 |
/// | PRADC | 1 |
pub const PRR: *mut u8 = 0x45 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
/// | CLKPCE | 10000000 |
pub const CLKPR: *mut u8 = 0x46 as *mut u8;

/// General Timer Conuter Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR10 | 1 |
pub const GTCCR: *mut u8 = 0x48 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0B: *mut u8 = 0x49 as *mut u8;

/// Debug Wire Data Register.
pub const DWDR: *mut u8 = 0x4E as *mut u8;

/// Timer/Counter  Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WGM0 | 11 |
/// | COM0A | 11000000 |
/// | COM0B | 110000 |
pub const TCCR0A: *mut u8 = 0x4F as *mut u8;

/// BOD Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODS | 10 |
/// | BODSE | 1 |
pub const BODCR: *mut u8 = 0x50 as *mut u8;

/// Oscillator Calibration Register.
pub const OSCCAL: *mut u8 = 0x51 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u8 = 0x52 as *mut u8;

/// Timer/Counter Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0A | 10000000 |
/// | CS0 | 111 |
/// | FOC0B | 1000000 |
/// | WGM02 | 1000 |
pub const TCCR0B: *mut u8 = 0x53 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | WDRF | 1000 |
/// | BORF | 100 |
/// | PORF | 1 |
pub const MCUSR: *mut u8 = 0x54 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SM | 11000 |
/// | ISC0 | 11 |
/// | PUD | 1000000 |
/// | SE | 100000 |
pub const MCUCR: *mut u8 = 0x55 as *mut u8;

/// Timer/Counter0 Output Compare Register.
pub const OCR0A: *mut u8 = 0x56 as *mut u8;

/// Store Program Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PGWRT | 100 |
/// | SPMEN | 1 |
/// | CTPB | 10000 |
/// | RFLB | 1000 |
/// | PGERS | 10 |
pub const SPMCSR: *mut u8 = 0x57 as *mut u8;

/// Timer/Counter0 Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF0B | 1000 |
/// | TOV0 | 10 |
/// | OCF0A | 100 |
pub const TIFR0: *mut u8 = 0x58 as *mut u8;

/// Timer/Counter0 Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0A | 100 |
/// | TOIE0 | 10 |
/// | OCIE0B | 1000 |
pub const TIMSK0: *mut u8 = 0x59 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1000000 |
/// | PCIF | 100000 |
pub const GIFR: *mut u8 = 0x5A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1000000 |
/// | PCIE | 100000 |
pub const GIMSK: *mut u8 = 0x5B as *mut u8;

/// Stack Pointer Low Byte.
pub const SPL: *mut u8 = 0x5D as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | C | 1 |
/// | H | 100000 |
/// | N | 100 |
/// | S | 10000 |
/// | V | 1000 |
/// | Z | 10 |
/// | I | 10000000 |
/// | T | 1000000 |
pub const SREG: *mut u8 = 0x5F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSRA`
pub const ADATE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSRB`
pub const ACME: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `BODCR`
pub const BODS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BODCR`
pub const BODSE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `CLKPR`
pub const CLKPCE: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `DIDR0`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EEPM: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIFR`
pub const PCIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GIMSK`
pub const INT0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIMSK`
pub const PCIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR10: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const DWEN: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `HIGH`
pub const RSTDISBL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `HIGH`
pub const BODLEVEL: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `HIGH`
pub const SELFPRGEN: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const WDTON: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOW`
pub const CKDIV8: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const SUT_CKSEL: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `LOW`
pub const EESAVE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGWRT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const SPMEN: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const CTPB: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const RFLB: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SPMCSR`
pub const PGERS: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const WGM0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

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

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDTIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDCE: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDTIE: *mut u8 = 0x40 as *mut u8;

/// `ANALOG_ADC_AUTO_TRIGGER2` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_auto_trigger2 {
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

/// `CPU_SLEEP_MODE2` value group
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode2 {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction (If Available).
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=4.3 V.
   pub const _4V3: u32 = 0x0;
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
   /// Brown-out detection at VCC=1.8 V.
   pub const _1V8: u32 = 0x2;
   /// Brown-out detection disabled.
   pub const DISABLED: u32 = 0x3;
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
   /// Ext. Clock; Start-up time: 14 CK + 0 ms.
   pub const EXTCLK_14CK_0MS: u32 = 0x0;
   /// Ext. Clock; Start-up time: 14 CK + 4 ms.
   pub const EXTCLK_14CK_4MS: u32 = 0x4;
   /// Ext. Clock; Start-up time: 14 CK + 64 ms.
   pub const EXTCLK_14CK_64MS: u32 = 0x8;
   /// Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 0 ms.
   pub const INTRCOSC_4MHZ8_14CK_0MS: u32 = 0x1;
   /// Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 4 ms.
   pub const INTRCOSC_4MHZ8_14CK_4MS: u32 = 0x5;
   /// Int. RC Osc. 4.8 MHz; Start-up time: 14 CK + 64 ms.
   pub const INTRCOSC_4MHZ8_14CK_64MS: u32 = 0x9;
   /// Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 0 ms.
   pub const INTRCOSC_9MHZ6_14CK_0MS: u32 = 0x2;
   /// Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 4 ms.
   pub const INTRCOSC_9MHZ6_14CK_4MS: u32 = 0x6;
   /// Int. RC Osc. 9.6 MHz; Start-up time: 14 CK + 64 ms.
   pub const INTRCOSC_9MHZ6_14CK_64MS: u32 = 0xA;
   /// Int. RC Osc. 128 kHz; Start-up time: 14 CK + 0 ms.
   pub const INTRCOSC_128KHZ_14CK_0MS: u32 = 0x3;
   /// Int. RC Osc. 128 kHz; Start-up time: 14 CK + 4 ms.
   pub const INTRCOSC_128KHZ_14CK_4MS: u32 = 0x7;
   /// Int. RC Osc. 128 kHz; Start-up time: 14 CK + 64 ms.
   pub const INTRCOSC_128KHZ_14CK_64MS: u32 = 0xB;
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

/// `INTERRUPT_SENSE_CONTROL2` value group
#[allow(non_upper_case_globals)]
pub mod interrupt_sense_control2 {
   /// Low Level of INTX.
   pub const VAL_0x00: u32 = 0x0;
   /// Any Logical Change in INTX.
   pub const VAL_0x01: u32 = 0x1;
   /// Falling Edge of INTX.
   pub const VAL_0x02: u32 = 0x2;
   /// Rising Edge of INTX.
   pub const VAL_0x03: u32 = 0x3;
}

/// Oscillator Calibration Values
#[allow(non_upper_case_globals)]
pub mod osccal_value_addresses {
   /// 9.6 MHz.
   pub const _9_6_MHz: u32 = 0x0;
   /// 4.8 MHz.
   pub const _4_8_MHz: u32 = 0x1;
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

