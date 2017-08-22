extern crate xmltree;

/// The extension on the pack files.
const PACK_FILE_EXT: &'static str = "atdf";

const PACKS: &'static [&'static str] = &[
    "atmega", "tiny", "xmegaa", "xmegab",
    "xmegac", "xmegad", "xmegae", "automotive",
];

use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Pack {
    pub device: Device,
    pub variants: Vec<Variant>,
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    pub address_spaces: Vec<AddressSpace>,
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: String,
    pub pinout: Option<String>,
    pub package: String,
    pub temperature_min: i32,
    pub temperature_max: i32,
    pub voltage_min: f32,
    pub voltage_max: f32,
    pub speed_max_hz: u64,
}

#[derive(Clone, Debug)]
pub struct AddressSpace {
    pub id: String,
    pub name: String,
    pub start_address: u32,
    pub size: u32,
    pub segments: Vec<MemorySegment>,
}

#[derive(Clone, Debug)]
pub struct MemorySegment {
    pub start_address: u32,
    pub size: u32,
    pub ty: String,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub name: String,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub instances: Vec<Instance>,
    pub register_groups: Vec<RegisterGroup>,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub name: String,
    pub signals: Vec<Signal>,
}

#[derive(Clone, Debug)]
pub struct RegisterGroup {
    pub name: String,
    pub caption: String,
    pub registers: Vec<Register>,
}

#[derive(Clone, Debug)]
pub struct Register {
    pub name: String,
    pub caption: String,
    pub offset: u32,
    pub size: u32,
    pub mask: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Signal {
    pub pad: String,
    pub group: Option<String>,
    pub index: Option<u8>,
}

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let packs = pack::load_all(&crate_root.join("packs")).unwrap();
    gen::all(&crate_root.join("src").join("gen"), &packs).unwrap();
}

impl Register {
    /// Get the union between two descriptions of the same register.
    pub fn union(&self, with: &Self) -> Self {
        assert_eq!(self.name, with.name,
                   "can only take the union between different descriptions of the same register");

        let mut result = self.clone();

        match (result.mask, with.mask) {
            (None, Some(v)) => result.mask = Some(v), // rhs is more specific
            _ => (),
        }

        result
    }
}

mod gen {
    use super::*;
    use std::io::prelude::*;
    use std::io;
    use std::fs::{self, File};
    use std::path::Path;

