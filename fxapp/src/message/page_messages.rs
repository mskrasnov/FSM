/* page_messages.rs
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

use iced::Task;

use crate::{
    ferrix::Ferrix,
    message::Message,
    pages::{
        PageVariant, dmi::DMIPageMessage, drm::DRMPageMessage, freq::ProcFreqMessage,
        mem::MemoryPageMessage, proc::ProcPageMessage, sysmon::SysMonPageMessage,
    },
};

#[derive(Debug, Clone)]
pub enum PageMessage {
    SysMonPage(SysMonPageMessage),
    ProcPage(ProcPageMessage),
    CpuFreqMessage(ProcFreqMessage),
    DMIPage(DMIPageMessage),
    MemPage(MemoryPageMessage),
    DRMPage(DRMPageMessage),

    ExportSingle(PageVariant),
}

impl PageMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::ExportSingle(page) => {
                match page {
                    PageVariant::Processors => {
                        let data = fx.proc_page.proc_data.unwrap();
                        let contents = serde_json::to_string(data).unwrap();
                        std::fs::write(format!("Export Page {page:?}.json"), contents).unwrap();
                    }
                    PageVariant::DMITables => {
                        let data = fx.dmi_page.dmi.unwrap();
                        let contents = serde_json::to_string(data).unwrap();
                        std::fs::write(format!("Export Page {page:?}.json"), contents).unwrap();
                    }
                    _ => {}
                }
                Task::none()
            }
            Self::SysMonPage(smp) => smp.update(fx),
            Self::ProcPage(pm) => pm.update(&mut fx.proc_page),
            Self::CpuFreqMessage(cfm) => cfm.update(&mut fx.freq_page),
            Self::DMIPage(dp) => dp.update(&mut fx.dmi_page),
            Self::MemPage(mm) => mm.update(&mut fx.mem_page),
            Self::DRMPage(drm) => drm.update(&mut fx.drm_page),
        }
    }
}
