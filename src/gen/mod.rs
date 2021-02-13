// Device definitions

/// The module containing the values for the 'atmega329pa' microcontroller
#[cfg(any(avr_mcu_atmega329pa, feature = "all-mcus"))] pub mod atmega329pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega329pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega329pa))] pub mod current { pub use super::atmega329pa::*; }

/// The module containing the values for the 'atmega1608' microcontroller
#[cfg(any(avr_mcu_atmega1608, feature = "all-mcus"))] pub mod atmega1608;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1608'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1608))] pub mod current { pub use super::atmega1608::*; }

/// The module containing the values for the 'atmega4809' microcontroller
#[cfg(any(avr_mcu_atmega4809, feature = "all-mcus"))] pub mod atmega4809;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega4809'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega4809))] pub mod current { pub use super::atmega4809::*; }

/// The module containing the values for the 'atmega1280' microcontroller
#[cfg(any(avr_mcu_atmega1280, feature = "all-mcus"))] pub mod atmega1280;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1280'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1280))] pub mod current { pub use super::atmega1280::*; }

/// The module containing the values for the 'atmega16a' microcontroller
#[cfg(any(avr_mcu_atmega16a, feature = "all-mcus"))] pub mod atmega16a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16a))] pub mod current { pub use super::atmega16a::*; }

/// The module containing the values for the 'atmega256rfr2' microcontroller
#[cfg(any(avr_mcu_atmega256rfr2, feature = "all-mcus"))] pub mod atmega256rfr2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega256rfr2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega256rfr2))] pub mod current { pub use super::atmega256rfr2::*; }

/// The module containing the values for the 'atmega645' microcontroller
#[cfg(any(avr_mcu_atmega645, feature = "all-mcus"))] pub mod atmega645;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega645'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega645))] pub mod current { pub use super::atmega645::*; }

/// The module containing the values for the 'atmega325a' microcontroller
#[cfg(any(avr_mcu_atmega325a, feature = "all-mcus"))] pub mod atmega325a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega325a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega325a))] pub mod current { pub use super::atmega325a::*; }

/// The module containing the values for the 'atmega64' microcontroller
#[cfg(any(avr_mcu_atmega64, feature = "all-mcus"))] pub mod atmega64;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega64'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega64))] pub mod current { pub use super::atmega64::*; }

/// The module containing the values for the 'at90usb1286' microcontroller
#[cfg(any(avr_mcu_at90usb1286, feature = "all-mcus"))] pub mod at90usb1286;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90usb1286'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90usb1286))] pub mod current { pub use super::at90usb1286::*; }

/// The module containing the values for the 'atmega88p' microcontroller
#[cfg(any(avr_mcu_atmega88p, feature = "all-mcus"))] pub mod atmega88p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega88p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega88p))] pub mod current { pub use super::atmega88p::*; }

/// The module containing the values for the 'atmega1284p' microcontroller
#[cfg(any(avr_mcu_atmega1284p, feature = "all-mcus"))] pub mod atmega1284p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1284p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1284p))] pub mod current { pub use super::atmega1284p::*; }

/// The module containing the values for the 'atmega32m1' microcontroller
#[cfg(any(avr_mcu_atmega32m1, feature = "all-mcus"))] pub mod atmega32m1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32m1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32m1))] pub mod current { pub use super::atmega32m1::*; }

/// The module containing the values for the 'atmega64hve2' microcontroller
#[cfg(any(avr_mcu_atmega64hve2, feature = "all-mcus"))] pub mod atmega64hve2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega64hve2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega64hve2))] pub mod current { pub use super::atmega64hve2::*; }

/// The module containing the values for the 'atmega1281' microcontroller
#[cfg(any(avr_mcu_atmega1281, feature = "all-mcus"))] pub mod atmega1281;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1281'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1281))] pub mod current { pub use super::atmega1281::*; }

/// The module containing the values for the 'atmega165pa' microcontroller
#[cfg(any(avr_mcu_atmega165pa, feature = "all-mcus"))] pub mod atmega165pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega165pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega165pa))] pub mod current { pub use super::atmega165pa::*; }

/// The module containing the values for the 'atmega644p' microcontroller
#[cfg(any(avr_mcu_atmega644p, feature = "all-mcus"))] pub mod atmega644p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega644p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega644p))] pub mod current { pub use super::atmega644p::*; }

/// The module containing the values for the 'atmega649' microcontroller
#[cfg(any(avr_mcu_atmega649, feature = "all-mcus"))] pub mod atmega649;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega649'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega649))] pub mod current { pub use super::atmega649::*; }

/// The module containing the values for the 'atmega325pa' microcontroller
#[cfg(any(avr_mcu_atmega325pa, feature = "all-mcus"))] pub mod atmega325pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega325pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega325pa))] pub mod current { pub use super::atmega325pa::*; }

/// The module containing the values for the 'atmega2561' microcontroller
#[cfg(any(avr_mcu_atmega2561, feature = "all-mcus"))] pub mod atmega2561;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega2561'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega2561))] pub mod current { pub use super::atmega2561::*; }

/// The module containing the values for the 'at90pwm3b' microcontroller
#[cfg(any(avr_mcu_at90pwm3b, feature = "all-mcus"))] pub mod at90pwm3b;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm3b'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm3b))] pub mod current { pub use super::at90pwm3b::*; }

/// The module containing the values for the 'atmega16m1' microcontroller
#[cfg(any(avr_mcu_atmega16m1, feature = "all-mcus"))] pub mod atmega16m1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16m1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16m1))] pub mod current { pub use super::atmega16m1::*; }

/// The module containing the values for the 'atmega32hvbrevb' microcontroller
#[cfg(any(avr_mcu_atmega32hvbrevb, feature = "all-mcus"))] pub mod atmega32hvbrevb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32hvbrevb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32hvbrevb))] pub mod current { pub use super::atmega32hvbrevb::*; }

/// The module containing the values for the 'atmega325p' microcontroller
#[cfg(any(avr_mcu_atmega325p, feature = "all-mcus"))] pub mod atmega325p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega325p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega325p))] pub mod current { pub use super::atmega325p::*; }

/// The module containing the values for the 'atmega1284rfr2' microcontroller
#[cfg(any(avr_mcu_atmega1284rfr2, feature = "all-mcus"))] pub mod atmega1284rfr2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1284rfr2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1284rfr2))] pub mod current { pub use super::atmega1284rfr2::*; }

/// The module containing the values for the 'atmega406' microcontroller
#[cfg(any(avr_mcu_atmega406, feature = "all-mcus"))] pub mod atmega406;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega406'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega406))] pub mod current { pub use super::atmega406::*; }

/// The module containing the values for the 'atmega645a' microcontroller
#[cfg(any(avr_mcu_atmega645a, feature = "all-mcus"))] pub mod atmega645a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega645a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega645a))] pub mod current { pub use super::atmega645a::*; }

/// The module containing the values for the 'atmega2560' microcontroller
#[cfg(any(avr_mcu_atmega2560, feature = "all-mcus"))] pub mod atmega2560;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega2560'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega2560))] pub mod current { pub use super::atmega2560::*; }

/// The module containing the values for the 'atmega6450p' microcontroller
#[cfg(any(avr_mcu_atmega6450p, feature = "all-mcus"))] pub mod atmega6450p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega6450p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega6450p))] pub mod current { pub use super::atmega6450p::*; }

/// The module containing the values for the 'atmega328' microcontroller
///
/// This is the default MCU when targeting a non-AVR target.
#[cfg(any(not(target_arch = "avr"), avr_mcu_atmega328, feature = "all-mcus"))] pub mod atmega328;
/// Contains definitions for the current AVR device being targeted. **The 'atmega328' is the default when targeting non-AVR devices.**
#[cfg(not(target_arch = "avr"))] pub mod current { pub use super::atmega328::*; }
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega328'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega328))] pub mod current { pub use super::atmega328::*; }

/// The module containing the values for the 'atmega162' microcontroller
#[cfg(any(avr_mcu_atmega162, feature = "all-mcus"))] pub mod atmega162;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega162'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega162))] pub mod current { pub use super::atmega162::*; }

/// The module containing the values for the 'atmega8u2' microcontroller
#[cfg(any(avr_mcu_atmega8u2, feature = "all-mcus"))] pub mod atmega8u2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega8u2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega8u2))] pub mod current { pub use super::atmega8u2::*; }

