extern crate avr_mcu;

use clap::{App, Arg};

use std::path::{Path, PathBuf};

const DEFAULT_MCU_FOR_NON_AVR: &'static str = "atmega328";

#[derive(Clone, Debug)]
struct Config {
    output_directory: PathBuf,
    mcus: Vec<avr_mcu::Mcu>,
}

fn get_cli_config() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version("1.0")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("out-dir")
            .short("o")
            .long("out-dir")
            .value_name("DIRECTORY")
            .help("Sets the directory path that will contain the newly generated Rust source code files")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("mcus")
            .short("m")
            .long("mcus")
            .value_name("COMMA SEPARATED MCU LIST")
            .help("Generate outputs for a specific microcontroller or microcontrollers rather than the default of all")
            .takes_value(true))
            .get_matches();
    let output_directory = Path::new(matches.value_of("out-dir").unwrap()).to_owned();

    let mcus = matches.values_of("mcus")
        .map(|m| m.flat_map(|a| a.split(",").map(|s| avr_mcu::microcontroller(s).clone())).collect())
        .unwrap_or_else(|| avr_mcu::microcontrollers().to_owned() );

    Config { output_directory, mcus }
}

fn main() {
    let config = get_cli_config();

    gen::all(&config.output_directory, &config.mcus).unwrap();
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

        const CURRENT_MOD_SUMMARY: &'static str = "Contains definitions for the current AVR device being targeted.";

        writeln!(mod_rs, "// Device definitions")?;
        writeln!(mod_rs)?;
        for module_name in module_names {
            if module_name == crate::DEFAULT_MCU_FOR_NON_AVR {
                writeln!(mod_rs, "/// The module containing the values for the '{}' microcontroller", module_name)?;
                writeln!(mod_rs, "///")?;
                writeln!(mod_rs, "/// This is the default MCU when targeting a non-AVR target.")?;

                writeln!(mod_rs, "#[cfg(any(not(target_arch = \"avr\"), avr_mcu_{}, feature = \"all-mcus\"))] pub mod {};", module_name, module_name)?;

                writeln!(mod_rs, "/// {} **The '{}' is the default when targeting non-AVR devices.**", CURRENT_MOD_SUMMARY, module_name)?;

                writeln!(mod_rs, "#[cfg(not(target_arch = \"avr\"))] pub mod current {{ pub use super::{}::*; }}", module_name)?;

                writeln!(mod_rs, "/// {} **This is currently the '{}'**.", CURRENT_MOD_SUMMARY, module_name)?;
                writeln!(mod_rs, "#[cfg(all(target_arch = \"avr\", avr_mcu_{}))] pub mod current {{ pub use super::{}::*; }}",
                         module_name, module_name)?;
            } else {
                writeln!(mod_rs, "/// The module containing the values for the '{}' microcontroller", module_name)?;
                writeln!(mod_rs, "#[cfg(any(avr_mcu_{}, feature = \"all-mcus\"))] pub mod {};", module_name, module_name)?;
                writeln!(mod_rs, "/// {} **This is currently the '{}'**.", CURRENT_MOD_SUMMARY, module_name)?;
                writeln!(mod_rs, "#[cfg(all(target_arch = \"avr\", avr_mcu_{}))] pub mod current {{ pub use super::{}::*; }}",
                         module_name, module_name)?;
            }
            writeln!(mod_rs)?;
        }
        writeln!(mod_rs)?;

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

    pub fn mcu_module_doc(mcu: &Mcu, w: &mut dyn Write)
        -> Result<(), io::Error> {
        writeln!(w, "//! The AVR {} microcontroller", mcu.device.name)?;
        writeln!(w, "//!")?;

        writeln!(w, "//! # Variants")?;
        writeln!(w, "//! |        | Pinout | Mcu age | Operating temperature | Operating voltage | Max speed |")?;
        writeln!(w, "//! |--------|--------|---------|-----------------------|-------------------|-----------|")?;
        for variant in mcu.variants.iter() {
            let pinout_label = variant.pinout.as_ref().map(|p| p.replace('_', "-").to_owned()).unwrap_or_else(|| String::new());
            let speed_mhz = variant.speed_max_hz / 1_000_000;
            writeln!(w, "//! | {} | {} | {} | {}°C - {}°C | {}V - {}V | {} MHz |",
                     variant.name, pinout_label,
                     variant.package, variant.temperature_min,
                     variant.temperature_max, variant.voltage_min, variant.voltage_max,
                     speed_mhz)?;
        }
        writeln!(w, "//!")?;

        Ok(())
    }

    pub fn mcu_module_code(mcu: &Mcu, w: &mut dyn Write)
        -> Result<(), io::Error>  {
        let registers = ordered_registers(mcu);
        let register_bitfields = documentable_bitfields(&registers);

        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w)?;

        for register in registers.iter() {
            let ty = integer_type(register.size);

            if !register.caption.is_empty() {
                writeln!(w, "/// {}", format_caption(&register.caption))?;
            } else {
                writeln!(w, "/// `{}` register", register.name)?;
            }

            let mut bitfields: Vec<&'_ Bitfield> = register_bitfields.iter().filter_map(|&(reg,bitfield)| {
                if reg == register { Some(bitfield) } else { None }
            }).collect();
            bitfields.sort_by_key(|b| b.mask);

            if bitfields.len() > 0 {
                writeln!(w, "///")?;
                writeln!(w, "/// Bitfields:")?;
                writeln!(w, "///")?;
                writeln!(w, "/// | Name | Mask (binary) |")?;
                writeln!(w, "/// | ---- | ------------- |")?;
                for bitfield in bitfields {
                    writeln!(w, "/// | {} | {:b} |", bitfield.name, bitfield.mask)?;
                }
            }

            writeln!(w, "pub const {}: *mut {} = {:#X} as *mut {};",
                     register.name, ty, register.offset, ty)?;
            writeln!(w)?;
        }

        for (register, bitfield) in register_bitfields {
            let ty = integer_type(bitfield.size);

            writeln!(w, "/// Bitfield on register `{}`", register.name)?;
            writeln!(w, "pub const {}: *mut {} = {:#X} as *mut {};",
                     bitfield.name, ty, bitfield.mask, ty)?;
            writeln!(w)?;

        }

        for value_group in ordered_value_groups(&mcu) {
            if !value_group.caption.is_empty() {
                writeln!(w, "/// {}", value_group.caption)?;
            } else {
                writeln!(w, "/// `{}` value group", value_group.name)?;
            }
            writeln!(w, "#[allow(non_upper_case_globals)]")? ;
            writeln!(w, "pub mod {} {{", value_group.name.to_lowercase())?;
            for val in unique_value_group_values(&value_group).iter() {
                let first_char = val.name.chars().next().expect("empty value name");
                let name = if !first_char.is_alphabetic() && first_char != '_' {
                    format!("_{}", val.name)
                } else {
                    val.name.to_owned()
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
        let mut result = caption.to_owned();
        // Escape special characters in markdown
        result = result.replace("[", "\\[").replace("]", "\\]");

        // Trim and make a sentence
        result = result.trim().to_owned();
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
        let mut value_groups = Vec::new();
        for module in mcu.modules.iter() {
            for value_group in module.value_groups.iter() {
                value_groups.push(value_group.clone());
            }
        }
        value_groups = unique_value_groups(&value_groups);
        value_groups.sort_by(|vg1, vg2| vg1.partial_cmp(vg2).unwrap());

        value_groups
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

    fn unique_value_groups(value_groups: &[ValueGroup]) -> Vec<ValueGroup> {
        let mut value_groups = value_groups.to_owned();
        loop {
            let mut remove_idx = None;
            for (idx, value_group) in value_groups.iter().enumerate() {
                if value_groups.iter().filter(|vg| {
                    if vg.name == value_group.name {
                        assert_eq!(*vg, value_group);
                        true
                    } else {
                        false
                    }
                }).count() > 1 {
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
        for value in values.iter() {
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
