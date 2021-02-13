//! The AVR ATtiny4 microcontroller
//!
//! # Variants
//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |
//! |--------|--------|---------|-----------------------|-------------------|-----------|
//! | ATtiny4-TSHR | SOT23-6 | SOT23-6 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny4-MAHR | UDFN8 | UDFN8 | -40°C - 85°C | 1.8V - 5.5V | 12 MHz |
//! | ATtiny4-TS8R | SOT23-6 | SOT23-6 | -40°C - 125°C | 1.8V - 5.5V | 10 MHz |
//!

#![allow(non_upper_case_globals)]

/// Port B Data register.
pub const PINB: *mut u8 = 0x0 as *mut u8;

/// `BYTE0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | RSTDISBL | 1 |
/// | WDTON | 10 |
/// | CKOUT | 100 |
pub const BYTE0: *mut u8 = 0x0 as *mut u8;

/// `LOCKBIT` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | LB | 11 |
pub const LOCKBIT: *mut u8 = 0x0 as *mut u8;

/// Data Direction Register, Port B.
pub const DDRB: *mut u8 = 0x1 as *mut u8;

/// Input Pins, Port B.
pub const PORTB: *mut u8 = 0x2 as *mut u8;

/// Pull-up Enable Control Register.
pub const PUEB: *mut u8 = 0x3 as *mut u8;

/// Port Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | BBMB | 10 |
pub const PORTCR: *mut u8 = 0xC as *mut u8;

/// Pin Change Mask Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCINT | 1111 |
pub const PCMSK: *mut u8 = 0x10 as *mut u8;

/// Pin Change Interrupt Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIF0 | 1 |
pub const PCIFR: *mut u8 = 0x11 as *mut u8;

/// Pin Change Interrupt Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PCIE0 | 1 |
pub const PCICR: *mut u8 = 0x12 as *mut u8;

/// External Interrupt Mask register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INT0 | 1 |
pub const EIMSK: *mut u8 = 0x13 as *mut u8;

/// External Interrupt Flag register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | INTF0 | 1 |
pub const EIFR: *mut u8 = 0x14 as *mut u8;

/// External Interrupt Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ISC00 | 1 |
/// | ISC01 | 10 |
pub const EICRA: *mut u8 = 0x15 as *mut u8;

/// `DIDR0` register
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | AIN1D | 10 |
/// | AIN0D | 1 |
pub const DIDR0: *mut u8 = 0x17 as *mut u8;

/// Analog Comparator Control And Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ACI | 10000 |
/// | ACD | 10000000 |
/// | ACO | 100000 |
/// | ACIE | 1000 |
/// | ACIC | 100 |
/// | ACIS | 11 |
pub const ACSR: *mut u8 = 0x1F as *mut u8;

/// Input Capture Register  Bytes.
pub const ICR0: *mut u16 = 0x22 as *mut u16;

/// Input Capture Register  Bytes low byte.
pub const ICR0L: *mut u8 = 0x22 as *mut u8;

/// Input Capture Register  Bytes high byte.
pub const ICR0H: *mut u8 = 0x23 as *mut u8;

/// Timer/Counter0 Output Compare Register B.
pub const OCR0B: *mut u16 = 0x24 as *mut u16;

/// Timer/Counter0 Output Compare Register B  low byte.
pub const OCR0BL: *mut u8 = 0x24 as *mut u8;

/// Timer/Counter0 Output Compare Register B  high byte.
pub const OCR0BH: *mut u8 = 0x25 as *mut u8;

/// Timer/Counter 0 Output Compare Register A.
pub const OCR0A: *mut u16 = 0x26 as *mut u16;

/// Timer/Counter 0 Output Compare Register A  low byte.
pub const OCR0AL: *mut u8 = 0x26 as *mut u8;

/// Timer/Counter 0 Output Compare Register A  high byte.
pub const OCR0AH: *mut u8 = 0x27 as *mut u8;

