//! Constants and definitions for AVR microcontrollers
//!
//! # Library structure
//!
//! This library contains definitions for every 8-bit AVR microcontroller.
//!
//! The information for each device is separated into submodules, named after
//! the microcontroller itself.

#![no_std]

pub use self::gen::*;

mod gen;

