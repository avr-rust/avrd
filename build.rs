extern crate xmltree;

/// The extension on the pack files.
const PACK_FILE_EXT: &'static str = "atdf";

use std::path::Path;

#[derive(Debug)]
pub struct Pack {
    pub device: Device,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub address_spaces: Vec<AddressSpace>,
    pub modules: Vec<Module>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct AddressSpace {
    pub id: String,
    pub name: String,
    pub start_address: u32,
    pub size: u32,
    pub segments: Vec<MemorySegment>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub instances: Vec<Instance>,
    pub register_groups: Vec<RegisterGroup>,
}

#[derive(Debug)]
pub struct Instance {
    pub name: String,
    pub signals: Vec<Signal>,
}

#[derive(Debug)]
pub struct RegisterGroup {
    pub name: String,
    pub caption: String,
    pub registers: Vec<Register>,
}

#[derive(Debug)]
pub struct Register {
    pub name: String,
    pub caption: String,
    pub offset: u32,
    pub size: u32,
    pub mask: u32,
}

#[derive(Debug)]
pub struct Signal {
    pub pad: String,
    pub index: Option<u8>,
}

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let packs = pack::load_all(&crate_root.join("packs")).unwrap();
    gen::all(&crate_root.join("src").join("gen"), &packs).unwrap();
}

mod gen {
    use super::*;
    use std::io::prelude::*;
    use std::io::{self, Cursor};
    use std::fs::{self, File};
    use std::path::Path;

    pub fn all(path: &Path, packs: &Vec<Pack>) -> Result<(), io::Error> {
        fs::create_dir_all(path)?;

        let mut module_names = Vec::new();

        // Create modules for each pack.
        for pack in packs.iter() {
            let module_name = self::pack_module_name(pack);
            let registers = self::registers(&pack.device);

            let mut file = File::create(path.join(format!("{}.rs", module_name))).unwrap();
            writeln!(file, "{}", registers).unwrap();

            module_names.push(module_name);
        }

        // Create mod.rs in output folder.
        let mut mod_rs = File::create(path.join("mod.rs"))?;
        for module_name in module_names {
            writeln!(mod_rs, "pub mod {};", module_name).unwrap();
        }

        Ok(())
    }

    /// Gets the module name for a pack.
    fn pack_module_name(pack: &Pack) -> String {
        pack.device.name.to_lowercase()
    }

    pub fn registers(device: &Device) -> String {
        let mut b = Cursor::new(Vec::new());

        for module in device.modules.iter() {
            for register_group in module.register_groups.iter() {
                for register in register_group.registers.iter() {
                    writeln!(b, "pub const {}: *mut u8 = {} as *mut u8;", register.name, register.offset).unwrap();
                }
            }
        }

        String::from_utf8(b.into_inner()).unwrap()
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
        let pack_paths = find_packs(&path.join("atmega")).unwrap();
        Ok(pack_paths.into_iter().map(|path| self::load(&path).unwrap()).collect())
    }

    pub fn load(path: &Path) -> Result<Pack, io::Error> {
        let mut file = File::open(path)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let root = Element::parse(body.as_bytes()).unwrap();

        Ok(self::read_pack(&root))
    }

    fn read_pack(root: &Element) -> Pack {
        let device_element = root.get_child("devices").unwrap().get_child("device").unwrap();
        let device = self::read_device(&device_element);
        let variants = root.get_child("variants").unwrap().children.iter().map(self::read_variant).collect();

        Pack {
            device: device,
            variants: variants,
        }
    }

    fn read_device(device: &Element) -> Device {
        let modules = device.get_child("peripherals").unwrap()
                            .children.iter()
                            .map(self::read_module)
                            .collect();

        let address_spaces = device.get_child("address-spaces")
                                   .unwrap().children.iter()
                                   .map(self::read_address_space)
                                   .collect();

        let device_name = device.attributes.get("name").unwrap().clone();

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
        let pad = signal.attributes.get("pad").unwrap().clone();
        let index = signal.attributes.get("index").map(|i| i.parse().unwrap());
        Signal {
            pad: pad,
            index: index,
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
            "register" => Some(self::read_register(register_group)),
            _ => unimplemented!(),
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
            mask: read_int(register.attributes.get("mask")).clone()
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