/// Timer/Counter0  low byte.
pub const TCNT0L: *mut u8 = 0x28 as *mut u8;

/// Timer/Counter0.
pub const TCNT0: *mut u16 = 0x28 as *mut u16;

/// Timer/Counter0  high byte.
pub const TCNT0H: *mut u8 = 0x29 as *mut u8;

/// Overflow Interrupt Enable.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TOV0 | 1 |
/// | ICF0 | 100000 |
/// | OCF0A | 10 |
/// | OCF0B | 100 |
pub const TIFR0: *mut u8 = 0x2A as *mut u8;

/// Timer Interrupt Mask Register 0.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | OCIE0B | 100 |
/// | ICIE0 | 100000 |
/// | TOIE0 | 1 |
/// | OCIE0A | 10 |
pub const TIMSK0: *mut u8 = 0x2B as *mut u8;

/// Timer/Counter 0 Control Register C.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | FOC0A | 10000000 |
/// | FOC0B | 1000000 |
pub const TCCR0C: *mut u8 = 0x2C as *mut u8;

/// Timer/Counter 0 Control Register B.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | ICNC0 | 10000000 |
/// | CS0 | 111 |
/// | ICES0 | 1000000 |
pub const TCCR0B: *mut u8 = 0x2D as *mut u8;

/// Timer/Counter 0 Control Register A.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | COM0B | 110000 |
/// | COM0A | 11000000 |
pub const TCCR0A: *mut u8 = 0x2E as *mut u8;

/// General Timer/Counter Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | TSM | 10000000 |
/// | PSR | 1 |
pub const GTCCR: *mut u8 = 0x2F as *mut u8;

/// Watchdog Timer Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | WDP | 100111 |
/// | WDIF | 10000000 |
/// | WDIE | 1000000 |
/// | WDE | 1000 |
pub const WDTCSR: *mut u8 = 0x31 as *mut u8;

/// Non-Volatile Memory Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | NVMBSY | 10000000 |
pub const NVMCSR: *mut u8 = 0x32 as *mut u8;

/// Non-Volatile Memory Command.
pub const NVMCMD: *mut u8 = 0x33 as *mut u8;

/// Vcc Level Monitoring Control and Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | VLM | 111 |
/// | VLMF | 10000000 |
/// | VLMIE | 1000000 |
pub const VLMCSR: *mut u8 = 0x34 as *mut u8;

/// Power Reduction Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | PRTIM0 | 1 |
/// | PRADC | 10 |
pub const PRR: *mut u8 = 0x35 as *mut u8;

/// Clock Prescale Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKPS | 1111 |
pub const CLKPSR: *mut u8 = 0x36 as *mut u8;

/// Clock Main Settings Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | CLKMS | 11 |
pub const CLKMSR: *mut u8 = 0x37 as *mut u8;

/// Oscillator Calibration Value.
pub const OSCCAL: *mut u8 = 0x39 as *mut u8;

/// Sleep Mode Control Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | SE | 1 |
/// | SM | 1110 |
pub const SMCR: *mut u8 = 0x3A as *mut u8;

/// Reset Flag Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | EXTRF | 10 |
/// | PORF | 1 |
/// | WDRF | 1000 |
pub const RSTFLR: *mut u8 = 0x3B as *mut u8;

/// Configuration Change Protection.
pub const CCP: *mut u8 = 0x3C as *mut u8;

/// Stack Pointer.
pub const SP: *mut u16 = 0x3D as *mut u16;

/// Stack Pointer  low byte.
pub const SPL: *mut u8 = 0x3D as *mut u8;

/// Stack Pointer  high byte.
pub const SPH: *mut u8 = 0x3E as *mut u8;

/// Status Register.
///
/// Bitfields:
///
/// | Name | Mask (binary) |
/// | ---- | ------------- |
/// | V | 1000 |
/// | C | 1 |
/// | I | 10000000 |
/// | Z | 10 |
/// | T | 1000000 |
/// | S | 10000 |
/// | H | 100000 |
/// | N | 100 |
pub const SREG: *mut u8 = 0x3F as *mut u8;

