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

//! DMI table viewer page

use crate::{
    DataLoadingState, Message,
    dmi::DMIData,
    fl,
    widgets::table::{InfoRow, fmt_bool, fmt_val, fmt_vec, hdr_name, text_fmt_val},
};
use ferrix_lib::dmi::{
    Baseboard, Bios, Chassis, ChassisSecurityStatusData, ChassisStateData, Processor,
};
use ferrix_widgets::headers::header;
use iced::{
    Element, Length,
    widget::{Id, column, container, scrollable, table, text},
};

pub fn dmi_page<'a>(dmi: &'a DataLoadingState<DMIData>) -> container::Container<'a, Message> {
    match dmi {
        DataLoadingState::Loaded(data) => {
            let bios = bios_table(&data.bios);
            let baseboard = baseboard_table(&data.baseboard);
            let chassis = chassis_table(&data.chassis);
            let proc = processor_table(&data.processor);

            container(
                scrollable(column![bios, baseboard, chassis, proc,].spacing(5))
                    .spacing(5)
                    .id(Id::new(super::Page::DMI.page_id())),
            )
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn bios_table<'a>(bios: &'a DataLoadingState<Bios>) -> container::Container<'a, Message> {
    let bios_data = match bios {
        DataLoadingState::Loading => container(text(fl!("ldr-page-tooltip"))),
        DataLoadingState::Error(why) => container(text(why).style(text::danger)),
        DataLoadingState::Loaded(bios) => {
            let rows = vec![
                InfoRow::new("BIOS Vendor", bios.vendor.clone()),
                InfoRow::new("Version", bios.version.clone()),
                InfoRow::new(
                    "Starting address segment",
                    match bios.starting_address_segment {
                        Some(sas) => Some(format!("0x{sas:05X}")),
                        None => None,
                    },
                ),
                InfoRow::new("Release date", bios.release_date.clone()),
                InfoRow::new(
                    "ROM Size",
                    match &bios.rom_size {
                        Some(rs) => Some(rs.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "System BIOS Revision",
                    Some(format!(
                        "{}.{}",
                        bios.system_bios_major_release.unwrap_or(0),
                        bios.system_bios_minor_release.unwrap_or(0)
                    )),
                ),
                InfoRow::new(
                    "Embedded controller firmware Revision",
                    Some(format!(
                        "{}.{}",
                        bios.e_c_firmware_major_release.unwrap_or(0),
                        bios.e_c_firmware_minor_release.unwrap_or(0)
                    )),
                ),
                InfoRow::new(
                    "Extended BIOS ROM Size",
                    match &bios.extended_rom_size {
                        Some(ers) => Some(ers.to_string()),
                        None => None,
                    },
                ),
            ];
            container(
                column![
                    text("Summary").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                    bios_characteristics_table(bios),
                    bios_ext0_table(bios),
                    bios_ext1_table(bios),
                ]
                .spacing(5),
            )
        }
    };

    container(column![header("BIOS (Type 0)"), bios_data,].spacing(5))
}

fn bios_characteristics_table<'a>(bios: &'a Bios) -> container::Container<'a, Message> {
    match &bios.characteristics {
        None => container(text("BIOS Characteristics Table is empty!").style(text::danger)),
        Some(c) => {
            let rows = vec![
                InfoRow::new(
                    "BIOS Characteristics aren’t supported",
                    fmt_bool(Some(c.bios_characteristics_not_supported)),
                ),
                InfoRow::new("ISA is supported", fmt_bool(Some(c.isa_supported))),
                InfoRow::new("MCA is supported", fmt_bool(Some(c.mca_supported))),
                InfoRow::new("EISA is supported", fmt_bool(Some(c.eisa_supported))),
                InfoRow::new("PCI is supported", fmt_bool(Some(c.pci_supported))),
                InfoRow::new("PCMCIA is supported", fmt_bool(Some(c.pcmcia_supported))),
                InfoRow::new(
                    "Plug-n-Play is supported",
                    fmt_bool(Some(c.plug_and_play_supported)),
                ),
                InfoRow::new("APM is supported", fmt_bool(Some(c.apm_supported))),
                InfoRow::new(
                    "BIOS is upgadeable (flash)",
                    fmt_bool(Some(c.bios_upgradeable)),
                ),
                InfoRow::new(
                    "BIOS shadowing is allowed",
                    fmt_bool(Some(c.bios_shadowing_allowed)),
                ),
                InfoRow::new("VL-VESA is supported", fmt_bool(Some(c.vlvesa_supported))),
                InfoRow::new(
                    "ESCD support is available",
                    fmt_bool(Some(c.escd_support_available)),
                ),
                InfoRow::new(
                    "Boot from CD is supported",
                    fmt_bool(Some(c.boot_from_cdsupported)),
                ),
                InfoRow::new(
                    "Boot from PCMCIA is supported",
                    fmt_bool(Some(c.boot_from_pcmcia_supported)),
                ),
                InfoRow::new(
                    "BIOS ROM is socketed (e.g. PLCC/SOP socket)",
                    fmt_bool(Some(c.bios_rom_socketed)),
                ),
                InfoRow::new(
                    "EDD specification is supported",
                    fmt_bool(Some(c.edd_specification_supported)),
                ),
                InfoRow::new(
                    "Japanese floppy for NEX 9800 1.2 MB is supported",
                    fmt_bool(Some(c.floppy_nec_japanese_supported)),
                ),
                InfoRow::new(
                    "Japanese floppy for Toshiba 1.2 MB is supported",
                    fmt_bool(Some(c.floppy_toshiba_japanese_supported)),
                ),
                InfoRow::new(
                    "5.25\"/360 KB floppy services are supported",
                    fmt_bool(Some(c.floppy_525_360_supported)),
                ),
                InfoRow::new(
                    "5.25\"/1.2 MB floppy services are supported",
                    fmt_bool(Some(c.floppy_525_12_supported)),
                ),
                InfoRow::new(
                    "3.5\"/720 KB floppy services are supported",
                    fmt_bool(Some(c.floppy_35_720_supported)),
                ),
                InfoRow::new(
                    "3.5\"/2.88 MB floppy services are supported",
                    fmt_bool(Some(c.floppy_35_288_supported)),
                ),
                InfoRow::new(
                    "PrintScreen service are supported",
                    fmt_bool(Some(c.print_screen_service_supported)),
                ),
                InfoRow::new(
                    "8042 keyboard services are supported",
                    fmt_bool(Some(c.keyboard_8042services_supported)),
                ),
                InfoRow::new(
                    "Serial services are supported",
                    fmt_bool(Some(c.serial_services_supported)),
                ),
                InfoRow::new(
                    "Printer services are supported",
                    fmt_bool(Some(c.printer_services_supported)),
                ),
                InfoRow::new(
                    "CGA/Mono Video Services are supported",
                    fmt_bool(Some(c.cga_mono_video_services_supported)),
                ),
                InfoRow::new("NEC PC-98 supported", fmt_bool(Some(c.nec_pc_98supported))),
            ];
            container(
                column![
                    text("BIOS Characteristics").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

fn bios_ext0_table<'a>(b: &'a Bios) -> container::Container<'a, Message> {
    match &b.characteristics_extension0 {
        None => container(text("Characteristics extension byte 0 not found!").style(text::danger)),
        Some(b) => {
            let rows = vec![
                InfoRow::new("ACPI is supported", fmt_bool(Some(b.acpi_is_supported))),
                InfoRow::new(
                    "USB Legacy is supported",
                    fmt_bool(Some(b.usb_legacy_is_supported)),
                ),
                InfoRow::new("AGP is supported", fmt_bool(Some(b.agp_is_supported))),
                InfoRow::new(
                    "I20 boot is supported",
                    fmt_bool(Some(b.i2oboot_is_supported)),
                ),
                InfoRow::new(
                    "LS-120 SuperDisk boot is supported",
                    fmt_bool(Some(b.ls120super_disk_boot_is_supported)),
                ),
                InfoRow::new(
                    "ATAPI ZIP drive boot is supported",
                    fmt_bool(Some(b.atapi_zip_drive_boot_is_supported)),
                ),
                InfoRow::new(
                    "1394 boot is supported",
                    fmt_bool(Some(b.boot_1394is_supported)),
                ),
                InfoRow::new(
                    "Smart battery is supported",
                    fmt_bool(Some(b.smart_battery_is_supported)),
                ),
            ];
            container(
                column![
                    text("BIOS Characteristics Extension byte 0").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

fn bios_ext1_table<'a>(b: &'a Bios) -> container::Container<'a, Message> {
    match &b.characteristics_extension1 {
        None => container(text("Characteristics extension byte 1 not found!").style(text::danger)),
        Some(b) => {
            let rows = vec![
                InfoRow::new(
                    "BIOS Boot Specification is supported",
                    fmt_bool(Some(b.bios_boot_specification_is_supported)),
                ),
                InfoRow::new(
                    "Function key-initiated network service boot is supported",
                    fmt_bool(Some(b.fkey_initiated_network_boot_is_supported)),
                ),
                InfoRow::new(
                    "Targeted content distribution is supported",
                    fmt_bool(Some(b.targeted_content_distribution_is_supported)),
                ),
                InfoRow::new(
                    "UEFI Specification is supported",
                    fmt_bool(Some(b.uefi_specification_is_supported)),
                ),
                InfoRow::new(
                    "SMBIOS table describes a virtual machine",
                    fmt_bool(Some(b.smbios_table_describes_avirtual_machine)),
                ),
                InfoRow::new(
                    "Manufacturing mode is supported",
                    fmt_bool(Some(b.manufacturing_mode_is_supported)),
                ),
                InfoRow::new(
                    "Manufacturing mode is enabled",
                    fmt_bool(Some(b.manufacturing_mode_is_enabled)),
                ),
            ];
            container(
                column![
                    text("BIOS Characteristics Extension byte 1").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

fn baseboard_table<'a>(bb: &'a DataLoadingState<Baseboard>) -> container::Container<'a, Message> {
    let bb_data = match bb {
        DataLoadingState::Loading => container(text(fl!("ldr-page-tooltip"))),
        DataLoadingState::Error(why) => container(text(why).style(text::danger)),
        DataLoadingState::Loaded(bb) => {
            let rows = vec![
                InfoRow::new("Manufacturer", bb.manufacturer.clone()),
                InfoRow::new("Product", bb.product.clone()),
                InfoRow::new("Serial number", bb.serial_number.clone()),
                InfoRow::new("Asset tag", bb.asset_tag.clone()),
                InfoRow::new("Location in chassis", bb.location_in_chassis.clone()),
                InfoRow::new("Chassis handle", fmt_val(bb.chassis_handle)),
            ];

            let features = match &bb.feature_flags {
                Some(bf) => {
                    let rows = vec![
                        InfoRow::new("Hosting board", fmt_bool(Some(bf.hosting_board))),
                        InfoRow::new(
                            "Requires daughter board",
                            fmt_bool(Some(bf.requires_daughterboard)),
                        ),
                        InfoRow::new("Removable?", fmt_bool(Some(bf.is_removable))),
                        InfoRow::new("Replaceable?", fmt_bool(Some(bf.is_replaceable))),
                        InfoRow::new("Hot swappable?", fmt_bool(Some(bf.is_hot_swappable))),
                    ];

                    container(
                        column![
                            text("Baseboard features").style(text::warning),
                            container(kv_info_table(rows)).style(container::rounded_box),
                        ]
                        .spacing(5),
                    )
                }
                None => container(text("Baseboard features is empty!").style(text::danger)),
            };

            let btype = match &bb.board_type {
                Some(bt) => {
                    let rows = vec![
                        InfoRow::new("Raw value", Some(format!("{}", bt.raw))),
                        InfoRow::new("Type", Some(bt.value.to_string())),
                    ];

                    container(
                        column![
                            text("Baseboard type").style(text::warning),
                            container(kv_info_table(rows)).style(container::rounded_box),
                        ]
                        .spacing(5),
                    )
                }
                None => container(text("Unknown baseboard type!").style(text::danger)),
            };

            container(
                column![
                    text("Summary").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                    features,
                    btype,
                ]
                .spacing(5),
            )
        }
    };

    container(column![header("Base Board (Type 2)"), bb_data,].spacing(5))
}

fn chassis_table<'a>(c: &'a DataLoadingState<Chassis>) -> container::Container<'a, Message> {
    let c_data = match c {
        DataLoadingState::Loading => container(text(fl!("ldr-page-tooltip"))),
        DataLoadingState::Error(why) => container(text(why).style(text::danger)),
        DataLoadingState::Loaded(c) => {
            let rows = vec![
                InfoRow::new("Manufacturer", c.manufacturer.clone()),
                InfoRow::new("Version", c.version.clone()),
                InfoRow::new("Serial number", c.serial_number.clone()),
                InfoRow::new("Asset tag", c.asset_tag_number.clone()),
                InfoRow::new("OEM Defined", fmt_val(c.oem_defined)),
                InfoRow::new("Contained elements", fmt_val(c.contained_element_count)),
                InfoRow::new(
                    "Contained elements record length",
                    fmt_val(c.contained_element_record_length),
                ),
                InfoRow::new("SKU Number", c.sku_number.clone()),
                InfoRow::new("Bootup state", chassis_state(&c.bootup_state)),
                InfoRow::new("Power Supply state", chassis_state(&c.power_supply_state)),
                InfoRow::new("Thermal state", chassis_state(&c.thermal_state)),
                InfoRow::new("Security status", security_status(&c.security_status)),
            ];

            let chassis_type = match &c.chassis_type {
                Some(ct) => {
                    let rows = vec![
                        InfoRow::new("Raw", fmt_val(Some(ct.raw))),
                        InfoRow::new("Type", Some(ct.value.to_string())),
                        InfoRow::new("Lock presence", Some(ct.lock_presence.to_string())),
                    ];
                    container(
                        column![
                            text("Chassis type").style(text::warning),
                            container(kv_info_table(rows)).style(container::rounded_box)
                        ]
                        .spacing(5),
                    )
                }
                None => container(text("Unknown chassis type").style(text::danger)),
            };

            container(
                column![
                    text("Summary").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                    chassis_type,
                ]
                .spacing(5),
            )
        }
    };

    let chassis_view = column![header("Chassis (Type 3)"), c_data,].spacing(5);

    container(chassis_view)
}

fn chassis_state<'a>(state: &'a Option<ChassisStateData>) -> Option<String> {
    state
        .as_ref()
        .and_then(|state| Some(format!("{} (raw: {})", &state.value, state.raw)))
}

fn security_status<'a>(status: &'a Option<ChassisSecurityStatusData>) -> Option<String> {
    status
        .as_ref()
        .and_then(|state| Some(format!("{} (raw: {})", &state.value, state.raw)))
}

fn processor_table<'a>(p: &'a DataLoadingState<Processor>) -> container::Container<'a, Message> {
    let p_data = match p {
        DataLoadingState::Loading => container(text(fl!("ldr-page-tooltip"))),
        DataLoadingState::Error(why) => container(text(why).style(text::danger)),
        DataLoadingState::Loaded(p) => {
            let rows = vec![
                InfoRow::new(
                    "Raw Processor ID",
                    match p.processor_id {
                        Some(pid) => fmt_vec(&Some(pid.to_vec())),
                        None => None,
                    },
                ),
                InfoRow::new("Socket reference designation", p.socked_designation.clone()),
                InfoRow::new(
                    "Processor type",
                    match &p.processor_type {
                        Some(pt) => Some(pt.value.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Processor family",
                    match &p.processor_family {
                        Some(pf) => Some(pf.value.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Processor family #2",
                    match &p.processor_family_2 {
                        Some(pf) => Some(pf.value.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new("Processor manufacturer", p.processor_manufacturer.clone()),
                InfoRow::new("Processor version", p.processor_version.clone()),
                InfoRow::new(
                    "External clock",
                    match &p.external_clock {
                        Some(ec) => Some(ec.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Max speed",
                    match &p.max_speed {
                        Some(ms) => Some(ms.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Current speed",
                    match &p.current_speed {
                        Some(cs) => Some(cs.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Socket populated",
                    match &p.status {
                        Some(ps) => fmt_bool(Some(ps.socket_populated)),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "CPU Status",
                    match &p.status {
                        Some(ps) => Some(ps.cpu_status.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Processor Upgrade",
                    match &p.processor_upgrade {
                        Some(pu) => Some(pu.value.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new("Serial number", p.serial_number.clone()),
                InfoRow::new("Asset tag", p.asset_tag.clone()),
                InfoRow::new("Part number", p.part_number.clone()),
                InfoRow::new(
                    "Core count",
                    match &p.core_count {
                        Some(cc) => Some(cc.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Cores enabled",
                    match &p.cores_enabled {
                        Some(ce) => Some(ce.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Thread count",
                    match &p.thread_count {
                        Some(tc) => Some(tc.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Core count #2",
                    match &p.core_count_2 {
                        Some(cc) => Some(cc.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Cores enabled #2",
                    match &p.cores_enabled_2 {
                        Some(ce) => Some(ce.to_string()),
                        None => None,
                    },
                ),
                InfoRow::new(
                    "Thread count #2",
                    match &p.thread_count_2 {
                        Some(tc) => Some(tc.to_string()),
                        None => None,
                    },
                ),
            ];
            container(
                column![
                    text("Summary").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                    processor_characteristics_table(p),
                    processor_voltage_table(p),
                ]
                .spacing(5),
            )
        }
    };

    container(column![header("Processor (Type 4)"), p_data,].spacing(5))
}

fn processor_voltage_table<'a>(p: &'a Processor) -> container::Container<'a, Message> {
    let voltage = &p.voltage;
    match voltage {
        None => container(text("Unknown processor voltage!").style(text::danger)),
        Some(v) => {
            let mut rows = vec![];
            match v {
                ferrix_lib::dmi::ProcessorVoltage::CurrentVolts(volts) => {
                    rows.push(InfoRow::new("Current voltage", fmt_val(Some(volts))))
                }
                ferrix_lib::dmi::ProcessorVoltage::SupportedVolts(volts) => {
                    rows.push(InfoRow::new(
                        "5.0V Supported",
                        fmt_bool(Some(volts.volts_5_0)),
                    ));
                    rows.push(InfoRow::new(
                        "3.3V Supported",
                        fmt_bool(Some(volts.volts_3_3)),
                    ));
                    rows.push(InfoRow::new(
                        "2.9V Supported",
                        fmt_bool(Some(volts.volts_2_9)),
                    ));
                    rows.push(InfoRow::new(
                        "Other supported voltages",
                        fmt_vec(&Some(volts.voltages.clone())),
                    ));
                }
            }

            container(
                column![
                    text("Processor voltage").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

fn processor_characteristics_table<'a>(p: &'a Processor) -> container::Container<'a, Message> {
    let chars = &p.processors_characteristics;
    match chars {
        None => container(text("Processor characteristics is not present!").style(text::danger)),
        Some(c) => {
            let rows = vec![
                InfoRow::new("64-bit capable", fmt_bool(Some(c.bit_64capable))),
                InfoRow::new("128-bit capable", fmt_bool(Some(c.bit_128capable))),
                InfoRow::new("Multi core", fmt_bool(Some(c.multi_core))),
                InfoRow::new("Hardware thread", fmt_bool(Some(c.hardware_thread))),
                InfoRow::new("Execute protection", fmt_bool(Some(c.execute_protection))),
                InfoRow::new(
                    "Enhanced Virtualization",
                    fmt_bool(Some(c.enhanced_virtualization)),
                ),
                InfoRow::new(
                    "Power/performance control",
                    fmt_bool(Some(c.power_perfomance_control)),
                ),
                InfoRow::new("ARM64 SoC ID", fmt_bool(Some(c.arm_64soc_id))),
            ];

            container(
                column![
                    text("Processor characteristics").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                ]
                .spacing(5),
            )
        }
    }
}

/*******************************************************
 *******************************************************/

fn kv_info_table<'a, V>(rows: Vec<InfoRow<V>>) -> Element<'a, Message>
where
    V: ToString + Clone + 'a,
{
    let columns = [
        table::column(hdr_name(fl!("hdr-param")), |row: InfoRow<V>| {
            text(row.param_header).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(2)),
        table::column(hdr_name(fl!("hdr-value")), |row: InfoRow<V>| {
            text_fmt_val(row.value)
        })
        .width(Length::FillPortion(5)),
    ];

    table(columns, rows).padding(2).width(Length::Fill).into()
}
