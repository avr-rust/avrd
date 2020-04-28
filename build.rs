extern crate avr_mcu;

use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mcus = match cfg!(feature = "all_mcus") {
        true => avr_mcu::microcontrollers().to_owned(),
        false => vec![avr_mcu::current::mcu().expect("no target cpu set")],
    };

    gen::all(&crate_root.join("src").join("gen"), &mcus).unwrap();
}

mod gen {
    use avr_mcu::*;
    use std::collections::{HashMap, HashSet};
    use std::fs::{self, File};
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn all(path: &Path, mcus: &[Mcu]) -> Result<(), io::Error> {
        fs::create_dir_all(path)?;
        let module_names: Vec<String> = mcus
            .iter()
            .map(|mcu| mcu.device.name.to_lowercase())
            .collect();

        // Create modules for each mcu.
        for (mcu, module_name) in mcus.iter().zip(module_names.iter()) {
            let module_path = path.join(format!("{}.rs", module_name));
            eprintln!("Generating module for {}", mcu.device.name);
            generate_mcu_module(mcu, &module_path)?;
        }

        generate_entry_module(path, &module_names)
    }

    /// Generate a `mod.rs` file that binds a list of submodules.
    fn generate_entry_module(output_path: &Path, module_names: &[String]) -> Result<(), io::Error> {
        let mut mod_rs = File::create(output_path.join("mod.rs"))?;

        writeln!(mod_rs, "// Device definitions")?;
        for name in module_names {
            writeln!(mod_rs, "pub mod {};", name)?;
        }
        writeln!(
            mod_rs,
            "\n\
             /// Contains definitions for the current AVR device\n\
             //\n
             // **NOTE**: We are showing the ATmega328 here, even though the library\n\
             // is not targeting a real AVR device. If you compile this library for\n\
             // a specific AVR MCU, the module for that device will aliased here.\n\
             // If we are targeting a non-AVR device, just pick the ATmega328p so\n\
             // that users can see what the API would look like\n\
             //\n\
             // Note that we reexport rather than alias so that we can add a note about\n\
             // this behaviour to the documentation.\n\
             #[cfg(not(target_arch = \"avr\"))]\n\
             pub mod current {{ pub use super::atmega328::*; }}\n\
             \n\
             /// Contains definitions for the current AVR device\n\
             // If we are targeting AVR, lookup the current device's module\n\
             // and alias it to the `current` module.\n\
             #[cfg(target_arch = \"avr\")]\n\
             pub mod current {{\n    \
             // NOTE: 'target_cpu' is a cfg flag specific to the avr-rust fork"
        )?;
        for name in module_names {
            writeln!(
                mod_rs,
                "    #[cfg(target_cpu = \"{}\")] pub use super::{} as current;",
                name, name
            )?;
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

    pub fn mcu_module_doc(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
        writeln!(w, "//! The AVR {} microcontroller", mcu.device.name)?;
        writeln!(w,
            "//!\n\
             //! # Variants\n\
             //! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |\n\
             //! |--------|--------|---------|-----------------------|-------------------|-----------|"
        )?;
        for variant in &mcu.variants {
            let pinout_label = variant
                .pinout
                .as_ref()
                .map(|p| p.replace('_', "-").to_owned())
                .unwrap_or_default();
            let speed_mhz = variant.speed_max_hz / 1_000_000;
            writeln!(
                w,
                "//! | {} | {} | {} | {}°C - {}°C | {}V - {}V | {} MHz |",
                variant.name,
                pinout_label,
                variant.package,
                variant.temperature_min,
                variant.temperature_max,
                variant.voltage_min,
                variant.voltage_max,
                speed_mhz
            )?;
        }
        writeln!(w, "//!")?;

        Ok(())
    }

    pub fn mcu_module_code(mcu: &Mcu, w: &mut impl Write) -> Result<(), io::Error> {
        let registers = ordered_registers(mcu);
        let register_bitfields = documentable_bitfields(&registers);

        writeln!(w, "#![allow(non_upper_case_globals)]\n")?;
        for register in &registers {
            let ty = integer_type(register.size);

            match !register.caption.is_empty() {
                true => writeln!(w, "/// {}", format_caption(&register.caption))?,
                false => writeln!(w, "/// `{}` register", register.name)?,
            }

            let mut bitfields = register_bitfields
                .iter()
                .filter(|(reg, _)| *reg == register)
                .map(|(_, bitfield)| bitfield)
                .peekable();

            if bitfields.peek().is_some() {
                writeln!(
                    w,
                    "///\n\
                     /// Bitfields:\n\
                     ///\n\
                     /// | Name | Mask (binary) |\n\
                     /// | ---- | ------------- |"
                )?;
                for bitfield in bitfields {
                    writeln!(w, "/// | {} | {:b} |", bitfield.name, bitfield.mask)?;
                }
            }

            writeln!(
                w,
                "pub const {}: *mut {} = {:#X} as *mut {};",
                register.name, ty, register.offset, ty
            )?;
            writeln!(w)?;
        }

        for (register, bitfield) in register_bitfields {
            let ty = integer_type(bitfield.size);

            writeln!(w, "/// Bitfield on register `{}`", register.name)?;
            writeln!(
                w,
                "pub const {}: *mut {} = {:#X} as *mut {};",
                bitfield.name, ty, bitfield.mask, ty
            )?;
            writeln!(w)?;
        }

        for value_group in ordered_value_groups(&mcu) {
            match !value_group.caption.is_empty() {
                true => writeln!(w, "/// {}", value_group.caption)?,
                false => writeln!(w, "/// `{}` value group", value_group.name)?,
            };
            writeln!(w, "#[allow(non_upper_case_globals)]")?;
            writeln!(w, "pub mod {} {{", value_group.name.to_lowercase())?;
            for val in &unique_value_group_values(&value_group) {
                let first_char = val.name.chars().next().expect("empty value name");
                let name = match !first_char.is_alphabetic() && first_char != '_' {
                    true => format!("_{}", val.name),
                    false => val.name.to_owned(),
                };

                if !val.caption.is_empty() {
                    let ty = "u32";
                    writeln!(w, "   /// {}", format_caption(&val.caption))?;
                    writeln!(w, "   pub const {}: {} = {:#X};", name, ty, val.value)?;
                }
            }
            writeln!(w, "}}")?;
            writeln!(w)?;
        }

        Ok(())
    }

    fn format_caption(caption: &str) -> String {
        // Escape special characters and trim
        let mut result = caption
            .replace("[", "\\[")
            .replace("]", "\\]")
            .trim()
            .to_owned();
        if !result.ends_with('.') {
            result.push('.')
        }
        result
    }

    fn ordered_registers(mcu: &Mcu) -> Vec<Register> {
        let mut unique_registers = self::unique_registers(mcu);
        insert_high_low_variants(&mut unique_registers);
        let mut registers: Vec<Register> = unique_registers.into_iter().map(|a| a.1).collect();
        registers.sort_by_key(|r| r.offset);
        registers
    }

    fn ordered_value_groups(mcu: &Mcu) -> Vec<ValueGroup> {
        let mut all_value_groups = Vec::new();
        for module in &mcu.modules {
            all_value_groups.extend(module.value_groups.iter().cloned());
        }
        let mut all_value_groups = unique_value_groups(&all_value_groups);
        all_value_groups.sort_by(|vg1, vg2| vg1.partial_cmp(vg2).unwrap());
        all_value_groups
    }

    fn insert_high_low_variants(registers: &mut HashMap<String, Register>) {
        let wide_registers: Vec<_> = registers
            .values()
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
            Register {
                name: r.name.clone() + "H",
                caption: r.caption.clone() + " high byte",
                offset: r.offset + 1,
                size: r.size / 2,
                mask: None,
                bitfields: Vec::new(), // these are already in parent.
                rw: r.rw.clone(),
            },
            Register {
                name: r.name.clone() + "L",
                caption: r.caption.clone() + " low byte",
                offset: r.offset + 0,
                size: r.size / 2,
                mask: None,
                bitfields: Vec::new(), // these are already in parent.
                rw: r.rw.clone(),
            },
        )
    }

    fn unique_registers(mcu: &Mcu) -> HashMap<String, Register> {
        let mut result = HashMap::new();

        for module in &mcu.modules {
            for register_group in &module.register_groups {
                for register in &register_group.registers {
                    // Check if we've already seen this register.
                    // Remove it if so and combine it with the current Register.
                    let r: Register = match result.remove(&register.name) {
                        Some(ref existing) => register.union(existing),
                        None => register.clone(),
                    };
                    result.insert(r.name.clone(), r);
                }
            }
        }

        result
    }

    fn unique_value_groups(value_groups: &[ValueGroup]) -> Vec<ValueGroup> {
        let mut value_groups = value_groups.to_owned();
        loop {
            let mut remove_idx = None;
            for (idx, value_group) in value_groups.iter().enumerate() {
                if value_groups
                    .iter()
                    .filter(|vg| vg.name == value_group.name)
                    .inspect(|vg| assert_eq!(*vg, value_group))
                    .count()
                    > 1
                {
                    remove_idx = Some(idx);
                }
            }
            match remove_idx {
                Some(idx) => value_groups.remove(idx),
                None => break,
            };
        }
        value_groups
    }

    /// Collect al the values in a ValueGroup and deduplicate the,/
    ///
    /// As in the bitfield descriptions, the value-group decriptions contain
    /// values with duplicate names. We will skip every value which is
    /// ambiguous. See `documentable_bitfields` for more context on this
    /// issue.
    fn unique_value_group_values(value_group: &ValueGroup) -> Vec<Value> {
        let mut values = value_group.values.clone();
        let mut remove_names = HashSet::new();
        for value in &values {
            if values.iter().filter(|v| v.name == value.name).count() > 1 {
                remove_names.insert(value.name.to_owned());
            }
        }
        for name in remove_names {
            values.retain(|v| v.name != name);
        }
        values
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

        for register in registers {
            for bitfield in &register.bitfields {
                let bitfields = history.entry(&bitfield.name).or_insert(Vec::new());
                // Track the bitfield of this bitfield.
                bitfields.push((register, bitfield));
            }
        }

        // Convert the hash map to a list and sort it so it is deterministic.
        let mut register_bitfields: Vec<_> = history
            .into_iter()
            .map(|(_, register_bitfields)| register_bitfields)
            .collect();
        register_bitfields.sort_by_key(|register_bitfields| &register_bitfields[0].0.name);

        let unique_bitfields = register_bitfields
            .into_iter()
            .filter(|bitfields| bitfields.len() == 1)
            .map(|bitfields| bitfields.into_iter().next().unwrap());

        // Skip bitmasks that cover all bits or share the same name as their parent register.
        // There are strange cases like this in the packfiles.
        let bitfields = unique_bitfields.filter(|&(register, bitfield)| {
            let full_mask = match register.size {
                1 => 0xff,
                2 => 0xffff,
                _ => panic!("register is too large"),
            };

            (bitfield.mask != full_mask)
                && (bitfield.name != register.name)
                && !register_names.contains(&bitfield.name[..])
        });

        bitfields.collect()
    }
}
