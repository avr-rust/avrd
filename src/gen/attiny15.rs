//! The AVR ATtiny15 microcontroller
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
/// | LB | 110 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// `LOW` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BODLEVEL | 10000000 |
/// | CKSEL | 11 |
/// | SPIEN | 100000 |
/// | RSTDISBL | 10000 |
/// | BODEN | 1000000 |
pub const LOW: *mut u8 = 0x0 as *mut u8;

/// ADC Data Register  Bytes.
pub const ADC: *mut u16 = 0x4 as *mut u16;

/// ADC Data Register  Bytes low byte.
pub const ADCL: *mut u8 = 0x4 as *mut u8;

/// ADC Data Register  Bytes high byte.
pub const ADCH: *mut u8 = 0x5 as *mut u8;

/// The ADC Control and Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ADIF | 10000 |
/// | ADSC | 1000000 |
/// | ADPS | 111 |
/// | ADIE | 1000 |
/// | ADFR | 100000 |
/// | ADEN | 10000000 |
pub const ADCSR: *mut u8 = 0x6 as *mut u8;

/// The ADC multiplexer Selection Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | REFS | 11000000 |
/// | ADLAR | 100000 |
/// | MUX | 111 |
pub const ADMUX: *mut u8 = 0x7 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACBG | 1000000 |
/// | ACIE | 1000 |
/// | ACO | 100000 |
/// | ACIS | 11 |
pub const ACSR: *mut u8 = 0x8 as *mut u8;

/// Input Pins, Port B.
pub const PINB: *mut u8 = 0x16 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x17 as *mut u8;

/// Data Register, Port B.
pub const PORTB: *mut u8 = 0x18 as *mut u8;

/// EEPROM Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EERE | 1 |
/// | EEWE | 10 |
/// | EERIE | 1000 |
/// | EEMWE | 100 |
pub const EECR: *mut u8 = 0x1C as *mut u8;

/// EEPROM Data Register.
pub const EEDR: *mut u8 = 0x1D as *mut u8;

/// EEPROM Read/Write Access.
pub const EEAR: *mut u8 = 0x1E as *mut u8;

/// Watchdog Timer Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDE | 1000 |
/// | WDP | 111 |
/// | WDTOE | 10000 |
pub const WDTCR: *mut u8 = 0x21 as *mut u8;

/// Special Function IO Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PSR0 | 1 |
/// | PSR1 | 10 |
/// | FOC1A | 100 |
pub const SFIOR: *mut u8 = 0x2C as *mut u8;

/// Output Compare Register.
pub const OCR1B: *mut u8 = 0x2D as *mut u8;

/// Output Compare Register.
pub const OCR1A: *mut u8 = 0x2E as *mut u8;

/// Timer/Counter Register.
pub const TCNT1: *mut u8 = 0x2F as *mut u8;

/// Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CTC1 | 10000000 |
/// | COM1A | 110000 |
/// | PWM1 | 1000000 |
/// | CS1 | 1111 |
pub const TCCR1: *mut u8 = 0x30 as *mut u8;

/// Status Register.
pub const OSCCAL: *mut u8 = 0x31 as *mut u8;

/// Timer Counter 0.
pub const TCNT0: *mut u8 = 0x32 as *mut u8;

/// Timer/Counter0 Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CS01 | 10 |
/// | CS02 | 100 |
/// | CS00 | 1 |
pub const TCCR0: *mut u8 = 0x33 as *mut u8;

/// MCU Status register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDRF | 1000 |
/// | BORF | 100 |
/// | PORF | 1 |
/// | EXTRF | 10 |
pub const MCUSR: *mut u8 = 0x34 as *mut u8;

/// MCU Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 100000 |
/// | ISC0 | 11 |
/// | PUD | 1000000 |
/// | SM | 11000 |
pub const MCUCR: *mut u8 = 0x35 as *mut u8;

/// Timer/Counter Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCF1A | 1000000 |
/// | TOV1 | 100 |
pub const TIFR: *mut u8 = 0x38 as *mut u8;

/// Timer/Counter Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOIE1 | 100 |
/// | OCIE1A | 1000000 |
pub const TIMSK: *mut u8 = 0x39 as *mut u8;

/// General Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF | 100000 |
/// | INTF0 | 1000000 |
pub const GIFR: *mut u8 = 0x3A as *mut u8;

/// General Interrupt Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1000000 |
/// | PCIE | 100000 |
pub const GIMSK: *mut u8 = 0x3B as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | Z | 10 |
/// | N | 100 |
/// | T | 1000000 |
/// | H | 100000 |
/// | S | 10000 |
/// | V | 1000 |
/// | I | 10000000 |
/// | C | 1 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACBG: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `ADCSR`
pub const ADIF: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ADCSR`
pub const ADSC: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `ADCSR`
pub const ADPS: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `ADCSR`
pub const ADIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ADCSR`
pub const ADFR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADCSR`
pub const ADEN: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ADMUX`
pub const REFS: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `ADMUX`
pub const ADLAR: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ADMUX`
pub const MUX: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `EECR`
pub const EERE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EECR`
pub const EEWE: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EECR`
pub const EERIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `EECR`
pub const EEMWE: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `GIFR`
pub const PCIF: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `GIFR`
pub const INTF0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIMSK`
pub const INT0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `GIMSK`
pub const PCIE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x6 as *mut u8;