/// The module containing the values for the 'atmega3290pa' microcontroller
#[cfg(any(avr_mcu_atmega3290pa, feature = "all-mcus"))] pub mod atmega3290pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3290pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3290pa))] pub mod current { pub use super::atmega3290pa::*; }

/// The module containing the values for the 'atmega16hvb' microcontroller
#[cfg(any(avr_mcu_atmega16hvb, feature = "all-mcus"))] pub mod atmega16hvb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16hvb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16hvb))] pub mod current { pub use super::atmega16hvb::*; }

/// The module containing the values for the 'atmega48a' microcontroller
#[cfg(any(avr_mcu_atmega48a, feature = "all-mcus"))] pub mod atmega48a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega48a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega48a))] pub mod current { pub use super::atmega48a::*; }

/// The module containing the values for the 'atmega640' microcontroller
#[cfg(any(avr_mcu_atmega640, feature = "all-mcus"))] pub mod atmega640;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega640'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega640))] pub mod current { pub use super::atmega640::*; }

/// The module containing the values for the 'atmega8515' microcontroller
#[cfg(any(avr_mcu_atmega8515, feature = "all-mcus"))] pub mod atmega8515;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega8515'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega8515))] pub mod current { pub use super::atmega8515::*; }

/// The module containing the values for the 'atmega165p' microcontroller
#[cfg(any(avr_mcu_atmega165p, feature = "all-mcus"))] pub mod atmega165p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega165p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega165p))] pub mod current { pub use super::atmega165p::*; }

/// The module containing the values for the 'atmega169a' microcontroller
#[cfg(any(avr_mcu_atmega169a, feature = "all-mcus"))] pub mod atmega169a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega169a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega169a))] pub mod current { pub use super::atmega169a::*; }

/// The module containing the values for the 'atmega3250p' microcontroller
#[cfg(any(avr_mcu_atmega3250p, feature = "all-mcus"))] pub mod atmega3250p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3250p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3250p))] pub mod current { pub use super::atmega3250p::*; }

/// The module containing the values for the 'at90pwm216' microcontroller
#[cfg(any(avr_mcu_at90pwm216, feature = "all-mcus"))] pub mod at90pwm216;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm216'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm216))] pub mod current { pub use super::at90pwm216::*; }

/// The module containing the values for the 'atmega644rfr2' microcontroller
#[cfg(any(avr_mcu_atmega644rfr2, feature = "all-mcus"))] pub mod atmega644rfr2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega644rfr2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega644rfr2))] pub mod current { pub use super::atmega644rfr2::*; }

/// The module containing the values for the 'atmega3209' microcontroller
#[cfg(any(avr_mcu_atmega3209, feature = "all-mcus"))] pub mod atmega3209;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3209'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3209))] pub mod current { pub use super::atmega3209::*; }

/// The module containing the values for the 'atmega16hva' microcontroller
#[cfg(any(avr_mcu_atmega16hva, feature = "all-mcus"))] pub mod atmega16hva;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16hva'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16hva))] pub mod current { pub use super::atmega16hva::*; }

/// The module containing the values for the 'atmega88pa' microcontroller
#[cfg(any(avr_mcu_atmega88pa, feature = "all-mcus"))] pub mod atmega88pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega88pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega88pa))] pub mod current { pub use super::atmega88pa::*; }

/// The module containing the values for the 'at90usb646' microcontroller
#[cfg(any(avr_mcu_at90usb646, feature = "all-mcus"))] pub mod at90usb646;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90usb646'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90usb646))] pub mod current { pub use super::at90usb646::*; }

/// The module containing the values for the 'atmega32a' microcontroller
#[cfg(any(avr_mcu_atmega32a, feature = "all-mcus"))] pub mod atmega32a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32a))] pub mod current { pub use super::atmega32a::*; }

/// The module containing the values for the 'atmega3250pa' microcontroller
#[cfg(any(avr_mcu_atmega3250pa, feature = "all-mcus"))] pub mod atmega3250pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3250pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3250pa))] pub mod current { pub use super::atmega3250pa::*; }

/// The module containing the values for the 'atmega164pa' microcontroller
#[cfg(any(avr_mcu_atmega164pa, feature = "all-mcus"))] pub mod atmega164pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega164pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega164pa))] pub mod current { pub use super::atmega164pa::*; }

/// The module containing the values for the 'atmega165a' microcontroller
#[cfg(any(avr_mcu_atmega165a, feature = "all-mcus"))] pub mod atmega165a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega165a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega165a))] pub mod current { pub use super::atmega165a::*; }

/// The module containing the values for the 'atmega324pa' microcontroller
#[cfg(any(avr_mcu_atmega324pa, feature = "all-mcus"))] pub mod atmega324pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega324pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega324pa))] pub mod current { pub use super::atmega324pa::*; }

/// The module containing the values for the 'atmega128' microcontroller
#[cfg(any(avr_mcu_atmega128, feature = "all-mcus"))] pub mod atmega128;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega128'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega128))] pub mod current { pub use super::atmega128::*; }

/// The module containing the values for the 'atmega88' microcontroller
#[cfg(any(avr_mcu_atmega88, feature = "all-mcus"))] pub mod atmega88;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega88'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega88))] pub mod current { pub use super::atmega88::*; }

/// The module containing the values for the 'atmega649p' microcontroller
#[cfg(any(avr_mcu_atmega649p, feature = "all-mcus"))] pub mod atmega649p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega649p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega649p))] pub mod current { pub use super::atmega649p::*; }

/// The module containing the values for the 'atmega328p' microcontroller
#[cfg(any(avr_mcu_atmega328p, feature = "all-mcus"))] pub mod atmega328p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega328p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega328p))] pub mod current { pub use super::atmega328p::*; }

/// The module containing the values for the 'atmega644' microcontroller
#[cfg(any(avr_mcu_atmega644, feature = "all-mcus"))] pub mod atmega644;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega644'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega644))] pub mod current { pub use super::atmega644::*; }

/// The module containing the values for the 'atmega168pb' microcontroller
#[cfg(any(avr_mcu_atmega168pb, feature = "all-mcus"))] pub mod atmega168pb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega168pb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega168pb))] pub mod current { pub use super::atmega168pb::*; }

/// The module containing the values for the 'atmega6490p' microcontroller
#[cfg(any(avr_mcu_atmega6490p, feature = "all-mcus"))] pub mod atmega6490p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega6490p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega6490p))] pub mod current { pub use super::atmega6490p::*; }

/// The module containing the values for the 'at90pwm81' microcontroller
#[cfg(any(avr_mcu_at90pwm81, feature = "all-mcus"))] pub mod at90pwm81;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm81'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm81))] pub mod current { pub use super::at90pwm81::*; }

/// The module containing the values for the 'atmega2564rfr2' microcontroller
#[cfg(any(avr_mcu_atmega2564rfr2, feature = "all-mcus"))] pub mod atmega2564rfr2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega2564rfr2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega2564rfr2))] pub mod current { pub use super::atmega2564rfr2::*; }

/// The module containing the values for the 'atmega3290p' microcontroller
#[cfg(any(avr_mcu_atmega3290p, feature = "all-mcus"))] pub mod atmega3290p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3290p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3290p))] pub mod current { pub use super::atmega3290p::*; }

/// The module containing the values for the 'atmega8535' microcontroller
#[cfg(any(avr_mcu_atmega8535, feature = "all-mcus"))] pub mod atmega8535;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega8535'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega8535))] pub mod current { pub use super::atmega8535::*; }

/// The module containing the values for the 'atmega329a' microcontroller
#[cfg(any(avr_mcu_atmega329a, feature = "all-mcus"))] pub mod atmega329a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega329a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega329a))] pub mod current { pub use super::atmega329a::*; }

/// The module containing the values for the 'atmega48pa' microcontroller
#[cfg(any(avr_mcu_atmega48pa, feature = "all-mcus"))] pub mod atmega48pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega48pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega48pa))] pub mod current { pub use super::atmega48pa::*; }

/// The module containing the values for the 'atmega645p' microcontroller
#[cfg(any(avr_mcu_atmega645p, feature = "all-mcus"))] pub mod atmega645p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega645p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega645p))] pub mod current { pub use super::atmega645p::*; }

