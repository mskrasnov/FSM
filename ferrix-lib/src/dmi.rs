/* dmi.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! DMI table parser. Uses `smbios-lib` to data provider
//!
//! ## Usage
//!
//! ```no-test
//! use ferrix::dmi::DMITable;
//! let dmi = DMITable::new().unwrap();
//! dbg!(dmi);
//! ```

use std::fmt::Display;

use crate::traits::ToJson;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
pub use smbioslib::SMBiosData;

/// A structure containing data from the DMI table
///
/// ## Usage
///
/// ```no-test
/// use ferrix::dmi::DMITable;
/// let dmi = DMITable::new().unwrap();
/// dbg!(dmi);
/// ```
#[derive(Debug, Serialize, Clone)]
pub struct DMITable {
    /// Information about BIOS (Type 0)
    pub bios: Bios,

    /// Information about system (Type 1)
    pub system: System,

    /// Information about baseboard (or module) - Type 2
    pub baseboard: Baseboard,

    /// Information about system enclosure or chassis - Type 3
    pub chassis: Chassis,

    /// Information about processor - Type 4
    pub processor: Processor,

    // /// Information about memory controller (Type 5)
    // pub memory_controller: MemoryController,
    //
    // /// Information about memory module (Type 6)
    // pub memory_module: MemoryModule,
    /// Information about CPU cache (Type 7)
    // pub caches: Caches,

    /// Information about port connectors (Type 8)
    // pub ports: PortConnectors,

    /// Information about physical memory array (Type 16)
    // pub mem_array: MemoryArray,

    /// Information about installed memory devices (Type 17)
    pub mem_devices: MemoryDevices,
}

impl DMITable {
    /// Get information from DMI table
    ///
    /// > **NOTE:** This data DOES NOT NEED to be updated periodically!
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Ok(Self {
            bios: Bios::new_from_table(&table)?,
            system: System::new_from_table(&table)?,
            baseboard: Baseboard::new_from_table(&table)?,
            chassis: Chassis::new_from_table(&table)?,
            processor: Processor::new_from_table(&table)?,
            // caches: Caches::new_from_table(&table)?,
            // ports: PortConnectors::new_from_table(&table)?,
            // mem_array: MemoryArray::new_from_table(&table)?,
            mem_devices: MemoryDevices::new_from_table(&table)?,
        })
    }

    /// Performs serialization of structure data in JSON.
    ///
    /// The returned value will be a SINGLE LINE of JSON data
    /// intended for reading by third-party software or for
    /// transmission over the network.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Performs serialization in "pretty" JSON
    ///
    /// JSON will contain unnecessary newline transitions and spaces
    /// to visually separate the blocks. It is well suited for human
    /// reading and analysis.
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    /// Performs data serialization in XML format
    pub fn to_xml(&self) -> Result<String> {
        let xml = DMITableXml::from(self);
        xml.to_xml()
    }
}

impl ToJson for DMITable {}

/****************************** NOTE *********************************
 * Дичайший костыль для того, чтобы структура XML была корректной    *
 * Если не обернуть данные в поле "hardware" (тег <hardware> в XML), *
 * то итоговый XML просто не распарсится.                            *
 *********************************************************************/
#[derive(Serialize, Clone)]
pub struct DMITableXml<'a> {
    pub hardware: &'a DMITable,
}

impl<'a> DMITableXml<'a> {
    pub fn to_xml(&self) -> Result<String> {
        Ok(xml_serde::to_string(&self)?)
    }
}

impl<'a> From<&'a DMITable> for DMITableXml<'a> {
    fn from(value: &'a DMITable) -> Self {
        Self { hardware: value }
    }
}

macro_rules! impl_from_struct {
    ($s:ident, $p:path, {
        $(
            $field_name:ident : $field_type:ty
        ),* $(,)?
    }) => {
        impl From<$p> for $s {
            fn from(value: $p) -> Self {
                Self {
                    $(
                        $field_name: value.$field_name,
                    )*
                }
            }
        }

        impl ToJson for $s {}
    };
}

/// Each SMBIOS structure has a handle or instance value associated
/// with it. Some structs will reference other structures by using
/// this value.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Handle(pub u16);

impl From<smbioslib::Handle> for Handle {
    fn from(value: smbioslib::Handle) -> Self {
        Self(value.0)
    }
}

impl Handle {
    pub fn from_opt(opt: Option<smbioslib::Handle>) -> Option<Self> {
        match opt {
            Some(handle) => Some(Handle::from(handle)),
            None => None,
        }
    }
}

impl Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// BIOS ROM Size
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RomSize {
    /// Size of this ROM in bytes
    Kilobytes(u16),

    /// Extended size of the physical device(s) containing the
    /// BIOS (in MB)
    Megabytes(u16),

    /// Extended size of the physical device(s) containing the
    /// BIOS (in GB)
    Gigabytes(u16),

    /// Extended size of the physical device(s) containing the
    /// BIOS in raw form.
    ///
    /// The standard currently only defines MB and GB as given
    /// in the high nibble (bits 15-14).
    Undefined(u16),

    SeeExtendedRomSize,
}

impl From<smbioslib::RomSize> for RomSize {
    fn from(value: smbioslib::RomSize) -> Self {
        match value {
            smbioslib::RomSize::Kilobytes(s) => Self::Kilobytes(s),
            smbioslib::RomSize::Megabytes(s) => Self::Megabytes(s),
            smbioslib::RomSize::Gigabytes(s) => Self::Gigabytes(s),
            smbioslib::RomSize::Undefined(s) => Self::Undefined(s),
            smbioslib::RomSize::SeeExtendedRomSize => Self::SeeExtendedRomSize,
        }
    }
}

impl Display for RomSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Kilobytes(n) => format!("{n} KB"),
                Self::Megabytes(n) => format!("{n} MB"),
                Self::Gigabytes(n) => format!("{n} GB"),
                Self::Undefined(n) => format!("{n} ??"),
                Self::SeeExtendedRomSize => format!("see extended ROM size"),
            }
        )
    }
}

/// Information about BIOS/UEFI
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bios {
    /// BIOS vendor's name
    pub vendor: Option<String>,

    /// BIOS version
    pub version: Option<String>,

    /// BIOS starting address segment
    pub starting_address_segment: Option<u16>,

    /// BIOS release date
    pub release_date: Option<String>,

    /// BIOS ROM size
    pub rom_size: Option<RomSize>,

    /// BIOS characteristics
    pub characteristics: Option<BiosCharacteristics>,

    /// BIOS vendor reserved characteristics
    pub bios_vendor_reserved_characteristics: Option<u16>,

    /// System vendor reserved characteristics
    pub system_vendor_reserved_characteristics: Option<u16>,

    /// Characteristics extension byte 0
    pub characteristics_extension0: Option<BiosCharacteristicsExtension0>,

    /// Characteristics extension byte 1
    pub characteristics_extension1: Option<BiosCharacteristicsExtension1>,

    /// System BIOS major release
    pub system_bios_major_release: Option<u8>,

    /// System BIOS minor release
    pub system_bios_minor_release: Option<u8>,

    /// Embedded controller firmware major release
    pub e_c_firmware_major_release: Option<u8>,

    /// Embedded controller firmware minor release
    pub e_c_firmware_minor_release: Option<u8>,

    /// Extended BIOS ROM size
    pub extended_rom_size: Option<RomSize>,
}

impl Bios {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// [`Self::new_from_table(table: &SMBiosData)`] in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosInformation| Some(f))
            .ok_or(anyhow!("Failed to get information about BIOS (type 0)!"))?;

        Ok(Self {
            vendor: t.vendor().ok(),
            version: t.version().ok(),
            starting_address_segment: t.starting_address_segment(),
            release_date: t.release_date().ok(),
            rom_size: match t.rom_size() {
                Some(s) => Some(RomSize::from(s)),
                None => None,
            },
            characteristics: match t.characteristics() {
                Some(c) => Some(BiosCharacteristics::from(c)),
                None => None,
            },
            bios_vendor_reserved_characteristics: t.bios_vendor_reserved_characteristics(),
            system_vendor_reserved_characteristics: t.system_vendor_reserved_characteristics(),
            characteristics_extension0: match t.characteristics_extension0() {
                Some(ce0) => Some(BiosCharacteristicsExtension0::from(ce0)),
                None => None,
            },
            characteristics_extension1: match t.characteristics_extension1() {
                Some(ce1) => Some(BiosCharacteristicsExtension1::from(ce1)),
                None => None,
            },
            system_bios_major_release: t.system_bios_major_release(),
            system_bios_minor_release: t.system_bios_minor_release(),
            e_c_firmware_major_release: t.e_c_firmware_major_release(),
            e_c_firmware_minor_release: t.e_c_firmware_minor_release(),
            extended_rom_size: match t.extended_rom_size() {
                Some(s) => Some(RomSize::from(s)),
                None => None,
            },
        })
    }
}

impl ToJson for Bios {}

/// BIOS characteristics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiosCharacteristics {
    /// Unknown
    pub unknown: bool,

    /// BIOS Characteristics aren't supported
    pub bios_characteristics_not_supported: bool,

    /// ISA is supported
    pub isa_supported: bool,

    /// MCA is supported
    pub mca_supported: bool,

    /// EISA is supported
    pub eisa_supported: bool,

    /// PCI is supported
    pub pci_supported: bool,

    /// PCMCIA is supported
    pub pcmcia_supported: bool,

    /// Plug-n-play is supported
    pub plug_and_play_supported: bool,

    /// APM is supported
    pub apm_supported: bool,

    /// BIOS is upgradeable (Flash)
    pub bios_upgradeable: bool,

    /// BIOS shadowing is allowed
    pub bios_shadowing_allowed: bool,

    /// VL-VESA is supported
    pub vlvesa_supported: bool,

    /// ESCD support is available
    pub escd_support_available: bool,

    /// Boot from CD is supported
    pub boot_from_cdsupported: bool,

    /// Selectable boot is supported
    pub selectable_boot_supported: bool,

    /// BIOS ROM is socketed (e.g. PLCC or SOP socket)
    pub bios_rom_socketed: bool,

    /// Boot from PCMCIA is supported
    pub boot_from_pcmcia_supported: bool,

    /// EDD specification is supported
    pub edd_specification_supported: bool,

    /// Japanese floppy for NEC 9800 1.2 MB (3.5", 1K bytes/sector,
    /// 360 RPM) is supported
    pub floppy_nec_japanese_supported: bool,

    /// Japanese floppy for Toshiba 1.2 MB (3.5", 360 RPM) is
    /// supported
    pub floppy_toshiba_japanese_supported: bool,

    /// 5.25" / 360 KB floppy services are supported
    pub floppy_525_360_supported: bool,

    /// 5.25" / 1.2 MB floppy services are supported
    pub floppy_525_12_supported: bool,

    /// 3.5" / 720 KB floppy services are supported
    pub floppy_35_720_supported: bool,

    /// 3.5" 2.88 MB floppy services are supported
    pub floppy_35_288_supported: bool,

    /// PrintScreen service is supported
    pub print_screen_service_supported: bool,

    /// 8042 keyboard services are supported
    pub keyboard_8042services_supported: bool,

    /// Serial services are supported
    pub serial_services_supported: bool,

    /// Printer services are supported
    pub printer_services_supported: bool,

    /// CGA/Mono Video Services are supported
    pub cga_mono_video_services_supported: bool,

    /// NEC PC-98 supported
    pub nec_pc_98supported: bool,
}

impl From<smbioslib::BiosCharacteristics> for BiosCharacteristics {
    fn from(value: smbioslib::BiosCharacteristics) -> Self {
        Self {
            unknown: value.unknown(),
            bios_characteristics_not_supported: value.bios_characteristics_not_supported(),
            isa_supported: value.isa_supported(),
            mca_supported: value.mca_supported(),
            eisa_supported: value.eisa_supported(),
            pci_supported: value.pci_supported(),
            pcmcia_supported: value.pcmcia_supported(),
            plug_and_play_supported: value.plug_and_play_supported(),
            apm_supported: value.apm_supported(),
            bios_upgradeable: value.bios_upgradeable(),
            bios_shadowing_allowed: value.bios_shadowing_allowed(),
            vlvesa_supported: value.vlvesa_supported(),
            escd_support_available: value.escd_support_available(),
            boot_from_cdsupported: value.boot_from_cdsupported(),
            selectable_boot_supported: value.selectable_boot_supported(),
            bios_rom_socketed: value.bios_rom_socketed(),
            boot_from_pcmcia_supported: value.boot_from_pcmcia_supported(),
            edd_specification_supported: value.edd_specification_supported(),
            floppy_nec_japanese_supported: value.floppy_nec_japanese_supported(),
            floppy_toshiba_japanese_supported: value.floppy_toshiba_japanese_supported(),
            floppy_525_360_supported: value.floppy_525_360_supported(),
            floppy_525_12_supported: value.floppy_525_12_supported(),
            floppy_35_720_supported: value.floppy_35_720_supported(),
            floppy_35_288_supported: value.floppy_35_288_supported(),
            print_screen_service_supported: value.print_screen_service_supported(),
            keyboard_8042services_supported: value.keyboard_8042services_supported(),
            serial_services_supported: value.serial_services_supported(),
            printer_services_supported: value.printer_services_supported(),
            cga_mono_video_services_supported: value.cga_mono_video_services_supported(),
            nec_pc_98supported: value.nec_pc_98supported(),
        }
    }
}
impl ToJson for BiosCharacteristics {}

/// Characteristics extension byte 0
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiosCharacteristicsExtension0 {
    /// ACPI is supported
    pub acpi_is_supported: bool,

    /// USB Legacy is supported
    pub usb_legacy_is_supported: bool,

    /// AGP is supported
    pub agp_is_supported: bool,

    /// I2O boot is supported
    pub i2oboot_is_supported: bool,

    /// LS-120 SuperDisk boot is supported
    pub ls120super_disk_boot_is_supported: bool,

    /// ATAPI ZIP drive boot is supported
    pub atapi_zip_drive_boot_is_supported: bool,

    /// 1394 boot is supported
    pub boot_1394is_supported: bool,

    /// Smart battery is supported
    pub smart_battery_is_supported: bool,
}

impl From<smbioslib::BiosCharacteristicsExtension0> for BiosCharacteristicsExtension0 {
    fn from(value: smbioslib::BiosCharacteristicsExtension0) -> Self {
        Self {
            acpi_is_supported: value.acpi_is_supported(),
            usb_legacy_is_supported: value.usb_legacy_is_supported(),
            agp_is_supported: value.agp_is_supported(),
            i2oboot_is_supported: value.i2oboot_is_supported(),
            ls120super_disk_boot_is_supported: value.ls120super_disk_boot_is_supported(),
            atapi_zip_drive_boot_is_supported: value.atapi_zip_drive_boot_is_supported(),
            boot_1394is_supported: value.boot_1394is_supported(),
            smart_battery_is_supported: value.smart_battery_is_supported(),
        }
    }
}
impl ToJson for BiosCharacteristicsExtension0 {}

/// Characteristics extension byte 0
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiosCharacteristicsExtension1 {
    /// BIOS Boot Specification is supported
    pub bios_boot_specification_is_supported: bool,

    /// Function key-initiated network service boot is supported.
    /// When function key-uninitiated network service boot is not
    /// supported, a network adapter option ROM may choose to offer
    /// this functionality on its own, thus offering this capability
    /// to legacy systems. When the function is supported, the
    /// network adapter option ROM shall not offer this capability
    pub fkey_initiated_network_boot_is_supported: bool,

    /// Enable targeted content distribution. The manufacturer has
    /// ensured that the SMBIOS data is useful in identifying the
    /// computer for targeted delivery of model-specific software
    /// and firmware content through third-party content
    /// distribution services
    pub targeted_content_distribution_is_supported: bool,

    /// UEFI Specification is supported
    pub uefi_specification_is_supported: bool,

    /// SMBIOS table describes a virtual machine
    pub smbios_table_describes_avirtual_machine: bool,

    /// Manufacturing mode is supported. (Manufacturing mode is a
    /// special boot mode, not normally available to end users, that
    /// modifies BIOS features and settings for use while the
    /// computer is being manufactured and tested)
    pub manufacturing_mode_is_supported: bool,

    /// Manufacturing mode is enabled
    pub manufacturing_mode_is_enabled: bool,
}

impl From<smbioslib::BiosCharacteristicsExtension1> for BiosCharacteristicsExtension1 {
    fn from(value: smbioslib::BiosCharacteristicsExtension1) -> Self {
        Self {
            bios_boot_specification_is_supported: value.bios_boot_specification_is_supported(),
            fkey_initiated_network_boot_is_supported: value
                .fkey_initiated_network_boot_is_supported(),
            targeted_content_distribution_is_supported: value
                .targeted_content_distribution_is_supported(),
            uefi_specification_is_supported: value.uefi_specification_is_supported(),
            smbios_table_describes_avirtual_machine: value
                .smbios_table_describes_avirtual_machine(),
            manufacturing_mode_is_supported: value.manufacturing_mode_is_supported(),
            manufacturing_mode_is_enabled: value.manufacturing_mode_is_enabled(),
        }
    }
}
impl ToJson for BiosCharacteristicsExtension1 {}

/// System UUID Data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SystemUuidData {
    IdNotPresentButSettable,
    IdNotPresent,
    Uuid(SystemUuid),
}

impl From<smbioslib::SystemUuidData> for SystemUuidData {
    fn from(value: smbioslib::SystemUuidData) -> Self {
        match value {
            smbioslib::SystemUuidData::IdNotPresentButSettable => Self::IdNotPresentButSettable,
            smbioslib::SystemUuidData::IdNotPresent => Self::IdNotPresent,
            smbioslib::SystemUuidData::Uuid(u) => Self::Uuid(SystemUuid::from(u)),
        }
    }
}

impl Display for SystemUuidData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IdNotPresentButSettable => format!("ID not present but settable"),
                Self::IdNotPresent => format!("ID not present"),
                Self::Uuid(uuid) => format!("{uuid}"),
            }
        )
    }
}

/// System UUID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemUuid {
    /// Raw byte array for this UUID
    pub raw: [u8; 16],
}

impl_from_struct!(SystemUuid, smbioslib::SystemUuid, {
    raw: [u8; 16],
});

