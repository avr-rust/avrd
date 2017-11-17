extern crate avr_mcu;

use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mcus = if cfg!(feature = "all_mcus") {
        avr_mcu::microcontrollers().to_owned()
    } else {
        // By default, when compiling for AVR we should hard error if
        // microcontroller is not specified.
        if cfg!(arch = "avr") {
            let current_mcu = avr_mcu::current::mcu()
                .expect("no target cpu set");
            vec![current_mcu]
        } else {
            // On non-avr architectures, support all microcontrollers.
            avr_mcu::microcontrollers().to_owned()
        }
    };

    // Useful for test
    // let mcus = vec![
    //     avr_mcu::microcontroller("ata5795").clone(),
    //     avr_mcu::microcontroller("atmega328").clone(), // required when compiling for PC
    // ];

    gen::all(&crate_root.join("src").join("gen"), &mcus).unwrap();
}

mod gen {
    use avr_mcu::*;
    use std::collections::{HashMap, HashSet};
    use std::fs::{self, File};
    use std::io::prelude::*;
    use std::io;
    use std::path::Path;

    pub fn all(path: &Path, mcus: &[Mcu]) -> Result<(), io::Error> {
        fs::create_dir_all(path)?;

        let mut module_names = Vec::new();

        // Create modules for each mcu.
        for mcu in mcus.iter() {
            let module_name = self::mcu_module_name(mcu);
            let module_path = path.join(format!("{}.rs", module_name));

            eprintln!("generating module for {}", mcu.device.name);
            generate_mcu_module(mcu, &module_path)?;
            module_names.push(module_name);
        }

        generate_entry_module(path, &module_names)
    }

    /// Generate a `mod.rs` file that binds a list of submodules.
    fn generate_entry_module(output_path: &Path, module_names: &[String]) -> Result<(), io::Error> {
        let mut mod_rs = File::create(output_path.join("mod.rs"))?;

        writeln!(mod_rs, "// Device definitions")?;
        for module_name in module_names {
            writeln!(mod_rs, "pub mod {};", module_name)?;
        }
        writeln!(mod_rs)?;

        const CURRENT_MOD_SUMMARY: &'static str = "Contains definitions for the current AVR device";

        writeln!(mod_rs, "/// {}", CURRENT_MOD_SUMMARY)?;
        writeln!(mod_rs, "///")?;
        writeln!(mod_rs, "/// **NOTE**: We are showing the ATmega328 here, even though the library")?;
        writeln!(mod_rs, "/// is not targeting a real AVR device. If you compile this library for")?;
        writeln!(mod_rs, "/// a specific AVR MCU, the module for that device will aliased here.")?;
        writeln!(mod_rs, "// If we are targeting a non-AVR device, just pick the ATmega328p so")?;
        writeln!(mod_rs, "// that users can see what the API would look like")?;
        writeln!(mod_rs, "//")?;
        writeln!(mod_rs, "// Note that we reexport rather than alias so that we can add a note about")?;
        writeln!(mod_rs, "// this behaviour to the documentation.")?;
        writeln!(mod_rs, "#[cfg(not(target_arch = \"avr\"))]")?;
        writeln!(mod_rs, "pub mod current {{ pub use super::atmega328::*; }}")?;
        writeln!(mod_rs)?;
        writeln!(mod_rs, "/// {}", CURRENT_MOD_SUMMARY)?;
        writeln!(mod_rs, "// If we are targeting AVR, lookup the current device's module")?;
        writeln!(mod_rs, "// and alias it to the `current` module.")?;
        writeln!(mod_rs, "#[cfg(target_arch = \"avr\")]")?;
        writeln!(mod_rs, "pub mod current {{")?;
        writeln!(mod_rs, "    // NOTE: 'target_cpu' is a cfg flag specific to the avr-rust fork")?;
        for module_name in module_names {
            writeln!(mod_rs, "    #[cfg(target_cpu = \"{}\")] pub use super::{} as current;",
                     module_name, module_name)?;
        }
        writeln!(mod_rs, "}}")?;

        Ok(())
    }