/// The module containing the values for the 'atmega168p' microcontroller
#[cfg(any(avr_mcu_atmega168p, feature = "all-mcus"))] pub mod atmega168p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega168p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega168p))] pub mod current { pub use super::atmega168p::*; }

/// The module containing the values for the 'atmega8hva' microcontroller
#[cfg(any(avr_mcu_atmega8hva, feature = "all-mcus"))] pub mod atmega8hva;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega8hva'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega8hva))] pub mod current { pub use super::atmega8hva::*; }

/// The module containing the values for the 'atmega169p' microcontroller
#[cfg(any(avr_mcu_atmega169p, feature = "all-mcus"))] pub mod atmega169p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega169p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega169p))] pub mod current { pub use super::atmega169p::*; }

/// The module containing the values for the 'atmega3208' microcontroller
#[cfg(any(avr_mcu_atmega3208, feature = "all-mcus"))] pub mod atmega3208;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3208'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3208))] pub mod current { pub use super::atmega3208::*; }

/// The module containing the values for the 'atmega6450' microcontroller
#[cfg(any(avr_mcu_atmega6450, feature = "all-mcus"))] pub mod atmega6450;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega6450'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega6450))] pub mod current { pub use super::atmega6450::*; }

/// The module containing the values for the 'atmega168a' microcontroller
#[cfg(any(avr_mcu_atmega168a, feature = "all-mcus"))] pub mod atmega168a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega168a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega168a))] pub mod current { pub use super::atmega168a::*; }

/// The module containing the values for the 'at90can128' microcontroller
#[cfg(any(avr_mcu_at90can128, feature = "all-mcus"))] pub mod at90can128;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90can128'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90can128))] pub mod current { pub use super::at90can128::*; }

/// The module containing the values for the 'atmega328pb' microcontroller
#[cfg(any(avr_mcu_atmega328pb, feature = "all-mcus"))] pub mod atmega328pb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega328pb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega328pb))] pub mod current { pub use super::atmega328pb::*; }

/// The module containing the values for the 'atmega16u4' microcontroller
#[cfg(any(avr_mcu_atmega16u4, feature = "all-mcus"))] pub mod atmega16u4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16u4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16u4))] pub mod current { pub use super::atmega16u4::*; }

/// The module containing the values for the 'at90pwm316' microcontroller
#[cfg(any(avr_mcu_at90pwm316, feature = "all-mcus"))] pub mod at90pwm316;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm316'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm316))] pub mod current { pub use super::at90pwm316::*; }

/// The module containing the values for the 'atmega88pb' microcontroller
#[cfg(any(avr_mcu_atmega88pb, feature = "all-mcus"))] pub mod atmega88pb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega88pb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega88pb))] pub mod current { pub use super::atmega88pb::*; }

/// The module containing the values for the 'atmega32c1' microcontroller
#[cfg(any(avr_mcu_atmega32c1, feature = "all-mcus"))] pub mod atmega32c1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32c1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32c1))] pub mod current { pub use super::atmega32c1::*; }

/// The module containing the values for the 'atmega1609' microcontroller
#[cfg(any(avr_mcu_atmega1609, feature = "all-mcus"))] pub mod atmega1609;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1609'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1609))] pub mod current { pub use super::atmega1609::*; }

/// The module containing the values for the 'atmega644pa' microcontroller
#[cfg(any(avr_mcu_atmega644pa, feature = "all-mcus"))] pub mod atmega644pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega644pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega644pa))] pub mod current { pub use super::atmega644pa::*; }

/// The module containing the values for the 'atmega168pa' microcontroller
#[cfg(any(avr_mcu_atmega168pa, feature = "all-mcus"))] pub mod atmega168pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega168pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega168pa))] pub mod current { pub use super::atmega168pa::*; }

/// The module containing the values for the 'atmega329' microcontroller
#[cfg(any(avr_mcu_atmega329, feature = "all-mcus"))] pub mod atmega329;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega329'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega329))] pub mod current { pub use super::atmega329::*; }

/// The module containing the values for the 'at90usb1287' microcontroller
#[cfg(any(avr_mcu_at90usb1287, feature = "all-mcus"))] pub mod at90usb1287;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90usb1287'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90usb1287))] pub mod current { pub use super::at90usb1287::*; }

/// The module containing the values for the 'atmega64m1' microcontroller
#[cfg(any(avr_mcu_atmega64m1, feature = "all-mcus"))] pub mod atmega64m1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega64m1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega64m1))] pub mod current { pub use super::atmega64m1::*; }

/// The module containing the values for the 'atmega649a' microcontroller
#[cfg(any(avr_mcu_atmega649a, feature = "all-mcus"))] pub mod atmega649a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega649a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega649a))] pub mod current { pub use super::atmega649a::*; }

/// The module containing the values for the 'atmega64rfr2' microcontroller
#[cfg(any(avr_mcu_atmega64rfr2, feature = "all-mcus"))] pub mod atmega64rfr2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega64rfr2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega64rfr2))] pub mod current { pub use super::atmega64rfr2::*; }

/// The module containing the values for the 'atmega48p' microcontroller
#[cfg(any(avr_mcu_atmega48p, feature = "all-mcus"))] pub mod atmega48p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega48p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega48p))] pub mod current { pub use super::atmega48p::*; }

/// The module containing the values for the 'atmega164p' microcontroller
#[cfg(any(avr_mcu_atmega164p, feature = "all-mcus"))] pub mod atmega164p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega164p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega164p))] pub mod current { pub use super::atmega164p::*; }

/// The module containing the values for the 'atmega1284' microcontroller
#[cfg(any(avr_mcu_atmega1284, feature = "all-mcus"))] pub mod atmega1284;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega1284'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega1284))] pub mod current { pub use super::atmega1284::*; }

/// The module containing the values for the 'atmega16' microcontroller
#[cfg(any(avr_mcu_atmega16, feature = "all-mcus"))] pub mod atmega16;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16))] pub mod current { pub use super::atmega16::*; }

/// The module containing the values for the 'atmega3290a' microcontroller
#[cfg(any(avr_mcu_atmega3290a, feature = "all-mcus"))] pub mod atmega3290a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3290a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3290a))] pub mod current { pub use super::atmega3290a::*; }

/// The module containing the values for the 'atmega6450a' microcontroller
#[cfg(any(avr_mcu_atmega6450a, feature = "all-mcus"))] pub mod atmega6450a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega6450a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega6450a))] pub mod current { pub use super::atmega6450a::*; }

/// The module containing the values for the 'at90usb162' microcontroller
#[cfg(any(avr_mcu_at90usb162, feature = "all-mcus"))] pub mod at90usb162;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90usb162'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90usb162))] pub mod current { pub use super::at90usb162::*; }

/// The module containing the values for the 'at90pwm1' microcontroller
#[cfg(any(avr_mcu_at90pwm1, feature = "all-mcus"))] pub mod at90pwm1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm1'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm1))] pub mod current { pub use super::at90pwm1::*; }

/// The module containing the values for the 'atmega164a' microcontroller
#[cfg(any(avr_mcu_atmega164a, feature = "all-mcus"))] pub mod atmega164a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega164a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega164a))] pub mod current { pub use super::atmega164a::*; }

/// The module containing the values for the 'atmega809' microcontroller
#[cfg(any(avr_mcu_atmega809, feature = "all-mcus"))] pub mod atmega809;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega809'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega809))] pub mod current { pub use super::atmega809::*; }

/// The module containing the values for the 'atmega329p' microcontroller
#[cfg(any(avr_mcu_atmega329p, feature = "all-mcus"))] pub mod atmega329p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega329p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega329p))] pub mod current { pub use super::atmega329p::*; }

/// The module containing the values for the 'atmega4808' microcontroller
#[cfg(any(avr_mcu_atmega4808, feature = "all-mcus"))] pub mod atmega4808;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega4808'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega4808))] pub mod current { pub use super::atmega4808::*; }

/// The module containing the values for the 'atmega128a' microcontroller
#[cfg(any(avr_mcu_atmega128a, feature = "all-mcus"))] pub mod atmega128a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega128a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega128a))] pub mod current { pub use super::atmega128a::*; }