impl Display for SystemUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_low = u32::from_le_bytes([self.raw[0], self.raw[1], self.raw[2], self.raw[3]]);
        let time_mid = u16::from_le_bytes([self.raw[4], self.raw[5]]);
        let time_hi = u16::from_le_bytes([self.raw[6], self.raw[7]]);

        let mut s = format!("{:08x}-{:04x}-{:04x}-", time_low, time_mid, time_hi);
        let mut i = 8;

        while i <= 9 {
            s.push_str(&format!("{:02x}", self.raw[i]));
            i += 1;
        }
        s.push('-');
        while i < 16 {
            s.push_str(&format!("{:02x}", self.raw[i]));
            i += 1;
        }
        write!(f, "{s}",)
    }
}

/// System wakeup data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemWakeUpTypeData {
    /// Raw value
    ///
    /// Is most usable when `value` is None
    pub raw: u8,

    pub value: SystemWakeUpType,
}

impl From<smbioslib::SystemWakeUpTypeData> for SystemWakeUpTypeData {
    fn from(value: smbioslib::SystemWakeUpTypeData) -> Self {
        Self {
            raw: value.raw,
            value: SystemWakeUpType::from(value.value),
        }
    }
}

/// System wakeup type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SystemWakeUpType {
    Other,
    Unknown,
    ApmTimer,
    ModernRing,
    LanRemote,
    PowerSwitch,
    PciPme,
    ACPowerRestored,
    None,
}

impl From<smbioslib::SystemWakeUpType> for SystemWakeUpType {
    fn from(value: smbioslib::SystemWakeUpType) -> Self {
        match value {
            smbioslib::SystemWakeUpType::Other => Self::Other,
            smbioslib::SystemWakeUpType::Unknown => Self::Unknown,
            smbioslib::SystemWakeUpType::ApmTimer => Self::ApmTimer,
            smbioslib::SystemWakeUpType::ModernRing => Self::ModernRing,
            smbioslib::SystemWakeUpType::LanRemote => Self::LanRemote,
            smbioslib::SystemWakeUpType::PowerSwitch => Self::PowerSwitch,
            smbioslib::SystemWakeUpType::PciPme => Self::PciPme,
            smbioslib::SystemWakeUpType::ACPowerRestored => Self::ACPowerRestored,
            smbioslib::SystemWakeUpType::None => Self::None,
        }
    }
}

impl Display for SystemWakeUpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::ApmTimer => "APM Timer",
                Self::ModernRing => "Modern Ring",
                Self::LanRemote => "LAN Remote",
                Self::PowerSwitch => "Power Switch",
                Self::PciPme => "PCI PME#",
                Self::ACPowerRestored => "AC Power Restored",
                Self::None => "Unknown to this standard, check the raw value",
            }
        )
    }
}

/// Attributes of the overall system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    /// System manufacturer
    pub manufacturer: Option<String>,

    /// System product name
    pub product_name: Option<String>,

    /// System version
    pub version: Option<String>,

    /// Serial number
    pub serial_number: Option<String>,

    /// System UUID
    pub uuid: Option<SystemUuidData>,

    /// Wake-up type
    ///
    /// Identifies the event that caused the system to power up
    pub wakeup_type: Option<SystemWakeUpTypeData>,

    /// SKU Number (particular computer information for sale.
    /// Also called a product ID or purchase order number.
    /// Typically for a given system board from a given OEM,
    /// there are tens of unique processor, memory, hard
    /// drive, and optical drive configurations).
    pub sku_number: Option<String>,

    /// Family to which a particular computer belongs
    pub family: Option<String>,
}

impl System {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosSystemInformation| Some(f))
            .ok_or(anyhow!("Failed to get information about system (type 1)!"))?;

        Ok(Self {
            manufacturer: t.manufacturer().ok(),
            product_name: t.product_name().ok(),
            version: t.version().ok(),
            serial_number: t.serial_number().ok(),
            uuid: match t.uuid() {
                Some(u) => Some(SystemUuidData::from(u)),
                None => None,
            },
            wakeup_type: match t.wakeup_type() {
                Some(wt) => Some(SystemWakeUpTypeData::from(wt)),
                None => None,
            },
            sku_number: t.sku_number().ok(),
            family: t.family().ok(),
        })
    }
}

impl ToJson for System {}

/// Board type data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoardTypeData {
    pub raw: u8,
    pub value: BoardType,
}

impl From<smbioslib::BoardTypeData> for BoardTypeData {
    fn from(value: smbioslib::BoardTypeData) -> Self {
        Self {
            raw: value.raw,
            value: BoardType::from(value.value),
        }
    }
}

/// Board type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BoardType {
    Unknown,
    Other,
    ServerBlade,
    ConnectivitySwitch,
    SystemManagementModule,
    ProcessorModule,
    IOModule,
    MemoryModule,
    Daughterboard,
    Motherboard,
    ProcessorMemoryModule,
    ProcessorIOModule,
    InterconnectBoard,
    None,
}

impl From<smbioslib::BoardType> for BoardType {
    fn from(value: smbioslib::BoardType) -> Self {
        match value {
            smbioslib::BoardType::Unknown => Self::Unknown,
            smbioslib::BoardType::Other => Self::Other,
            smbioslib::BoardType::ServerBlade => Self::ServerBlade,
            smbioslib::BoardType::ConnectivitySwitch => Self::ConnectivitySwitch,
            smbioslib::BoardType::SystemManagementModule => Self::SystemManagementModule,
            smbioslib::BoardType::ProcessorModule => Self::ProcessorModule,
            smbioslib::BoardType::IOModule => Self::IOModule,
            smbioslib::BoardType::MemoryModule => Self::MemoryModule,
            smbioslib::BoardType::Daughterboard => Self::Daughterboard,
            smbioslib::BoardType::Motherboard => Self::Motherboard,
            smbioslib::BoardType::ProcessorMemoryModule => Self::ProcessorMemoryModule,
            smbioslib::BoardType::ProcessorIOModule => Self::ProcessorIOModule,
            smbioslib::BoardType::InterconnectBoard => Self::InterconnectBoard,
            smbioslib::BoardType::None => Self::None,
        }
    }
}

impl Display for BoardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "Unknown",
                Self::Other => "Other",
                Self::ServerBlade => "Server Blade",
                Self::ConnectivitySwitch => "Connectivity Switch",
                Self::SystemManagementModule => "System Management Module",
                Self::ProcessorModule => "Processor Module",
                Self::IOModule => "I/O Module",
                Self::MemoryModule => "Memory Module",
                Self::Daughterboard => "Daughter Board",
                Self::Motherboard => "Motherboard (includes processor, memory, and I/O)",
                Self::ProcessorMemoryModule => "Processor or Memory Module",
                Self::ProcessorIOModule => "Processor or I/O Module",
                Self::InterconnectBoard => "Interconnect Board",
                Self::None => "Unknown to this standard, check the raw value",
            }
        )
    }
}

/// Information about baseboard/module
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Baseboard {
    /// Baseboard manufacturer
    pub manufacturer: Option<String>,

    /// Baseboard product
    pub product: Option<String>,

    /// Baseboard serial number
    pub serial_number: Option<String>,

    /// Asset tag
    pub asset_tag: Option<String>,

    /// Baseboard feature flags
    pub feature_flags: Option<BaseboardFeatures>,

    /// The board's location within the chassis
    pub location_in_chassis: Option<String>,

    /// Handle, or instance number, associated with the chassis in
    /// which this board resides.
    pub chassis_handle: Option<Handle>,

    /// Type of baseboard
    pub board_type: Option<BoardTypeData>,
}

impl Baseboard {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosBaseboardInformation| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about baseboard/module (type 2)!"
            ))?;
        Ok(Self {
            manufacturer: t.manufacturer().ok(),
            product: t.product().ok(),
            serial_number: t.serial_number().ok(),
            asset_tag: t.asset_tag().ok(),
            feature_flags: match t.feature_flags() {
                Some(ff) => Some(BaseboardFeatures::from(ff)),
                None => None,
            },
            location_in_chassis: t.location_in_chassis().ok(),
            chassis_handle: match t.chassis_handle() {
                Some(h) => Some(Handle::from(h)),
                None => None,
            },
            board_type: match t.board_type() {
                Some(bt) => Some(BoardTypeData::from(bt)),
                None => None,
            },
        })
    }
}

impl ToJson for Baseboard {}

/// Baseboard feature flags
///
/// Collection of flags that identify features of this board
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseboardFeatures {
    /// Set if the board is a hosting board (e.g. motherboard)
    pub hosting_board: bool,

    /// Set if the board requires at least one daughter board or
    /// auxiliary card to function properly
    pub requires_daughterboard: bool,

    /// Set if the board is removable; it is designed to be taken in
    /// and out of the chassis without impairing the function of
    /// the chassis
    pub is_removable: bool,

    /// Set if the board is replaceable; it is possible to replace
    /// (either as a field repair or as an upgrade) the board with a
    /// physically different board. The board is inherently removable
    pub is_replaceable: bool,

    /// Set if the board if hot swappable; it is possible to replace
    /// the board with a physically different but equivalent board
    /// while power is applied to the board. The board is
    /// inherently replaceable and removable.
    pub is_hot_swappable: bool,
}

impl From<smbioslib::BaseboardFeatures> for BaseboardFeatures {
    fn from(value: smbioslib::BaseboardFeatures) -> Self {
        Self {
            hosting_board: value.hosting_board(),
            requires_daughterboard: value.requires_daughterboard(),
            is_removable: value.is_removable(),
            is_replaceable: value.is_replaceable(),
            is_hot_swappable: value.is_hot_swappable(),
        }
    }
}
impl ToJson for BaseboardFeatures {}

/// Chassis type data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChassisTypeData {
    pub raw: u8,
    pub value: ChassisType,
    pub lock_presence: ChassisLockPresence,
}

impl From<smbioslib::ChassisTypeData> for ChassisTypeData {
    fn from(value: smbioslib::ChassisTypeData) -> Self {
        Self {
            raw: value.raw,
            value: ChassisType::from(value.value),
            lock_presence: ChassisLockPresence::from(value.lock_presence),
        }
    }
}

/// Chassis type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChassisType {
    Other,
    Unknown,
    Desktop,
    LowProfileDesktop,
    PizzaBox,
    MiniTower,
    Tower,
    Portable,
    Laptop,
    Notebook,
    HandHeld,
    DockingStation,
    AllInOne,
    SubNotebook,
    SpaceSaving,
    LunchBox,
    MainServerChassis,
    ExpansionChassis,
    SubChassis,
    BusExpansionChassis,
    PeripheralChassis,
    RaidChassis,
    RackMountChassis,
    SealedCasePC,
    MultiSystemChassis,
    CompactPci,
    AdvancedTca,
    Blade,
    BladeEnclosure,
    Tablet,
    Convertible,
    Detachable,
    IoTGateway,
    EmbeddedPC,
    MiniPC,
    StickPC,
    None,
}

impl From<smbioslib::ChassisType> for ChassisType {
    fn from(value: smbioslib::ChassisType) -> Self {
        match value {
            smbioslib::ChassisType::Other => Self::Other,
            smbioslib::ChassisType::Unknown => Self::Unknown,
            smbioslib::ChassisType::Desktop => Self::Desktop,
            smbioslib::ChassisType::LowProfileDesktop => Self::LowProfileDesktop,
            smbioslib::ChassisType::PizzaBox => Self::PizzaBox,
            smbioslib::ChassisType::MiniTower => Self::MiniTower,
            smbioslib::ChassisType::Tower => Self::Tower,
            smbioslib::ChassisType::Portable => Self::Portable,
            smbioslib::ChassisType::Laptop => Self::Laptop,
            smbioslib::ChassisType::Notebook => Self::Notebook,
            smbioslib::ChassisType::HandHeld => Self::HandHeld,
            smbioslib::ChassisType::DockingStation => Self::DockingStation,
            smbioslib::ChassisType::AllInOne => Self::AllInOne,
            smbioslib::ChassisType::SubNotebook => Self::SubNotebook,
            smbioslib::ChassisType::SpaceSaving => Self::SpaceSaving,
            smbioslib::ChassisType::LunchBox => Self::LunchBox,
            smbioslib::ChassisType::MainServerChassis => Self::MainServerChassis,
            smbioslib::ChassisType::ExpansionChassis => Self::ExpansionChassis,
            smbioslib::ChassisType::SubChassis => Self::SubChassis,
            smbioslib::ChassisType::BusExpansionChassis => Self::BusExpansionChassis,
            smbioslib::ChassisType::PeripheralChassis => Self::PeripheralChassis,
            smbioslib::ChassisType::RaidChassis => Self::RaidChassis,
            smbioslib::ChassisType::RackMountChassis => Self::RackMountChassis,
            smbioslib::ChassisType::SealedCasePC => Self::SealedCasePC,
            smbioslib::ChassisType::MultiSystemChassis => Self::MultiSystemChassis,
            smbioslib::ChassisType::CompactPci => Self::CompactPci,
            smbioslib::ChassisType::AdvancedTca => Self::AdvancedTca,
            smbioslib::ChassisType::Blade => Self::Blade,
            smbioslib::ChassisType::BladeEnclosure => Self::BladeEnclosure,
            smbioslib::ChassisType::Tablet => Self::Tablet,
            smbioslib::ChassisType::Convertible => Self::Convertible,
            smbioslib::ChassisType::Detachable => Self::Detachable,
            smbioslib::ChassisType::IoTGateway => Self::IoTGateway,
            smbioslib::ChassisType::EmbeddedPC => Self::EmbeddedPC,
            smbioslib::ChassisType::MiniPC => Self::MiniPC,
            smbioslib::ChassisType::StickPC => Self::StickPC,
            smbioslib::ChassisType::None => Self::None,
        }
    }
}

impl Display for ChassisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::Desktop => "Desktop",
                Self::LowProfileDesktop => "Low profile desktop",
                Self::PizzaBox => "Pizza Box",
                Self::MiniTower => "Mini Tower",
                Self::Tower => "Tower",
                Self::Portable => "Portable",
                Self::Laptop => "Laptop",
                Self::Notebook => "Notebook",
                Self::HandHeld => "Hand Held",
                Self::DockingStation => "Docking Station",
                Self::AllInOne => "All In One",
                Self::SubNotebook => "Sub Notebook",
                Self::SpaceSaving => "Space Saving",
                Self::LunchBox => "Lunch Box",
                Self::MainServerChassis => "Main server chassis",
                Self::ExpansionChassis => "Expansion chassis",
                Self::SubChassis => "Sub chassis",
                Self::BusExpansionChassis => "Bus expansion chassis",
                Self::PeripheralChassis => "Peripheral chassis",
                Self::RaidChassis => "RAID chassis",
                Self::RackMountChassis => "Rack Mount chassis",
                Self::SealedCasePC => "Sealed-case chassis",
                Self::MultiSystemChassis => "Multi-system chassis",
                Self::CompactPci => "Compact PCI",
                Self::AdvancedTca => "Advanced TCA",
                Self::Blade => "Blade",
                Self::BladeEnclosure => "Blade encloser",
                Self::Tablet => "Tablet",
                Self::Convertible => "Convertivle",
                Self::Detachable => "Detachable",
                Self::IoTGateway => "IoT Gateway",
                Self::EmbeddedPC => "Embedded PC",
                Self::MiniPC => "Mini PC",
                Self::StickPC => "Stick PC",
                Self::None => "Unknown to this standard, check the raw value",
            }
        )
    }
}

/// Chassis lock presence
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChassisLockPresence {
    Present,
    NotPresent,
}

impl From<smbioslib::ChassisLockPresence> for ChassisLockPresence {
    fn from(value: smbioslib::ChassisLockPresence) -> Self {
        match value {
            smbioslib::ChassisLockPresence::Present => Self::Present,
            smbioslib::ChassisLockPresence::NotPresent => Self::NotPresent,
        }
    }
}

impl Display for ChassisLockPresence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Present => "Present",
                Self::NotPresent => "Not present",
            }
        )
    }
}

/// Chassis state data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChassisStateData {
    pub raw: u8,
    pub value: ChassisState,
}

impl From<smbioslib::ChassisStateData> for ChassisStateData {
    fn from(value: smbioslib::ChassisStateData) -> Self {
        Self {
            raw: value.raw,
            value: ChassisState::from(value.value),
        }
    }
}

/// Chassis state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChassisState {
    Other,
    Unknown,
    Safe,
    Warning,
    Critical,
    NonRecoverable,
    None,
}

impl From<smbioslib::ChassisState> for ChassisState {
    fn from(value: smbioslib::ChassisState) -> Self {
        match value {
            smbioslib::ChassisState::Other => Self::Other,
            smbioslib::ChassisState::Unknown => Self::Unknown,
            smbioslib::ChassisState::Safe => Self::Safe,
            smbioslib::ChassisState::Warning => Self::Warning,
            smbioslib::ChassisState::Critical => Self::Critical,
            smbioslib::ChassisState::NonRecoverable => Self::NonRecoverable,
            smbioslib::ChassisState::None => Self::None,
        }
    }
}

impl Display for ChassisState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::Safe => "Safe",
                Self::Warning => "Warning",
                Self::Critical => "Critical",
                Self::NonRecoverable => "Non-recoverable",
                Self::None => "Unknown to this standard, check the raw value",
            }
        )
    }
}

/// Chassis security status data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChassisSecurityStatusData {
    pub raw: u8,
    pub value: ChassisSecurityStatus,
}

impl From<smbioslib::ChassisSecurityStatusData> for ChassisSecurityStatusData {
    fn from(value: smbioslib::ChassisSecurityStatusData) -> Self {
        Self {
            raw: value.raw,
            value: ChassisSecurityStatus::from(value.value),
        }
    }
}

/// Chassis security status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChassisSecurityStatus {
    Other,
    Unknown,
    StatusNone,
    ExternalInterfaceLockedOut,
    ExternalInterfaceEnabled,
    None,
}

