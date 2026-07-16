/* dmi.rs
 *
 * Copyright 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>
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

use crate::{
    fl,
    message::{DataReceiver, Message, PageMessage},
    widgets::table::{InfoRow, fmt_bool, fmt_val, fmt_vec, hdr_name, text_fmt_val},
};
use ferrix_data::{dmi::DMIData, load_state::LoadState};
use ferrix_lib::dmi::{
    Baseboard, Bios, Chassis, ChassisSecurityStatusData, ChassisStateData, MemoryDevice,
    MemoryDevices, MemoryOperatingModeCapabilities, MemoryTypeDetails, Processor, System,
};
use ferrix_widgets::{headers::header, separated_view::SeparatedView};
use iced::{
    Element, Length, Task,
    widget::{Column, button, column, container, row, scrollable, table, text},
};

use super::{PageData, PageView};

#[derive(Debug, Clone)]
pub struct DMIPage {
    pub dmi: LoadState<DMIData>,
    pub is_polkit: bool,
    pub selected_table: SelectedTable,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum SelectedTable {
    #[default]
    Bios,
    System,
    Baseboard,
    Chassis,
    Processor,
    MemoryController,
    MemoryModules,
    CPUCache,
    PortConnectors,
    PhysicalMemoryArray,
    InstalledMemoryDevices,
}

impl SelectedTable {
    pub fn view<'a>(&'a self, dmi: &'a DMIData) -> Element<'a, Message> {
        match self {
            Self::Bios => bios_table(&dmi.bios),
            Self::System => system_table(&dmi.system),
            Self::Baseboard => baseboard_table(&dmi.baseboard),
            Self::Chassis => chassis_table(&dmi.chassis),
            Self::Processor => processor_table(&dmi.processor),
            Self::InstalledMemoryDevices => memory_devices_table(&dmi.memory_devices),
            _ => super::todo(),
        }
    }
}

impl DMIPage {
    pub fn new() -> Self {
        Self {
            dmi: LoadState::Loading,
            is_polkit: false,
            selected_table: SelectedTable::default(),
        }
    }

    fn get_pages_list<'a>(&'a self) -> Vec<Element<'a, Message>> {
        let pages_items = [
            ("[Type  0] BIOS", SelectedTable::Bios),
            ("[Type  1] System", SelectedTable::System),
            ("[Type  2] Baseboard", SelectedTable::Baseboard),
            ("[Type  3] Chassis", SelectedTable::Chassis),
            ("[Type  4] Processor", SelectedTable::Processor),
            // (
            //     "[Type  5] Memory Controller",
            //     SelectedTable::MemoryController,
            // ),
            // ("[Type  6] Memory Modules", SelectedTable::MemoryModules),
            // ("[Type  7] CPU Cache", SelectedTable::CPUCache),
            // ("[Type  8] Port Connectors", SelectedTable::PortConnectors),
            // (
            //     "[Type 16] Physical Memory Array",
            //     SelectedTable::PhysicalMemoryArray,
            // ),
            (
                "[Type 17] Installed Memory Devices",
                SelectedTable::InstalledMemoryDevices,
            ),
        ];
        let mut pages = Vec::with_capacity(pages_items.len());

        for page in pages_items {
            let b = button(page.0)
                .on_press(Message::PageMessage(PageMessage::DMIPage(
                    DMIPageMessage::TableSelected(page.1),
                )))
                .style(if page.1 == self.selected_table {
                    button::subtle
                } else {
                    button::text
                })
                .height(Length::Fill)
                .padding(2)
                .into();
            pages.push(b);
        }
        pages
    }

    fn table_view<'a>(&'a self, dmi: &'a DMIData) -> Element<'a, Message> {
        scrollable(self.selected_table.view(dmi))
            .spacing(5)
            .id(Self::page_id())
            .into()
    }
}

impl<'a> PageView<'a> for DMIPage {
    fn page_id() -> &'static str {
        "dmi"
    }

    fn scrolled_page_id() -> Option<&'static str> {
        Some("tables")
    }

    fn page_title() -> String {
        fl!("page-dmi")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_title_controls(&'a self) -> Option<Element<'a, Message>> {
        let update_button = button(text(fl!("err-page-update")))
            .on_press(Message::DataReceiver(DataReceiver::DMIDataRefresh))
            .padding(3);
        let export_button =
            button(text(fl!("sidebar-export")))
                .padding(3)
                .on_press(Message::PageMessage(PageMessage::ExportSingle(
                    crate::pages::PageVariant::DMITables,
                )));

        Some(row![update_button, export_button].spacing(5).into())
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.dmi {
            LoadState::Loaded(dmi) => {
                let first_panel = container(Column::from_vec(self.get_pages_list()))
                    .style(container::rounded_box)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(2);
                let second_panel = self.table_view(dmi);

                let view = SeparatedView::new(first_panel, second_panel)
                    .set_fpane_id(Self::scrolled_page_id().unwrap_or(""))
                    .set_spane_id(Self::page_id())
                    .set_fpane_max_height(Length::Fixed(120.))
                    .set_spane_max_height(Length::Fill);
                view.view().into()
            }
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::DMIDataRefresh),
        }
    }
}

impl PageData for DMIPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { ferrix_data::dmi::get_dmi_data().await },
            |val| DataReceiver::DMIDataReceived(val),
        )
    }
}

fn bios_table<'a>(bios: &'a LoadState<Bios>) -> Element<'a, Message> {
    let bios_data = match bios {
        LoadState::Loading => container(text(fl!("ldr-page-tooltip")).style(text::warning)).into(),
        LoadState::Error(why) => container(text(why).style(text::danger)).into(),
        LoadState::Loaded(bios) => {
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
                    text("General").style(text::warning),
                    container(kv_info_table(rows)).style(container::rounded_box),
                    bios_characteristics_table(bios),
                    bios_ext0_table(bios),
                    bios_ext1_table(bios),
                ]
                .spacing(5),
            )
        }
    };

    bios_data.into()
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

fn system_table<'a>(sys: &'a LoadState<System>) -> Element<'a, Message> {
    let sys_data = match sys {
        LoadState::Loading => container(text(fl!("ldr-page-tooltip")).style(text::warning)),
        LoadState::Error(why) => container(text(why).style(text::danger)),
        LoadState::Loaded(sys) => {
            let rows = vec![
                InfoRow::new("Manufacturer", sys.manufacturer.clone()),
                InfoRow::new("Product name", sys.product_name.clone()),
                InfoRow::new("System version", sys.version.clone()),
                InfoRow::new("Serial number", sys.serial_number.clone()),
                InfoRow::new("System UUID", sys.uuid.clone().map(|uuid| uuid.to_string())),
                InfoRow::new(
                    "Wake-up type",
                    sys.wakeup_type
                        .clone()
                        .map(|wt| format!("{} (raw: {})", wt.value, wt.raw)),
                ),
                InfoRow::new("SKU Number", sys.sku_number.clone()),
                InfoRow::new("Family", sys.family.clone()),
            ];
            container(kv_info_table(rows)).style(container::rounded_box)
        }
    };
    sys_data.into()
}

fn baseboard_table<'a>(bb: &'a LoadState<Baseboard>) -> Element<'a, Message> {
    let bb_data = match bb {
        LoadState::Loading => container(text(fl!("ldr-page-tooltip")).style(text::warning)),
        LoadState::Error(why) => container(text(why).style(text::danger)),
        LoadState::Loaded(bb) => {
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

    bb_data.into()
}

fn chassis_table<'a>(c: &'a LoadState<Chassis>) -> Element<'a, Message> {
    let c_data = match c {
        LoadState::Loading => container(text(fl!("ldr-page-tooltip"))),
        LoadState::Error(why) => container(text(why).style(text::danger)),
        LoadState::Loaded(c) => {
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

    // let chassis_view = column![header("Chassis (Type 3)"), c_data,].spacing(5);
    c_data.into()
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

fn processor_table<'a>(p: &'a LoadState<Processor>) -> Element<'a, Message> {
    let p_data = match p {
        LoadState::Loading => container(text(fl!("ldr-page-tooltip"))),
        LoadState::Error(why) => container(text(why).style(text::danger)),
        LoadState::Loaded(p) => {
            let rows = vec![
                InfoRow::new(
                    "Raw Processor ID",
                    match p.processor_id {
                        Some(pid) => {
                            let val = pid
                                .iter()
                                .map(|val| format!("{val:02X}"))
                                .collect::<Vec<_>>();
                            fmt_vec(&Some(val))
                        }
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

    // container(column![header("Processor (Type 4)"), p_data,].spacing(5))
    p_data.into()
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

fn memory_devices_table<'a>(m: &'a LoadState<MemoryDevices>) -> Element<'a, Message> {
    match m {
        LoadState::Loading => container(text(fl!("ldr-page-tooltip")).style(text::warning)).into(),
        LoadState::Error(why) => container(text(why).style(text::danger)).into(),
        LoadState::Loaded(m) => {
            let mut devices = Column::with_capacity(m.memory.len()).spacing(5);
            let mut i = 0;
            for device in &m.memory {
                devices = devices.push(
                    column![header(format!("Device #{i}")), memory_device_table(device),]
                        .spacing(5),
                );
                i += 1;
            }
            devices.into()
        }
    }
}

fn memory_device_table<'a>(m: &'a MemoryDevice) -> Element<'a, Message> {
    let rows = vec![
        InfoRow::new("Manufacturer", m.manufacturer.clone()),
        InfoRow::new("Serial number", m.serial_number.clone()),
        InfoRow::new("Asset tag", m.asset_tag.clone()),
        InfoRow::new("Part number", m.part_number.clone()),
        InfoRow::new("Size", m.size.clone().map(|s| s.to_string())),
        InfoRow::new(
            "Extended size",
            m.extended_size.clone().map(|es| es.to_string()),
        ),
        InfoRow::new(
            "Memory speed",
            m.speed.clone().map(|speed| speed.to_string()),
        ),
        InfoRow::new(
            "Configured speed",
            m.configured_memory_speed.clone().map(|cms| cms.to_string()),
        ),
        InfoRow::new(
            "Extended speed",
            m.extended_speed.clone().map(|es| es.to_string()),
        ),
        InfoRow::new(
            "Extended configured speed",
            m.extended_configured_speed
                .clone()
                .map(|ecs| ecs.to_string()),
        ),
        InfoRow::new(
            "Form factor",
            m.form_factor
                .clone()
                .map(|ff| format!("{} (raw: {})", ff.value, ff.raw)),
        ),
        InfoRow::new(
            "Memory type",
            m.memory_type
                .clone()
                .map(|mt| format!("{} (raw: {})", mt.value, mt.raw)),
        ),
        InfoRow::new(
            "Memory technology",
            m.memory_technology
                .clone()
                .map(|mt| format!("{} (raw: {})", mt.value, mt.raw)),
        ),
        InfoRow::new(
            "Voltage, min",
            m.minimum_voltage.map(|mv| format!("{mv} mV")),
        ),
        InfoRow::new(
            "Voltage, max",
            m.maximum_voltage.map(|mv| format!("{mv} mV")),
        ),
        InfoRow::new(
            "Voltage, configured",
            m.configured_voltage.map(|cv| format!("{cv} mV")),
        ),
        InfoRow::new(
            "Non-volatile portion size",
            m.non_volatile_size.clone().map(|nvs| nvs.to_string()),
        ),
        InfoRow::new(
            "Volatile portion size",
            m.volatile_size.clone().map(|vs| vs.to_string()),
        ),
        InfoRow::new(
            "Cache portion size",
            m.cache_size.clone().map(|cs| cs.to_string()),
        ),
        InfoRow::new(
            "Logical size",
            m.logical_size.clone().map(|ls| ls.to_string()),
        ),
        InfoRow::new("Total width", m.total_width.map(|tw| format!("{tw} bits"))),
        InfoRow::new("Data width", m.data_width.map(|dw| format!("{dw} bits"))),
        InfoRow::new("Device set", fmt_val(m.device_set)),
        InfoRow::new(
            "Memory device socket/board position",
            m.device_locator.clone(),
        ),
        InfoRow::new("Bank location", m.bank_locator.clone()),
        InfoRow::new("Attributes", fmt_val(m.attributes)),
        // TODO: memory operating mode
        InfoRow::new("Firmware version", m.firmware_version.clone()),
        InfoRow::new("Module manufacturer ID", fmt_val(m.module_manufacturer_id)),
        InfoRow::new("Module product ID", fmt_val(m.module_product_id)),
        InfoRow::new(
            "Memory subsystem controller manufacturer ID",
            fmt_val(m.memory_subsystem_controller_manufacturer_id),
        ),
        InfoRow::new(
            "Memory subsystem controller product ID",
            fmt_val(m.memory_subsystem_controller_product_id),
        ),
        InfoRow::new("PMIC0 manufacturer ID", fmt_val(m.pmic0_manufacturer_id)),
        InfoRow::new("PMIC0 revision number", fmt_val(m.pmic0_revision_number)),
        InfoRow::new("RCD manufacturer ID", fmt_val(m.rcd_manufacturer_id)),
        InfoRow::new("RCD0 revision number", fmt_val(m.rcd_revision_number)),
        InfoRow::new(
            "Physical memory array handle",
            fmt_val(m.physical_memory_array_handle.clone()),
        ),
        InfoRow::new(
            "Memory error information handle",
            fmt_val(m.memory_error_information_handle.clone()),
        ),
    ];
    let main_table = container(kv_info_table(rows)).style(container::rounded_box);

    column![
        main_table,
        memory_device_type_details_table(&m.type_detail),
        memory_device_operating_mode(&m.memory_operating_mode_capability),
    ]
    .spacing(5)
    .into()
}

fn memory_device_type_details_table<'a>(td: &'a Option<MemoryTypeDetails>) -> Element<'a, Message> {
    match td {
        Some(td) => {
            let rows = vec![
                InfoRow::new("Other", fmt_bool(Some(td.other))),
                InfoRow::new("Unknown", fmt_bool(Some(td.unknown))),
                InfoRow::new("Fast paged", fmt_bool(Some(td.fast_paged))),
                InfoRow::new("Static column", fmt_bool(Some(td.static_column))),
                InfoRow::new("Pseudo-static", fmt_bool(Some(td.pseudo_static))),
                InfoRow::new("RAMBUS", fmt_bool(Some(td.ram_bus))),
                InfoRow::new("Synchronous", fmt_bool(Some(td.synchronous))),
                InfoRow::new("CMOS", fmt_bool(Some(td.cmos))),
                InfoRow::new("EDO", fmt_bool(Some(td.edo))),
                InfoRow::new("Window DRAM", fmt_bool(Some(td.window_dram))),
                InfoRow::new("Cache DRAM", fmt_bool(Some(td.cache_dram))),
                InfoRow::new("Non-volatile", fmt_bool(Some(td.non_volatile))),
                InfoRow::new("Registered (buffered)", fmt_bool(Some(td.registered))),
                InfoRow::new("Unbuffered (unregistered)", fmt_bool(Some(td.unbuffered))),
                InfoRow::new("LRDIMM", fmt_bool(Some(td.lrdimm))),
                InfoRow::new("Raw value", Some(format!("{:0b}", td.raw))),
            ];
            column![
                text("Type details").style(text::warning),
                container(kv_info_table(rows)).style(container::rounded_box),
            ]
            .spacing(5)
            .into()
        }
        None => text("Type details is not present").into(),
    }
}

fn memory_device_operating_mode<'a>(
    mo: &'a Option<MemoryOperatingModeCapabilities>,
) -> Element<'a, Message> {
    match mo {
        Some(mo) => {
            let rows = vec![
                InfoRow::new("Other", fmt_bool(Some(mo.other))),
                InfoRow::new("Unknown", fmt_bool(Some(mo.unknown))),
                InfoRow::new("Volatile memory", fmt_bool(Some(mo.volatile_memory))),
                InfoRow::new(
                    "Byte-accessible persistent memory",
                    fmt_bool(Some(mo.byte_accessible_persistent_memory)),
                ),
                InfoRow::new(
                    "Block-accessible persistent memory",
                    fmt_bool(Some(mo.block_accessible_persistent_memory)),
                ),
                InfoRow::new("Raw value", Some(format!("{:0b}", mo.raw))),
            ];
            column![
                text("Memory device operating mode capability").style(text::warning),
                container(kv_info_table(rows)).style(container::rounded_box),
            ]
            .spacing(5)
            .into()
        }
        None => text("Memory operating mode info is not present").into(),
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
            text(row.header).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(2)),
        table::column(hdr_name(fl!("hdr-value")), |row: InfoRow<V>| {
            text_fmt_val(row.value)
        })
        .width(Length::FillPortion(5)),
    ];

    table(columns, rows).padding(2).width(Length::Fill).into()
}

#[derive(Debug, Clone)]
pub enum DMIPageMessage {
    TableSelected(SelectedTable),
}

impl DMIPageMessage {
    pub fn update<'a>(self, page: &'a mut DMIPage) -> Task<Message> {
        match self {
            Self::TableSelected(table) => {
                page.selected_table = table;
                Task::none()
            }
        }
    }
}