/// The module containing the values for the 'at90can32' microcontroller
#[cfg(any(avr_mcu_at90can32, feature = "all-mcus"))] pub mod at90can32;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90can32'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90can32))] pub mod current { pub use super::at90can32::*; }

/// The module containing the values for the 'atmega32u2' microcontroller
#[cfg(any(avr_mcu_atmega32u2, feature = "all-mcus"))] pub mod atmega32u2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32u2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32u2))] pub mod current { pub use super::atmega32u2::*; }

/// The module containing the values for the 'atmega3290' microcontroller
#[cfg(any(avr_mcu_atmega3290, feature = "all-mcus"))] pub mod atmega3290;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3290'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3290))] pub mod current { pub use super::atmega3290::*; }

/// The module containing the values for the 'at90usb647' microcontroller
#[cfg(any(avr_mcu_at90usb647, feature = "all-mcus"))] pub mod at90usb647;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90usb647'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90usb647))] pub mod current { pub use super::at90usb647::*; }

/// The module containing the values for the 'atmega3250' microcontroller
#[cfg(any(avr_mcu_atmega3250, feature = "all-mcus"))] pub mod atmega3250;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3250'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3250))] pub mod current { pub use super::atmega3250::*; }

/// The module containing the values for the 'atmega644a' microcontroller
#[cfg(any(avr_mcu_atmega644a, feature = "all-mcus"))] pub mod atmega644a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega644a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega644a))] pub mod current { pub use super::atmega644a::*; }

/// The module containing the values for the 'atmega8' microcontroller
#[cfg(any(avr_mcu_atmega8, feature = "all-mcus"))] pub mod atmega8;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega8'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega8))] pub mod current { pub use super::atmega8::*; }

/// The module containing the values for the 'atmega3250a' microcontroller
#[cfg(any(avr_mcu_atmega3250a, feature = "all-mcus"))] pub mod atmega3250a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega3250a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega3250a))] pub mod current { pub use super::atmega3250a::*; }

/// The module containing the values for the 'atmega8a' microcontroller
#[cfg(any(avr_mcu_atmega8a, feature = "all-mcus"))] pub mod atmega8a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega8a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega8a))] pub mod current { pub use super::atmega8a::*; }

/// The module containing the values for the 'atmega32u4' microcontroller
#[cfg(any(avr_mcu_atmega32u4, feature = "all-mcus"))] pub mod atmega32u4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32u4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32u4))] pub mod current { pub use super::atmega32u4::*; }

/// The module containing the values for the 'atmega325' microcontroller
#[cfg(any(avr_mcu_atmega325, feature = "all-mcus"))] pub mod atmega325;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega325'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega325))] pub mod current { pub use super::atmega325::*; }

/// The module containing the values for the 'atmega16hvbrevb' microcontroller
#[cfg(any(avr_mcu_atmega16hvbrevb, feature = "all-mcus"))] pub mod atmega16hvbrevb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16hvbrevb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16hvbrevb))] pub mod current { pub use super::atmega16hvbrevb::*; }

/// The module containing the values for the 'atmega324a' microcontroller
#[cfg(any(avr_mcu_atmega324a, feature = "all-mcus"))] pub mod atmega324a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega324a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega324a))] pub mod current { pub use super::atmega324a::*; }

/// The module containing the values for the 'at90usb82' microcontroller
#[cfg(any(avr_mcu_at90usb82, feature = "all-mcus"))] pub mod at90usb82;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90usb82'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90usb82))] pub mod current { pub use super::at90usb82::*; }

/// The module containing the values for the 'atmega32hvb' microcontroller
#[cfg(any(avr_mcu_atmega32hvb, feature = "all-mcus"))] pub mod atmega32hvb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32hvb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32hvb))] pub mod current { pub use super::atmega32hvb::*; }

/// The module containing the values for the 'atmega88a' microcontroller
#[cfg(any(avr_mcu_atmega88a, feature = "all-mcus"))] pub mod atmega88a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega88a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega88a))] pub mod current { pub use super::atmega88a::*; }

/// The module containing the values for the 'atmega6490' microcontroller
#[cfg(any(avr_mcu_atmega6490, feature = "all-mcus"))] pub mod atmega6490;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega6490'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega6490))] pub mod current { pub use super::atmega6490::*; }

/// The module containing the values for the 'atmega64a' microcontroller
#[cfg(any(avr_mcu_atmega64a, feature = "all-mcus"))] pub mod atmega64a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega64a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega64a))] pub mod current { pub use super::atmega64a::*; }

/// The module containing the values for the 'atmega32' microcontroller
#[cfg(any(avr_mcu_atmega32, feature = "all-mcus"))] pub mod atmega32;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega32'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega32))] pub mod current { pub use super::atmega32::*; }

/// The module containing the values for the 'atmega324p' microcontroller
#[cfg(any(avr_mcu_atmega324p, feature = "all-mcus"))] pub mod atmega324p;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega324p'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega324p))] pub mod current { pub use super::atmega324p::*; }

/// The module containing the values for the 'atmega16u2' microcontroller
#[cfg(any(avr_mcu_atmega16u2, feature = "all-mcus"))] pub mod atmega16u2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega16u2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega16u2))] pub mod current { pub use super::atmega16u2::*; }

/// The module containing the values for the 'atmega169pa' microcontroller
#[cfg(any(avr_mcu_atmega169pa, feature = "all-mcus"))] pub mod atmega169pa;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega169pa'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega169pa))] pub mod current { pub use super::atmega169pa::*; }

/// The module containing the values for the 'at90can64' microcontroller
#[cfg(any(avr_mcu_at90can64, feature = "all-mcus"))] pub mod at90can64;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90can64'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90can64))] pub mod current { pub use super::at90can64::*; }

/// The module containing the values for the 'atmega808' microcontroller
#[cfg(any(avr_mcu_atmega808, feature = "all-mcus"))] pub mod atmega808;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega808'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega808))] pub mod current { pub use super::atmega808::*; }

/// The module containing the values for the 'at90pwm161' microcontroller
#[cfg(any(avr_mcu_at90pwm161, feature = "all-mcus"))] pub mod at90pwm161;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm161'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm161))] pub mod current { pub use super::at90pwm161::*; }

/// The module containing the values for the 'atmega6490a' microcontroller
#[cfg(any(avr_mcu_atmega6490a, feature = "all-mcus"))] pub mod atmega6490a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega6490a'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega6490a))] pub mod current { pub use super::atmega6490a::*; }

/// The module containing the values for the 'atmega64c1' microcontroller
#[cfg(any(avr_mcu_atmega64c1, feature = "all-mcus"))] pub mod atmega64c1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega64c1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega64c1))] pub mod current { pub use super::atmega64c1::*; }

/// The module containing the values for the 'atmega128rfr2' microcontroller
#[cfg(any(avr_mcu_atmega128rfr2, feature = "all-mcus"))] pub mod atmega128rfr2;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega128rfr2'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega128rfr2))] pub mod current { pub use super::atmega128rfr2::*; }

/// The module containing the values for the 'atmega324pb' microcontroller
#[cfg(any(avr_mcu_atmega324pb, feature = "all-mcus"))] pub mod atmega324pb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega324pb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega324pb))] pub mod current { pub use super::atmega324pb::*; }

/// The module containing the values for the 'atmega128rfa1' microcontroller
#[cfg(any(avr_mcu_atmega128rfa1, feature = "all-mcus"))] pub mod atmega128rfa1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega128rfa1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega128rfa1))] pub mod current { pub use super::atmega128rfa1::*; }

/// The module containing the values for the 'atmega48' microcontroller
#[cfg(any(avr_mcu_atmega48, feature = "all-mcus"))] pub mod atmega48;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega48'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega48))] pub mod current { pub use super::atmega48::*; }

/// The module containing the values for the 'atmega168' microcontroller
#[cfg(any(avr_mcu_atmega168, feature = "all-mcus"))] pub mod atmega168;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega168'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega168))] pub mod current { pub use super::atmega168::*; }

/// The module containing the values for the 'atmega48pb' microcontroller
#[cfg(any(avr_mcu_atmega48pb, feature = "all-mcus"))] pub mod atmega48pb;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atmega48pb'**.
#[cfg(all(target_arch = "avr", avr_mcu_atmega48pb))] pub mod current { pub use super::atmega48pb::*; }