    /// Generates a self-contained module for each individual  mcu.
    fn generate_mcu_module(mcu: &Mcu, path: &Path) -> Result<(), io::Error> {
        let mut file = File::create(path)?;

        self::mcu_module_doc(mcu, &mut file)?;
        writeln!(file)?;
        self::mcu_module_code(mcu, &mut file)?;

        Ok(())
    }

    /// Gets the module name for a mcu.
    fn mcu_module_name(mcu: &Mcu) -> String {
        mcu.device.name.to_lowercase()
    }

    pub fn mcu_module_doc(mcu: &Mcu, w: &mut Write)
        -> Result<(), io::Error> {
        writeln!(w, "//! The AVR {} microcontroller", mcu.device.name)?;
        writeln!(w, "//!")?;

        writeln!(w, "//! # Variants")?;
        writeln!(w, "//! |        | Pinout | Mcuage | Operating temperature | Operating voltage | Max speed |")?;
        writeln!(w, "//! |--------|--------|---------|-----------------------|-------------------|-----------|")?;
        for variant in mcu.variants.iter() {
            let speed_mhz = variant.speed_max_hz / 1_000_000;
            writeln!(w, "//! | {} | {} | {} | {}°C - {}°C | {}V - {}V | {} MHz |",
                     variant.name, variant.pinout.clone().unwrap_or_else(|| String::new()),
                     variant.package, variant.temperature_min,
                     variant.temperature_max, variant.voltage_min, variant.voltage_max,
                     speed_mhz)?;
        }
        writeln!(w, "//!")?;

        Ok(())
    }

    pub fn mcu_module_code(mcu: &Mcu, w: &mut Write)
        -> Result<(), io::Error>  {
        let registers = ordered_registers(mcu);
        let register_bitfields = documentable_bitfields(&registers);

        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w)?;

        for register in registers.iter() {
            let ty = integer_type(register.size);

            if !register.caption.is_empty() {
                let mut caption = register.caption.trim().to_owned();
                if !caption.ends_with('.') { caption.push('.') }

                writeln!(w, "/// {}", caption)?;
            } else {
                writeln!(w, "/// {} register", register.name)?;
            }

            let mut bitfields = register_bitfields.iter().filter_map(|&(reg,bitfield)| {
                if reg == register { Some(bitfield) } else { None }
            }).peekable();

            if bitfields.peek().is_some() {
                writeln!(w, "///")?;
                writeln!(w, "/// Bitfields:")?;
                writeln!(w, "///")?;
                writeln!(w, "/// | Name | Mask (binary) |")?;
                writeln!(w, "/// | ---- | ------------- |")?;
                while let Some(bitfield) = bitfields.next() {
                    writeln!(w, "/// | {} | {:b} |", bitfield.name, bitfield.mask)?;
                }
            }

            writeln!(w, "pub const {}: *mut {} = {:#X} as *mut {};",
                     register.name, ty, register.offset, ty)?;
            writeln!(w)?;
        }

        for (register, bitfield) in register_bitfields {
            let ty = integer_type(bitfield.size);

            writeln!(w, "/// Bitfield on register {}", register.name)?;
            writeln!(w, "pub const {}: *mut {} = {:#X} as *mut {};",
                     bitfield.name, ty, bitfield.mask, ty)?;
            writeln!(w)?;

        }