/// Bitfield on register `ACSR`
pub const ACI: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACD: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACO: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIE: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIC: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `ACSR`
pub const ACIS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `BYTE0`
pub const RSTDISBL: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `BYTE0`
pub const WDTON: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `BYTE0`
pub const CKOUT: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `CLKMSR`
pub const CLKMS: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `CLKPSR`
pub const CLKPS: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `DIDR0`
pub const AIN1D: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `DIDR0`
pub const AIN0D: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC00: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EICRA`
pub const ISC01: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `EIFR`
pub const INTF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `EIMSK`
pub const INT0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `GTCCR`
pub const TSM: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `GTCCR`
pub const PSR: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `LOCKBIT`
pub const LB: *mut u8 = 0x3 as *mut u8;

/// Bitfield on register `NVMCSR`
pub const NVMBSY: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `PCICR`
pub const PCIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCIFR`
pub const PCIF0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PCMSK`
pub const PCINT: *mut u8 = 0xF as *mut u8;

/// Bitfield on register `PORTCR`
pub const BBMB: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `PRR`
pub const PRTIM0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `PRR`
pub const PRADC: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const EXTRF: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const PORF: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `RSTFLR`
pub const WDRF: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SMCR`
pub const SE: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SMCR`
pub const SM: *mut u8 = 0xE as *mut u8;

/// Bitfield on register `SREG`
pub const V: *mut u8 = 0x8 as *mut u8;

/// Bitfield on register `SREG`
pub const C: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `SREG`
pub const I: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `SREG`
pub const Z: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `SREG`
pub const T: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `SREG`
pub const S: *mut u8 = 0x10 as *mut u8;

/// Bitfield on register `SREG`
pub const H: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `SREG`
pub const N: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0B: *mut u8 = 0x30 as *mut u8;

/// Bitfield on register `TCCR0A`
pub const COM0A: *mut u8 = 0xC0 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const ICNC0: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const CS0: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `TCCR0B`
pub const ICES0: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TCCR0C`
pub const FOC0A: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `TCCR0C`
pub const FOC0B: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `TIFR0`
pub const TOV0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIFR0`
pub const ICF0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `TIFR0`
pub const OCF0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0B: *mut u8 = 0x4 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const ICIE0: *mut u8 = 0x20 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const TOIE0: *mut u8 = 0x1 as *mut u8;

/// Bitfield on register `TIMSK0`
pub const OCIE0A: *mut u8 = 0x2 as *mut u8;

/// Bitfield on register `VLMCSR`
pub const VLM: *mut u8 = 0x7 as *mut u8;

/// Bitfield on register `VLMCSR`
pub const VLMF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `VLMCSR`
pub const VLMIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDP: *mut u8 = 0x27 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIF: *mut u8 = 0x80 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDIE: *mut u8 = 0x40 as *mut u8;

/// Bitfield on register `WDTCSR`
pub const WDE: *mut u8 = 0x8 as *mut u8;

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

/// Sleep Mode
#[allow(non_upper_case_globals)]
pub mod cpu_sleep_mode_3bits {
   /// Idle.
   pub const IDLE: u32 = 0x0;
   /// ADC Noise Reduction.
   pub const ADC: u32 = 0x1;
   /// Power Down.
   pub const PDOWN: u32 = 0x2;
   /// Reserved.
   pub const VAL_0x03: u32 = 0x3;
   /// Standby.
   pub const STDBY: u32 = 0x4;
   /// Reserved.
   pub const VAL_0x05: u32 = 0x5;
   /// Reserved.
   pub const VAL_0x06: u32 = 0x6;
   /// Reserved.
   pub const VAL_0x07: u32 = 0x7;
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