/// The module containing the values for the 'at90pwm2b' microcontroller
#[cfg(any(avr_mcu_at90pwm2b, feature = "all-mcus"))] pub mod at90pwm2b;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'at90pwm2b'**.
#[cfg(all(target_arch = "avr", avr_mcu_at90pwm2b))] pub mod current { pub use super::at90pwm2b::*; }

/// The module containing the values for the 'attiny102' microcontroller
#[cfg(any(avr_mcu_attiny102, feature = "all-mcus"))] pub mod attiny102;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny102'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny102))] pub mod current { pub use super::attiny102::*; }

/// The module containing the values for the 'attiny13' microcontroller
#[cfg(any(avr_mcu_attiny13, feature = "all-mcus"))] pub mod attiny13;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny13'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny13))] pub mod current { pub use super::attiny13::*; }

/// The module containing the values for the 'attiny817' microcontroller
#[cfg(any(avr_mcu_attiny817, feature = "all-mcus"))] pub mod attiny817;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny817'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny817))] pub mod current { pub use super::attiny817::*; }

/// The module containing the values for the 'attiny1616' microcontroller
#[cfg(any(avr_mcu_attiny1616, feature = "all-mcus"))] pub mod attiny1616;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny1616'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny1616))] pub mod current { pub use super::attiny1616::*; }

/// The module containing the values for the 'attiny841' microcontroller
#[cfg(any(avr_mcu_attiny841, feature = "all-mcus"))] pub mod attiny841;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny841'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny841))] pub mod current { pub use super::attiny841::*; }

/// The module containing the values for the 'attiny12' microcontroller
#[cfg(any(avr_mcu_attiny12, feature = "all-mcus"))] pub mod attiny12;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny12'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny12))] pub mod current { pub use super::attiny12::*; }

/// The module containing the values for the 'attiny261' microcontroller
#[cfg(any(avr_mcu_attiny261, feature = "all-mcus"))] pub mod attiny261;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny261'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny261))] pub mod current { pub use super::attiny261::*; }

/// The module containing the values for the 'attiny212' microcontroller
#[cfg(any(avr_mcu_attiny212, feature = "all-mcus"))] pub mod attiny212;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny212'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny212))] pub mod current { pub use super::attiny212::*; }

/// The module containing the values for the 'attiny104' microcontroller
#[cfg(any(avr_mcu_attiny104, feature = "all-mcus"))] pub mod attiny104;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny104'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny104))] pub mod current { pub use super::attiny104::*; }

/// The module containing the values for the 'attiny417' microcontroller
#[cfg(any(avr_mcu_attiny417, feature = "all-mcus"))] pub mod attiny417;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny417'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny417))] pub mod current { pub use super::attiny417::*; }

/// The module containing the values for the 'attiny2313a' microcontroller
#[cfg(any(avr_mcu_attiny2313a, feature = "all-mcus"))] pub mod attiny2313a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny2313a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny2313a))] pub mod current { pub use super::attiny2313a::*; }

/// The module containing the values for the 'attiny3217' microcontroller
#[cfg(any(avr_mcu_attiny3217, feature = "all-mcus"))] pub mod attiny3217;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny3217'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny3217))] pub mod current { pub use super::attiny3217::*; }

/// The module containing the values for the 'attiny15' microcontroller
#[cfg(any(avr_mcu_attiny15, feature = "all-mcus"))] pub mod attiny15;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny15'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny15))] pub mod current { pub use super::attiny15::*; }

/// The module containing the values for the 'attiny85' microcontroller
#[cfg(any(avr_mcu_attiny85, feature = "all-mcus"))] pub mod attiny85;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny85'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny85))] pub mod current { pub use super::attiny85::*; }

/// The module containing the values for the 'attiny167' microcontroller
#[cfg(any(avr_mcu_attiny167, feature = "all-mcus"))] pub mod attiny167;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny167'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny167))] pub mod current { pub use super::attiny167::*; }

/// The module containing the values for the 'attiny43u' microcontroller
#[cfg(any(avr_mcu_attiny43u, feature = "all-mcus"))] pub mod attiny43u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny43u'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny43u))] pub mod current { pub use super::attiny43u::*; }

/// The module containing the values for the 'attiny814' microcontroller
#[cfg(any(avr_mcu_attiny814, feature = "all-mcus"))] pub mod attiny814;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny814'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny814))] pub mod current { pub use super::attiny814::*; }

/// The module containing the values for the 'attiny44a' microcontroller
#[cfg(any(avr_mcu_attiny44a, feature = "all-mcus"))] pub mod attiny44a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny44a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny44a))] pub mod current { pub use super::attiny44a::*; }

/// The module containing the values for the 'attiny416' microcontroller
#[cfg(any(avr_mcu_attiny416, feature = "all-mcus"))] pub mod attiny416;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny416'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny416))] pub mod current { pub use super::attiny416::*; }

/// The module containing the values for the 'attiny461' microcontroller
#[cfg(any(avr_mcu_attiny461, feature = "all-mcus"))] pub mod attiny461;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny461'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny461))] pub mod current { pub use super::attiny461::*; }

/// The module containing the values for the 'attiny84a' microcontroller
#[cfg(any(avr_mcu_attiny84a, feature = "all-mcus"))] pub mod attiny84a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny84a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny84a))] pub mod current { pub use super::attiny84a::*; }

/// The module containing the values for the 'attiny26' microcontroller
#[cfg(any(avr_mcu_attiny26, feature = "all-mcus"))] pub mod attiny26;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny26'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny26))] pub mod current { pub use super::attiny26::*; }

/// The module containing the values for the 'attiny25' microcontroller
#[cfg(any(avr_mcu_attiny25, feature = "all-mcus"))] pub mod attiny25;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny25'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny25))] pub mod current { pub use super::attiny25::*; }

/// The module containing the values for the 'attiny1617' microcontroller
#[cfg(any(avr_mcu_attiny1617, feature = "all-mcus"))] pub mod attiny1617;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny1617'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny1617))] pub mod current { pub use super::attiny1617::*; }

/// The module containing the values for the 'attiny1614' microcontroller
#[cfg(any(avr_mcu_attiny1614, feature = "all-mcus"))] pub mod attiny1614;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny1614'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny1614))] pub mod current { pub use super::attiny1614::*; }

/// The module containing the values for the 'attiny816' microcontroller
#[cfg(any(avr_mcu_attiny816, feature = "all-mcus"))] pub mod attiny816;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny816'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny816))] pub mod current { pub use super::attiny816::*; }

/// The module containing the values for the 'attiny828' microcontroller
#[cfg(any(avr_mcu_attiny828, feature = "all-mcus"))] pub mod attiny828;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny828'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny828))] pub mod current { pub use super::attiny828::*; }

/// The module containing the values for the 'attiny24' microcontroller
#[cfg(any(avr_mcu_attiny24, feature = "all-mcus"))] pub mod attiny24;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny24'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny24))] pub mod current { pub use super::attiny24::*; }

/// The module containing the values for the 'attiny441' microcontroller
#[cfg(any(avr_mcu_attiny441, feature = "all-mcus"))] pub mod attiny441;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny441'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny441))] pub mod current { pub use super::attiny441::*; }

/// The module containing the values for the 'attiny87' microcontroller
#[cfg(any(avr_mcu_attiny87, feature = "all-mcus"))] pub mod attiny87;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny87'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny87))] pub mod current { pub use super::attiny87::*; }

/// The module containing the values for the 'attiny3214' microcontroller
#[cfg(any(avr_mcu_attiny3214, feature = "all-mcus"))] pub mod attiny3214;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny3214'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny3214))] pub mod current { pub use super::attiny3214::*; }

/// The module containing the values for the 'attiny861a' microcontroller
#[cfg(any(avr_mcu_attiny861a, feature = "all-mcus"))] pub mod attiny861a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny861a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny861a))] pub mod current { pub use super::attiny861a::*; }

/// The module containing the values for the 'attiny2313' microcontroller
#[cfg(any(avr_mcu_attiny2313, feature = "all-mcus"))] pub mod attiny2313;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny2313'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny2313))] pub mod current { pub use super::attiny2313::*; }

