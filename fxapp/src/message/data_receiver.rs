/* data_receiver.rs
 *
 * Copyright 2026 Michail Krasnov <mskrasnov07@ya.ru>
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

use ferrix_data::{dmi::DMIData, firmware::FResult, load_state::LoadState};
use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    cpu_freq::CpuFreq,
    drm::Video,
    net::Networks,
    parts::Mounts,
    ram::{RAM, Swaps},
    vulnerabilities::Vulnerabilities,
};
use iced::Task;

use crate::{ferrix::Ferrix, message::Message, pages::PageData};

#[derive(Debug, Clone)]
pub enum DataReceiver {
    GetProcData,
    ProcDataReceived(LoadState<Processors>),

    GetProcStat,
    ProcStatReceived(LoadState<Stat>),

    GetCpuVulnsData,
    CpuVulnsDataReceived(LoadState<Vulnerabilities>),

    GetCpuFreqData,
    CpuFreqDataReceived(LoadState<CpuFreq>),

    GetRAMData,
    RAMDataReceived((LoadState<RAM>, LoadState<Swaps>)),

    GetFilesystemsData,
    FilesystemsDataReceived(LoadState<Mounts>),

    GetDMIData,
    DMIDataRefresh,
    DMIDataReceived(LoadState<DMIData>),

    GetBatData,
    BatDataReceived(LoadState<BatInfo>),

    GetDRMData,
    DRMDataReceived(LoadState<Video>),

    GetNetworkData,
    NetworkDataReceived(LoadState<Networks>),

    GetFirmwareData,
    FirmwareDataRefresh,
    FirmwareDataReceived(LoadState<FResult>),
}

impl DataReceiver {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::GetProcData => {
                crate::pages::proc::ProcPage::get_data().map(Message::DataReceiver)
            }
            Self::ProcDataReceived(val) => {
                fx.proc_page.proc_data = val;
                Task::none()
            }
            Self::GetProcStat => {
                crate::pages::sysmon::SysMonPage::get_data().map(Message::DataReceiver)
            }
            Self::ProcStatReceived(val) => {
                if fx.sysmon_page.curr_proc_stat.is_some() {
                    fx.sysmon_page.prev_proc_stat = fx.sysmon_page.curr_proc_stat.clone();
                } else if fx.sysmon_page.curr_proc_stat.is_none()
                    && fx.sysmon_page.prev_proc_stat.is_none()
                {
                    fx.sysmon_page.prev_proc_stat = val.clone();
                }
                fx.sysmon_page.curr_proc_stat = val;
                Task::none()
            }
            Self::GetCpuVulnsData => {
                crate::pages::vuln::VulnPage::get_data().map(Message::DataReceiver)
            }
            Self::CpuVulnsDataReceived(val) => {
                fx.vulns_page.vulns = val;
                Task::none()
            }
            Self::GetCpuFreqData => {
                crate::pages::freq::CpuFreqPage::get_data().map(Message::DataReceiver)
            }
            Self::CpuFreqDataReceived(val) => {
                fx.freq_page.freqs = val;
                Task::none()
            }
            Self::GetRAMData => {
                crate::pages::mem::MemoryPage::get_data().map(Message::DataReceiver)
            }
            Self::RAMDataReceived(val) => {
                (fx.mem_page.ram_data, fx.mem_page.swap_data) = val;
                Task::none()
            }
            Self::FilesystemsDataReceived(val) => {
                fx.fs_page.mounts = val;
                Task::none()
            }
            Self::GetFilesystemsData => {
                crate::pages::fs::FSPage::get_data().map(Message::DataReceiver)
            }
            Self::GetDMIData => {
                if fx.dmi_page.is_polkit {
                    crate::pages::dmi::DMIPage::get_data().map(Message::DataReceiver)
                } else {
                    Task::none()
                }
            }
            Self::DMIDataRefresh => {
                fx.dmi_page.is_polkit = false;
                crate::pages::dmi::DMIPage::get_data().map(Message::DataReceiver)
            }
            Self::DMIDataReceived(val) => {
                fx.dmi_page.dmi = val;
                Task::none()
            }
            Self::GetBatData => {
                crate::pages::battery::BatPage::get_data().map(Message::DataReceiver)
            }
            Self::BatDataReceived(val) => {
                fx.bat_page.bat_info = val;
                Task::none()
            }
            Self::GetDRMData => crate::pages::drm::DRMPage::get_data().map(Message::DataReceiver),
            Self::DRMDataReceived(val) => {
                fx.drm_page.drm = val;
                Task::none()
            }
            Self::GetNetworkData => {
                crate::pages::netlist::NetStatPage::get_data().map(Message::DataReceiver)
            }
            Self::NetworkDataReceived(val) => {
                fx.net_pages.net = val;
                Task::none()
            }
            Self::GetFirmwareData => {
                if fx.firmware_page.is_polkit {
                    crate::pages::firmware::FirmwarePage::get_data().map(Message::DataReceiver)
                } else {
                    Task::none()
                }
            }
            Self::FirmwareDataRefresh => {
                fx.firmware_page.is_polkit = false;
                crate::pages::firmware::FirmwarePage::get_data().map(Message::DataReceiver)
            }
            Self::FirmwareDataReceived(val) => {
                fx.firmware_page.firmware = val;
                Task::none()
            }
        }
    }
}