    pub fn all(path: &Path, packs: &Vec<Pack>) -> Result<(), io::Error> {
        fs::create_dir_all(path)?;

        let mut module_names = Vec::new();

        // Create modules for each pack.
        for pack in packs.iter() {
            let module_name = self::pack_module_name(pack);
            let module_path = path.join(format!("{}.rs", module_name));

            generate_pack_module(pack, &module_path)?;
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

    /// Generates a self-contained module for a single pack file.
    fn generate_pack_module(pack: &Pack, path: &Path) -> Result<(), io::Error> {
        let mut file = File::create(path)?;

        self::pack_module_doc(pack, &mut file)?;
        writeln!(file)?;
        self::pack_registers(pack, &mut file)?;

        Ok(())
    }

    /// Gets the module name for a pack.
    fn pack_module_name(pack: &Pack) -> String {
        pack.device.name.to_lowercase()
    }

    pub fn pack_module_doc(pack: &Pack, w: &mut Write)
        -> Result<(), io::Error> {
        writeln!(w, "//! The AVR {} microcontroller", pack.device.name)?;
        writeln!(w, "//!")?;

        writeln!(w, "//! # Variants")?;
        writeln!(w, "//! |        | Pinout | Package | Operating temperature | Operating voltage | Max speed |")?;
        writeln!(w, "//! |--------|--------|---------|-----------------------|-------------------|-----------|")?;
        for variant in pack.variants.iter() {
            let speed_mhz = variant.speed_max_hz / 1_000_000;
            writeln!(w, "//! | {} | {} | {} | {}°C - {}°C | {}V - {}V | {} MHz |",
                     variant.name, variant.pinout.clone().unwrap_or_else(|| String::new()),
                     variant.package, variant.temperature_min,
                     variant.temperature_max, variant.voltage_min, variant.voltage_max,
                     speed_mhz)?;
        }
        writeln!(w, "//!")?;

        writeln!(w, "//! # Registers by module (not exhaustive)")?;

        for module in modules_by_relevance(pack.device.modules.clone()) {
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

    pub fn pack_registers(pack: &Pack, w: &mut Write)
        -> Result<(), io::Error>  {
        for register in ordered_registers(pack) {
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

    fn ordered_registers(pack: &Pack) -> Vec<Register> {
        let mut unique_registers = self::unique_registers(pack);
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

    fn unique_registers(pack: &Pack) -> HashMap<String, Register> {
        let mut result = HashMap::new();

        for module in pack.modules.iter() {
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

mod pack {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;
    use std::{io, fs};
    use std::path::{Path, PathBuf};

    use xmltree::Element;

    pub fn load_all(path: &Path) -> Result<Vec<Pack>, io::Error> {
        let mut pack_paths = Vec::new();

        for pack_name in PACKS {
            pack_paths.extend(find_packs(&path.join(pack_name)).unwrap());
        }

        Ok(pack_paths.into_iter().map(|path| self::load(&path).unwrap()).collect())
    }

    pub fn load(path: &Path) -> Result<Pack, io::Error> {
        let mut file = File::open(path)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        println!("loading pack '{}'", path.display());
        let root = Element::parse(body.as_bytes()).unwrap();

        Ok(self::read_pack(&root))
    }

    fn read_pack(root: &Element) -> Pack {
        let device_element = root.get_child("devices").unwrap().get_child("device").unwrap();

        let device = self::read_device(&device_element);
        let variants = root.get_child("variants").unwrap().children.iter().map(self::read_variant).collect();
        let modules = root.get_child("modules").unwrap().children.iter().map(self::read_module);

        Pack {
            device: device,
            variants: variants,
            modules: modules.collect(),
        }
    }

    fn read_device(device: &Element) -> Device {
        let device_name = device.attributes.get("name").unwrap().clone();

        let modules = device.get_child("peripherals").unwrap()
                            .children.iter()
                            .map(self::read_module)
                            .collect();

        let address_spaces = device.get_child("address-spaces")
                                   .unwrap().children.iter()
                                   .map(self::read_address_space)
                                   .collect();

        Device {
            name: device_name,
            address_spaces: address_spaces,
            modules: modules,
        }
    }

    fn read_module(module: &Element) -> Module {
        let module_name = module.attributes.get("name").unwrap().clone();
        let mut register_groups = Vec::new();
        let mut instances = Vec::new();

        for child in module.children.iter() {
            match &child.name[..] {
                "instance" => instances.push(read_instance(child)),
                "register-group" => register_groups.push(read_register_group(child)),
                // Unimplemented tags.
                _ => (),
            }
        }

        Module {
            name: module_name,
            instances: instances,
            register_groups: register_groups,
        }
    }

    fn read_variant(variant: &Element) -> Variant {
        Variant {
            name: variant.attributes.get("ordercode").unwrap().clone(),
            temperature_min: variant.attributes.get("tempmin").unwrap().parse().unwrap(),
            temperature_max: variant.attributes.get("tempmax").unwrap().parse().unwrap(),
            voltage_min: variant.attributes.get("vccmin").unwrap().parse().unwrap(),
            voltage_max: variant.attributes.get("vccmax").unwrap().parse().unwrap(),
            package: variant.attributes.get("package").unwrap().clone(),
            pinout: variant.attributes.get("pinout").map(|p| p.clone()),
            speed_max_hz: variant.attributes.get("speedmax").unwrap().parse().unwrap(),
        }
    }

    fn read_instance(instance: &Element) -> Instance {
        let instance_name = instance.attributes.get("name").unwrap().clone();

        let signals = match instance.get_child("signals") {
            Some(signals) => signals.children.iter().map(read_signal).collect(),
            None =>  Vec::new(),
        };

        Instance {
            name: instance_name,
            signals: signals,
        }
    }

    fn read_signal(signal: &Element) -> Signal {
        Signal {
            pad: signal.attributes.get("pad").unwrap().clone(),
            group: signal.attributes.get("pad").map(|p| p.clone()),
            index: signal.attributes.get("index").map(|i| i.parse().unwrap()),
        }
    }

    /// Reads a register group.
    ///
    /// This looks like so
    ///
    /// ```xml
    /// <register-group caption="EEPROM" name="EEPROM">
    ///   <register caption="EEPROM Address Register  Bytes" name="EEAR" offset="0x41" size="2" mask="0x01FF"/>
    ///   <register caption="EEPROM Data Register" name="EEDR" offset="0x40" size="1" mask="0xFF"/>
    /// </register-group>
    fn read_register_group(register_group: &Element) -> RegisterGroup {
        let (name, caption) = (register_group.attributes.get("name").unwrap(),
                               register_group.attributes.get("caption").unwrap());
        let registers = register_group.children.iter().filter_map(|child| match &child.name[..] {
            "register" => Some(self::read_register(child)),
            // FIXME: leave this out for now, ATtiny816 has nested register-group
            // _ => panic!("unknown register-group child: '{}'", child.name),
            _ => None,
        }).collect();

        RegisterGroup {
            name: name.clone(),
            caption: caption.clone(),
            registers: registers,
        }
    }

    /// Reads a register.
    ///
    /// This looks like
    ///
    /// ```xml
    /// <register caption="EEPROM Address Register  Bytes" name="EEAR" offset="0x41" size="2" mask="0x01FF"/>
    /// ```
    fn read_register(register: &Element) -> Register {
        Register {
            name: register.attributes.get("name").unwrap().clone(),
            caption: register.attributes.get("caption").unwrap().clone(),
            offset: read_int(register.attributes.get("offset")).clone(),
            size: register.attributes.get("size").unwrap().parse().unwrap(),
            mask: read_opt_int(register.attributes.get("mask")).clone()
        }
    }

    /// Reads an eddress space.
    ///
    /// This looks like
    ///
    /// ```xml
    /// <address-space endianness="little" name="signatures" id="signatures" start="0" size="3">
    ///   <memory-segment start="0" size="3" type="signatures" rw="R" exec="0" name="SIGNATURES"/>
    /// </address-space>
    /// ```
    fn read_address_space(address_space: &Element) -> AddressSpace {
        let id = address_space.attributes.get("id").unwrap().clone();
        let start_address = read_int(address_space.attributes.get("start"));
        let size = read_int(address_space.attributes.get("size"));
        let segments = address_space.children.iter().map(read_memory_segment).collect();

        AddressSpace {
            id: id,
            name: address_space.attributes.get("name").unwrap().clone(),
            start_address: start_address,
            size: size,
            segments: segments,
        }
    }

    /// Reads a memory segment.
    ///
    /// ```xml
    /// <memory-segment start="0" size="3" type="signatures" rw="R" exec="0" name="SIGNATURES"/>
    /// ```
    fn read_memory_segment(memory_segment: &Element) -> MemorySegment {
        let default_perms = "".to_owned();

        let start_address = read_int(memory_segment.attributes.get("start"));
        let size = read_int(memory_segment.attributes.get("size"));
        let ty = memory_segment.attributes.get("type").unwrap().clone();
        let rw = memory_segment.attributes.get("rw").unwrap_or(&default_perms);
        let exec = memory_segment.attributes.get("exec").unwrap_or(&default_perms);
        let name = memory_segment.attributes.get("name").unwrap().clone();
        let page_size = memory_segment.attributes.get("pagesize").map(|p| read_int(Some(p)));

        let readable = rw.contains("r") || rw.contains("R");
        let writable = rw.contains("w") || rw.contains("W");
        let executable = exec == "1";

        MemorySegment {
            start_address, size, ty, name, readable, writable, executable,
            page_size
        }
    }

    fn read_int(value: Option<&String>) -> u32 {
        let value = value.unwrap();

        if value.starts_with("0x") {
            read_hex(Some(value))
        } else {
            value.parse().unwrap()
        }
    }

    fn read_opt_int(value: Option<&String>) -> Option<u32> {
        value.map(|v| {
            if v.starts_with("0x") {
                read_hex(Some(v))
            } else {
                v.parse().unwrap()
            }
        })
    }

    fn read_hex(value: Option<&String>) -> u32 {
        let value = value.unwrap().replacen("0x", "", 1);
        u32::from_str_radix(&value, 16).unwrap()
    }

    /// Finds all pack files in a directory.
    fn find_packs(in_dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
        let mut paths = Vec::new();

        for entry in fs::read_dir(in_dir)? {
            let entry = entry?;
            if let Some(PACK_FILE_EXT) = entry.path().extension().map(|s| s.to_str().unwrap()) {
                paths.push(entry.path());
            }
        }
        Ok(paths)
    }
}

