/* ferrix.rs
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

use iced::{Element, Subscription, Task, color, widget::row};
use std::time::Duration;

use crate::{
    message::{DataReceiver, KeyboardAndMouse, Message},
    navigation,
    pages::{self, PageData, PageVariant},
};

#[derive(Debug)]
pub struct Ferrix {
    pub active_page: PageVariant,

    pub proc_page: pages::proc::ProcPage,
    pub mem_page: pages::mem::MemoryPage,
    pub fs_page: pages::fs::FSPage,
    pub dmi_page: pages::dmi::DMIPage,
    pub bat_page: pages::battery::BatPage,
    pub firmware_page: pages::firmware::FirmwarePage,
}

impl Ferrix {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                active_page: PageVariant::Processors,
                proc_page: pages::proc::ProcPage::new(),
                fs_page: pages::fs::FSPage::new(),
                mem_page: pages::mem::MemoryPage::new(),
                dmi_page: pages::dmi::DMIPage::new(),
                bat_page: pages::battery::BatPage::new(),
                firmware_page: pages::firmware::FirmwarePage::new(),
            },
            Task::batch([
                pages::proc::ProcPage::get_data().map(Message::DataReceiver),
                pages::mem::MemoryPage::get_data().map(Message::DataReceiver),
            ]),
        )
    }

    pub fn theme(&self) -> iced::Theme {
        let mut palette = iced::Theme::GruvboxDark.palette();
        palette.success = color!(0x98971a);
        palette.danger = color!(0xfb4934);
        palette.warning = color!(0xfabd2f);
        palette.primary = color!(0xfabd2f);

        iced::Theme::custom("Ferrix Dark Theme", palette)
    }

    pub fn select_page(&mut self, page: PageVariant) -> Task<Message> {
        self.active_page = page;
        match page {
            PageVariant::Processors if self.proc_page.proc_data.is_none() => {
                pages::proc::ProcPage::get_data().map(Message::DataReceiver)
            }
            PageVariant::FileSystems if self.fs_page.mounts.is_none() => {
                pages::fs::FSPage::get_data().map(Message::DataReceiver)
            }
            PageVariant::DMITables if self.dmi_page.dmi.is_none() => {
                pages::dmi::DMIPage::get_data().map(Message::DataReceiver)
            }
            PageVariant::Battery if self.bat_page.bat_info.is_none() => {
                pages::battery::BatPage::get_data().map(Message::DataReceiver)
            }
            PageVariant::FirmwareAttributes if self.firmware_page.firmware.is_none() => {
                pages::firmware::FirmwarePage::get_data().map(Message::DataReceiver)
            }
            _ => Task::none(),
        }
    }

    pub fn message(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::DataReceiver(drm) => drm.update(self),
            Message::KeyboardAndMouse(key) => key.update(self),
            Message::PageMessage(page) => page.update(self),
            Message::SelectPage(page) => self.select_page(page),
            _ => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let scripts = vec![
            iced::event::listen()
                .map(|event| Message::KeyboardAndMouse(KeyboardAndMouse::Event(event))),
            iced::time::every(Duration::from_secs_f32(1.))
                .map(|_| Message::DataReceiver(DataReceiver::GetRAMData)),
        ];
        Subscription::batch(scripts)
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let page = self.active_page.view(&self);

        row![navigation::sidebar(self.active_page), page]
            .spacing(5)
            .padding(5)
            .into()
    }
}