/// The module containing the values for the 'attiny48' microcontroller
#[cfg(any(avr_mcu_attiny48, feature = "all-mcus"))] pub mod attiny48;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny48'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny48))] pub mod current { pub use super::attiny48::*; }

/// The module containing the values for the 'attiny45' microcontroller
#[cfg(any(avr_mcu_attiny45, feature = "all-mcus"))] pub mod attiny45;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny45'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny45))] pub mod current { pub use super::attiny45::*; }

/// The module containing the values for the 'attiny13a' microcontroller
#[cfg(any(avr_mcu_attiny13a, feature = "all-mcus"))] pub mod attiny13a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny13a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny13a))] pub mod current { pub use super::attiny13a::*; }

/// The module containing the values for the 'attiny5' microcontroller
#[cfg(any(avr_mcu_attiny5, feature = "all-mcus"))] pub mod attiny5;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny5'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny5))] pub mod current { pub use super::attiny5::*; }

/// The module containing the values for the 'attiny4' microcontroller
#[cfg(any(avr_mcu_attiny4, feature = "all-mcus"))] pub mod attiny4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny4'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny4))] pub mod current { pub use super::attiny4::*; }

/// The module containing the values for the 'attiny40' microcontroller
#[cfg(any(avr_mcu_attiny40, feature = "all-mcus"))] pub mod attiny40;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny40'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny40))] pub mod current { pub use super::attiny40::*; }

/// The module containing the values for the 'attiny11' microcontroller
#[cfg(any(avr_mcu_attiny11, feature = "all-mcus"))] pub mod attiny11;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny11'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny11))] pub mod current { pub use super::attiny11::*; }

/// The module containing the values for the 'attiny24a' microcontroller
#[cfg(any(avr_mcu_attiny24a, feature = "all-mcus"))] pub mod attiny24a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny24a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny24a))] pub mod current { pub use super::attiny24a::*; }

/// The module containing the values for the 'attiny840' microcontroller
#[cfg(any(avr_mcu_attiny840, feature = "all-mcus"))] pub mod attiny840;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny840'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny840))] pub mod current { pub use super::attiny840::*; }

/// The module containing the values for the 'attiny10' microcontroller
#[cfg(any(avr_mcu_attiny10, feature = "all-mcus"))] pub mod attiny10;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny10'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny10))] pub mod current { pub use super::attiny10::*; }

/// The module containing the values for the 'attiny80' microcontroller
#[cfg(any(avr_mcu_attiny80, feature = "all-mcus"))] pub mod attiny80;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny80'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny80))] pub mod current { pub use super::attiny80::*; }

/// The module containing the values for the 'attiny414' microcontroller
#[cfg(any(avr_mcu_attiny414, feature = "all-mcus"))] pub mod attiny414;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny414'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny414))] pub mod current { pub use super::attiny414::*; }

/// The module containing the values for the 'attiny4313' microcontroller
#[cfg(any(avr_mcu_attiny4313, feature = "all-mcus"))] pub mod attiny4313;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny4313'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny4313))] pub mod current { pub use super::attiny4313::*; }

/// The module containing the values for the 'attiny214' microcontroller
#[cfg(any(avr_mcu_attiny214, feature = "all-mcus"))] pub mod attiny214;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny214'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny214))] pub mod current { pub use super::attiny214::*; }

/// The module containing the values for the 'attiny261a' microcontroller
#[cfg(any(avr_mcu_attiny261a, feature = "all-mcus"))] pub mod attiny261a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny261a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny261a))] pub mod current { pub use super::attiny261a::*; }

/// The module containing the values for the 'attiny461a' microcontroller
#[cfg(any(avr_mcu_attiny461a, feature = "all-mcus"))] pub mod attiny461a;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny461a'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny461a))] pub mod current { pub use super::attiny461a::*; }

/// The module containing the values for the 'attiny44' microcontroller
#[cfg(any(avr_mcu_attiny44, feature = "all-mcus"))] pub mod attiny44;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny44'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny44))] pub mod current { pub use super::attiny44::*; }

/// The module containing the values for the 'attiny412' microcontroller
#[cfg(any(avr_mcu_attiny412, feature = "all-mcus"))] pub mod attiny412;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny412'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny412))] pub mod current { pub use super::attiny412::*; }

/// The module containing the values for the 'attiny20' microcontroller
#[cfg(any(avr_mcu_attiny20, feature = "all-mcus"))] pub mod attiny20;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny20'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny20))] pub mod current { pub use super::attiny20::*; }

/// The module containing the values for the 'attiny84' microcontroller
#[cfg(any(avr_mcu_attiny84, feature = "all-mcus"))] pub mod attiny84;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny84'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny84))] pub mod current { pub use super::attiny84::*; }

/// The module containing the values for the 'attiny9' microcontroller
#[cfg(any(avr_mcu_attiny9, feature = "all-mcus"))] pub mod attiny9;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny9'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny9))] pub mod current { pub use super::attiny9::*; }

/// The module containing the values for the 'attiny1634' microcontroller
#[cfg(any(avr_mcu_attiny1634, feature = "all-mcus"))] pub mod attiny1634;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny1634'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny1634))] pub mod current { pub use super::attiny1634::*; }

/// The module containing the values for the 'attiny861' microcontroller
#[cfg(any(avr_mcu_attiny861, feature = "all-mcus"))] pub mod attiny861;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny861'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny861))] pub mod current { pub use super::attiny861::*; }

/// The module containing the values for the 'attiny3216' microcontroller
#[cfg(any(avr_mcu_attiny3216, feature = "all-mcus"))] pub mod attiny3216;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny3216'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny3216))] pub mod current { pub use super::attiny3216::*; }

/// The module containing the values for the 'attiny88' microcontroller
#[cfg(any(avr_mcu_attiny88, feature = "all-mcus"))] pub mod attiny88;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'attiny88'**.
#[cfg(all(target_arch = "avr", avr_mcu_attiny88))] pub mod current { pub use super::attiny88::*; }

/// The module containing the values for the 'atxmega192a3' microcontroller
#[cfg(any(avr_mcu_atxmega192a3, feature = "all-mcus"))] pub mod atxmega192a3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega192a3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega192a3))] pub mod current { pub use super::atxmega192a3::*; }

/// The module containing the values for the 'atxmega32a4u' microcontroller
#[cfg(any(avr_mcu_atxmega32a4u, feature = "all-mcus"))] pub mod atxmega32a4u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32a4u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32a4u))] pub mod current { pub use super::atxmega32a4u::*; }

/// The module containing the values for the 'atxmega64a1u' microcontroller
#[cfg(any(avr_mcu_atxmega64a1u, feature = "all-mcus"))] pub mod atxmega64a1u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64a1u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64a1u))] pub mod current { pub use super::atxmega64a1u::*; }

/// The module containing the values for the 'atxmega16a4u' microcontroller
#[cfg(any(avr_mcu_atxmega16a4u, feature = "all-mcus"))] pub mod atxmega16a4u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega16a4u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega16a4u))] pub mod current { pub use super::atxmega16a4u::*; }

/// The module containing the values for the 'atxmega256a3' microcontroller
#[cfg(any(avr_mcu_atxmega256a3, feature = "all-mcus"))] pub mod atxmega256a3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega256a3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega256a3))] pub mod current { pub use super::atxmega256a3::*; }

/// The module containing the values for the 'atxmega128a3' microcontroller
#[cfg(any(avr_mcu_atxmega128a3, feature = "all-mcus"))] pub mod atxmega128a3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128a3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128a3))] pub mod current { pub use super::atxmega128a3::*; }

/// The module containing the values for the 'atxmega256a3b' microcontroller
#[cfg(any(avr_mcu_atxmega256a3b, feature = "all-mcus"))] pub mod atxmega256a3b;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega256a3b'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega256a3b))] pub mod current { pub use super::atxmega256a3b::*; }

/// The module containing the values for the 'atxmega256a3u' microcontroller
#[cfg(any(avr_mcu_atxmega256a3u, feature = "all-mcus"))] pub mod atxmega256a3u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega256a3u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega256a3u))] pub mod current { pub use super::atxmega256a3u::*; }

/// The module containing the values for the 'atxmega128a3u' microcontroller
#[cfg(any(avr_mcu_atxmega128a3u, feature = "all-mcus"))] pub mod atxmega128a3u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128a3u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128a3u))] pub mod current { pub use super::atxmega128a3u::*; }

