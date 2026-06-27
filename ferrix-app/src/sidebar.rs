/* sidebar.rs
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

//! Sidebar widget

use crate::{Page, fl, messages::Message, widgets::sidebar_button};
use ferrix_widgets::button::icon_button;
use iced::{
    Alignment::Center,
    Element, Length,
    widget::{Id, column, container, row, scrollable, text},
};

pub fn sidebar<'a>(cur_page: Page) -> Element<'a, Message> {
    let buttons = row![
        icon_button("export", fl!("sidebar-export")).on_press(Message::SelectPage(Page::Export)),
        icon_button("settings", fl!("sidebar-settings"))
            .on_press(Message::SelectPage(Page::Settings)),
        icon_button("about", fl!("sidebar-about")).on_press(Message::SelectPage(Page::About)),
    ]
    .spacing(5)
    .align_y(Center);

    let pages = [
        Item::Group(fl!("sidebar-basic")),
        Item::Page(Page::Dashboard),
        Item::Page(Page::SystemMonitor),
        Item::Group(fl!("sidebar-hardware")),
        Item::Page(Page::Processors),
        Item::Page(Page::CPUFrequency),
        Item::Page(Page::CPUVulnerabilities),
        Item::Page(Page::Memory),
        Item::Page(Page::FileSystems),
        Item::Page(Page::Network),
        Item::Page(Page::NetStat),
        Item::Page(Page::DMI),
        Item::Page(Page::Battery),
        Item::Page(Page::Screen),
        Item::Page(Page::Sensors),
        Item::Group(fl!("sidebar-admin")),
        Item::Page(Page::Distro),
        // Item::Page(Page::Session),
        Item::Page(Page::Users),
        Item::Page(Page::Groups),
        Item::Page(Page::Environment),
        Item::Page(Page::SystemManager),
        Item::Page(Page::Software),
        Item::Group(fl!("sidebar-system")),
        Item::Page(Page::Kernel),
        Item::Page(Page::KModules),
        Item::Page(Page::Firmware),
        Item::Page(Page::SystemMisc),
        Item::Group(fl!("sidebar-manage")),
        Item::Page(Page::Settings),
        Item::Page(Page::About),
    ];
    let mut pages_list = iced::widget::Column::with_capacity(pages.len()).spacing(3);

    for page in pages {
        pages_list = pages_list.push(page.widget(cur_page));
    }

    container(column![
        buttons,
        scrollable(pages_list).spacing(5).id(Id::new("sidebar")),
    ])
    .padding(5)
    .style(container::bordered_box)
    .height(Length::Fill)
    .into()
}

enum Item {
    Group(String),
    Page(Page),
}

impl Item {
    pub fn widget<'a>(self, cur_page: Page) -> Element<'a, Message> {
        match self {
            Self::Group(name) => text(name).style(text::secondary).into(),
            Self::Page(page) => sidebar_button(page, cur_page).into(),
        }
    }
}
