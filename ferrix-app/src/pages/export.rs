/* export.rs
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

//! Export Manager page

use crate::{
    Page,
    export::{ExportFormat, ExportMode, ExportPages},
    ferrix::ExportManager,
    fl,
    messages::{ExportManagerMessage, Message},
    widgets::items_list::{items_group, items_list_container, list_item},
};
use iced::{
    Alignment::Center,
    Element, Length,
    widget::{button, checkbox, column, container, pick_list, row, scrollable, space, text},
};

pub fn export_page<'a>(export_mgr: &'a ExportManager) -> Element<'a, Message> {
    let export_settings_changer = items_group(
        column![
            list_item(
                "Формат экспорта",
                pick_list(ExportFormat::ALL, Some(export_mgr.format), |format| {
                    Message::ExportManager(ExportManagerMessage::FormatSelected(format))
                })
            ),
            list_item(
                "Тип экспортируемых данных",
                pick_list(ExportMode::ALL, Some(export_mgr.mode), |mode| {
                    Message::ExportManager(ExportManagerMessage::ModeSelected(mode))
                })
            ),
        ]
        .spacing(5),
    );

    let mut layout = column![export_settings_changer].spacing(5);

    if export_mgr.mode == ExportMode::Selected {
        layout = layout.push(export_pages_list(&export_mgr.selected_pages));
    }

    layout = layout.push(
        row![
            text(export_mgr.status.to_string()),
            space::horizontal(),
            button("Экспорт").on_press(Message::ExportManager(ExportManagerMessage::ExportData(
                export_mgr.output_path.clone(),
            ))),
        ]
        .align_y(Center),
    );

    items_list_container(scrollable(layout).spacing(5))
}

fn export_pages_list<'a>(pages: &'a ExportPages) -> container::Container<'a, Message> {
    let list = column![
        text("Выберите нужную информацию, которая будет экспортирована:"),
        checkbox(pages.proc)
            .label(fl!("page-procs"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::Processors
            ))),
        checkbox(pages.cpu_freq)
            .label(fl!("page-cpufreq"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::CPUFrequency
            ))),
        checkbox(pages.cpu_vuln)
            .label(fl!("page-vuln"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::CPUVulnerabilities
            ))),
        checkbox(pages.mem)
            .label(fl!("page-memory"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Memory))),
        checkbox(pages.fs)
            .label(fl!("page-fsystems"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::FileSystems
            ))),
        checkbox(pages.net)
            .label(fl!("page-net"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Network))),
        checkbox(pages.dmi)
            .label(fl!("page-dmi"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::DMI))),
        checkbox(pages.bat)
            .label(fl!("page-battery"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Battery))),
        checkbox(pages.screen)
            .label(fl!("page-screen"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Screen))),
        checkbox(pages.distro)
            .label(fl!("page-distro"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Distro))),
        checkbox(pages.users)
            .label(fl!("page-users"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Users))),
        checkbox(pages.groups)
            .label(fl!("page-groups"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Groups))),
        checkbox(pages.env)
            .label(fl!("page-env"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::Environment
            ))),
        checkbox(pages.sys_mgr)
            .label(fl!("page-sysmgr"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::SystemManager
            ))),
        checkbox(pages.soft)
            .label(fl!("page-software"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Software))),
        checkbox(pages.kernel)
            .label(fl!("page-kernel"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::Kernel))),
        checkbox(pages.kmods)
            .label(fl!("page-kmods"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(Page::KModules))),
        checkbox(pages.sysmisc)
            .label(fl!("page-sysmisc"))
            .on_toggle(|_| Message::ExportManager(ExportManagerMessage::PageAdded(
                Page::SystemMisc
            ))),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .spacing(5);

    container(items_group(list))
}
