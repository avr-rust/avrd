extern crate xmltree;

use std::path::Path;

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub variants: Vec<Variant>,
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
    pub page_size: u32,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub instances: Vec<Instance>,
}

#[derive(Debug)]
pub struct Instance {
    pub name: String,
    pub signals: Vec<Signal>,
}

#[derive(Debug)]
pub struct Signal {
    pub pad: String,
    pub index: Option<u8>,
}

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let devices = pack::load_all(&crate_root.join("packs")).unwrap();
    gen::all(&crate_root.join("src").join("gen"), &devices).unwrap();
}

mod gen {
    use super::*;
    use std::io::prelude::*;
    use std::io::{self, Cursor};
    use std::fs::{self, File};
    use std::path::Path;

    pub fn all(path: &Path, devices: &Vec<Device>) -> Result<(), io::Error> {
        fs::create_dir_all(path)?;

        let device_names: Vec<String> = devices.iter().map(|device| {
            let normalised_name = device.name.to_lowercase();
            let registers = self::registers(device);
            let mut file = File::create(path.join(format!("{}.rs", normalised_name))).unwrap();
            writeln!(file, "{}", registers).unwrap();
            normalised_name
        }).collect();

        let mut mod_rs = File::create(path.join("mod.rs"))?;
        for device_name in device_names {
            writeln!(mod_rs, "pub mod {};", device_name).unwrap();
        }

        Ok(())
    }

    pub fn registers(device: &Device) -> String {
        let mut b = Cursor::new(Vec::new());

        for module in device.modules.iter() {
            for instance in module.instances.iter() {
                let offset = 2;
                let addr = offset + 20;
                writeln!(b, "pub const {}: *mut u8 = {} as *mut u8;", instance.name, addr).unwrap();
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

    pub fn load_all(path: &Path) -> Result<Vec<Device>, io::Error> {
        let pack_paths = find_packs(&path.join("atmega")).unwrap();
        Ok(pack_paths.into_iter().map(|path| self::load(&path).unwrap()).collect())
    }

    pub fn load(path: &Path) -> Result<Device, io::Error> {
        let mut file = File::open(path)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let root = Element::parse(body.as_bytes()).unwrap();
        let device = root.get_child("devices").unwrap().get_child("device").unwrap();

        let address_spaces = Vec::new();

        let modules: Vec<_> = device.get_child("peripherals").unwrap().children.iter().map(|module| {
            let module_name = module.attributes.get("name").unwrap().clone();

            let instances = module.children.iter().filter_map(|child| {
                if child.name != "instance" { return None; }
                let instance = child;

                let instance_name = instance.attributes.get("name").unwrap().clone();

                let signals = match instance.get_child("signals") {
                    Some(signals) => signals.children.iter().map(|signal| {
                        let pad = signal.attributes.get("pad").unwrap().clone();
                        let index = signal.attributes.get("index").map(|i| i.parse().unwrap());
                        Signal {
                            pad: pad,
                            index: index,
                        }
                    }).collect(),
                    None =>  Vec::new(),
                };

                Some(Instance {
                    name: instance_name,
                    signals: signals,
                })

            }).collect();

            Module {
                name: module_name,
                instances: instances,
            }
        }).collect();

        // let address_spaces = device.get_child("address-spaces").unwrap().children.iter().map(|addr| {
        //     AddressSpace {
        //         name: addr.attributes.get("name").unwrap().clone(),
        //         start: u32::from_str_radix(addr.attributes.get("id").unknown()).unwrap(),
        //     }
        // }).collect();

        let name = device.attributes.get("name").unwrap();

        let variants = root.get_child("variants").unwrap().children.iter().map(|variant| {
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
        }).collect();

        Ok(Device {
            name: name.clone(),
            variants: variants,
            address_spaces: address_spaces,
            modules: modules,
        })
    }

    fn find_packs(in_dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
        let mut paths = Vec::new();

        for entry in fs::read_dir(in_dir)? {
            let entry = entry?;
            if let Some("atdf") = entry.path().extension().map(|s| s.to_str().unwrap()) {
                paths.push(entry.path());
            }
        }
        Ok(paths)
    }
}

