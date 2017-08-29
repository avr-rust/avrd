extern crate avr_mcu;
use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mcus = if cfg!(feature = "all_mcus") {
        avr_mcu::microcontrollers().to_owned()
    } else {
        let current_mcu = avr_mcu::current::mcu()
            .expect("no target cpu set");
        vec![current_mcu]
    };

    gen::all(&crate_root.join("src").join("gen"), &mcus).unwrap();
}

mod gen {
    use avr_mcu::*;
    use std::collections::HashMap;
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
        self::mcu_registers(mcu, &mut file)?;

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

        writeln!(w, "//! # Registers by module (not exhaustive)")?;

        for module in modules_by_relevance(mcu.device.modules.clone()) {
            writeln!(w, "//!")?;
            writeln!(w, "//! ## {} modules",
                     module.name)?;
            writeln!(w, "//!")?;

            for instance in module.instances.iter() {
                writeln!(w, "//! * {}",
                         instance.name)?;

                for signal in instance.signals.iter() {
                    if let Some(ref group) = signal.group {
                        writeln!(w, "//!     * {} ({})", group, signal.pad)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn modules_by_relevance(mut modules: Vec<Module>) -> Vec<Module> {
        modules = modules.into_iter().filter(should_document_module).collect();
        modules.sort_by_key(|m| match &m.name[..] {
            "PORT" => 1,
            _ => 2,
        });
        modules
    }

    fn should_document_module(module: &Module) -> bool {
        match &module.name[..] {
            "AC" => false,
            "ADC" => true, // Analog to digital converters.
            "AD_CONVERTER" => false,
            "AES" => false,
            "AFE" => false,
            "ANALOG_COMPARATOR" => false,
            "AWEX" => false,
            "BANDGAP" => false,
            "BATTERY_PROTECTION" => false,
            "BOD" => false,
            "BOOT_LOAD" => false,
            "CALIB" => false,
            "CAN" => false,
            "CCL" => false,
            "CELL_BALANCING" => false,
            "CFD" => false,
            "CLKCTRL" => false,
            "CHARGER_DETECT" => false,
            "CHFLT" => false,
            "CLK" => false,
            "COULOMB_COUNTER" => false,
            "CPU" => false,
            "CPUINT" => false,
            "CRC" => false,
            "CRCSCAN" => false,
            "CURRENT_SOURCE" => false,
            "DAC" => false,
            "DDDLFRX" => false,
            "DEBOUNCE" => false,
            "DEBUG" => false,
            "DEMOD" => false,
            "DEVICEID" => false,
            "DFIFO" => false,
            "DFLL" => false,
            "DMA" => false,
            "EBI" => false,
            "EDMA" => false,
            "EEPROM" => true, // EEPROM information
            "EUSART" => false,
            "EVSYS" => false,
            "EXINT" => false,
            "EXTERNAL_INTERRUPT" => false,
            "FAULT" => false,
            "FE" => false,
            "FET" => false,
            "FLASH" => true, // Flash information
            "FREQS" => false,
            "FRSYNC" => false,
            "FUSE" => false,
            "GPIO" => false, // General purpose IOs
            "GPIOREGS" => false,
            "GPIOREGS_DVCC" => false,
            "GPIOREGS_LFVCC" => false,
            "HIRES" => false,
            "IDCHK" => false,
            "IDSCAN" => false,
            "INT" => true, // Unsure what this means, sounds relevant
            "IRCOM" => false,
            "IRLED" => false,
            "JTAG" => true, // JTAG information.
            "LCD" => false,
            "LED" => false,
            "LFRX" => false,
            "LF_FIFO" => false,
            "LF_PROTOCOL_HANDLER" => false,
            "LF_RECEIVER" => false,
            "LF_RSSI" => false,
            "LF_TIMER" => false,
            "LF_TRANSPONDER" => false,
            "LINUART" => false,
            "LIN_UART" => false,
            "LOCKBIT" => false,
            "MCU" => false,
            "MEM" => false,
            "MISC" => false,
            "MOD" => false,
            "NVM" => false,
            "NVMCTRL" => false,
            "OCCOUNT" => false,
            "OSC" => false,
            "PLL" => false,
            "PMIC" => false,
            "PORT" => true, // Port information
            "PORTA" => false,
            "PORTB" => false,
            "PORTC" => false,
            "PORTCFG" => false,
            "PORTD" => false,
            "PORTMUX" => false,
            "PORTS" => false,
            "PR" => false,
            "PS2" => false,
            "PSC" => false,
            "PTC" => false,
            "PWRCTRL" => false,
            "RSSIB" => false,
            "RST" => false,
            "RSTCTRL" => false,
            "RTC" => false,
            "RTC32" => false,
            "RTC_TIMER" => false,
            "RXBUF" => false,
            "RXDSP" => false,
            "SENSOR_INTERFACE" => false,
            "SFIFO" => false,
            "SIGROW" => false,
            "SLEEP" => false,
            "SLPCTRL" => false,
            "SPI" => false, // SPI information
            "SPI2" => false,
            "SSM" => false,
            "SUP" => false,
            "SYMCH" => false,
            "SYMCNT" => false,
            "SYSCFG" => false,
            "TC" => false,
            "TC10" => false,
            "TC16" => false,
            "TC2" => false,
            "TC8" => false,
            "TC8_ASYNC" => false,
            "TCA" => false,
            "TCB" => false,
            "TCD" => false,
            "TEMPER" => false,
            "TIMER0_WDT" => false,
            "TIMER1" => false,
            "TIMER2" => false,
            "TIMER3" => false,
            "TIMER4" => false,
            "TIMER5" => false,
            "TIMER_COUNTER_0" => false,
            "TIMER_COUNTER_1" => false,
            "TIMER_COUNTER_2" => false,
            "TIMER_COUNTER_3" => false,
            "TMO" => false,
            "TOCPM" => false,
            "TPLF_CAL" => false,
            "TRACE" => false,
            "TRX24" => false,
            "TWI" => false,
            "TWI1" => false,
            "TWI2" => false,
            "TXDSP" => false,
            "TXM" => false,
            "USART" => true, // Serial
            "USART0" => false,
            "USB" => true, // Universal serial bus
            "USB_DEVICE" => false,
            "USB_GLOBAL" => false,
            "USB_HOST" => false,
            "USERROW" => false,
            "USI" => false,
            "VBAT" => false,
            "VMON" => false,
            "VOLTAGE_REGULATOR" => false,
            "VPORT" => false,
            "VREF" => false,
            "VX_MODE" => false,
            "WAKEUP_TIMER" => false,
            "WDT" => false,
            "WEX" => false,
            "XCL" => false,
            "XOCD" => false,
            _ => panic!("unknown module type: '{}'", module.name),
        }
    }

    pub fn mcu_registers(mcu: &Mcu, w: &mut Write)
        -> Result<(), io::Error>  {
        for register in ordered_registers(mcu) {
            let ty = if register.size == 1 { "u8" } else { "u16" };

            if !register.caption.is_empty() {
                let mut caption = register.caption.trim().to_owned();
                if !caption.ends_with('.') { caption.push('.') }

                writeln!(w, "/// {}", caption).unwrap();
            }
            writeln!(w, "pub const {}: *mut {} = {:#X} as *mut {};",
                     register.name, ty, register.offset, ty)?;
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
                       mask: None },
            Register { name: r.name.clone() + "L",
                       caption: r.caption.clone() + " low byte",
                       offset: r.offset + 0,
                       size: r.size / 2,
                       mask: None },
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
}