impl From<smbioslib::ChassisSecurityStatus> for ChassisSecurityStatus {
    fn from(value: smbioslib::ChassisSecurityStatus) -> Self {
        match value {
            smbioslib::ChassisSecurityStatus::Other => Self::Other,
            smbioslib::ChassisSecurityStatus::Unknown => Self::Unknown,
            smbioslib::ChassisSecurityStatus::StatusNone => Self::StatusNone,
            smbioslib::ChassisSecurityStatus::ExternalInterfaceLockedOut => {
                Self::ExternalInterfaceLockedOut
            }
            smbioslib::ChassisSecurityStatus::ExternalInterfaceEnabled => {
                Self::ExternalInterfaceEnabled
            }
            smbioslib::ChassisSecurityStatus::None => Self::None,
        }
    }
}

impl Display for ChassisSecurityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::StatusNone => "None",
                Self::ExternalInterfaceLockedOut => "External interface locked out",
                Self::ExternalInterfaceEnabled => "External interface enabled",
                Self::None => "Unknown to this standard, check the raw value",
            }
        )
    }
}

/// Chassis height
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChassisHeight {
    Unspecified,
    U(u8),
    SpecifiedInRackHeight,
}

impl From<smbioslib::ChassisHeight> for ChassisHeight {
    fn from(value: smbioslib::ChassisHeight) -> Self {
        match value {
            smbioslib::ChassisHeight::Unspecified => Self::Unspecified,
            smbioslib::ChassisHeight::U(u) => Self::U(u),
            smbioslib::ChassisHeight::SpecifiedInRackHeight => Self::SpecifiedInRackHeight,
        }
    }
}

impl Display for ChassisHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unspecified => format!("Unspecified"),
                Self::U(u) => format!("{u} U (1 U = 1.75 inch or 4.445 cm)"),
                Self::SpecifiedInRackHeight => "Height is specified in the Rack Height".to_string(),
            }
        )
    }
}

/// Number of Power Cords
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PowerCords {
    Unspecified,
    Count(u8),
}

impl From<smbioslib::PowerCords> for PowerCords {
    fn from(value: smbioslib::PowerCords) -> Self {
        match value {
            smbioslib::PowerCords::Unspecified => Self::Unspecified,
            smbioslib::PowerCords::Count(cnt) => Self::Count(cnt),
        }
    }
}

impl Display for PowerCords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unspecified => format!("Unspecified"),
                Self::Count(cnt) => format!("{cnt}"),
            }
        )
    }
}

/// Information about system enclosure or chassis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chassis {
    /// Enclosure/chassis manufacturer
    pub manufacturer: Option<String>,

    /// Chassis type
    pub chassis_type: Option<ChassisTypeData>,

    /// Version
    pub version: Option<String>,

    /// Serial number
    pub serial_number: Option<String>,

    /// Asset tag number
    pub asset_tag_number: Option<String>,

    /// State of the enclosure whet it was last booted
    pub bootup_state: Option<ChassisStateData>,

    /// State of the enclosure's power supply when last booted
    pub power_supply_state: Option<ChassisStateData>,

    /// Thermal state of the enclosure when last booted
    pub thermal_state: Option<ChassisStateData>,

    /// Physical security status of the enclosure when last booted
    pub security_status: Option<ChassisSecurityStatusData>,

    /// OEM- or BIOS vendor-specific information
    pub oem_defined: Option<u32>,

    /// Height of the enclosure, in 'U's
    ///
    /// A U is a standard unit of measure for the height of a rack
    /// or rack-mountable component and is equal to 1.75 inches or
    /// 4.445 cm
    pub height: Option<ChassisHeight>,

    /// Number of power cords associated with the enclosure/chassis
    pub number_of_power_cords: Option<PowerCords>,

    /// Number of Contained Element records that follow, in the
    /// range 0 to 255 Each Contained Element group comprises m
    /// bytes, as specified by the Contained Element Record Length
    /// field that follows. If no Contained Elements are included,
    /// this field is set to 0.
    pub contained_element_count: Option<u8>,

    /// Byte length of eact Contained Element record that follows,
    /// in the range 0 to 255. If no Contained Elements are included,
    /// this field is set to 0
    pub contained_element_record_length: Option<u8>,

    // TODO: this struct doesn't included ContainedElements<'_> iter!
    /// Chassis or enclosure SKU number
    pub sku_number: Option<String>,
}

impl Chassis {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosSystemChassisInformation| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about system enclosure/chassis (type 3)!"
            ))?;

        Ok(Self {
            manufacturer: t.manufacturer().ok(),
            chassis_type: match t.chassis_type() {
                Some(ct) => Some(ChassisTypeData::from(ct)),
                None => None,
            },
            version: t.version().ok(),
            serial_number: t.serial_number().ok(),
            asset_tag_number: t.asset_tag_number().ok(),
            bootup_state: match t.bootup_state() {
                Some(bs) => Some(ChassisStateData::from(bs)),
                None => None,
            },
            power_supply_state: match t.power_supply_state() {
                Some(pss) => Some(ChassisStateData::from(pss)),
                None => None,
            },
            thermal_state: match t.thermal_state() {
                Some(ts) => Some(ChassisStateData::from(ts)),
                None => None,
            },
            security_status: match t.security_status() {
                Some(ss) => Some(ChassisSecurityStatusData::from(ss)),
                None => None,
            },
            oem_defined: t.oem_defined(),
            height: match t.height() {
                Some(h) => Some(ChassisHeight::from(h)),
                None => None,
            },
            number_of_power_cords: match t.number_of_power_cords() {
                Some(npc) => Some(PowerCords::from(npc)),
                None => None,
            },
            contained_element_count: t.contained_element_count(),
            contained_element_record_length: t.contained_element_record_length(),
            sku_number: t.sku_number().ok(),
        })
    }
}

impl ToJson for Chassis {}

/// Information about processor
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Processor {
    /// Socket reference designation
    pub socked_designation: Option<String>,

    /// Processor type
    pub processor_type: Option<ProcessorTypeData>,

    /// Processor family
    pub processor_family: Option<ProcessorFamilyData>,

    /// Processor manufacturer
    pub processor_manufacturer: Option<String>,

    /// Raw processor identification data
    pub processor_id: Option<[u8; 8]>,

    /// Processor version
    pub processor_version: Option<String>,

    /// Processor voltage
    pub voltage: Option<ProcessorVoltage>,

    /// External clock frequency, MHz. If the value is unknown,
    /// the field is set to 0
    pub external_clock: Option<ProcessorExternalClock>,

    /// Maximum CPU speed (in MHz) supported *by the system* for this
    /// processor socket
    pub max_speed: Option<ProcessorSpeed>,

    /// Current speed
    ///
    /// This field identifies the processor's speed at system boot;
    /// the processor may support more than one speed
    pub current_speed: Option<ProcessorSpeed>,

    /// Status bit field
    pub status: Option<ProcessorStatus>,

    /// Processor upgrade
    pub processor_upgrade: Option<ProcessorUpgradeData>,

    /// Attributes of the primary (Level 1) cache for this processor
    pub l1cache_handle: Option<Handle>,

    /// Attributes of the primary (Level 2) cache for this processor
    pub l2cache_handle: Option<Handle>,

    /// Attributes of the primary (Level 3) cache for this processor
    pub l3cache_handle: Option<Handle>,

    /// Serial number of this processor
    pub serial_number: Option<String>,

    /// Asset tag of this proc
    pub asset_tag: Option<String>,

    /// Part number of this processor
    pub part_number: Option<String>,

    /// Number of cores per processor socket
    pub core_count: Option<CoreCount>,

    /// Number of enabled cores per processor socket
    pub cores_enabled: Option<CoresEnabled>,

    /// Number of threads per processor socket
    pub thread_count: Option<ThreadCount>,

    /// Function that processor supports
    pub processors_characteristics: Option<ProcessorCharacteristics>,

    /// Processor family 2
    pub processor_family_2: Option<ProcessorFamilyData2>,

    /// Number of cores per proc socket (if cores > 255)
    pub core_count_2: Option<CoreCount2>,

    /// Number of enabled cores per proc socket (if ecores > 255)
    pub cores_enabled_2: Option<CoresEnabled2>,

    /// Number of threads per proc socket (if threads > 255)
    pub thread_count_2: Option<ThreadCount2>,

    /// Number of threads the BIOS has enabled and available for
    /// OS use
    pub thread_enabled: Option<ThreadEnabled>,
}

impl Processor {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosProcessorInformation| Some(f))
            .ok_or(anyhow!("Failed to get information about CPU (type 4)!"))?;

        Ok(Self {
            socked_designation: t.socket_designation().ok(),
            processor_type: match t.processor_type() {
                Some(pt) => Some(ProcessorTypeData::from(pt)),
                None => None,
            },
            processor_family: match t.processor_family() {
                Some(pf) => Some(ProcessorFamilyData::from(pf)),
                None => None,
            },
            processor_manufacturer: t.processor_manufacturer().ok(),
            processor_id: match t.processor_id() {
                Some(p_id) => Some(*p_id),
                None => None,
            },
            processor_version: t.processor_version().ok(),
            voltage: match t.voltage() {
                Some(v) => Some(ProcessorVoltage::from(v)),
                None => None,
            },
            external_clock: match t.external_clock() {
                Some(ec) => Some(ProcessorExternalClock::from(ec)),
                None => None,
            },
            max_speed: match t.max_speed() {
                Some(ms) => Some(ProcessorSpeed::from(ms)),
                None => None,
            },
            current_speed: match t.current_speed() {
                Some(cs) => Some(ProcessorSpeed::from(cs)),
                None => None,
            },
            status: match t.status() {
                Some(s) => Some(ProcessorStatus::from(s)),
                None => None,
            },
            processor_upgrade: match t.processor_upgrade() {
                Some(pu) => Some(ProcessorUpgradeData::from(pu)),
                None => None,
            },
            l1cache_handle: Handle::from_opt(t.l1cache_handle()),
            l2cache_handle: Handle::from_opt(t.l2cache_handle()),
            l3cache_handle: Handle::from_opt(t.l3cache_handle()),
            serial_number: t.serial_number().ok(),
            asset_tag: t.asset_tag().ok(),
            part_number: t.part_number().ok(),
            core_count: match t.core_count() {
                Some(cc) => Some(CoreCount::from(cc)),
                None => None,
            },
            cores_enabled: match t.cores_enabled() {
                Some(ce) => Some(CoresEnabled::from(ce)),
                None => None,
            },
            thread_count: match t.thread_count() {
                Some(tc) => Some(ThreadCount::from(tc)),
                None => None,
            },
            processors_characteristics: match t.processor_characteristics() {
                Some(pc) => Some(ProcessorCharacteristics::from(pc)),
                None => None,
            },
            processor_family_2: match t.processor_family_2() {
                Some(pf2) => Some(ProcessorFamilyData2::from(pf2)),
                None => None,
            },
            core_count_2: match t.core_count_2() {
                Some(cc2) => Some(CoreCount2::from(cc2)),
                None => None,
            },
            cores_enabled_2: match t.cores_enabled_2() {
                Some(ce2) => Some(CoresEnabled2::from(ce2)),
                None => None,
            },
            thread_count_2: match t.thread_count_2() {
                Some(tc2) => Some(ThreadCount2::from(tc2)),
                None => None,
            },
            thread_enabled: match t.thread_enabled() {
                Some(te) => Some(ThreadEnabled::from(te)),
                None => None,
            },
        })
    }
}

impl ToJson for Processor {}

/// Defines which functions the processor supports
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorCharacteristics {
    /// Bit 1 unknown
    pub unknown: bool,

    /// 64-bit capable
    pub bit_64capable: bool,

    /// Multi-core
    pub multi_core: bool,

    /// Hardware thread
    pub hardware_thread: bool,

    /// Execute protection
    pub execute_protection: bool,

    /// Enhanced Virtualization
    pub enhanced_virtualization: bool,

    /// Power/perfomance control
    pub power_perfomance_control: bool,

    /// 128-bit capable
    pub bit_128capable: bool,

    /// Arm64 SoC ID
    pub arm_64soc_id: bool,
}

impl From<smbioslib::ProcessorCharacteristics> for ProcessorCharacteristics {
    fn from(value: smbioslib::ProcessorCharacteristics) -> Self {
        Self {
            unknown: value.unknown(),
            bit_64capable: value.bit_64capable(),
            multi_core: value.multi_core(),
            hardware_thread: value.hardware_thread(),
            execute_protection: value.execute_protection(),
            enhanced_virtualization: value.enhanced_virtualization(),
            power_perfomance_control: value.power_performance_control(),
            bit_128capable: value.bit_128capable(),
            arm_64soc_id: value.arm_64soc_id(),
        }
    }
}
impl ToJson for ProcessorCharacteristics {}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessorTypeData {
    pub raw: u8,
    pub value: ProcessorType,
}

impl From<smbioslib::ProcessorTypeData> for ProcessorTypeData {
    fn from(value: smbioslib::ProcessorTypeData) -> Self {
        Self {
            raw: value.raw,
            value: ProcessorType::from(value.value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProcessorType {
    Other,
    Unknown,
    CentralProcessor,
    MathProcessor,
    DspProcessor,
    VideoProcessor,
    None,
}

impl Display for ProcessorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::CentralProcessor => "Central Processor",
                Self::MathProcessor => "Math Processor",
                Self::DspProcessor => "DSP Processor",
                Self::VideoProcessor => "Video Processor",
                Self::None => "A value unknown for this standard, check the raw value",
            }
        )
    }
}