/// Bitfield on register `LOW`
pub const BODLEVEL: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `LOW`
pub const CKSEL: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `LOW`
pub const SPIEN: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `LOW`
pub const RSTDISBL: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `LOW`
pub const BODEN: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SE: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `MCUCR`
pub const ISC0: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `MCUCR`
pub const PUD: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `MCUCR`
pub const SM: *mut u8 = 0x18 as *mut u8;

/// Bitfield on register `MCUSR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `MCUSR`
pub const BORF: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `MCUSR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `MCUSR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFIOR`
pub const PSR0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SFIOR`
pub const PSR1: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SFIOR`
pub const FOC1A: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR0`
pub const CS01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TCCR0`
pub const CS02: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR0`
pub const CS00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TCCR1`
pub const CTC1: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR1`
pub const COM1A: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR1`
pub const PWM1: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR1`
pub const CS1: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `TIFR`
pub const OCF1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR`
pub const TOV1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK`
pub const TOIE1: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK`
pub const OCIE1A: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDP: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `WDTCR`
pub const WDTOE: *mut u8 = 0x10 as *mut u8;

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

/// `ANALOG_ADC_V_REF` value group
#[allow(non_upper_case_globals)]
pub mod analog_adc_v_ref {
   /// AVCC.
   pub const VAL_0x00: u32 = 0x0;
   /// AREF.
   pub const VAL_0x01: u32 = 0x1;
   /// Internal Voltage Referance.
   pub const VAL_0x02: u32 = 0x2;
   /// Internal Voltage Reference With External Capacitor at AREF Pin.
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

/// `CLK_COMP_MATCH_OUT_MODE` value group
#[allow(non_upper_case_globals)]
pub mod clk_comp_match_out_mode {
   /// Disconnected from OCn/PWMn.
   pub const VAL_0x00: u32 = 0x0;
   /// Toggle OCn/PWMn output line.
   pub const VAL_0x01: u32 = 0x1;
   /// Clear OCn/PWMn output line.
   pub const VAL_0x02: u32 = 0x2;
   /// set OCn/PWMn output line.
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

/// `CLK_SEL_4BIT_FAST` value group
#[allow(non_upper_case_globals)]
pub mod clk_sel_4bit_fast {
   /// No Clock Source (Stopped).
   pub const VAL_0x00: u32 = 0x0;
   /// Running, CLK*16.
   pub const VAL_0x01: u32 = 0x1;
   /// Running, CLK*8.
   pub const VAL_0x02: u32 = 0x2;
   /// Running, CLK*4.
   pub const VAL_0x03: u32 = 0x3;
   /// Running, CLK*2.
   pub const VAL_0x04: u32 = 0x4;
   /// Running, No Prescaling.
   pub const VAL_0x05: u32 = 0x5;
   /// Running, CLK/2.
   pub const VAL_0x06: u32 = 0x6;
   /// Running, CLK/4.
   pub const VAL_0x07: u32 = 0x7;
   /// Running, CLK/8.
   pub const VAL_0x08: u32 = 0x8;
   /// Running, CLK/16.
   pub const VAL_0x09: u32 = 0x9;
   /// Running, CLK/32.
   pub const VAL_0x0A: u32 = 0xA;
   /// Running, CLK/64.
   pub const VAL_0x0B: u32 = 0xB;
   /// Running, CLK/128.
   pub const VAL_0x0C: u32 = 0xC;
   /// Running, CLK/256.
   pub const VAL_0x0D: u32 = 0xD;
   /// Running, CLK/512.
   pub const VAL_0x0E: u32 = 0xE;
   /// Running, CLK/1024.
   pub const VAL_0x0F: u32 = 0xF;
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

/// `ENUM_BODLEVEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_bodlevel {
   /// Brown-out detection at VCC=2.7 V.
   pub const _2V7: u32 = 0x1;
   /// Brown-out detection at VCC=4.0 V.
   pub const _4V0: u32 = 0x0;
}

/// `ENUM_LB` value group
#[allow(non_upper_case_globals)]
pub mod enum_lb {
   /// Further programming and verification disabled.
   pub const PROG_VER_DISABLED: u32 = 0x0;
   /// Further programming disabled.
   pub const PROG_DISABLED: u32 = 0x1;
   /// No memory lock features enabled.
   pub const NO_LOCK: u32 = 0x3;
}

/// `ENUM_SUT_CKSEL` value group
#[allow(non_upper_case_globals)]
pub mod enum_sut_cksel {
   /// Very quickly rising power.
   pub const VQUICKPWR: u32 = 0x3;
   /// Quickly rising power.
   pub const QUICKPWR: u32 = 0x2;
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
   /// 1.6 MHz.
   pub const _1_6_MHz: u32 = 0x0;
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

