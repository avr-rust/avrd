//! Constants and definitions for AVR microcontrollers
//!
//! # Library structure
//!
//! This library contains definitions for every 8-bit AVR microcontroller.
//!
//! The information for each device is separated into submodules, named after
//! the microcontroller itself.

pub use self::gen::*;

mod gen;