impl From<smbioslib::ProcessorType> for ProcessorType {
    fn from(value: smbioslib::ProcessorType) -> Self {
        match value {
            smbioslib::ProcessorType::Other => Self::Other,
            smbioslib::ProcessorType::Unknown => Self::Unknown,
            smbioslib::ProcessorType::CentralProcessor => Self::CentralProcessor,
            smbioslib::ProcessorType::MathProcessor => Self::MathProcessor,
            smbioslib::ProcessorType::DspProcessor => Self::DspProcessor,
            smbioslib::ProcessorType::VideoProcessor => Self::VideoProcessor,
            smbioslib::ProcessorType::None => Self::None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessorFamilyData {
    pub raw: u8,
    pub value: ProcessorFamily,
}

impl From<smbioslib::ProcessorFamilyData> for ProcessorFamilyData {
    fn from(value: smbioslib::ProcessorFamilyData) -> Self {
        Self {
            raw: value.raw,
            value: ProcessorFamily::from(value.value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProcessorFamily {
    Other,
    Unknown,
    IntelPentiumProcessor,
    PentiumProProcessor,
    PentiumIIProcessor,
    PentiumprocessorwithMMXtechnology,
    IntelCeleronProcessor,
    PentiumIIXeonProcessor,
    PentiumIIIProcessor,
    M1Family,
    M2Family,
    IntelCeleronMProcessor,
    IntelPentium4HTProcessor,
    AMDDuronProcessorFamily,
    K5Family,
    K6Family,
    K62,
    K63,
    AMDAthlonProcessorFamily,
    AMD29000Family,
    K62Plus,
    IntelCoreDuoProcessor,
    IntelCoreDuomobileProcessor,
    IntelCoreSolomobileProcessor,
    IntelAtomProcessor,
    IntelCoreMProcessor,
    IntelCorem3Processor,
    IntelCorem5Processor,
    IntelCorem7Processor,
    AMDTurionIIUltraDualCoreMobileMProcessorFamily,
    AMDTurionIIDualCoreMobileMProcessorFamily,
    AMDAthlonIIDualCoreMProcessorFamily,
    AMDOpteron6100SeriesProcessor,
    AMDOpteron4100SeriesProcessor,
    AMDOpteron6200SeriesProcessor,
    AMDOpteron4200SeriesProcessor,
    AMDFXSeriesProcessor,
    AMDCSeriesProcessor,
    AMDESeriesProcessor,
    AMDASeriesProcessor,
    AMDGSeriesProcessor,
    AMDZSeriesProcessor,
    AMDRSeriesProcessor,
    AMDOpteron4300SeriesProcessor,
    AMDOpteron6300SeriesProcessor,
    AMDOpteron3300SeriesProcessor,
    AMDFireProSeriesProcessor,
    AMDAthlonX4QuadCoreProcessorFamily,
    AMDOpteronX1000SeriesProcessor,
    AMDOpteronX2000SeriesAPU,
    AMDOpteronASeriesProcessor,
    AMDOpteronX3000SeriesAPU,
    AMDZenProcessorFamily,
    Itaniumprocessor,
    AMDAthlon64ProcessorFamily,
    AMDOpteronProcessorFamily,
    AMDSempronProcessorFamily,
    AMDTurion64MobileTechnology,
    DualCoreAMDOpteronProcessorFamily,
    AMDAthlon64X2DualCoreProcessorFamily,
    AMDTurion64X2MobileTechnology,
    QuadCoreAMDOpteronProcessorFamily,
    ThirdGenerationAMDOpteronProcessorFamily,
    AMDPhenomFXQuadCoreProcessorFamily,
    AMDPhenomX4QuadCoreProcessorFamily,
    AMDPhenomX2DualCoreProcessorFamily,
    AMDAthlonX2DualCoreProcessorFamily,
    QuadCoreIntelXeonProcessor3200Series,
    DualCoreIntelXeonProcessor3000Series,
    QuadCoreIntelXeonProcessor5300Series,
    DualCoreIntelXeonProcessor5100Series,
    DualCoreIntelXeonProcessor5000Series,
    DualCoreIntelXeonProcessorLV,
    DualCoreIntelXeonProcessorULV,
    DualCoreIntelXeonProcessor7100Series,
    QuadCoreIntelXeonProcessor5400Series,
    QuadCoreIntelXeonProcessor,
    DualCoreIntelXeonProcessor5200Series,
    DualCoreIntelXeonProcessor7200Series,
    QuadCoreIntelXeonProcessor7300Series,
    QuadCoreIntelXeonProcessor7400Series,
    MultiCoreIntelXeonProcessor7400Series,
    PentiumIIIXeonProcessor,
    PentiumIIIProcessorwithIntelSpeedStepTechnology,
    Pentium4Processor,
    IntelXeonProcessor,
    IntelXeonProcessorMP,
    AMDAthlonXPProcessorFamily,
    AMDAthlonMPProcessorFamily,
    IntelItanium2Processor,
    IntelPentiumMProcessor,
    IntelCeleronDProcessor,
    IntelPentiumDProcessor,
    IntelPentiumProcessorExtremeEdition,
    IntelCoreSoloProcessor,
    IntelCore2DuoProcessor,
    IntelCore2SoloProcessor,
    IntelCore2ExtremeProcessor,
    IntelCore2QuadProcessor,
    IntelCore2ExtremeMobileProcessor,
    IntelCore2DuoMobileProcessor,
    IntelCore2SoloMobileProcessor,
    IntelCorei7Processor,
    DualCoreIntelCeleronProcessor,
    IntelCorei5processor,
    IntelCorei3processor,
    IntelCorei9processor,
    MultiCoreIntelXeonProcessor,
    DualCoreIntelXeonProcessor3xxxSeries,
    QuadCoreIntelXeonProcessor3xxxSeries,
    DualCoreIntelXeonProcessor5xxxSeries,
    QuadCoreIntelXeonProcessor5xxxSeries,
    DualCoreIntelXeonProcessor7xxxSeries,
    QuadCoreIntelXeonProcessor7xxxSeries,
    MultiCoreIntelXeonProcessor7xxxSeries,
    MultiCoreIntelXeonProcessor3400Series,
    AMDOpteron3000SeriesProcessor,
    AMDSempronIIProcessor,
    EmbeddedAMDOpteronQuadCoreProcessorFamily,
    AMDPhenomTripleCoreProcessorFamily,
    AMDTurionUltraDualCoreMobileProcessorFamily,
    AMDTurionDualCoreMobileProcessorFamily,
    AMDAthlonDualCoreProcessorFamily,
    AMDSempronSIProcessorFamily,
    AMDPhenomIIProcessorFamily,
    AMDAthlonIIProcessorFamily,
    SixCoreAMDOpteronProcessorFamily,
    AMDSempronMProcessorFamily,
    SeeProcessorFamily2,
    ARMv7,
    ARMv8,
    ARMv9,
    ARM,
    StrongARM,
    VideoProcessor,
    None,
}

impl Display for ProcessorFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::IntelPentiumProcessor => "Intel® Pentium® processor",
                Self::PentiumProProcessor => "Intel® Pentium® Pro processor",
                Self::PentiumIIProcessor => "Pentium® II processor",
                Self::PentiumprocessorwithMMXtechnology =>
                    "Pentium® processor with MMX™ technology",
                Self::IntelCeleronProcessor => "Intel® Celeron® processor",
                Self::PentiumIIXeonProcessor => "Pentium® II Xeon™ processor",
                Self::PentiumIIIProcessor => "Pentium® III processor",
                Self::M1Family => "M1 Family",
                Self::M2Family => "M2 Family",
                Self::IntelCeleronMProcessor => "Intel® Celeron® M processor",
                Self::IntelPentium4HTProcessor => "Intel® Pentium® 4 HT processor",
                Self::AMDDuronProcessorFamily => "AMD Duron™ Processor Family",
                Self::K5Family => "K5 Family",
                Self::K6Family => "K6 Family",
                Self::K62 => "K6-2",
                Self::K63 => "K6-3",
                Self::AMDAthlonProcessorFamily => "AMD Athlon™ Processor Family",
                Self::AMD29000Family => "AMD29000 Family",
                Self::K62Plus => "K6-2+",
                Self::IntelCoreDuoProcessor => "Intel® Core™ Duo processor",
                Self::IntelCoreDuomobileProcessor => "Intel® Core™ Duo mobile processor",
                Self::IntelCoreSolomobileProcessor => "Intel® Core™ Solo mobile processor",
                Self::IntelAtomProcessor => "Intel® Atom™ processor",
                Self::IntelCoreMProcessor => "Intel® Core™ M processor",
                Self::IntelCorem3Processor => "Intel® Core™ m3 processor",
                Self::IntelCorem5Processor => "Intel® Core™ m5 processor",
                Self::IntelCorem7Processor => "Intel® Core™ m7 processor",
                Self::AMDTurionIIUltraDualCoreMobileMProcessorFamily =>
                    "AMD Turion™ II Ultra Dual-Core Mobile M Processor Family",
                Self::AMDTurionIIDualCoreMobileMProcessorFamily =>
                    "AMD Turion™ II Dual-Core Mobile M Processor Family",
                Self::AMDAthlonIIDualCoreMProcessorFamily =>
                    "AMD Athlon™ II Dual-Core M Processor Family",
                Self::AMDOpteron6100SeriesProcessor => "AMD Opteron™ 6100 Series Processor",
                Self::AMDOpteron4100SeriesProcessor => "AMD Opteron™ 4100 Series Processor",
                Self::AMDOpteron6200SeriesProcessor => "AMD Opteron™ 6200 Series Processor",
                Self::AMDOpteron4200SeriesProcessor => "AMD Opteron™ 4200 Series Processor",
                Self::AMDFXSeriesProcessor => "AMD FX™ Series Processor",
                Self::AMDCSeriesProcessor => "AMD C-Series Processor",
                Self::AMDESeriesProcessor => "AMD E-Series Processor",
                Self::AMDASeriesProcessor => "AMD A-Series Processor",
                Self::AMDGSeriesProcessor => "AMD G-Series Processor",
                Self::AMDZSeriesProcessor => "AMD Z-Series Processor",
                Self::AMDRSeriesProcessor => "AMD R-Series Processor",
                Self::AMDOpteron4300SeriesProcessor => "AMD Opteron™ 4300 Series Processor",
                Self::AMDOpteron6300SeriesProcessor => "AMD Opteron™ 6300 Series Processor",
                Self::AMDOpteron3300SeriesProcessor => "AMD Opteron™ 3300 Series Processor",
                Self::AMDFireProSeriesProcessor => "AMD FirePro™ Series Processor",
                Self::AMDAthlonX4QuadCoreProcessorFamily =>
                    "AMD Athlon(TM) X4 Quad-Core Processor Family",
                Self::AMDOpteronX1000SeriesProcessor => "AMD Opteron(TM) X1000 Series Processor",
                Self::AMDOpteronX2000SeriesAPU => "AMD Opteron(TM) X2000 Series APU",
                Self::AMDOpteronASeriesProcessor => "AMD Opteron(TM) A-Series Processor",
                Self::AMDOpteronX3000SeriesAPU => "AMD Opteron(TM) X3000 Series APU",
                Self::AMDZenProcessorFamily => "AMD Zen Processor Family",
                Self::Itaniumprocessor => "Itanium™ processor",
                Self::AMDAthlon64ProcessorFamily => "AMD Athlon™ 64 Processor Family",
                Self::AMDOpteronProcessorFamily => "AMD Opteron™ Processor Family",
                Self::AMDSempronProcessorFamily => "AMD Sempron™ Processor Family",
                Self::AMDTurion64MobileTechnology => "AMD Turion™ 64 Mobile Technology",
                Self::DualCoreAMDOpteronProcessorFamily =>
                    "Dual-Core AMD Opteron™ Processor Family",
                Self::AMDAthlon64X2DualCoreProcessorFamily =>
                    "AMD Athlon™ 64 X2 Dual-Core Processor Family",
                Self::AMDTurion64X2MobileTechnology => "AMD Turion™ 64 X2 Mobile Technology",
                Self::QuadCoreAMDOpteronProcessorFamily =>
                    "Quad-Core AMD Opteron™ Processor Family",
                Self::ThirdGenerationAMDOpteronProcessorFamily =>
                    "Third-Generation AMD Opteron™ Processor Family",
                Self::AMDPhenomFXQuadCoreProcessorFamily =>
                    "AMD Phenom™ FX Quad-Core Processor Family",
                Self::AMDPhenomX4QuadCoreProcessorFamily =>
                    "AMD Phenom™ X4 Quad-Core Processor Family",
                Self::AMDPhenomX2DualCoreProcessorFamily =>
                    "AMD Phenom™ X2 Dual-Core Processor Family",
                Self::AMDAthlonX2DualCoreProcessorFamily =>
                    "AMD Athlon™ X2 Dual-Core Processor Family",
                Self::QuadCoreIntelXeonProcessor3200Series =>
                    "Quad-Core Intel® Xeon® processor 3200 Series",
                Self::DualCoreIntelXeonProcessor3000Series =>
                    "Dual-Core Intel® Xeon® processor 3000 Series",
                Self::QuadCoreIntelXeonProcessor5300Series =>
                    "Quad-Core Intel® Xeon® processor 5300 Series",
                Self::DualCoreIntelXeonProcessor5100Series =>
                    "Dual-Core Intel® Xeon® processor 5100 Series",
                Self::DualCoreIntelXeonProcessor5000Series =>
                    "Dual-Core Intel® Xeon® processor 5000 Series",
                Self::DualCoreIntelXeonProcessorLV => "Dual-Core Intel® Xeon® processor LV",
                Self::DualCoreIntelXeonProcessorULV => "Dual-Core Intel® Xeon® processor ULV",
                Self::DualCoreIntelXeonProcessor7100Series =>
                    "Dual-Core Intel® Xeon® processor 7100 Series",
                Self::QuadCoreIntelXeonProcessor5400Series =>
                    "Quad-Core Intel® Xeon® processor 5400 Series",
                Self::QuadCoreIntelXeonProcessor => "Quad-Core Intel® Xeon® processor",
                Self::DualCoreIntelXeonProcessor5200Series =>
                    "Dual-Core Intel® Xeon® processor 5200 Series",
                Self::DualCoreIntelXeonProcessor7200Series =>
                    "Dual-Core Intel® Xeon® processor 7200 Series",
                Self::QuadCoreIntelXeonProcessor7300Series =>
                    "Quad-Core Intel® Xeon® processor 7300 Series",
                Self::QuadCoreIntelXeonProcessor7400Series =>
                    "Quad-Core Intel® Xeon® processor 7400 Series",
                Self::MultiCoreIntelXeonProcessor7400Series =>
                    "Multi-Core Intel® Xeon® processor 7400 Series",
                Self::PentiumIIIXeonProcessor => "Pentium® III Xeon™ processor",
                Self::PentiumIIIProcessorwithIntelSpeedStepTechnology =>
                    "Pentium® III Processor with Intel® SpeedStep™ Technology",
                Self::Pentium4Processor => "Pentium® 4 Processor",
                Self::IntelXeonProcessor => "Intel® Xeon® processor",
                Self::IntelXeonProcessorMP => "Intel® Xeon™ processor MP",
                Self::AMDAthlonXPProcessorFamily => "AMD Athlon™ XP Processor Family",
                Self::AMDAthlonMPProcessorFamily => "AMD Athlon™ MP Processor Family",
                Self::IntelItanium2Processor => "Intel® Itanium® 2 processor",
                Self::IntelPentiumMProcessor => "Intel® Pentium® M processor",
                Self::IntelCeleronDProcessor => "Intel® Celeron® D processor",
                Self::IntelPentiumDProcessor => "Intel® Pentium® D processor",
                Self::IntelPentiumProcessorExtremeEdition =>
                    "Intel® Pentium® Processor Extreme Edition",
                Self::IntelCoreSoloProcessor => "Intel® Core™ Solo Processor",
                Self::IntelCore2DuoProcessor => "Intel® Core™ 2 Duo Processor",
                Self::IntelCore2SoloProcessor => "Intel® Core™ 2 Solo processor",
                Self::IntelCore2ExtremeProcessor => "Intel® Core™ 2 Extreme processor",
                Self::IntelCore2QuadProcessor => "Intel® Core™ 2 Quad processor",
                Self::IntelCore2ExtremeMobileProcessor => "Intel® Core™ 2 Extreme mobile processor",
                Self::IntelCore2DuoMobileProcessor => "Intel® Core™ 2 Duo mobile processor",
                Self::IntelCore2SoloMobileProcessor => "Intel® Core™ 2 Solo mobile processor",
                Self::IntelCorei7Processor => "Intel® Core™ i7 processor",
                Self::DualCoreIntelCeleronProcessor => "Dual-Core Intel® Celeron® processor",
                Self::IntelCorei5processor => "Intel® Core™ i5 processor",
                Self::IntelCorei3processor => "Intel® Core™ i3 processor",
                Self::IntelCorei9processor => "Intel® Core™ i9 processor",
                Self::MultiCoreIntelXeonProcessor => "Multi-Core Intel® Xeon® processor",
                Self::DualCoreIntelXeonProcessor3xxxSeries =>
                    "Dual-Core Intel® Xeon® processor 3xxx Series",
                Self::QuadCoreIntelXeonProcessor3xxxSeries =>
                    "Quad-Core Intel® Xeon® processor 3xxx Series",
                Self::DualCoreIntelXeonProcessor5xxxSeries =>
                    "Dual-Core Intel® Xeon® processor 5xxx Series",
                Self::QuadCoreIntelXeonProcessor5xxxSeries =>
                    "Quad-Core Intel® Xeon® processor 5xxx Series",
                Self::DualCoreIntelXeonProcessor7xxxSeries =>
                    "Dual-Core Intel® Xeon® processor 7xxx Series",
                Self::QuadCoreIntelXeonProcessor7xxxSeries =>
                    "Quad-Core Intel® Xeon® processor 7xxx Series",
                Self::MultiCoreIntelXeonProcessor7xxxSeries =>
                    "Multi-Core Intel® Xeon® processor 7xxx Series",
                Self::MultiCoreIntelXeonProcessor3400Series =>
                    "Multi-Core Intel® Xeon® processor 3400 Series",
                Self::AMDOpteron3000SeriesProcessor => "AMD Opteron™ 3000 Series Processor",
                Self::AMDSempronIIProcessor => "AMD Sempron™ II Processor",
                Self::EmbeddedAMDOpteronQuadCoreProcessorFamily =>
                    "Embedded AMD Opteron™ Quad-Core Processor Family",
                Self::AMDPhenomTripleCoreProcessorFamily =>
                    "AMD Phenom™ Triple-Core Processor Family",
                Self::AMDTurionUltraDualCoreMobileProcessorFamily =>
                    "AMD Turion™ Ultra Dual-Core Mobile Processor Family",
                Self::AMDTurionDualCoreMobileProcessorFamily =>
                    "AMD Turion™ Dual-Core Mobile Processor Family",
                Self::AMDAthlonDualCoreProcessorFamily => "AMD Athlon™ Dual-Core Processor Family",
                Self::AMDSempronSIProcessorFamily => "AMD Sempron™ SI Processor Family",
                Self::AMDPhenomIIProcessorFamily => "AMD Phenom™ II Processor Family",
                Self::AMDAthlonIIProcessorFamily => "AMD Athlon™ II Processor Family",
                Self::SixCoreAMDOpteronProcessorFamily => "Six-Core AMD Opteron™ Processor Family",
                Self::AMDSempronMProcessorFamily => "AMD Sempron™ M Processor Family",
                Self::SeeProcessorFamily2 => "See the next processor family field",
                Self::ARMv7 => "ARMv7",
                Self::ARMv8 => "ARMv8",
                Self::ARMv9 => "ARMv9",
                Self::ARM => "ARM",
                Self::StrongARM => "StrongARM",
                Self::VideoProcessor => "Video Processor",
                Self::None => "A value unknown to this standard, check the raw value",
            }
        )
    }
}

impl From<smbioslib::ProcessorFamily> for ProcessorFamily {
    fn from(value: smbioslib::ProcessorFamily) -> Self {
        match value {
            smbioslib::ProcessorFamily::Other => Self::Other,
            smbioslib::ProcessorFamily::Unknown => Self::Unknown,
            smbioslib::ProcessorFamily::IntelPentiumProcessor => Self::IntelPentiumProcessor,
            smbioslib::ProcessorFamily::PentiumProProcessor => Self::PentiumProProcessor,
            smbioslib::ProcessorFamily::PentiumIIProcessor => Self::PentiumIIProcessor,
            smbioslib::ProcessorFamily::PentiumprocessorwithMMXtechnology => {
                Self::PentiumprocessorwithMMXtechnology
            }
            smbioslib::ProcessorFamily::IntelCeleronProcessor => Self::IntelCeleronProcessor,
            smbioslib::ProcessorFamily::PentiumIIXeonProcessor => Self::PentiumIIXeonProcessor,
            smbioslib::ProcessorFamily::PentiumIIIProcessor => Self::PentiumIIIProcessor,
            smbioslib::ProcessorFamily::M1Family => Self::M1Family,
            smbioslib::ProcessorFamily::M2Family => Self::M2Family,
            smbioslib::ProcessorFamily::IntelCeleronMProcessor => Self::IntelCeleronMProcessor,
            smbioslib::ProcessorFamily::IntelPentium4HTProcessor => Self::IntelPentium4HTProcessor,
            smbioslib::ProcessorFamily::AMDDuronProcessorFamily => Self::AMDDuronProcessorFamily,
            smbioslib::ProcessorFamily::K5Family => Self::K5Family,
            smbioslib::ProcessorFamily::K6Family => Self::K6Family,
            smbioslib::ProcessorFamily::K62 => Self::K62,
            smbioslib::ProcessorFamily::K63 => Self::K63,
            smbioslib::ProcessorFamily::AMDAthlonProcessorFamily => Self::AMDAthlonProcessorFamily,
            smbioslib::ProcessorFamily::AMD29000Family => Self::AMD29000Family,
            smbioslib::ProcessorFamily::K62Plus => Self::K62Plus,
            smbioslib::ProcessorFamily::IntelCoreDuoProcessor => Self::IntelCoreDuoProcessor,
            smbioslib::ProcessorFamily::IntelCoreDuomobileProcessor => {
                Self::IntelCoreDuomobileProcessor
            }
            smbioslib::ProcessorFamily::IntelCoreSolomobileProcessor => {
                Self::IntelCoreSolomobileProcessor
            }
            smbioslib::ProcessorFamily::IntelAtomProcessor => Self::IntelAtomProcessor,
            smbioslib::ProcessorFamily::IntelCoreMProcessor => Self::IntelCoreMProcessor,
            smbioslib::ProcessorFamily::IntelCorem3Processor => Self::IntelCorem3Processor,
            smbioslib::ProcessorFamily::IntelCorem5Processor => Self::IntelCorem5Processor,
            smbioslib::ProcessorFamily::IntelCorem7Processor => Self::IntelCorem7Processor,
            smbioslib::ProcessorFamily::AMDTurionIIUltraDualCoreMobileMProcessorFamily => {
                Self::AMDTurionIIUltraDualCoreMobileMProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDTurionIIDualCoreMobileMProcessorFamily => {
                Self::AMDTurionIIDualCoreMobileMProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDAthlonIIDualCoreMProcessorFamily => {
                Self::AMDAthlonIIDualCoreMProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDOpteron6100SeriesProcessor => {
                Self::AMDOpteron6100SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteron4100SeriesProcessor => {
                Self::AMDOpteron4100SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteron6200SeriesProcessor => {
                Self::AMDOpteron6200SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteron4200SeriesProcessor => {
                Self::AMDOpteron4200SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDFXSeriesProcessor => Self::AMDFXSeriesProcessor,
            smbioslib::ProcessorFamily::AMDCSeriesProcessor => Self::AMDCSeriesProcessor,
            smbioslib::ProcessorFamily::AMDESeriesProcessor => Self::AMDESeriesProcessor,
            smbioslib::ProcessorFamily::AMDASeriesProcessor => Self::AMDASeriesProcessor,
            smbioslib::ProcessorFamily::AMDGSeriesProcessor => Self::AMDGSeriesProcessor,
            smbioslib::ProcessorFamily::AMDZSeriesProcessor => Self::AMDZSeriesProcessor,
            smbioslib::ProcessorFamily::AMDRSeriesProcessor => Self::AMDRSeriesProcessor,
            smbioslib::ProcessorFamily::AMDOpteron4300SeriesProcessor => {
                Self::AMDOpteron4300SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteron6300SeriesProcessor => {
                Self::AMDOpteron6300SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteron3300SeriesProcessor => {
                Self::AMDOpteron3300SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDFireProSeriesProcessor => {
                Self::AMDFireProSeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDAthlonX4QuadCoreProcessorFamily => {
                Self::AMDAthlonX4QuadCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDOpteronX1000SeriesProcessor => {
                Self::AMDOpteronX1000SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteronX2000SeriesAPU => Self::AMDOpteronX2000SeriesAPU,
            smbioslib::ProcessorFamily::AMDOpteronASeriesProcessor => {
                Self::AMDOpteronASeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDOpteronX3000SeriesAPU => Self::AMDOpteronX3000SeriesAPU,
            smbioslib::ProcessorFamily::AMDZenProcessorFamily => Self::AMDZenProcessorFamily,
            smbioslib::ProcessorFamily::Itaniumprocessor => Self::Itaniumprocessor,
            smbioslib::ProcessorFamily::AMDAthlon64ProcessorFamily => {
                Self::AMDAthlon64ProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDOpteronProcessorFamily => {
                Self::AMDOpteronProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDSempronProcessorFamily => {
                Self::AMDSempronProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDTurion64MobileTechnology => {
                Self::AMDTurion64MobileTechnology
            }
            smbioslib::ProcessorFamily::DualCoreAMDOpteronProcessorFamily => {
                Self::DualCoreAMDOpteronProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDAthlon64X2DualCoreProcessorFamily => {
                Self::AMDAthlon64X2DualCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDTurion64X2MobileTechnology => {
                Self::AMDTurion64X2MobileTechnology
            }
            smbioslib::ProcessorFamily::QuadCoreAMDOpteronProcessorFamily => {
                Self::QuadCoreAMDOpteronProcessorFamily
            }
            smbioslib::ProcessorFamily::ThirdGenerationAMDOpteronProcessorFamily => {
                Self::ThirdGenerationAMDOpteronProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDPhenomFXQuadCoreProcessorFamily => {
                Self::AMDPhenomFXQuadCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDPhenomX4QuadCoreProcessorFamily => {
                Self::AMDPhenomX4QuadCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDPhenomX2DualCoreProcessorFamily => {
                Self::AMDPhenomX2DualCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDAthlonX2DualCoreProcessorFamily => {
                Self::AMDAthlonX2DualCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor3200Series => {
                Self::QuadCoreIntelXeonProcessor3200Series
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor3000Series => {
                Self::DualCoreIntelXeonProcessor3000Series
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor5300Series => {
                Self::QuadCoreIntelXeonProcessor5300Series
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor5100Series => {
                Self::DualCoreIntelXeonProcessor5100Series
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor5000Series => {
                Self::DualCoreIntelXeonProcessor5000Series
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessorLV => {
                Self::DualCoreIntelXeonProcessorLV
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessorULV => {
                Self::DualCoreIntelXeonProcessorULV
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor7100Series => {
                Self::DualCoreIntelXeonProcessor7100Series
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor5400Series => {
                Self::QuadCoreIntelXeonProcessor5400Series
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor => {
                Self::QuadCoreIntelXeonProcessor
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor5200Series => {
                Self::DualCoreIntelXeonProcessor5200Series
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor7200Series => {
                Self::DualCoreIntelXeonProcessor7200Series
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor7300Series => {
                Self::QuadCoreIntelXeonProcessor7300Series
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor7400Series => {
                Self::QuadCoreIntelXeonProcessor7400Series
            }
            smbioslib::ProcessorFamily::MultiCoreIntelXeonProcessor7400Series => {
                Self::MultiCoreIntelXeonProcessor7400Series
            }
            smbioslib::ProcessorFamily::PentiumIIIXeonProcessor => Self::PentiumIIIXeonProcessor,
            smbioslib::ProcessorFamily::PentiumIIIProcessorwithIntelSpeedStepTechnology => {
                Self::PentiumIIIProcessorwithIntelSpeedStepTechnology
            }
            smbioslib::ProcessorFamily::Pentium4Processor => Self::Pentium4Processor,
            smbioslib::ProcessorFamily::IntelXeonProcessor => Self::IntelXeonProcessor,
            smbioslib::ProcessorFamily::IntelXeonProcessorMP => Self::IntelXeonProcessorMP,
            smbioslib::ProcessorFamily::AMDAthlonXPProcessorFamily => {
                Self::AMDAthlonXPProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDAthlonMPProcessorFamily => {
                Self::AMDAthlonMPProcessorFamily
            }
            smbioslib::ProcessorFamily::IntelItanium2Processor => Self::IntelItanium2Processor,
            smbioslib::ProcessorFamily::IntelPentiumMProcessor => Self::IntelPentiumMProcessor,
            smbioslib::ProcessorFamily::IntelCeleronDProcessor => Self::IntelCeleronDProcessor,
            smbioslib::ProcessorFamily::IntelPentiumDProcessor => Self::IntelPentiumDProcessor,
            smbioslib::ProcessorFamily::IntelPentiumProcessorExtremeEdition => {
                Self::IntelPentiumProcessorExtremeEdition
            }
            smbioslib::ProcessorFamily::IntelCoreSoloProcessor => Self::IntelCoreSoloProcessor,
            smbioslib::ProcessorFamily::IntelCore2DuoProcessor => Self::IntelCore2DuoProcessor,
            smbioslib::ProcessorFamily::IntelCore2SoloProcessor => Self::IntelCore2SoloProcessor,
            smbioslib::ProcessorFamily::IntelCore2ExtremeProcessor => {
                Self::IntelCore2ExtremeProcessor
            }
            smbioslib::ProcessorFamily::IntelCore2QuadProcessor => Self::IntelCore2QuadProcessor,
            smbioslib::ProcessorFamily::IntelCore2ExtremeMobileProcessor => {
                Self::IntelCore2ExtremeMobileProcessor
            }
            smbioslib::ProcessorFamily::IntelCore2DuoMobileProcessor => {
                Self::IntelCore2DuoMobileProcessor
            }
            smbioslib::ProcessorFamily::IntelCore2SoloMobileProcessor => {
                Self::IntelCore2SoloMobileProcessor
            }
            smbioslib::ProcessorFamily::IntelCorei7Processor => Self::IntelCorei7Processor,
            smbioslib::ProcessorFamily::DualCoreIntelCeleronProcessor => {
                Self::DualCoreIntelCeleronProcessor
            }
            smbioslib::ProcessorFamily::IntelCorei5processor => Self::IntelCorei5processor,
            smbioslib::ProcessorFamily::IntelCorei3processor => Self::IntelCorei3processor,
            smbioslib::ProcessorFamily::IntelCorei9processor => Self::IntelCorei9processor,
            smbioslib::ProcessorFamily::MultiCoreIntelXeonProcessor => {
                Self::MultiCoreIntelXeonProcessor
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor3xxxSeries => {
                Self::DualCoreIntelXeonProcessor3xxxSeries
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor3xxxSeries => {
                Self::QuadCoreIntelXeonProcessor3xxxSeries
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor5xxxSeries => {
                Self::DualCoreIntelXeonProcessor5xxxSeries
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor5xxxSeries => {
                Self::QuadCoreIntelXeonProcessor5xxxSeries
            }
            smbioslib::ProcessorFamily::DualCoreIntelXeonProcessor7xxxSeries => {
                Self::DualCoreIntelXeonProcessor7xxxSeries
            }
            smbioslib::ProcessorFamily::QuadCoreIntelXeonProcessor7xxxSeries => {
                Self::QuadCoreIntelXeonProcessor7xxxSeries
            }
            smbioslib::ProcessorFamily::MultiCoreIntelXeonProcessor7xxxSeries => {
                Self::MultiCoreIntelXeonProcessor7xxxSeries
            }
            smbioslib::ProcessorFamily::MultiCoreIntelXeonProcessor3400Series => {
                Self::MultiCoreIntelXeonProcessor3400Series
            }
            smbioslib::ProcessorFamily::AMDOpteron3000SeriesProcessor => {
                Self::AMDOpteron3000SeriesProcessor
            }
            smbioslib::ProcessorFamily::AMDSempronIIProcessor => Self::AMDSempronIIProcessor,
            smbioslib::ProcessorFamily::EmbeddedAMDOpteronQuadCoreProcessorFamily => {
                Self::EmbeddedAMDOpteronQuadCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDPhenomTripleCoreProcessorFamily => {
                Self::AMDPhenomTripleCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDTurionUltraDualCoreMobileProcessorFamily => {
                Self::AMDTurionUltraDualCoreMobileProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDTurionDualCoreMobileProcessorFamily => {
                Self::AMDTurionDualCoreMobileProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDAthlonDualCoreProcessorFamily => {
                Self::AMDAthlonDualCoreProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDSempronSIProcessorFamily => {
                Self::AMDSempronSIProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDPhenomIIProcessorFamily => {
                Self::AMDPhenomIIProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDAthlonIIProcessorFamily => {
                Self::AMDAthlonIIProcessorFamily
            }
            smbioslib::ProcessorFamily::SixCoreAMDOpteronProcessorFamily => {
                Self::SixCoreAMDOpteronProcessorFamily
            }
            smbioslib::ProcessorFamily::AMDSempronMProcessorFamily => {
                Self::AMDSempronMProcessorFamily
            }
            smbioslib::ProcessorFamily::SeeProcessorFamily2 => Self::SeeProcessorFamily2,
            smbioslib::ProcessorFamily::ARMv7 => Self::ARMv7,
            smbioslib::ProcessorFamily::ARMv8 => Self::ARMv8,
            smbioslib::ProcessorFamily::ARMv9 => Self::ARMv9,
            smbioslib::ProcessorFamily::ARM => Self::ARM,
            smbioslib::ProcessorFamily::StrongARM => Self::StrongARM,
            smbioslib::ProcessorFamily::VideoProcessor => Self::VideoProcessor,
            smbioslib::ProcessorFamily::None => Self::None,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessorFamilyData2 {
    pub raw: u16,
    pub value: ProcessorFamily,
}

impl From<smbioslib::ProcessorFamilyData2> for ProcessorFamilyData2 {
    fn from(value: smbioslib::ProcessorFamilyData2) -> Self {
        Self {
            raw: value.raw,
            value: ProcessorFamily::from(value.value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProcessorVoltage {
    CurrentVolts(f32),
    SupportedVolts(ProcessorSupportedVoltages),
}

impl From<smbioslib::ProcessorVoltage> for ProcessorVoltage {
    fn from(value: smbioslib::ProcessorVoltage) -> Self {
        match value {
            smbioslib::ProcessorVoltage::CurrentVolts(volts) => Self::CurrentVolts(volts),
            smbioslib::ProcessorVoltage::SupportedVolts(volts) => {
                Self::SupportedVolts(ProcessorSupportedVoltages::from(volts))
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessorSupportedVoltages {
    pub volts_5_0: bool,
    pub volts_3_3: bool,
    pub volts_2_9: bool,
    pub voltages: Vec<f32>,
}

impl From<smbioslib::ProcessorSupportedVoltages> for ProcessorSupportedVoltages {
    fn from(value: smbioslib::ProcessorSupportedVoltages) -> Self {
        Self {
            volts_5_0: value.volts_5_0(),
            volts_3_3: value.volts_3_3(),
            volts_2_9: value.volts_2_9(),
            voltages: value.voltages(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProcessorExternalClock {
    Unknown,
    MHz(u16),
}

impl Display for ProcessorExternalClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::MHz(mhz) => write!(f, "{} MHz", mhz),
        }
    }
}

impl From<smbioslib::ProcessorExternalClock> for ProcessorExternalClock {
    fn from(value: smbioslib::ProcessorExternalClock) -> Self {
        match value {
            smbioslib::ProcessorExternalClock::Unknown => Self::Unknown,
            smbioslib::ProcessorExternalClock::MHz(mhz) => Self::MHz(mhz),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProcessorSpeed {
    Unknown,
    MHz(u16),
}

impl Display for ProcessorSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::MHz(mhz) => write!(f, "{} MHz", mhz),
        }
    }
}

impl From<smbioslib::ProcessorSpeed> for ProcessorSpeed {
    fn from(value: smbioslib::ProcessorSpeed) -> Self {
        match value {
            smbioslib::ProcessorSpeed::Unknown => Self::Unknown,
            smbioslib::ProcessorSpeed::MHz(mhz) => Self::MHz(mhz),
        }
    }
}

/// Processor Socket and CPU Status
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessorStatus {
    pub raw: u8,

    /// CPU Socket Populated
    pub socket_populated: bool,

    /// CPU Status
    pub cpu_status: CpuStatus,
}

impl From<smbioslib::ProcessorStatus> for ProcessorStatus {
    fn from(value: smbioslib::ProcessorStatus) -> Self {
        Self {
            raw: value.raw,
            socket_populated: value.socket_populated(),
            cpu_status: CpuStatus::from(value.cpu_status()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CpuStatus {
    Unknown,
    Enabled,
    UserDisabled,
    BiosDisabled,
    Idle,
    Other,
    None,
}

impl Display for CpuStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "Unknown",
                Self::Enabled => "CPU Enabled",
                Self::UserDisabled => "CPU disabled by user through BIOS Setup",
                Self::BiosDisabled => "CPU disabled by BIOS (POST Error)",
                Self::Idle => "CPU is Idle, waiting to be enabled",
                Self::Other => "Other",
                Self::None => "A value unknown to this standard, check the raw value",
            }
        )
    }
}

impl From<smbioslib::CpuStatus> for CpuStatus {
    fn from(value: smbioslib::CpuStatus) -> Self {
        match value {
            smbioslib::CpuStatus::Unknown => Self::Unknown,
            smbioslib::CpuStatus::Enabled => Self::Enabled,
            smbioslib::CpuStatus::UserDisabled => Self::UserDisabled,
            smbioslib::CpuStatus::BiosDisabled => Self::BiosDisabled,
            smbioslib::CpuStatus::Idle => Self::Idle,
            smbioslib::CpuStatus::Other => Self::Other,
            smbioslib::CpuStatus::None => Self::None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessorUpgradeData {
    pub raw: u8,
    pub value: ProcessorUpgrade,
}

impl From<smbioslib::ProcessorUpgradeData> for ProcessorUpgradeData {
    fn from(value: smbioslib::ProcessorUpgradeData) -> Self {
        Self {
            raw: value.raw,
            value: ProcessorUpgrade::from(value.value),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ProcessorUpgrade {
    Other,
    Unknown,
    DaughterBoard,
    ZIFSocket,
    ReplaceablePiggyBack,
    NoUpgrade,
    LIFSocket,
    Slot1,
    Slot2,
    PinSocket370,
    SlotA,
    SlotM,
    Socket423,
    SocketASocket462,
    Socket478,
    Socket754,
    Socket940,
    Socket939,
    SocketmPGA604,
    SocketLGA771,
    SocketLGA775,
    SocketS1,
    SocketAM2,
    SocketF1207,
    SocketLGA1366,
    SocketG34,
    SocketAM3,
    SocketC32,
    SocketLGA1156,
    SocketLGA1567,
    SocketPGA988A,
    SocketBGA1288,
    SocketrPGA988B,
    SocketBGA1023,
    SocketBGA1224,
    SocketLGA1155,
    SocketLGA1356,
    SocketLGA2011,
    SocketFS1,
    SocketFS2,
    SocketFM1,
    SocketFM2,
    SocketLGA2011_3,
    SocketLGA1356_3,
    SocketLGA1150,
    SocketBGA1168,
    SocketBGA1234,
    SocketBGA1364,
    SocketAM4,
    SocketLGA1151,
    SocketBGA1356,
    SocketBGA1440,
    SocketBGA1515,
    SocketLGA3647_1,
    SocketSP3,
    SocketSP3r23,
    SocketLGA2066,
    SocketBGA1392,
    SocketBGA1510,
    SocketBGA1528,
    SocketLGA4189,
    SocketLGA1200,
    SocketLGA4677,
    SocketLGA1700,
    SocketBGA1744,
    SocketBGA1781,
    SocketBGA1211,
    SocketBGA2422,
    SocketLGA1211,
    SocketLGA2422,
    SocketLGA5773,
    SocketBGA5773,
    SocketAM5,
    SocketSP5,
    SocketSP6,
    SocketBGA883,
    SocketBGA1190,
    SocketBGA4129,
    SocketLGA4710,
    SocketLGA7529,
    SocketBGA1964,
    SocketBGA1792,
    SocketBGA2049,
    SocketBGA2551,
    SocketBGA2114,
    SocketBGA2833,
    SocketLGA1851,
    SeeSocketType,
    None,
}

impl Display for ProcessorUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::DaughterBoard => "Daughter Board",
                Self::ZIFSocket => "ZIF Socket",
                Self::ReplaceablePiggyBack => "Replaceable Piggy Back",
                Self::NoUpgrade => "No Upgrade",
                Self::LIFSocket => "LIF Socket",
                Self::Slot1 => "Slot #1",
                Self::Slot2 => "Slot #2",
                Self::SlotA => "Slot A",
                Self::SlotM => "Slot M",
                Self::PinSocket370 => "370-pin socket",
                Self::Socket423 => "Socket 423",
                Self::SocketASocket462 => "Socket A (Socket 462)",
                Self::Socket478 => "Socket 478",
                Self::Socket754 => "Socket 754",
                Self::Socket940 => "Socket 940",
                Self::Socket939 => "Socket 939",
                Self::SocketmPGA604 => "Socket mPGA604",
                Self::SocketLGA771 => "Socket LGA771",
                Self::SocketLGA775 => "Socket LGA775",
                Self::SocketS1 => "Socket S1",
                Self::SocketAM2 => "Socket AM2",
                Self::SocketF1207 => "Socket F (1207)",
                Self::SocketLGA1366 => "Socket LGA1366",
                Self::SocketG34 => "Socket G34",
                Self::SocketAM3 => "Socket AM3",
                Self::SocketC32 => "Socket C32",
                Self::SocketLGA1156 => "Socket LGA1156",
                Self::SocketLGA1567 => "Socket LGA1567",
                Self::SocketPGA988A => "Socket PGA988A",
                Self::SocketBGA1288 => "Socket BGA1288",
                Self::SocketrPGA988B => "Socket rPGA988B",
                Self::SocketBGA1023 => "Socket BGA1023",
                Self::SocketBGA1224 => "Socket BGA1224",
                Self::SocketLGA1155 => "Socket LGA1155",
                Self::SocketLGA1356 => "Socket LGA1356",
                Self::SocketLGA2011 => "Socket LGA2011",
                Self::SocketFS1 => "Socket FS1",
                Self::SocketFS2 => "Socket FS2",
                Self::SocketFM1 => "Socket FM1",
                Self::SocketFM2 => "Socket FM2",
                Self::SocketLGA2011_3 => "Socket LGA2011-3",
                Self::SocketLGA1356_3 => "Socket LGA1356-3",
                Self::SocketLGA1150 => "Socket LGA1150",
                Self::SocketBGA1168 => "Socket BGA1168",
                Self::SocketBGA1234 => "Socket BGA1234",
                Self::SocketBGA1364 => "Socket BGA1364",
                Self::SocketAM4 => "Socket AM4",
                Self::SocketLGA1151 => "Socket LGA1151",
                Self::SocketBGA1356 => "Socket BGA1356",
                Self::SocketBGA1440 => "Socket BGA1440",
                Self::SocketBGA1515 => "Socket BGA1515",
                Self::SocketLGA3647_1 => "Socket LGA3647-1",
                Self::SocketSP3 => "Socket SP3",
                Self::SocketSP3r23 => "Socket SP3r2",
                Self::SocketLGA2066 => "Socket LGA2066",
                Self::SocketBGA1392 => "Socket BGA1392",
                Self::SocketBGA1510 => "Socket BGA1510",
                Self::SocketBGA1528 => "Socket BGA1528",
                Self::SocketLGA4189 => "Socket LGA4189",
                Self::SocketLGA1200 => "Socket LGA1200",
                Self::SocketLGA4677 => "Socket LGA4677",
                Self::SocketLGA1700 => "Socket LGA1700",
                Self::SocketBGA1744 => "Socket BGA1744",
                Self::SocketBGA1781 => "Socket BGA1781",
                Self::SocketBGA1211 => "Socket BGA1211",
                Self::SocketBGA2422 => "Socket BGA2422",
                Self::SocketLGA1211 => "Socket LGA1211",
                Self::SocketLGA2422 => "Socket LGA2422",
                Self::SocketLGA5773 => "Socket LGA5773",
                Self::SocketBGA5773 => "Socket BGA5773",
                Self::SocketAM5 => "Socket AM5",
                Self::SocketSP5 => "Socket SP5",
                Self::SocketSP6 => "Socket SP6",
                Self::SocketBGA883 => "Socket BGA883",
                Self::SocketBGA1190 => "Socket BGA1190",
                Self::SocketBGA4129 => "Socket BGA4129",
                Self::SocketLGA4710 => "Socket LGA4710",
                Self::SocketLGA7529 => "Socket LGA7529",
                Self::SocketBGA1964 => "Socket BGA1964",
                Self::SocketBGA1792 => "Socket BGA1792",
                Self::SocketBGA2049 => "Socket BGA2049",
                Self::SocketBGA2551 => "Socket BGA2551",
                Self::SocketBGA2114 => "Socket BGA2114",
                Self::SocketBGA2833 => "Socket BGA2833",
                Self::SocketLGA1851 => "Socket LGA1851",
                Self::SeeSocketType => "See Socket Type",
                Self::None => "A value unknown to this standard, check the raw value",
            }
        )
    }
}

impl From<smbioslib::ProcessorUpgrade> for ProcessorUpgrade {
    fn from(value: smbioslib::ProcessorUpgrade) -> Self {
        match value {
            smbioslib::ProcessorUpgrade::Other => Self::Other,
            smbioslib::ProcessorUpgrade::Unknown => Self::Unknown,
            smbioslib::ProcessorUpgrade::DaughterBoard => Self::DaughterBoard,
            smbioslib::ProcessorUpgrade::ZIFSocket => Self::ZIFSocket,
            smbioslib::ProcessorUpgrade::ReplaceablePiggyBack => Self::ReplaceablePiggyBack,
            smbioslib::ProcessorUpgrade::NoUpgrade => Self::NoUpgrade,
            smbioslib::ProcessorUpgrade::LIFSocket => Self::LIFSocket,
            smbioslib::ProcessorUpgrade::Slot1 => Self::Slot1,
            smbioslib::ProcessorUpgrade::Slot2 => Self::Slot2,
            smbioslib::ProcessorUpgrade::PinSocket370 => Self::PinSocket370,
            smbioslib::ProcessorUpgrade::SlotA => Self::SlotA,
            smbioslib::ProcessorUpgrade::SlotM => Self::SlotM,
            smbioslib::ProcessorUpgrade::Socket423 => Self::Socket423,
            smbioslib::ProcessorUpgrade::SocketASocket462 => Self::SocketASocket462,
            smbioslib::ProcessorUpgrade::Socket478 => Self::Socket478,
            smbioslib::ProcessorUpgrade::Socket754 => Self::Socket754,
            smbioslib::ProcessorUpgrade::Socket940 => Self::Socket940,
            smbioslib::ProcessorUpgrade::Socket939 => Self::Socket939,
            smbioslib::ProcessorUpgrade::SocketmPGA604 => Self::SocketmPGA604,
            smbioslib::ProcessorUpgrade::SocketLGA771 => Self::SocketLGA771,
            smbioslib::ProcessorUpgrade::SocketLGA775 => Self::SocketLGA775,
            smbioslib::ProcessorUpgrade::SocketS1 => Self::SocketS1,
            smbioslib::ProcessorUpgrade::SocketAM2 => Self::SocketAM2,
            smbioslib::ProcessorUpgrade::SocketF1207 => Self::SocketF1207,
            smbioslib::ProcessorUpgrade::SocketLGA1366 => Self::SocketLGA1366,
            smbioslib::ProcessorUpgrade::SocketG34 => Self::SocketG34,
            smbioslib::ProcessorUpgrade::SocketAM3 => Self::SocketAM3,
            smbioslib::ProcessorUpgrade::SocketC32 => Self::SocketC32,
            smbioslib::ProcessorUpgrade::SocketLGA1156 => Self::SocketLGA1156,
            smbioslib::ProcessorUpgrade::SocketLGA1567 => Self::SocketLGA1567,
            smbioslib::ProcessorUpgrade::SocketPGA988A => Self::SocketPGA988A,
            smbioslib::ProcessorUpgrade::SocketBGA1288 => Self::SocketBGA1288,
            smbioslib::ProcessorUpgrade::SocketrPGA988B => Self::SocketrPGA988B,
            smbioslib::ProcessorUpgrade::SocketBGA1023 => Self::SocketBGA1023,
            smbioslib::ProcessorUpgrade::SocketBGA1224 => Self::SocketBGA1224,
            smbioslib::ProcessorUpgrade::SocketLGA1155 => Self::SocketLGA1155,
            smbioslib::ProcessorUpgrade::SocketLGA1356 => Self::SocketLGA1356,
            smbioslib::ProcessorUpgrade::SocketLGA2011 => Self::SocketLGA2011,
            smbioslib::ProcessorUpgrade::SocketFS1 => Self::SocketFS1,
            smbioslib::ProcessorUpgrade::SocketFS2 => Self::SocketFS2,
            smbioslib::ProcessorUpgrade::SocketFM1 => Self::SocketFM1,
            smbioslib::ProcessorUpgrade::SocketFM2 => Self::SocketFM2,
            smbioslib::ProcessorUpgrade::SocketLGA2011_3 => Self::SocketLGA2011_3,
            smbioslib::ProcessorUpgrade::SocketLGA1356_3 => Self::SocketLGA1356_3,
            smbioslib::ProcessorUpgrade::SocketLGA1150 => Self::SocketLGA1150,
            smbioslib::ProcessorUpgrade::SocketBGA1168 => Self::SocketBGA1168,
            smbioslib::ProcessorUpgrade::SocketBGA1234 => Self::SocketBGA1234,
            smbioslib::ProcessorUpgrade::SocketBGA1364 => Self::SocketBGA1364,
            smbioslib::ProcessorUpgrade::SocketAM4 => Self::SocketAM4,
            smbioslib::ProcessorUpgrade::SocketLGA1151 => Self::SocketLGA1151,
            smbioslib::ProcessorUpgrade::SocketBGA1356 => Self::SocketBGA1356,
            smbioslib::ProcessorUpgrade::SocketBGA1440 => Self::SocketBGA1440,
            smbioslib::ProcessorUpgrade::SocketBGA1515 => Self::SocketBGA1515,
            smbioslib::ProcessorUpgrade::SocketLGA3647_1 => Self::SocketLGA3647_1,
            smbioslib::ProcessorUpgrade::SocketSP3 => Self::SocketSP3,
            smbioslib::ProcessorUpgrade::SocketSP3r23 => Self::SocketSP3r23,
            smbioslib::ProcessorUpgrade::SocketLGA2066 => Self::SocketLGA2066,
            smbioslib::ProcessorUpgrade::SocketBGA1392 => Self::SocketBGA1392,
            smbioslib::ProcessorUpgrade::SocketBGA1510 => Self::SocketBGA1510,
            smbioslib::ProcessorUpgrade::SocketBGA1528 => Self::SocketBGA1528,
            smbioslib::ProcessorUpgrade::SocketLGA4189 => Self::SocketLGA4189,
            smbioslib::ProcessorUpgrade::SocketLGA1200 => Self::SocketLGA1200,
            smbioslib::ProcessorUpgrade::SocketLGA4677 => Self::SocketLGA4677,
            smbioslib::ProcessorUpgrade::SocketLGA1700 => Self::SocketLGA1700,
            smbioslib::ProcessorUpgrade::SocketBGA1744 => Self::SocketBGA1744,
            smbioslib::ProcessorUpgrade::SocketBGA1781 => Self::SocketBGA1781,
            smbioslib::ProcessorUpgrade::SocketBGA1211 => Self::SocketBGA1211,
            smbioslib::ProcessorUpgrade::SocketBGA2422 => Self::SocketBGA2422,
            smbioslib::ProcessorUpgrade::SocketLGA1211 => Self::SocketLGA1211,
            smbioslib::ProcessorUpgrade::SocketLGA2422 => Self::SocketLGA2422,
            smbioslib::ProcessorUpgrade::SocketLGA5773 => Self::SocketLGA5773,
            smbioslib::ProcessorUpgrade::SocketBGA5773 => Self::SocketBGA5773,
            smbioslib::ProcessorUpgrade::SocketAM5 => Self::SocketAM5,
            smbioslib::ProcessorUpgrade::SocketSP5 => Self::SocketSP5,
            smbioslib::ProcessorUpgrade::SocketSP6 => Self::SocketSP6,
            smbioslib::ProcessorUpgrade::SocketBGA883 => Self::SocketBGA883,
            smbioslib::ProcessorUpgrade::SocketBGA1190 => Self::SocketBGA1190,
            smbioslib::ProcessorUpgrade::SocketBGA4129 => Self::SocketBGA4129,
            smbioslib::ProcessorUpgrade::SocketLGA4710 => Self::SocketLGA4710,
            smbioslib::ProcessorUpgrade::SocketLGA7529 => Self::SocketLGA7529,
            smbioslib::ProcessorUpgrade::SocketBGA1964 => Self::SocketBGA1964,
            smbioslib::ProcessorUpgrade::SocketBGA2049 => Self::SocketBGA2049,
            smbioslib::ProcessorUpgrade::SocketBGA1792 => Self::SocketBGA1792,
            smbioslib::ProcessorUpgrade::SocketBGA2551 => Self::SocketBGA2551,
            smbioslib::ProcessorUpgrade::SocketBGA2114 => Self::SocketBGA2114,
            smbioslib::ProcessorUpgrade::SocketBGA2833 => Self::SocketBGA2833,
            smbioslib::ProcessorUpgrade::SocketLGA1851 => Self::SocketLGA1851,
            smbioslib::ProcessorUpgrade::SeeSocketType => Self::SeeSocketType,
            smbioslib::ProcessorUpgrade::None => Self::None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoreCount {
    Unknown,
    Count(u8),
    SeeCoreCount2,
}

impl Display for CoreCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::SeeCoreCount2 => write!(f, "See next core count entry"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::CoreCount> for CoreCount {
    fn from(value: smbioslib::CoreCount) -> Self {
        match value {
            smbioslib::CoreCount::Unknown => Self::Unknown,
            smbioslib::CoreCount::SeeCoreCount2 => Self::SeeCoreCount2,
            smbioslib::CoreCount::Count(cnt) => Self::Count(cnt),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoreCount2 {
    Unknown,
    Count(u16),
    Reserved,
}

impl Display for CoreCount2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::CoreCount2> for CoreCount2 {
    fn from(value: smbioslib::CoreCount2) -> Self {
        match value {
            smbioslib::CoreCount2::Unknown => Self::Unknown,
            smbioslib::CoreCount2::Reserved => Self::Reserved,
            smbioslib::CoreCount2::Count(cnt) => Self::Count(cnt),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoresEnabled {
    Unknown,
    Count(u8),
    SeeCoresEnabled2,
}

impl Display for CoresEnabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::SeeCoresEnabled2 => write!(f, "See next cores enabled entry"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::CoresEnabled> for CoresEnabled {
    fn from(value: smbioslib::CoresEnabled) -> Self {
        match value {
            smbioslib::CoresEnabled::Unknown => Self::Unknown,
            smbioslib::CoresEnabled::SeeCoresEnabled2 => Self::SeeCoresEnabled2,
            smbioslib::CoresEnabled::Count(cnt) => Self::Count(cnt),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoresEnabled2 {
    Unknown,
    Count(u16),
    Reserved,
}

impl Display for CoresEnabled2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::CoresEnabled2> for CoresEnabled2 {
    fn from(value: smbioslib::CoresEnabled2) -> Self {
        match value {
            smbioslib::CoresEnabled2::Unknown => Self::Unknown,
            smbioslib::CoresEnabled2::Reserved => Self::Reserved,
            smbioslib::CoresEnabled2::Count(cnt) => Self::Count(cnt),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ThreadCount {
    Unknown,
    Count(u8),
    SeeThreadCount2,
}

impl Display for ThreadCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::SeeThreadCount2 => write!(f, "See next thread enabled entry"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::ThreadCount> for ThreadCount {
    fn from(value: smbioslib::ThreadCount) -> Self {
        match value {
            smbioslib::ThreadCount::SeeThreadCount2 => Self::SeeThreadCount2,
            smbioslib::ThreadCount::Unknown => Self::Unknown,
            smbioslib::ThreadCount::Count(cnt) => Self::Count(cnt),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ThreadCount2 {
    Unknown,
    Count(u16),
    Reserved,
}

impl Display for ThreadCount2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::ThreadCount2> for ThreadCount2 {
    fn from(value: smbioslib::ThreadCount2) -> Self {
        match value {
            smbioslib::ThreadCount2::Reserved => Self::Reserved,
            smbioslib::ThreadCount2::Unknown => Self::Unknown,
            smbioslib::ThreadCount2::Count(cnt) => Self::Count(cnt),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ThreadEnabled {
    Unknown,
    Count(u16),
    Reserved,
}

impl Display for ThreadEnabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Count(cnt) => write!(f, "{}", cnt),
        }
    }
}

impl From<smbioslib::ThreadEnabled> for ThreadEnabled {
    fn from(value: smbioslib::ThreadEnabled) -> Self {
        match value {
            smbioslib::ThreadEnabled::Reserved => Self::Reserved,
            smbioslib::ThreadEnabled::Unknown => Self::Unknown,
            smbioslib::ThreadEnabled::Count(cnt) => Self::Count(cnt),
        }
    }
}

/// Attributes of each CPU cache device in the system
#[derive(Debug, Serialize)]
pub struct Caches {
    pub caches: Vec<Cache>,
}

impl Caches {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let mut caches = vec![];

        for cache_device in table.collect::<smbioslib::SMBiosCacheInformation>() {
            caches.push(Cache::from(cache_device));
        }

        Ok(Self { caches })
    }
}

impl ToJson for Caches {}

// struct_migrate!(CacheConfiguration, smbioslib::CacheConfiguaration, {
//     raw: u16,
// });

#[derive(Debug, Serialize)]
pub struct CacheConfiguaration {
    pub raw: u16,
}

/// This structure defines the attributes of CPU cache device in the
/// system. One structure is specified for each such device, whether
/// the device is internal to or external to the CPU module.
#[derive(Debug, Serialize)]
pub struct Cache {
    /// String number for reference designation
    pub socket_designation: Option<String>,

    /// Bit fields describing the cache configuration
    pub cache_configuration: Option<CacheConfiguaration>,

    /// Maximum size that can be installed
    pub maximum_cache_size: Option<smbioslib::CacheMemorySize>,

    /// Same format as Max Cache Size field; set 0 if no cache
    /// is installed.
    pub installed_size: Option<smbioslib::CacheMemorySize>,

    /// Supported SRAM type
    pub supported_sram_type: Option<smbioslib::SramTypes>,

    /// Current SRAM type
    pub current_sram_type: Option<smbioslib::SramTypes>,

    /// Cache module speed, in nanosecs. The value is 0 if the
    /// speed is unknown
    pub cache_speed: Option<u8>,

    /// Error-correction scheme supported by this cache component
    pub error_correction_type: Option<smbioslib::ErrorCorrectionTypeData>,

    /// Logical type of cache
    pub system_cache_type: Option<smbioslib::SystemCacheTypeData>,

    /// Associativity of the cache
    pub associativity: Option<smbioslib::CacheAssociativityData>,

    /// Maximum cache size
    pub maximum_cache_size_2: Option<smbioslib::CacheMemorySize>,

    /// Installed cache size
    pub installed_cache_size_2: Option<smbioslib::CacheMemorySize>,
}

impl<'a> From<smbioslib::SMBiosCacheInformation<'a>> for Cache {
    fn from(value: smbioslib::SMBiosCacheInformation) -> Self {
        Self {
            socket_designation: value.socket_designation().ok(),
            cache_configuration: match value.cache_configuration() {
                Some(conf) => Some(CacheConfiguaration { raw: conf.raw }),
                None => None,
            },
            maximum_cache_size: value.maximum_cache_size(),
            installed_size: value.installed_size(),
            supported_sram_type: value.supported_sram_type(),
            current_sram_type: value.current_sram_type(),
            cache_speed: value.cache_speed(),
            error_correction_type: value.error_correction_type(),
            system_cache_type: value.system_cache_type(),
            associativity: value.associativity(),
            maximum_cache_size_2: value.maximum_cache_size_2(),
            installed_cache_size_2: value.installed_cache_size_2(),
        }
    }
}
impl ToJson for Cache {}

/// Attributes of a system port connectors
#[derive(Debug, Serialize)]
pub struct PortConnectors {
    pub ports: Vec<Port>,
}

impl PortConnectors {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let mut ports = vec![];

        for port in table.collect::<smbioslib::SMBiosPortConnectorInformation>() {
            ports.push(Port::from(port));
        }

        Ok(Self { ports })
    }
}

impl ToJson for PortConnectors {}

/// Attributes of a system port connector (serial, parallel,
/// keyboard or mouse ports)
#[derive(Debug, Serialize)]
pub struct Port {
    /// Internal reference designator, that is, internal to the
    /// system enclosure
    pub internal_reference_designator: Option<String>,

    /// Internal connector type
    pub internal_connector_type: Option<smbioslib::PortInformationConnectorTypeData>,

    /// External reference designation, external to the system
    /// enclosure
    pub external_reference_designator: Option<String>,

    /// External connector type
    pub external_connector_type: Option<smbioslib::PortInformationConnectorTypeData>,

    /// Function of the port
    pub port_type: Option<smbioslib::PortInformationPortTypeData>,
}

impl<'a> From<smbioslib::SMBiosPortConnectorInformation<'a>> for Port {
    fn from(value: smbioslib::SMBiosPortConnectorInformation) -> Self {
        Self {
            internal_reference_designator: value.internal_reference_designator().ok(),
            internal_connector_type: value.internal_connector_type(),
            external_reference_designator: value.external_reference_designator().ok(),
            external_connector_type: value.external_connector_type(),
            port_type: value.port_type(),
        }
    }
}
impl ToJson for Port {}

/// Collection of memory devices that operate together to form a memory address space
#[derive(Debug, Serialize)]
pub struct MemoryArray {
    /// Physical location of the Memory Array, whether on the system
    /// board or an add-in board
    pub location: Option<smbioslib::MemoryArrayLocationData>,

    /// Which the array is used
    pub usage: Option<smbioslib::MemoryArrayUseData>,

    /// Primary hardware error correction or detection method
    /// supported by this memory array
    pub memory_error_correction: Option<smbioslib::MemoryArrayErrorCorrectionData>,

    /// Maximum memory capacity, in kbytes, for this array
    pub maximum_capacity: Option<smbioslib::MaximumMemoryCapacity>,

    /// Handle, or instance number, associated with any error that
    /// was previously detected for the array
    pub memory_error_information_handle: Option<smbioslib::Handle>,

    /// Number of slots/sockets available for memory devices in
    /// this array
    pub number_of_memory_devices: Option<u16>,

    /// Maximum memory capacity, in bytes, for this array. This
    /// field is only valid when the Maximum Capacity field
    /// contains 8000 0000h.
    pub extended_maximum_capacity: Option<u64>,
}

impl MemoryArray {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosPhysicalMemoryArray| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about memory array (type 16)!"
            ))?;

        Ok(Self {
            location: t.location(),
            usage: t.usage(),
            memory_error_correction: t.memory_error_correction(),
            maximum_capacity: t.maximum_capacity(),
            memory_error_information_handle: t.memory_error_information_handle(),
            number_of_memory_devices: t.number_of_memory_devices(),
            extended_maximum_capacity: t.extended_maximum_capacity(),
        })
    }
}

impl ToJson for MemoryArray {}

/// Information about all installed memory devices
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryDevices {
    pub memory: Vec<MemoryDevice>,
}

impl MemoryDevices {
    /// Creates a new instance of `Self`
    ///
    /// It is usually not required, since an instance of this
    /// structure will be created using the method
    /// `Self::new_from_table(table: &SMBiosData)` in the constructor
    /// [`DMITable::new()`].
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let mut memory = vec![];

        for mem in table.collect::<smbioslib::SMBiosMemoryDevice>() {
            memory.push(MemoryDevice::from(mem));
        }

        Ok(Self { memory })
    }
}

impl ToJson for MemoryDevices {}

/// Information about single memory device
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryDevice {
    /// Handle or instance number, associated with the physical
    /// memory array to which this device belongs
    pub physical_memory_array_handle: Option<Handle>,

    /// Handle or instance number, associated with any error that
    /// was previously detected for the device. If the system does
    /// not provide the error information structure, the field
    /// containes FFFEH
    pub memory_error_information_handle: Option<Handle>,

    /// Total width, in bits, of this memory device, including any
    /// check or error-correction bits
    pub total_width: Option<u16>,

    /// Data width, in bits, of this memory device
    pub data_width: Option<u16>,

    /// Size of memory device
    pub size: Option<MemorySize>,

    /// Form factor for this memory device
    pub form_factor: Option<MemoryFormFactorData>,

    /// Identifies when the Memory Device is one of a set of
    /// Memory Devices that must be populated with all devices
    /// of the same type and size, and the set to which this
    /// device belongs A value of 0 indicates that the device
    /// is not part of a set; a value of FFh indicates that the
    /// attribute is unknown
    pub device_set: Option<u8>,

    /// Physically-labeled socket or board position where the
    /// memory device is located
    pub device_locator: Option<String>,

    /// Physically-labeled bank where the memory device is located
    pub bank_locator: Option<String>,

    /// Type of memory used in this device
    pub memory_type: Option<MemoryDeviceTypeData>,

    /// Additional detail on the memory device type
    pub type_detail: Option<MemoryTypeDetails>,

    /// The maximum capable speed of the device (MT/s)
    pub speed: Option<MemorySpeed>,

    /// Manufacturer of this memory device
    pub manufacturer: Option<String>,

    /// Serial number of this memory device
    pub serial_number: Option<String>,

    /// Asset tag of this memory device
    pub asset_tag: Option<String>,

    /// Part number of this memory device
    pub part_number: Option<String>,

    /// Bits 7-4: reserved Bits 3-0: rank Value=0 for unknown rank information
    pub attributes: Option<u8>,

    /// Extended suze of the memory device in MB
    pub extended_size: Option<MemorySizeExtended>,

    /// Configured speed of the memory device, in megatransfers per second (MT/s)
    pub configured_memory_speed: Option<MemorySpeed>,

    /// Minimum operating voltage for this device, in millivolts
    pub minimum_voltage: Option<u16>,

    /// Maximum operating voltage for this device, in millivolts
    pub maximum_voltage: Option<u16>,

    /// Configured voltage for this device, in millivolts
    pub configured_voltage: Option<u16>,

    /// Memory technology type for this memory device
    pub memory_technology: Option<MemoryDeviceTechnologyData>,

    /// The operating modes supported by this memory device
    pub memory_operating_mode_capability: Option<MemoryOperatingModeCapabilities>,

    /// Firmware version of this memory device
    pub firmware_version: Option<String>,

    /// Two-byte module manufacturer ID found in the SPD of this
    /// memory device; LSB first
    pub module_manufacturer_id: Option<u16>,

    /// Two-byte module product id found in the SPD of this memory
    /// device; LSB first
    pub module_product_id: Option<u16>,

    /// Two-byte memory subsystem controller manufacturer ID found
    /// in the SPD of this memory device; LSB first
    pub memory_subsystem_controller_manufacturer_id: Option<u16>,

    /// Two-byte memory subsystem controller product ID found in
    /// the SPD of this memory device; LSB first
    pub memory_subsystem_controller_product_id: Option<u16>,

    /// Size of the Non-volatile portion of the memory device in
    /// Bytes, if any
    pub non_volatile_size: Option<MemoryIndicatedSize>,

    /// Size of the Volatile portion of the memory device in
    /// Bytes, if any
    pub volatile_size: Option<MemoryIndicatedSize>,

    /// Size of the Cache portion of the memory device in Bytes,
    /// if any
    pub cache_size: Option<MemoryIndicatedSize>,

    /// Size of the Logical memory device in Bytes
    pub logical_size: Option<MemoryIndicatedSize>,

    /// Extended speed of the memory device (complements the
    /// Speed field at offset 15h). Identifies the maximum capable
    /// speed of the device, in MT/s
    pub extended_speed: Option<MemorySpeedExtended>,

    /// Extended configured memory speed of the memory device
    /// (complements the `configure_memory_speed` field at offset
    /// 20h). Identifies the configured speed of the memory device,
    /// in MT/s
    pub extended_configured_speed: Option<MemorySpeedExtended>,

    /// Two-byte PMIC0 manufacturer ID found in the SPD of this
    /// memory device; LSB first
    pub pmic0_manufacturer_id: Option<u16>,

    /// PMIC 0 Revision Number found in the SPD of this memory
    /// device
    pub pmic0_revision_number: Option<u16>,

    /// Two-byte RCD manufacturer ID found in the SPD of this
    /// memory device; LSB first
    pub rcd_manufacturer_id: Option<u16>,

    /// RCD 0 Revision Number found in the SPD of this memory
    /// device
    pub rcd_revision_number: Option<u16>,
}

impl<'a> From<smbioslib::SMBiosMemoryDevice<'a>> for MemoryDevice {
    fn from(value: smbioslib::SMBiosMemoryDevice) -> Self {
        Self {
            physical_memory_array_handle: Handle::from_opt(value.physical_memory_array_handle()),
            memory_error_information_handle: Handle::from_opt(
                value.memory_error_information_handle(),
            ),
            total_width: value.total_width(),
            data_width: value.data_width(),
            size: value.size().map(|s| MemorySize::from(s)),
            form_factor: match value.form_factor() {
                Some(ff) => Some(MemoryFormFactorData::from(ff)),
                _ => None,
            },
            device_set: value.device_set(),
            device_locator: value.device_locator().ok(),
            bank_locator: value.bank_locator().ok(),
            memory_type: value.memory_type().map(|mt| MemoryDeviceTypeData::from(mt)),
            type_detail: value.type_detail().map(|td| MemoryTypeDetails::from(td)),
            speed: value.speed().map(|s| MemorySpeed::from(s)),
            manufacturer: value.manufacturer().ok(),
            serial_number: value.serial_number().ok(),
            asset_tag: value.asset_tag().ok(),
            part_number: value.part_number().ok(),
            attributes: value.attributes(),
            extended_size: value.extended_size().map(|es| MemorySizeExtended::from(es)),
            configured_memory_speed: value
                .configured_memory_speed()
                .map(|cms| MemorySpeed::from(cms)),
            minimum_voltage: value.minimum_voltage(),
            maximum_voltage: value.maximum_voltage(),
            configured_voltage: value.configured_voltage(),
            memory_technology: value
                .memory_technology()
                .map(|mt| MemoryDeviceTechnologyData::from(mt)),
            memory_operating_mode_capability: value
                .memory_operating_mode_capability()
                .map(|momc| MemoryOperatingModeCapabilities::from(momc)),
            firmware_version: value.firmware_version().ok(),
            module_manufacturer_id: value.module_manufacturer_id(),
            module_product_id: value.module_product_id(),
            memory_subsystem_controller_manufacturer_id: value
                .memory_subsystem_controller_manufacturer_id(),
            memory_subsystem_controller_product_id: value.memory_subsystem_controller_product_id(),
            non_volatile_size: value
                .non_volatile_size()
                .map(|n| MemoryIndicatedSize::from(n)),
            volatile_size: value.volatile_size().map(|n| MemoryIndicatedSize::from(n)),
            cache_size: value.cache_size().map(|n| MemoryIndicatedSize::from(n)),
            logical_size: value.logical_size().map(|n| MemoryIndicatedSize::from(n)),
            extended_speed: value
                .extended_speed()
                .map(|es| MemorySpeedExtended::from(es)),
            extended_configured_speed: value
                .extended_speed()
                .map(|ecs| MemorySpeedExtended::from(ecs)),
            pmic0_manufacturer_id: value.pmic0_manufacturer_id(),
            pmic0_revision_number: value.pmic0_revision_number(),
            rcd_manufacturer_id: value.rcd_manufacturer_id(),
            rcd_revision_number: value.rcd_revision_number(),
        }
    }
}

impl ToJson for MemoryDevice {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MemorySize {
    NotInstalled,
    Unknown,
    SeeExtendedSize,
    Kilobytes(u16),
    Megabytes(u16),
}

impl From<smbioslib::MemorySize> for MemorySize {
    fn from(value: smbioslib::MemorySize) -> Self {
        match value {
            smbioslib::MemorySize::NotInstalled => Self::NotInstalled,
            smbioslib::MemorySize::Unknown => Self::Unknown,
            smbioslib::MemorySize::SeeExtendedSize => Self::SeeExtendedSize,
            smbioslib::MemorySize::Kilobytes(kb) => Self::Kilobytes(kb),
            smbioslib::MemorySize::Megabytes(mb) => Self::Megabytes(mb),
        }
    }
}

impl Display for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotInstalled => "Not installed".to_string(),
                Self::Unknown => "Unknown".to_string(),
                Self::SeeExtendedSize => "See Extended Size".to_string(),
                Self::Kilobytes(n) => format!("{n} KB"),
                Self::Megabytes(n) => format!("{n} MB"),
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MemorySizeExtended {
    Megabytes(u32),
    SeeSize,
}

impl From<smbioslib::MemorySizeExtended> for MemorySizeExtended {
    fn from(value: smbioslib::MemorySizeExtended) -> Self {
        match value {
            smbioslib::MemorySizeExtended::Megabytes(mb) => Self::Megabytes(mb),
            smbioslib::MemorySizeExtended::SeeSize => Self::SeeSize,
        }
    }
}

impl Display for MemorySizeExtended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Megabytes(n) => format!("{n} MB"),
                Self::SeeSize => "See Size".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryFormFactorData {
    pub raw: u8,
    pub value: MemoryFormFactor,
}

impl From<smbioslib::MemoryFormFactorData> for MemoryFormFactorData {
    fn from(value: smbioslib::MemoryFormFactorData) -> Self {
        Self {
            raw: value.raw,
            value: MemoryFormFactor::from(value.value),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MemoryFormFactor {
    Other,
    Unknown,
    Simm,
    Sip,
    Chip,
    Dip,
    Zip,
    ProprietaryCard,
    Dimm,
    Tsop,
    RowOfChips,
    Rimm,
    Sodimm,
    Srimm,
    Fbdimm,
    Die,
    Camm,
    Cudimm,
    Csodimm,
    None,
}

impl From<smbioslib::MemoryFormFactor> for MemoryFormFactor {
    fn from(value: smbioslib::MemoryFormFactor) -> Self {
        match value {
            smbioslib::MemoryFormFactor::Other => Self::Other,
            smbioslib::MemoryFormFactor::Unknown => Self::Unknown,
            smbioslib::MemoryFormFactor::Simm => Self::Simm,
            smbioslib::MemoryFormFactor::Sip => Self::Sip,
            smbioslib::MemoryFormFactor::Chip => Self::Chip,
            smbioslib::MemoryFormFactor::Dip => Self::Dip,
            smbioslib::MemoryFormFactor::Zip => Self::Zip,
            smbioslib::MemoryFormFactor::ProprietaryCard => Self::ProprietaryCard,
            smbioslib::MemoryFormFactor::Dimm => Self::Dimm,
            smbioslib::MemoryFormFactor::Tsop => Self::Tsop,
            smbioslib::MemoryFormFactor::RowOfChips => Self::RowOfChips,
            smbioslib::MemoryFormFactor::Rimm => Self::Rimm,
            smbioslib::MemoryFormFactor::Sodimm => Self::Sodimm,
            smbioslib::MemoryFormFactor::Srimm => Self::Srimm,
            smbioslib::MemoryFormFactor::Fbdimm => Self::Fbdimm,
            smbioslib::MemoryFormFactor::Die => Self::Dip,
            smbioslib::MemoryFormFactor::Camm => Self::Camm,
            smbioslib::MemoryFormFactor::Cudimm => Self::Cudimm,
            smbioslib::MemoryFormFactor::Csodimm => Self::Csodimm,
            smbioslib::MemoryFormFactor::None => Self::None,
        }
    }
}

impl Display for MemoryFormFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::Simm => "SIMM",
                Self::Sip => "SIP",
                Self::Chip => "Chip",
                Self::Dip => "DIP",
                Self::Zip => "ZIP",
                Self::ProprietaryCard => "Proprietary Card",
                Self::Dimm => "DIMM",
                Self::Tsop => "TSOP",
                Self::RowOfChips => "Row of chips",
                Self::Rimm => "RIMM",
                Self::Sodimm => "SODIMM",
                Self::Srimm => "SRIMM",
                Self::Fbdimm => "FB-DIMM",
                Self::Die => "Die",
                Self::Camm => "CAMM",
                Self::Cudimm => "CUDIMM",
                Self::Csodimm => "CSODIMM",
                Self::None => "None",
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryDeviceTypeData {
    pub raw: u8,
    pub value: MemoryDeviceType,
}

impl From<smbioslib::MemoryDeviceTypeData> for MemoryDeviceTypeData {
    fn from(value: smbioslib::MemoryDeviceTypeData) -> Self {
        Self {
            raw: value.raw,
            value: MemoryDeviceType::from(value.value),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MemoryDeviceType {
    Other,
    Unknown,
    Dram,
    Edram,
    Vram,
    Sram,
    Ram,
    Rom,
    Flash,
    Eeprom,
    Feprom,
    Eprom,
    Cdram,
    ThreeDram,
    Sdram,
    Sgram,
    Rdram,
    Ddr,
    Ddr2,
    Ddr2Fbdimm,
    Ddr3,
    Fbd2,
    Ddr4,
    Lpddr,
    Lpddr2,
    Lpddr3,
    Lpddr4,
    LogicalNonVolatileDevice,
    Hbm,
    Hbm2,
    Ddr5,
    Lpddr5,
    Hbm3,
    Mrdimm,
    None,
}

impl From<smbioslib::MemoryDeviceType> for MemoryDeviceType {
    fn from(value: smbioslib::MemoryDeviceType) -> Self {
        match value {
            smbioslib::MemoryDeviceType::Other => Self::Other,
            smbioslib::MemoryDeviceType::Unknown => Self::Unknown,
            smbioslib::MemoryDeviceType::Dram => Self::Dram,
            smbioslib::MemoryDeviceType::Edram => Self::Edram,
            smbioslib::MemoryDeviceType::Vram => Self::Vram,
            smbioslib::MemoryDeviceType::Sram => Self::Sram,
            smbioslib::MemoryDeviceType::Ram => Self::Ram,
            smbioslib::MemoryDeviceType::Rom => Self::Rom,
            smbioslib::MemoryDeviceType::Flash => Self::Flash,
            smbioslib::MemoryDeviceType::Eeprom => Self::Eeprom,
            smbioslib::MemoryDeviceType::Feprom => Self::Feprom,
            smbioslib::MemoryDeviceType::Eprom => Self::Eprom,
            smbioslib::MemoryDeviceType::Cdram => Self::Cdram,
            smbioslib::MemoryDeviceType::ThreeDram => Self::ThreeDram,
            smbioslib::MemoryDeviceType::Sdram => Self::Sdram,
            smbioslib::MemoryDeviceType::Sgram => Self::Sgram,
            smbioslib::MemoryDeviceType::Rdram => Self::Rdram,
            smbioslib::MemoryDeviceType::Ddr => Self::Ddr,
            smbioslib::MemoryDeviceType::Ddr2 => Self::Ddr2,
            smbioslib::MemoryDeviceType::Ddr2Fbdimm => Self::Ddr2Fbdimm,
            smbioslib::MemoryDeviceType::Ddr3 => Self::Ddr3,
            smbioslib::MemoryDeviceType::Fbd2 => Self::Fbd2,
            smbioslib::MemoryDeviceType::Ddr4 => Self::Ddr4,
            smbioslib::MemoryDeviceType::Lpddr => Self::Lpddr,
            smbioslib::MemoryDeviceType::Lpddr2 => Self::Lpddr2,
            smbioslib::MemoryDeviceType::Lpddr3 => Self::Lpddr3,
            smbioslib::MemoryDeviceType::Lpddr4 => Self::Lpddr4,
            smbioslib::MemoryDeviceType::LogicalNonVolatileDevice => Self::LogicalNonVolatileDevice,
            smbioslib::MemoryDeviceType::Hbm => Self::Hbm,
            smbioslib::MemoryDeviceType::Hbm2 => Self::Hbm2,
            smbioslib::MemoryDeviceType::Ddr5 => Self::Ddr5,
            smbioslib::MemoryDeviceType::Lpddr5 => Self::Lpddr5,
            smbioslib::MemoryDeviceType::Hbm3 => Self::Hbm3,
            smbioslib::MemoryDeviceType::Mrdimm => Self::Mrdimm,
            smbioslib::MemoryDeviceType::None => Self::None,
        }
    }
}

impl Display for MemoryDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::Dram => "DRAM",
                Self::Edram => "EDRAM",
                Self::Vram => "VRAM",
                Self::Sram => "SRAM",
                Self::Ram => "RAM",
                Self::Rom => "ROM",
                Self::Flash => "Flash",
                Self::Eeprom => "EEPROM",
                Self::Feprom => "FEPROM",
                Self::Eprom => "EPROM",
                Self::Cdram => "CDRAM",
                Self::ThreeDram => "3DRAM",
                Self::Sdram => "SDRAM",
                Self::Sgram => "SGRAM",
                Self::Rdram => "RDRAM",
                Self::Ddr => "DDR",
                Self::Ddr2 => "DDR2",
                Self::Ddr2Fbdimm => "DDR2 FB-DIMM",
                Self::Ddr3 => "DDR3",
                Self::Fbd2 => "FBD2",
                Self::Ddr4 => "DDR4",
                Self::Lpddr => "LPDDR",
                Self::Lpddr2 => "LPDDR2",
                Self::Lpddr3 => "LPDDR3",
                Self::Lpddr4 => "LPDDR4",
                Self::LogicalNonVolatileDevice => "Logical non-volatile device",
                Self::Hbm => "HBM (High Bandwidth Memory)",
                Self::Hbm2 => "HBM2 (High Bandwidth Memory Generation 2)",
                Self::Ddr5 => "DDR5",
                Self::Lpddr5 => "LPDDR5",
                Self::Hbm3 => "HBM3",
                Self::Mrdimm => "MRDIMM",
                Self::None => "A value unknown to this standard",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryTypeDetails {
    /// Raw value
    pub raw: u16,

    /// Bit 1 other
    pub other: bool,

    /// But 2 unknown
    pub unknown: bool,

    /// Bit 3 fast-paged
    pub fast_paged: bool,

    /// Bit 4 static column
    pub static_column: bool,

    /// Bit 5 pseudo-static
    pub pseudo_static: bool,

    /// Bit 6 RAMBUS
    pub ram_bus: bool,

    /// Bit 7 synchronous
    pub synchronous: bool,

    /// Bit 8 CMOS
    pub cmos: bool,

    /// Bit 9 EDO
    pub edo: bool,

    /// Bit 10 Window DRAM
    pub window_dram: bool,

    /// Bit 11 Cache DRAM
    pub cache_dram: bool,

    // Bit 12 non-volatile
    pub non_volatile: bool,

    /// Bit 13 Registered (Buffered)
    pub registered: bool,

    /// Bit 14 Unbuffered (Unregistered)
    pub unbuffered: bool,

    /// Bit 15 LRDIMM
    pub lrdimm: bool,
}

impl From<smbioslib::MemoryTypeDetails> for MemoryTypeDetails {
    fn from(v: smbioslib::MemoryTypeDetails) -> Self {
        Self {
            raw: v.raw,
            other: v.other(),
            unknown: v.unknown(),
            fast_paged: v.fast_paged(),
            static_column: v.static_column(),
            pseudo_static: v.pseudo_static(),
            ram_bus: v.ram_bus(),
            synchronous: v.synchronous(),
            cmos: v.cmos(),
            edo: v.edo(),
            window_dram: v.window_dram(),
            cache_dram: v.cache_dram(),
            non_volatile: v.non_volatile(),
            registered: v.registered(),
            unbuffered: v.unbuffered(),
            lrdimm: v.lrdimm(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemorySpeed {
    Unknown,
    SeeExtendedSpeed,
    MTs(u16),
}

impl From<smbioslib::MemorySpeed> for MemorySpeed {
    fn from(value: smbioslib::MemorySpeed) -> Self {
        match value {
            smbioslib::MemorySpeed::Unknown => Self::Unknown,
            smbioslib::MemorySpeed::SeeExtendedSpeed => Self::SeeExtendedSpeed,
            smbioslib::MemorySpeed::MTs(mts) => Self::MTs(mts),
        }
    }
}

impl Display for MemorySpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::SeeExtendedSpeed => write!(f, "See Extended Speed"),
            Self::MTs(mts) => write!(f, "{mts} MT/s"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemorySpeedExtended {
    MTs(u32),
    SeeSpeed,
}

impl From<smbioslib::MemorySpeedExtended> for MemorySpeedExtended {
    fn from(value: smbioslib::MemorySpeedExtended) -> Self {
        match value {
            smbioslib::MemorySpeedExtended::MTs(mts) => Self::MTs(mts),
            smbioslib::MemorySpeedExtended::SeeSpeed => Self::SeeSpeed,
        }
    }
}

impl Display for MemorySpeedExtended {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MTs(mts) => write!(f, "{mts} MT/s"),
            Self::SeeSpeed => write!(f, "See Speed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryDeviceTechnologyData {
    pub raw: u8,
    pub value: MemoryDeviceTechnology,
}

impl From<smbioslib::MemoryDeviceTechnologyData> for MemoryDeviceTechnologyData {
    fn from(value: smbioslib::MemoryDeviceTechnologyData) -> Self {
        Self {
            raw: value.raw,
            value: MemoryDeviceTechnology::from(value.value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemoryDeviceTechnology {
    Other,
    Unknown,
    Dram,
    NvidimmN,
    NvidimmF,
    NvidimmP,
    IntelOptaneDcPersistentMemory,
    Mrdimm,
    None,
}

impl From<smbioslib::MemoryDeviceTechnology> for MemoryDeviceTechnology {
    fn from(value: smbioslib::MemoryDeviceTechnology) -> Self {
        match value {
            smbioslib::MemoryDeviceTechnology::Other => Self::Other,
            smbioslib::MemoryDeviceTechnology::Unknown => Self::Unknown,
            smbioslib::MemoryDeviceTechnology::Dram => Self::Dram,
            smbioslib::MemoryDeviceTechnology::NvdimmN => Self::NvidimmN,
            smbioslib::MemoryDeviceTechnology::NvdimmF => Self::NvidimmF,
            smbioslib::MemoryDeviceTechnology::NvdimmP => Self::NvidimmP,
            smbioslib::MemoryDeviceTechnology::IntelOptaneDcPersistentMemory => {
                Self::IntelOptaneDcPersistentMemory
            }
            smbioslib::MemoryDeviceTechnology::Mrdimm => Self::Mrdimm,
            smbioslib::MemoryDeviceTechnology::None => Self::None,
        }
    }
}

impl Display for MemoryDeviceTechnology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::Dram => "DRAM",
                Self::NvidimmN => "NVIDIMM-N",
                Self::NvidimmF => "NVIDIMM-F",
                Self::NvidimmP => "NVIDIMM-P",
                Self::IntelOptaneDcPersistentMemory => "Intel® Optane™ persistent memory",
                Self::Mrdimm => "MRDIMM (Deprecated)",
                Self::None => "???",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryOperatingModeCapabilities {
    /// Raw value
    pub raw: u16,

    /// Other
    pub other: bool,

    /// Unknown
    pub unknown: bool,

    /// Volatile memory
    pub volatile_memory: bool,

    /// Byte-accessible persistent memory
    pub byte_accessible_persistent_memory: bool,

    /// Block-accessible persistent memory
    pub block_accessible_persistent_memory: bool,
}

impl From<smbioslib::MemoryOperatingModeCapabilities> for MemoryOperatingModeCapabilities {
    fn from(v: smbioslib::MemoryOperatingModeCapabilities) -> Self {
        Self {
            raw: v.raw,
            other: v.other(),
            unknown: v.unknown(),
            volatile_memory: v.volatile_memory(),
            byte_accessible_persistent_memory: v.byte_accessible_persistent_memory(),
            block_accessible_persistent_memory: v.block_accessible_persistent_memory(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemoryIndicatedSize {
    Unknown,
    Bytes(u64),
}

impl From<smbioslib::MemoryIndicatedSize> for MemoryIndicatedSize {
    fn from(value: smbioslib::MemoryIndicatedSize) -> Self {
        match value {
            smbioslib::MemoryIndicatedSize::Unknown => Self::Unknown,
            smbioslib::MemoryIndicatedSize::Bytes(b) => Self::Bytes(b),
        }
    }
}

impl Display for MemoryIndicatedSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "Unknown".to_string(),
                Self::Bytes(n) => format!("{n} B"),
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CoolingDevice {
    /// Handle, or instance number, of the temperature probe monitoring this
    /// cooling device. A value of 0xFFFF indicates that no probe is provided.
    pub temperature_probe_handle: Option<Handle>,

    /// Cooling device type and status.
    pub device_type_and_status: Option<CoolingDeviceTypeAndStatus>,

    pub cooling_unit_group: Option<u8>,

    pub oem_defined: Option<u32>,
    pub rotational_speed: Option<RotationalSpeed>,
    pub description: String,
}

impl CoolingDevice {
    pub fn new() -> Result<Self> {
        let table = smbioslib::table_load_from_device()?;
        Self::new_from_table(&table)
    }

    pub fn new_from_table(table: &SMBiosData) -> Result<Self> {
        let t = table
            .find_map(|f: smbioslib::SMBiosCoolingDevice| Some(f))
            .ok_or(anyhow!(
                "Failed to get information about cooling device (type 27)!"
            ))?;

        Ok(Self {
            temperature_probe_handle: t.temperature_probe_handle().map(|h| Handle::from(h)),
            device_type_and_status: t
                .device_type_and_status()
                .map(|d| CoolingDeviceTypeAndStatus::from(d)),
            cooling_unit_group: t.cooling_unit_group(),
            oem_defined: t.oem_defined(),
            rotational_speed: t.nominal_speed().map(|r| RotationalSpeed::from(r)),
            description: t.description().to_string(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CoolingDeviceTypeAndStatus {
    /// Raw value
    pub raw: u8,

    /// Device status
    pub device_status: CoolingDeviceStatus,

    /// Device type
    pub device_type: CoolingDeviceType,
}

impl From<smbioslib::CoolingDeviceTypeAndStatus> for CoolingDeviceTypeAndStatus {
    fn from(value: smbioslib::CoolingDeviceTypeAndStatus) -> Self {
        Self {
            raw: value.raw,
            device_status: CoolingDeviceStatus::from(value.device_status),
            device_type: CoolingDeviceType::from(value.device_type),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoolingDeviceStatus {
    Other,
    Unknown,
    OK,
    NonCritical,
    Critical,
    NonRecoverable,
    None,
}

impl From<smbioslib::CoolingDeviceStatus> for CoolingDeviceStatus {
    fn from(value: smbioslib::CoolingDeviceStatus) -> Self {
        match value {
            smbioslib::CoolingDeviceStatus::Other => Self::Other,
            smbioslib::CoolingDeviceStatus::Unknown => Self::Unknown,
            smbioslib::CoolingDeviceStatus::OK => Self::OK,
            smbioslib::CoolingDeviceStatus::Critical => Self::Critical,
            smbioslib::CoolingDeviceStatus::NonCritical => Self::NonCritical,
            smbioslib::CoolingDeviceStatus::NonRecoverable => Self::NonRecoverable,
            smbioslib::CoolingDeviceStatus::None => Self::None,
        }
    }
}

impl Display for CoolingDeviceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::OK => "OK",
                Self::NonCritical => "Non-critical",
                Self::Critical => "Critical",
                Self::NonRecoverable => "Non-recoverable",
                Self::None => "None",
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoolingDeviceType {
    Other,
    Unknown,
    Fan,
    CentrifugalBlower,
    ChipFan,
    CabinetFan,
    PowerSupplyFan,
    HeatPipe,
    IntegratedRefrigeration,
    ActiveCooling,
    PassiveCooling,
    None,
}

impl From<smbioslib::CoolingDeviceType> for CoolingDeviceType {
    fn from(value: smbioslib::CoolingDeviceType) -> Self {
        match value {
            smbioslib::CoolingDeviceType::Other => Self::Other,
            smbioslib::CoolingDeviceType::Unknown => Self::Unknown,
            smbioslib::CoolingDeviceType::Fan => Self::Fan,
            smbioslib::CoolingDeviceType::CentrifugalBlower => Self::CentrifugalBlower,
            smbioslib::CoolingDeviceType::ChipFan => Self::ChipFan,
            smbioslib::CoolingDeviceType::CabinetFan => Self::CabinetFan,
            smbioslib::CoolingDeviceType::PowerSupplyFan => Self::PowerSupplyFan,
            smbioslib::CoolingDeviceType::HeatPipe => Self::HeatPipe,
            smbioslib::CoolingDeviceType::IntegratedRefrigeration => Self::IntegratedRefrigeration,
            smbioslib::CoolingDeviceType::ActiveCooling => Self::ActiveCooling,
            smbioslib::CoolingDeviceType::PassiveCooling => Self::PassiveCooling,
            smbioslib::CoolingDeviceType::None => Self::None,
        }
    }
}

impl Display for CoolingDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Other => "Other",
                Self::Unknown => "Unknown",
                Self::Fan => "FAN",
                Self::CentrifugalBlower => "Cetrifugal Blower",
                Self::ChipFan => "Chip FAN",
                Self::CabinetFan => "Cabinet FAN",
                Self::PowerSupplyFan => "Power Supply FAN",
                Self::HeatPipe => "Heat Pipe",
                Self::IntegratedRefrigeration => "Integrated Refrigeration",
                Self::ActiveCooling => "Active Cooling",
                Self::PassiveCooling => "Passive Cooling",
                Self::None => "None",
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum RotationalSpeed {
    Rpm(u16),
    Unknown,
}

impl From<smbioslib::RotationalSpeed> for RotationalSpeed {
    fn from(value: smbioslib::RotationalSpeed) -> Self {
        match value {
            smbioslib::RotationalSpeed::Rpm(rpm) => Self::Rpm(rpm),
            smbioslib::RotationalSpeed::Unknown => Self::Unknown,
        }
    }
}

impl Display for RotationalSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rpm(rpm) => write!(f, "{} RPM", rpm),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