/// The module containing the values for the 'atxmega256a3bu' microcontroller
#[cfg(any(avr_mcu_atxmega256a3bu, feature = "all-mcus"))] pub mod atxmega256a3bu;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega256a3bu'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega256a3bu))] pub mod current { pub use super::atxmega256a3bu::*; }

/// The module containing the values for the 'atxmega64a4u' microcontroller
#[cfg(any(avr_mcu_atxmega64a4u, feature = "all-mcus"))] pub mod atxmega64a4u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64a4u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64a4u))] pub mod current { pub use super::atxmega64a4u::*; }

/// The module containing the values for the 'atxmega16a4' microcontroller
#[cfg(any(avr_mcu_atxmega16a4, feature = "all-mcus"))] pub mod atxmega16a4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega16a4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega16a4))] pub mod current { pub use super::atxmega16a4::*; }

/// The module containing the values for the 'atxmega128a4u' microcontroller
#[cfg(any(avr_mcu_atxmega128a4u, feature = "all-mcus"))] pub mod atxmega128a4u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128a4u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128a4u))] pub mod current { pub use super::atxmega128a4u::*; }

/// The module containing the values for the 'atxmega64a3' microcontroller
#[cfg(any(avr_mcu_atxmega64a3, feature = "all-mcus"))] pub mod atxmega64a3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64a3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64a3))] pub mod current { pub use super::atxmega64a3::*; }

/// The module containing the values for the 'atxmega64a1' microcontroller
#[cfg(any(avr_mcu_atxmega64a1, feature = "all-mcus"))] pub mod atxmega64a1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64a1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64a1))] pub mod current { pub use super::atxmega64a1::*; }

/// The module containing the values for the 'atxmega32a4' microcontroller
#[cfg(any(avr_mcu_atxmega32a4, feature = "all-mcus"))] pub mod atxmega32a4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32a4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32a4))] pub mod current { pub use super::atxmega32a4::*; }

/// The module containing the values for the 'atxmega64a3u' microcontroller
#[cfg(any(avr_mcu_atxmega64a3u, feature = "all-mcus"))] pub mod atxmega64a3u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64a3u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64a3u))] pub mod current { pub use super::atxmega64a3u::*; }

/// The module containing the values for the 'atxmega192a3u' microcontroller
#[cfg(any(avr_mcu_atxmega192a3u, feature = "all-mcus"))] pub mod atxmega192a3u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega192a3u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega192a3u))] pub mod current { pub use super::atxmega192a3u::*; }

/// The module containing the values for the 'atxmega128a1u' microcontroller
#[cfg(any(avr_mcu_atxmega128a1u, feature = "all-mcus"))] pub mod atxmega128a1u;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128a1u'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128a1u))] pub mod current { pub use super::atxmega128a1u::*; }

/// The module containing the values for the 'atxmega128a1' microcontroller
#[cfg(any(avr_mcu_atxmega128a1, feature = "all-mcus"))] pub mod atxmega128a1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128a1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128a1))] pub mod current { pub use super::atxmega128a1::*; }

/// The module containing the values for the 'atxmega64b3' microcontroller
#[cfg(any(avr_mcu_atxmega64b3, feature = "all-mcus"))] pub mod atxmega64b3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64b3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64b3))] pub mod current { pub use super::atxmega64b3::*; }

/// The module containing the values for the 'atxmega64b1' microcontroller
#[cfg(any(avr_mcu_atxmega64b1, feature = "all-mcus"))] pub mod atxmega64b1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64b1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64b1))] pub mod current { pub use super::atxmega64b1::*; }

/// The module containing the values for the 'atxmega128b3' microcontroller
#[cfg(any(avr_mcu_atxmega128b3, feature = "all-mcus"))] pub mod atxmega128b3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128b3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128b3))] pub mod current { pub use super::atxmega128b3::*; }

/// The module containing the values for the 'atxmega128b1' microcontroller
#[cfg(any(avr_mcu_atxmega128b1, feature = "all-mcus"))] pub mod atxmega128b1;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128b1'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128b1))] pub mod current { pub use super::atxmega128b1::*; }

/// The module containing the values for the 'atxmega256c3' microcontroller
#[cfg(any(avr_mcu_atxmega256c3, feature = "all-mcus"))] pub mod atxmega256c3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega256c3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega256c3))] pub mod current { pub use super::atxmega256c3::*; }

/// The module containing the values for the 'atxmega32c3' microcontroller
#[cfg(any(avr_mcu_atxmega32c3, feature = "all-mcus"))] pub mod atxmega32c3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32c3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32c3))] pub mod current { pub use super::atxmega32c3::*; }

/// The module containing the values for the 'atxmega16c4' microcontroller
#[cfg(any(avr_mcu_atxmega16c4, feature = "all-mcus"))] pub mod atxmega16c4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega16c4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega16c4))] pub mod current { pub use super::atxmega16c4::*; }

/// The module containing the values for the 'atxmega384c3' microcontroller
#[cfg(any(avr_mcu_atxmega384c3, feature = "all-mcus"))] pub mod atxmega384c3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega384c3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega384c3))] pub mod current { pub use super::atxmega384c3::*; }

/// The module containing the values for the 'atxmega128c3' microcontroller
#[cfg(any(avr_mcu_atxmega128c3, feature = "all-mcus"))] pub mod atxmega128c3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128c3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128c3))] pub mod current { pub use super::atxmega128c3::*; }

/// The module containing the values for the 'atxmega32c4' microcontroller
#[cfg(any(avr_mcu_atxmega32c4, feature = "all-mcus"))] pub mod atxmega32c4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32c4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32c4))] pub mod current { pub use super::atxmega32c4::*; }

/// The module containing the values for the 'atxmega64c3' microcontroller
#[cfg(any(avr_mcu_atxmega64c3, feature = "all-mcus"))] pub mod atxmega64c3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64c3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64c3))] pub mod current { pub use super::atxmega64c3::*; }

/// The module containing the values for the 'atxmega192c3' microcontroller
#[cfg(any(avr_mcu_atxmega192c3, feature = "all-mcus"))] pub mod atxmega192c3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega192c3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega192c3))] pub mod current { pub use super::atxmega192c3::*; }

/// The module containing the values for the 'atxmega32d3' microcontroller
#[cfg(any(avr_mcu_atxmega32d3, feature = "all-mcus"))] pub mod atxmega32d3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32d3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32d3))] pub mod current { pub use super::atxmega32d3::*; }

/// The module containing the values for the 'atxmega64d4' microcontroller
#[cfg(any(avr_mcu_atxmega64d4, feature = "all-mcus"))] pub mod atxmega64d4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64d4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64d4))] pub mod current { pub use super::atxmega64d4::*; }

/// The module containing the values for the 'atxmega64d3' microcontroller
#[cfg(any(avr_mcu_atxmega64d3, feature = "all-mcus"))] pub mod atxmega64d3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega64d3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega64d3))] pub mod current { pub use super::atxmega64d3::*; }

/// The module containing the values for the 'atxmega16d4' microcontroller
#[cfg(any(avr_mcu_atxmega16d4, feature = "all-mcus"))] pub mod atxmega16d4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega16d4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega16d4))] pub mod current { pub use super::atxmega16d4::*; }

/// The module containing the values for the 'atxmega384d3' microcontroller
#[cfg(any(avr_mcu_atxmega384d3, feature = "all-mcus"))] pub mod atxmega384d3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega384d3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega384d3))] pub mod current { pub use super::atxmega384d3::*; }

/// The module containing the values for the 'atxmega128d4' microcontroller
#[cfg(any(avr_mcu_atxmega128d4, feature = "all-mcus"))] pub mod atxmega128d4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128d4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128d4))] pub mod current { pub use super::atxmega128d4::*; }

/// The module containing the values for the 'atxmega256d3' microcontroller
#[cfg(any(avr_mcu_atxmega256d3, feature = "all-mcus"))] pub mod atxmega256d3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega256d3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega256d3))] pub mod current { pub use super::atxmega256d3::*; }

/// The module containing the values for the 'atxmega32d4' microcontroller
#[cfg(any(avr_mcu_atxmega32d4, feature = "all-mcus"))] pub mod atxmega32d4;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32d4'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32d4))] pub mod current { pub use super::atxmega32d4::*; }

