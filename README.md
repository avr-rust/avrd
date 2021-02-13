# avrd

[![Crates.io](https://img.shields.io/crates/v/avrd.svg)](https://crates.io/crates/avrd)
[![Build Status](https://travis-ci.org/avr-rust/avrd.svg?branch=master)](https://travis-ci.org/avr-rust/avrd)
[![license](https://img.shields.io/github/license/avr-rust/avrd.svg)]()

AVR device definitons in Rust.

[Documentation](https://docs.rs/avrd)

This crate exposes information about different AVR microcontrollers so it can be used pragmatically.

You can use this crate directly as a depencency on the AVR bare metal, or on
different architectures.

The constants defined in this library facilitate raw IO port operations on AVR chips.

For example

```rust
extern crate avrd;
use avrd::current::*; // Import constants for the target MCU

volatile_store(PORTB, 0x1f);
```

## Usage on AVR

Just include the crate as a dependency and it will work.

## Enabling all microcontrollers at once

You may want to compile with the `all-mcus` feature enabled, which enables
modules for all microcontrollers to be compiled and included at once.