        Ok(())
    }

    fn ordered_registers(mcu: &Mcu) -> Vec<Register> {
        let mut unique_registers = self::unique_registers(mcu);
        insert_high_low_variants(&mut unique_registers);

        let mut registers: Vec<Register> = unique_registers.into_iter().map(|a| a.1).collect();
        registers.sort_by_key(|r| r.offset);

        registers
    }

    fn insert_high_low_variants(registers: &mut HashMap<String, Register>) {
        let wide_registers: Vec<_> = registers.values()
                                        .filter(|r| r.size == 2)
                                        .cloned()
                                        .collect();

        for r in wide_registers {
            let (high, low) = high_low_variants(&r);

            if !registers.contains_key(&high.name) {
                registers.insert(high.name.clone(), high);
            }
            if !registers.contains_key(&low.name) {
                registers.insert(low.name.clone(), low);
            }
        }
    }

    fn high_low_variants(r: &Register) -> (Register, Register) {
        assert_eq!(2, r.size, "only 16-bit registers have high low variants");

        (
            Register { name: r.name.clone() + "H",
                       caption: r.caption.clone() + " high byte",
                       offset: r.offset + 1,
                       size: r.size / 2,
                       mask: None,
                       bitfields: Vec::new(), // these are already in parent.
                       rw: r.rw.clone() },
            Register { name: r.name.clone() + "L",
                       caption: r.caption.clone() + " low byte",
                       offset: r.offset + 0,
                       size: r.size / 2,
                       mask: None,
                       bitfields: Vec::new(), // these are already in parent.
                       rw: r.rw.clone() },
        )
    }

    fn unique_registers(mcu: &Mcu) -> HashMap<String, Register> {
        let mut result = HashMap::new();

        for module in mcu.modules.iter() {
            for register_group in module.register_groups.iter() {
                for register in register_group.registers.iter() {
                    // Check if we've already seen this register.
                    // Remove it if so and combine it with the current Register.
                    let r: Register = if let Some(ref existing) = result.remove(&register.name) {
                        register.union(existing)
                    } else {
                        register.clone()
                    };

                    result.insert(r.name.clone(), r);
                }
            }
        }

        result
    }

    /// Gets the integer type of a specified width.
    fn integer_type(byte_count: u32) -> &'static str {
        match byte_count {
            1 => "u8",
            2 => "u16",
            4 => "u32",
            _ => panic!("failed to get type of {}", byte_count),
        }
    }

    /// Collects all bitfields for a set of registers that are documentable.
    ///
    /// Some pack files (such as the one for atmega328p) do not have unique
    /// bitfield names.
    ///
    /// For example, all of the timer control registers (`TCCR0A`, `TCCR1B`, ..)
    /// on the ATmega328p individually define bitfields named `WGM1` with
    /// different masks but a shared name. Note that even though these fields
    /// are in the packfiles, they do not appear in AVR-GCC's `iom328p.h`.
    ///
    /// This function specifically considers bitfields with unique
    /// names as documentable.
    ///
    /// In the case where we have an ambiguous name for a bitfield, it
    /// should be skipped.
    fn documentable_bitfields(registers: &[Register]) -> Vec<(&Register, &Bitfield)> {
        let register_names: HashSet<&str> = registers.iter().map(|r| &r.name[..]).collect();

        // A hash map of bitfield names to possible instantiations.
        let mut history: HashMap<&str, Vec<(&Register, &Bitfield)>> = HashMap::new();

        for register in registers.iter(){
            for bitfield in register.bitfields.iter() {
                let bitfields = history.entry(&bitfield.name).
                    or_insert_with(|| Vec::new());
                // Track the bitfield of this bitfield.
                bitfields.push((register, bitfield));
            }
        }

        // Convert the hash map to a list and sort it so it is deterministic.
        let mut register_bitfields: Vec<_> = history.into_iter().map(|(_, register_bitfields)| register_bitfields).collect();
        register_bitfields.sort_by_key(|register_bitfields| &register_bitfields[0].0.name);

        let unique_bitfields = register_bitfields.into_iter().filter_map(|register_bitfields| {
            if register_bitfields.len() == 1 {
                Some(register_bitfields.into_iter().next().unwrap())
            } else {
                None
            }
        });

        // Skip bitmasks that cover all bits or share the same name as their parent register.
        // There are strange cases like this in the packfiles.
        let bitfields = unique_bitfields.filter(|&(register, bitfield)| {
            let full_mask = match register.size {
                1 => 0xff,
                2 => 0xffff,
                _ => panic!("register is too large"),
            };

            bitfield.mask != full_mask &&
                bitfield.name != register.name &&
                !register_names.contains(&bitfield.name[..])
        });

        bitfields.collect()
    }
}
