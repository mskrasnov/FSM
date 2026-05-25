/* messages.rs
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

//! UI events handler & Data Updater

use crate::{
    DataLoadingState, Page, SETTINGS_PATH,
    export::{ExportData, ExportFormat, ExportMode, ExportStatus},
    ferrix::{ExportManager, Ferrix, FerrixState},
    load_state::LoadState,
    settings::{ChartLineThickness, FXSettings, Style},
    styles::CPU_CHARTS_COLORS,
    utils::{ToColor, get_home},
};
use ferrix_data::{FerrixData, System, dmi::DMIData, kmods::KResult};
use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    cpu_freq::CpuFreq,
    drm::Video,
    init::{BootTimestamps, Connection, SystemdServices},
    net::Networks,
    parts::Mounts,
    ram::{RAM, Swaps},
    soft::InstalledPackages,
    sys::{Groups, Kernel, OsRelease, Users},
    traits::ToJson,
    vulnerabilities::Vulnerabilities,
};
use ferrix_widgets::line_charts::LineSeries;
use iced::{
    Event, Task, color,
    keyboard::{Event as Kevent, Key, Modifiers, key},
    widget::{
        Id,
        operation::{self, AbsoluteOffset, RelativeOffset},
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    DataReceiver(DataReceiverMessage),
    ExportManager(ExportManagerMessage),
    Settings(SettingsMessage),
    Buttons(ButtonsMessage),

    SelectPage(Page),
    Keyboard(KeyboardMessage),
    Dummy,
}

impl Message {
    pub fn update<'a>(self, state: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::DataReceiver(data) => data.update(
                &mut state.data,
                &mut state.state,
                &mut state.settings,
                &mut state.export_manager,
                state.current_page,
            ),
            Self::ExportManager(export) => export.update(state),
            Self::Settings(settings) => settings.update(state),
            Self::Buttons(buttons) => buttons.update(state),

            Self::SelectPage(page) => state.select_page(page),
            Self::Keyboard(keyboard) => keyboard.update(state),
            Self::Dummy => Task::none(),
        }
    }
}

impl Ferrix {
    fn select_page(&mut self, page: Page) -> Task<Message> {
        self.current_page = page;
        self.scrolled_area_id = page.scrolled_list_id();

        if page == Page::Export {
            self.export_manager.status = ExportStatus::LoadingData;
        }
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum DataReceiverMessage {
    ClearAllData, // NOTE: run this BEFORE export!!!

    GetCPUData,
    CPUDataReceived(DataLoadingState<Processors>),

    AddCPUCoreLineSeries,
    ChangeShowCPUChartElements(usize),

    GetProcStat,
    ProcStatReceived(DataLoadingState<Stat>),

    GetCPUFrequency,
    CPUFrequencyReceived(DataLoadingState<CpuFreq>),

    GetCPUVulnerabilities,
    CPUVulnerabilitiesReveived(DataLoadingState<Vulnerabilities>),

    GetRAMData,
    RAMDataReceived(DataLoadingState<RAM>),

    GetSwapData,
    SwapDataReceived(DataLoadingState<Swaps>),

    AddTotalRAMUsage,

    GetStorageData,
    StorageDataReceived(DataLoadingState<Mounts>),

    GetNetworksData,
    NetworksDataReceived(DataLoadingState<Networks>),

    GetDMIData,
    DMIDataReceived(DataLoadingState<DMIData>),

    GetBatInfo,
    BatInfoReceived(DataLoadingState<BatInfo>),

    GetDRMData,
    DRMDataReceived(DataLoadingState<Video>),

    GetOsReleaseData,
    OsReleaseDataReceived(DataLoadingState<OsRelease>),

    GetKernelData,
    KernelDataReceived(DataLoadingState<Kernel>),

    GetKModsData,
    KModsDataReceived(DataLoadingState<KResult>),

    GetUsersData,
    UsersDataReceived(DataLoadingState<Users>),

    GetGroupsData,
    GroupsDataReceived(DataLoadingState<Groups>),

    GetSystemdServices,
    SystemdServicesReceived(
        (
            DataLoadingState<SystemdServices>,
            DataLoadingState<BootTimestamps>,
        ),
    ),

    GetPackagesList,
    PackagesListReceived(DataLoadingState<InstalledPackages>),

    GetSystemData,
    SystemDataReceived(DataLoadingState<System>),
}

impl DataReceiverMessage {
    pub fn update<'a>(
        self,
        fd: &'a mut FerrixData,
        fs: &'a mut FerrixState,
        settings: &'a mut FXSettings,
        export: &'a mut ExportManager,
        cur_page: Page,
    ) -> Task<Message> {
        match self {
            Self::ClearAllData => {
                export.status = ExportStatus::LoadingData;

                fs.is_dmi_polkit = false;
                fd.proc_data = LoadState::Loading;
                fd.cpu_freq = LoadState::Loading;
                fd.cpu_vulnerabilities = LoadState::Loading;
                fd.ram_data = LoadState::Loading;
                fd.swap_data = LoadState::Loading;
                fd.storages = LoadState::Loading;
                fd.networks = LoadState::Loading;
                fd.dmi_data = LoadState::Loading;
                fd.bat_data = LoadState::Loading;
                fd.drm_data = LoadState::Loading;
                fd.osrel_data = LoadState::Loading;
                fd.kernel_data = LoadState::Loading;
                fd.kmods_data = LoadState::Loading;
                fd.users_list = LoadState::Loading;
                fd.groups_list = LoadState::Loading;
                fd.sysd_services_list = LoadState::Loading;
                fd.boot_time = LoadState::Loading;
                fd.installed_pkgs_list = LoadState::Loading;
                fd.system = LoadState::Loading;

                Task::none()
            }
            Self::CPUDataReceived(state) => {
                fd.proc_data = state;
                Task::none()
            }
            Self::GetCPUData => crate::pages::cpu::ProcPage::get_data().map(Message::DataReceiver),
            Self::ProcStatReceived(state) => {
                if fd.curr_proc_stat.is_some() {
                    fd.prev_proc_stat = fd.curr_proc_stat.clone();
                } else if fd.curr_proc_stat.is_none() && fd.prev_proc_stat.is_none() {
                    fd.prev_proc_stat = state.clone();
                }
                fd.curr_proc_stat = state;
                Task::none()
            }
            Self::GetProcStat => crate::pages::SysmonPage::get_data().map(Message::DataReceiver),
            Self::AddCPUCoreLineSeries => {
                let curr_proc = &fd.curr_proc_stat;
                let prev_proc = &fd.prev_proc_stat;

                if curr_proc.is_none() || prev_proc.is_none() {
                    return Task::none();
                }
                let curr_proc = curr_proc.to_option().unwrap();
                let prev_proc = prev_proc.to_option().unwrap();

                if curr_proc.cpus.len() != prev_proc.cpus.len() {
                    return Task::none();
                }
                let len = curr_proc.cpus.len();

                let colors_set = &settings.chart_colors.colors;
                let def_colors = CPU_CHARTS_COLORS;

                for id in 0..len {
                    let percent =
                        curr_proc.cpus[id].usage_percentage(Some(prev_proc.cpus[id])) as f64;

                    if fs.show_cpus_chart.get(&id).is_none() {
                        let name = format!("CPU #{id}");
                        let color = {
                            let hm_color = colors_set.get(&name);
                            match hm_color {
                                Some(col) => col.to_color(),
                                None => {
                                    if def_colors.len() - 1 < id {
                                        color!(255, 255, 255)
                                    } else {
                                        def_colors[id]
                                    }
                                }
                            }
                        };
                        let mut line = LineSeries::new(name, color, fs.show_chart_elements);
                        line.push(percent);

                        fs.cpu_usage_chart.push_series(line);
                        fs.show_cpus_chart.insert(id);
                    } else {
                        fs.cpu_usage_chart.push_to(id, percent);
                    }
                }

                Task::none()
            }
            Self::ChangeShowCPUChartElements(elems) => {
                fs.show_chart_elements = elems;

                fs.cpu_usage_chart.set_max_values(elems);
                fs.ram_usage_chart.set_max_values(elems);

                Task::none()
            }
            Self::CPUFrequencyReceived(state) => {
                fd.cpu_freq = state;
                Task::none()
            }
            Self::GetCPUFrequency => {
                crate::pages::cpu_freq::ProcFreqPage::get_data().map(Message::DataReceiver)
            }
            Self::CPUVulnerabilitiesReveived(state) => {
                fd.cpu_vulnerabilities = state;
                Task::none()
            }
            Self::GetCPUVulnerabilities => {
                crate::pages::vulnerabilities::VulnPage::get_data().map(Message::DataReceiver)
            }
            Self::StorageDataReceived(state) => {
                fd.storages = state;
                Task::none()
            }
            Self::GetStorageData => Task::perform(
                async move {
                    let storage = Mounts::new();
                    match storage {
                        Ok(storage) => DataLoadingState::Loaded(storage),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(DataReceiverMessage::StorageDataReceived(val)),
            ),
            Self::NetworksDataReceived(state) => {
                fd.networks = state;
                Task::none()
            }
            Self::GetNetworksData => Task::perform(
                async move {
                    let net = Networks::new();
                    match net {
                        Ok(net) => DataLoadingState::Loaded(net),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(DataReceiverMessage::NetworksDataReceived(val)),
            ),
            Self::DMIDataReceived(state) => {
                if state.some_value() && fs.is_dmi_polkit {
                    fd.dmi_data = state;
                } else if !fs.is_dmi_polkit {
                    fd.dmi_data = state;
                }
                Task::none()
            }
            Self::GetDMIData => {
                if !fs.is_dmi_polkit
                    && ((fd.dmi_data.is_none() && cur_page == Page::DMI)
                        || export.selected_pages.dmi)
                {
                    fs.is_dmi_polkit = true;
                    Task::perform(
                        async move { ferrix_data::dmi::get_dmi_data().await },
                        |val| Message::DataReceiver(Self::DMIDataReceived(val)),
                    )
                } else {
                    Task::none()
                }
            }
            Self::BatInfoReceived(state) => {
                fd.bat_data = state;
                Task::none()
            }
            Self::GetBatInfo => Task::perform(
                async move {
                    let bat = BatInfo::new();
                    match bat {
                        Ok(bat) => DataLoadingState::Loaded(bat),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::BatInfoReceived(val)),
            ),
            Self::DRMDataReceived(state) => {
                fd.drm_data = state;
                Task::none()
            }
            Self::GetDRMData => Task::perform(
                async move {
                    let drm = Video::new();
                    match drm {
                        Ok(drm) => DataLoadingState::Loaded(drm),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::DRMDataReceived(val)),
            ),
            Self::RAMDataReceived(state) => {
                fd.ram_data = state;
                Task::none()
            }
            Self::GetRAMData => Task::perform(
                async move {
                    let ram = RAM::new();
                    match ram {
                        Ok(ram) => DataLoadingState::Loaded(ram),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::RAMDataReceived(val)),
            ),
            Self::SwapDataReceived(state) => {
                fd.swap_data = state;
                Task::none()
            }
            Self::GetSwapData => Task::perform(
                async move {
                    let swap = Swaps::new();
                    match swap {
                        Ok(swap) => DataLoadingState::Loaded(swap),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::SwapDataReceived(val)),
            ),
            Self::AddTotalRAMUsage => {
                let ram = &fd.ram_data;
                let swap = &fd.swap_data;

                if ram.is_none() {
                    return Task::none();
                }
                let ram = ram.to_option().unwrap();
                let ram_usage = ram.usage_percentage().unwrap_or(0.);

                let colors_set = &settings.chart_colors.colors;
                let def_colors = CPU_CHARTS_COLORS;
                let ram_color = match colors_set.get("RAM") {
                    Some(col) => col.to_color(),
                    None => color!(128, 64, 255),
                };

                if fs.ram_usage_chart.series_count() == 0 {
                    let mut ram_line =
                        LineSeries::new(format!("RAM"), ram_color, fs.show_chart_elements);
                    ram_line.push(ram_usage);
                    fs.ram_usage_chart.push_series(ram_line);
                } else {
                    fs.ram_usage_chart.push_to(0, ram_usage);
                }

                if let Some(swap) = swap.to_option() {
                    let len = swap.swaps.len();
                    let current_series_cnt = fs.ram_usage_chart.series_count();

                    for id in 0..len {
                        let series_idx = id + 1;
                        let swap_usage = swap.swaps[id].usage_percentage().unwrap_or(0.);
                        let swap_name = swap.swaps[id].filename.clone();

                        if series_idx >= current_series_cnt {
                            let color = match colors_set.get(&swap_name) {
                                Some(col) => col.to_color(),
                                None => {
                                    if def_colors.len() - 1 < id {
                                        color!(255, 255, 128)
                                    } else {
                                        def_colors[id]
                                    }
                                }
                            };
                            let mut line = LineSeries::new(
                                swap.swaps[id].filename.clone(),
                                color,
                                fs.show_chart_elements,
                            );
                            line.push(swap_usage);

                            fs.ram_usage_chart.push_series(line);
                        } else {
                            fs.ram_usage_chart.push_to(series_idx, swap_usage);
                        }
                    }
                }
                Task::none()
            }
            Self::OsReleaseDataReceived(state) => {
                fd.osrel_data = state;
                Task::none()
            }
            Self::GetOsReleaseData => Task::perform(
                async move {
                    let osrel = OsRelease::new();
                    match osrel {
                        Ok(osrel) => DataLoadingState::Loaded(osrel),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::OsReleaseDataReceived(val)),
            ),
            Self::KernelDataReceived(state) => {
                fd.kernel_data = state;
                Task::none()
            }
            Self::GetKernelData => Task::perform(
                async move {
                    let kern = Kernel::new();
                    match kern {
                        Ok(kern) => {
                            // kern.mods.modules.sort_by_key(|md| md.name.clone());
                            DataLoadingState::Loaded(kern)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::KernelDataReceived(val)),
            ),
            Self::KModsDataReceived(state) => {
                fd.kmods_data = state;
                Task::none()
            }
            Self::GetKModsData => {
                if !fs.is_kmods_polkit
                    && ((fd.kmods_data.is_none() && cur_page == Page::KModules)
                        || export.selected_pages.kmods)
                {
                    fs.is_kmods_polkit = true;
                    Task::perform(
                        async move { ferrix_data::kmods::get_kmods_list().await },
                        |val| Message::DataReceiver(Self::KModsDataReceived(val)),
                    )
                } else {
                    Task::none()
                }
            }
            Self::UsersDataReceived(state) => {
                fd.users_list = state;
                Task::none()
            }
            Self::GetUsersData => Task::perform(
                async move {
                    let users = Users::new();
                    match users {
                        Ok(mut users) => {
                            users.users.sort_by_key(|usr| usr.uid);
                            DataLoadingState::Loaded(users)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::UsersDataReceived(val)),
            ),
            Self::GroupsDataReceived(state) => {
                fd.groups_list = state;
                Task::none()
            }
            Self::GetGroupsData => Task::perform(
                async move {
                    let groups = Groups::new();
                    match groups {
                        Ok(mut groups) => {
                            groups.groups.sort_by_key(|grp| grp.gid);
                            DataLoadingState::Loaded(groups)
                        }
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::GroupsDataReceived(val)),
            ),
            Self::SystemdServicesReceived((sysd_services, boot_time)) => {
                fd.sysd_services_list = sysd_services;
                // dbg!(&boot_time);
                fd.boot_time = boot_time;
                Task::none()
            }
            Self::GetSystemdServices => Task::perform(
                async move {
                    let conn = Connection::session().await;
                    if let Err(why) = conn {
                        return (
                            DataLoadingState::Error(why.to_string()),
                            DataLoadingState::Loading,
                        );
                    }
                    let conn = conn.unwrap();

                    let srv_list = SystemdServices::new_from_connection(&conn).await;
                    match srv_list {
                        Ok(mut srv) => {
                            if srv.timestamps.total == 0 {
                                let a = srv.timestamps.calc_boot_time();
                                // dbg!(&a);
                                if let Err(why) = a {
                                    return (
                                        DataLoadingState::Loaded(srv),
                                        DataLoadingState::Error(why.to_string()),
                                    );
                                }
                            }

                            let boot_time = srv.timestamps;
                            (
                                DataLoadingState::Loaded(srv),
                                DataLoadingState::Loaded(boot_time),
                            )
                        }
                        Err(why) => (
                            DataLoadingState::Error(why.to_string()),
                            DataLoadingState::Error("".to_string()),
                        ),
                    }
                },
                |val| Message::DataReceiver(Self::SystemdServicesReceived(val)),
            ),
            Self::SystemDataReceived(state) => {
                fd.system = state;
                Task::none()
            }
            Self::GetPackagesList => Task::perform(
                async move {
                    let pkglist = InstalledPackages::get();
                    match pkglist {
                        Ok(pkglist) => DataLoadingState::Loaded(pkglist),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::PackagesListReceived(val)),
            ),
            Self::PackagesListReceived(state) => {
                fd.installed_pkgs_list = state;
                Task::none()
            }
            Self::GetSystemData => Task::perform(
                async move {
                    let sys = System::new();
                    match sys {
                        Ok(sys) => DataLoadingState::Loaded(sys),
                        Err(why) => DataLoadingState::Error(why.to_string()),
                    }
                },
                |val| Message::DataReceiver(Self::SystemDataReceived(val)),
            ),
        }
    }
}

pub type ExportToFilePath = String;

#[derive(Debug, Clone)]
pub enum ExportManagerMessage {
    ExportData(ExportToFilePath),
    FormatSelected(ExportFormat),
    ModeSelected(ExportMode),
    PageAdded(Page),
    // GetPagesData,
}

impl ExportManagerMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::ExportData(path) => fx.export_data(&path),
            Self::FormatSelected(format) => fx.set_export_format(format),
            Self::ModeSelected(mode) => fx.set_export_mode(mode),
            Self::PageAdded(page) => fx.add_page_to_export_queue(page),
        }
    }
}

impl Ferrix {
    fn export_data(&mut self, path: &str) -> Task<Message> {
        self.export_manager.status = ExportStatus::SerializingStructure;
        let export_data = ExportData::from(&self.data);
        let json = match self.export_manager.format {
            ExportFormat::CompressedJson => export_data.to_json().unwrap_or("{error}".to_string()),
            _ => export_data
                .to_json_pretty()
                .unwrap_or("{error}".to_string()),
        };
        self.export_manager.status = ExportStatus::WritingData;
        if let Err(why) = std::fs::write(path, json) {
            self.export_manager.status = ExportStatus::ErrorWritingData(why.to_string());
        }
        self.export_manager.status = ExportStatus::Complete;
        Task::none()
    }

    fn set_export_format(&mut self, format: ExportFormat) -> Task<Message> {
        self.export_manager.format = format;
        Task::none()
    }

    fn set_export_mode(&mut self, mode: ExportMode) -> Task<Message> {
        self.export_manager.mode = mode;
        Task::none()
    }

    fn add_page_to_export_queue(&mut self, page: Page) -> Task<Message> {
        if page.is_special() {
            return Task::none();
        }
        let export = &mut self.export_manager.selected_pages;
        match page {
            Page::Processors => export.proc = !export.proc,
            Page::CPUFrequency => export.cpu_freq = !export.cpu_freq,
            Page::CPUVulnerabilities => export.cpu_vuln = !export.cpu_vuln,
            Page::Memory => export.mem = !export.mem,
            Page::FileSystems => export.fs = !export.fs,
            Page::Network => export.net = !export.net,
            Page::DMI => export.dmi = !export.dmi,
            Page::Battery => export.bat = !export.bat,
            Page::Screen => export.screen = !export.screen,
            Page::Distro => export.distro = !export.distro,
            Page::Users => export.users = !export.users,
            Page::Groups => export.groups = !export.groups,
            Page::Environment => export.env = !export.env,
            Page::SystemManager => export.sys_mgr = !export.sys_mgr,
            Page::Software => export.soft = !export.soft,
            Page::Kernel => export.kernel = !export.kernel,
            Page::KModules => export.kmods = !export.kmods,
            Page::SystemMisc => export.sysmisc = !export.sysmisc,
            _ => {}
        }
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChangeStyle(Style),
    ChangeUpdatePeriod(u8),
    ChangeChartsUpdatePeriod(u8),
    ChangeChartLineThickness(ChartLineThickness),
    SetChartItemColor(String, (u8, u8, u8)),
}

impl SettingsMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::ChangeStyle(style) => fx.change_style(style),
            Self::ChangeUpdatePeriod(secs) => fx.change_update_period(secs),
            Self::ChangeChartsUpdatePeriod(secs) => fx.change_charts_update_period(secs),
            Self::ChangeChartLineThickness(thick) => fx.change_line_thickness(thick),
            Self::SetChartItemColor(item, color) => fx.set_chart_item_color(item, color),
        }
    }
}

impl Ferrix {
    fn change_style(&mut self, style: Style) -> Task<Message> {
        self.settings.style = style;
        self.state.cpu_usage_chart.set_style(&style.to_theme());
        self.state.ram_usage_chart.set_style(&style.to_theme());
        Task::none()
    }

    fn change_update_period(&mut self, per: u8) -> Task<Message> {
        self.settings.update_period = per;
        Task::none()
    }

    fn change_charts_update_period(&mut self, per: u8) -> Task<Message> {
        self.settings.charts_update_period_nsecs = per;
        Task::none()
    }

    fn change_line_thickness(&mut self, thick: ChartLineThickness) -> Task<Message> {
        self.settings.chart_line_thickness = thick;
        self.state
            .cpu_usage_chart
            .set_line_thickness(thick.to_u32());
        self.state
            .ram_usage_chart
            .set_line_thickness(thick.to_u32());

        Task::none()
    }

    fn set_chart_item_color(&mut self, item: String, color: (u8, u8, u8)) -> Task<Message> {
        self.settings.chart_colors.colors.insert(item, color);
        self.save_settings()
    }
}

#[derive(Debug, Clone)]
pub enum ButtonsMessage {
    LinkButtonPressed(String),
    SaveSettingsButtonPressed,
    CopyButtonPressed(String),

    ChangeLegendShow(bool),
    ProcessorSelected(usize),
}

impl ButtonsMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::LinkButtonPressed(url) => fx.go_to_url(&url),
            Self::SaveSettingsButtonPressed => fx.save_settings(),
            Self::CopyButtonPressed(s) => iced::clipboard::write(s),
            Self::ChangeLegendShow(show) => fx.set_show_charts_legend(show),
            Self::ProcessorSelected(id) => fx.proc_selected(id),
        }
    }
}

impl Ferrix {
    fn go_to_url(&self, url: &str) -> Task<Message> {
        // TODO: add error handling
        let _ = crate::utils::xdg_open(url);
        Task::none()
    }

    fn save_settings(&mut self) -> Task<Message> {
        // TODO: add error handling
        let _ = self
            .settings
            .write(get_home().join(".config").join(SETTINGS_PATH));
        Task::none()
    }

    fn set_show_charts_legend(&mut self, show: bool) -> Task<Message> {
        self.state.cpu_usage_chart.set_show_legend(show);
        self.state.ram_usage_chart.set_show_legend(show);
        self.state.show_charts_legend = show;
        Task::none()
    }

    fn proc_selected(&mut self, id: usize) -> Task<Message> {
        self.state.selected_proc = id;
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum KeyboardMessage {
    Event(Event),
}

fn get_id(page: Page, m: Modifiers) -> Id {
    if m.shift() {
        Id::new(page.scrolled_list_id().unwrap_or(""))
    } else {
        Id::new(page.page_id())
    }
}

const SCROLL_UP: f32 = -20.;
const SCROLL_DOWN: f32 = 20.;

fn scroll_up(page: Page, m: Modifiers) -> Task<Message> {
    let id = get_id(page, m);
    operation::scroll_by(
        id,
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_UP,
        },
    )
}

fn scroll_down(page: Page, m: Modifiers) -> Task<Message> {
    let id = get_id(page, m);
    operation::scroll_by(
        id,
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_DOWN,
        },
    )
}

fn scroll_sidebar_up() -> Task<Message> {
    operation::scroll_by(
        Id::new("sidebar"),
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_UP,
        },
    )
}

fn scroll_sidebar_down() -> Task<Message> {
    operation::scroll_by(
        Id::new("sidebar"),
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_DOWN,
        },
    )
}

fn snap_up(page: Page) -> Task<Message> {
    let id = Id::new(page.page_id());
    operation::snap_to(id, RelativeOffset::START)
}

fn snap_down(page: Page) -> Task<Message> {
    let id = Id::new(page.page_id());
    operation::snap_to(id, RelativeOffset::END)
}

impl KeyboardMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::Event(event) => match event {
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowDown),
                    modifiers,
                    ..
                }) if !modifiers.control() => scroll_down(fx.current_page, modifiers),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowUp),
                    modifiers,
                    ..
                }) if !modifiers.control() => scroll_up(fx.current_page, modifiers),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowDown),
                    modifiers,
                    ..
                }) if modifiers.control() => scroll_sidebar_down(),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowUp),
                    modifiers,
                    ..
                }) if modifiers.control() => scroll_sidebar_up(),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::PageDown),
                    ..
                }) => snap_down(fx.current_page),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::PageUp),
                    ..
                }) => snap_up(fx.current_page),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F1),
                    ..
                }) => fx.select_page(Page::About),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F2),
                    ..
                }) => fx.select_page(Page::Export),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F9),
                    ..
                }) => fx.select_page(Page::Settings),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) if modifiers.control() => fx.select_page(if modifiers.shift() {
                    fx.current_page.prev_page()
                } else {
                    fx.current_page.next_page()
                }),
                _ => Task::none(),
            },
        }
    }
}
