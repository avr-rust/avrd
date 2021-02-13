//! Constants and definitions for AVR microcontrollers
//!
//! # Library structure
//!
//! This library contains definitions for every 8-bit AVR microcontroller.
//!
//! The information for each device is separated into submodules, named after
//! the microcontroller itself.
//!
//! The [`current`][crate::current] module is an alias to whatever microcontroller
//! is currently being targeted.
//!
//! # Enabling all microcontrollers at once
//!
//! You may want to compile with the `all-mcus` feature enabled, which enables
//! modules for all microcontrollers to be compiled and included at once.

#![no_std]

pub use self::gen::*;

mod gen;