/// The module containing the values for the 'atxmega192d3' microcontroller
#[cfg(any(avr_mcu_atxmega192d3, feature = "all-mcus"))] pub mod atxmega192d3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega192d3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega192d3))] pub mod current { pub use super::atxmega192d3::*; }

/// The module containing the values for the 'atxmega128d3' microcontroller
#[cfg(any(avr_mcu_atxmega128d3, feature = "all-mcus"))] pub mod atxmega128d3;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega128d3'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega128d3))] pub mod current { pub use super::atxmega128d3::*; }

/// The module containing the values for the 'atxmega32e5' microcontroller
#[cfg(any(avr_mcu_atxmega32e5, feature = "all-mcus"))] pub mod atxmega32e5;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega32e5'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega32e5))] pub mod current { pub use super::atxmega32e5::*; }

/// The module containing the values for the 'atxmega16e5' microcontroller
#[cfg(any(avr_mcu_atxmega16e5, feature = "all-mcus"))] pub mod atxmega16e5;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega16e5'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega16e5))] pub mod current { pub use super::atxmega16e5::*; }

/// The module containing the values for the 'atxmega8e5' microcontroller
#[cfg(any(avr_mcu_atxmega8e5, feature = "all-mcus"))] pub mod atxmega8e5;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'atxmega8e5'**.
#[cfg(all(target_arch = "avr", avr_mcu_atxmega8e5))] pub mod current { pub use super::atxmega8e5::*; }

/// The module containing the values for the 'ata5790n' microcontroller
#[cfg(any(avr_mcu_ata5790n, feature = "all-mcus"))] pub mod ata5790n;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5790n'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5790n))] pub mod current { pub use super::ata5790n::*; }

/// The module containing the values for the 'ata5783' microcontroller
#[cfg(any(avr_mcu_ata5783, feature = "all-mcus"))] pub mod ata5783;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5783'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5783))] pub mod current { pub use super::ata5783::*; }

/// The module containing the values for the 'ata5787' microcontroller
#[cfg(any(avr_mcu_ata5787, feature = "all-mcus"))] pub mod ata5787;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5787'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5787))] pub mod current { pub use super::ata5787::*; }

/// The module containing the values for the 'ata5781' microcontroller
#[cfg(any(avr_mcu_ata5781, feature = "all-mcus"))] pub mod ata5781;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5781'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5781))] pub mod current { pub use super::ata5781::*; }

/// The module containing the values for the 'ata5831' microcontroller
#[cfg(any(avr_mcu_ata5831, feature = "all-mcus"))] pub mod ata5831;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5831'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5831))] pub mod current { pub use super::ata5831::*; }

/// The module containing the values for the 'ata8210' microcontroller
#[cfg(any(avr_mcu_ata8210, feature = "all-mcus"))] pub mod ata8210;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata8210'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata8210))] pub mod current { pub use super::ata8210::*; }

/// The module containing the values for the 'ata6613c' microcontroller
#[cfg(any(avr_mcu_ata6613c, feature = "all-mcus"))] pub mod ata6613c;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6613c'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6613c))] pub mod current { pub use super::ata6613c::*; }

/// The module containing the values for the 'ata8515' microcontroller
#[cfg(any(avr_mcu_ata8515, feature = "all-mcus"))] pub mod ata8515;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata8515'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata8515))] pub mod current { pub use super::ata8515::*; }

/// The module containing the values for the 'ata8510' microcontroller
#[cfg(any(avr_mcu_ata8510, feature = "all-mcus"))] pub mod ata8510;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata8510'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata8510))] pub mod current { pub use super::ata8510::*; }

/// The module containing the values for the 'ata6286' microcontroller
#[cfg(any(avr_mcu_ata6286, feature = "all-mcus"))] pub mod ata6286;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6286'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6286))] pub mod current { pub use super::ata6286::*; }

/// The module containing the values for the 'ata5505' microcontroller
#[cfg(any(avr_mcu_ata5505, feature = "all-mcus"))] pub mod ata5505;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5505'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5505))] pub mod current { pub use super::ata5505::*; }

/// The module containing the values for the 'ata5795' microcontroller
#[cfg(any(avr_mcu_ata5795, feature = "all-mcus"))] pub mod ata5795;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5795'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5795))] pub mod current { pub use super::ata5795::*; }

/// The module containing the values for the 'ata5782' microcontroller
#[cfg(any(avr_mcu_ata5782, feature = "all-mcus"))] pub mod ata5782;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5782'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5782))] pub mod current { pub use super::ata5782::*; }

/// The module containing the values for the 'ata5833' microcontroller
#[cfg(any(avr_mcu_ata5833, feature = "all-mcus"))] pub mod ata5833;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5833'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5833))] pub mod current { pub use super::ata5833::*; }

/// The module containing the values for the 'ata6616c' microcontroller
#[cfg(any(avr_mcu_ata6616c, feature = "all-mcus"))] pub mod ata6616c;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6616c'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6616c))] pub mod current { pub use super::ata6616c::*; }

/// The module containing the values for the 'ata6612c' microcontroller
#[cfg(any(avr_mcu_ata6612c, feature = "all-mcus"))] pub mod ata6612c;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6612c'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6612c))] pub mod current { pub use super::ata6612c::*; }

/// The module containing the values for the 'ata5790' microcontroller
#[cfg(any(avr_mcu_ata5790, feature = "all-mcus"))] pub mod ata5790;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5790'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5790))] pub mod current { pub use super::ata5790::*; }

/// The module containing the values for the 'ata8215' microcontroller
#[cfg(any(avr_mcu_ata8215, feature = "all-mcus"))] pub mod ata8215;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata8215'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata8215))] pub mod current { pub use super::ata8215::*; }

/// The module containing the values for the 'ata6614q' microcontroller
#[cfg(any(avr_mcu_ata6614q, feature = "all-mcus"))] pub mod ata6614q;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6614q'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6614q))] pub mod current { pub use super::ata6614q::*; }

/// The module containing the values for the 'ata5835' microcontroller
#[cfg(any(avr_mcu_ata5835, feature = "all-mcus"))] pub mod ata5835;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5835'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5835))] pub mod current { pub use super::ata5835::*; }

/// The module containing the values for the 'ata6617c' microcontroller
#[cfg(any(avr_mcu_ata6617c, feature = "all-mcus"))] pub mod ata6617c;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6617c'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6617c))] pub mod current { pub use super::ata6617c::*; }

/// The module containing the values for the 'ata5272' microcontroller
#[cfg(any(avr_mcu_ata5272, feature = "all-mcus"))] pub mod ata5272;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5272'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5272))] pub mod current { pub use super::ata5272::*; }

/// The module containing the values for the 'ata5791' microcontroller
#[cfg(any(avr_mcu_ata5791, feature = "all-mcus"))] pub mod ata5791;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5791'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5791))] pub mod current { pub use super::ata5791::*; }

/// The module containing the values for the 'ata5702m322' microcontroller
#[cfg(any(avr_mcu_ata5702m322, feature = "all-mcus"))] pub mod ata5702m322;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5702m322'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5702m322))] pub mod current { pub use super::ata5702m322::*; }

/// The module containing the values for the 'ata5700m322' microcontroller
#[cfg(any(avr_mcu_ata5700m322, feature = "all-mcus"))] pub mod ata5700m322;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5700m322'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5700m322))] pub mod current { pub use super::ata5700m322::*; }

/// The module containing the values for the 'ata664251' microcontroller
#[cfg(any(avr_mcu_ata664251, feature = "all-mcus"))] pub mod ata664251;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata664251'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata664251))] pub mod current { pub use super::ata664251::*; }

/// The module containing the values for the 'ata6285' microcontroller
#[cfg(any(avr_mcu_ata6285, feature = "all-mcus"))] pub mod ata6285;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata6285'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata6285))] pub mod current { pub use super::ata6285::*; }

/// The module containing the values for the 'ata5832' microcontroller
#[cfg(any(avr_mcu_ata5832, feature = "all-mcus"))] pub mod ata5832;
/// Contains definitions for the current AVR device being targeted. **This is currently the 'ata5832'**.
#[cfg(all(target_arch = "avr", avr_mcu_ata5832))] pub mod current { pub use super::ata5832::*; }


