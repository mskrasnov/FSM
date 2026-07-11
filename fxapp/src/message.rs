/* message.rs
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
    Ferrix,
    pages::{
        PageData, PageVariant, dmi::DMIPageMessage, mem::MemoryPageMessage, proc::ProcPageMessage,
    },
};
use ferrix_data::{dmi::DMIData, firmware::FResult, load_state::LoadState};
use ferrix_lib::{
    battery::BatInfo,
    cpu::Processors,
    ram::{RAM, Swaps},
};
use iced::{
    Event, Task,
    keyboard::{Event as Kevent, Key, Modifiers, key},
    widget::{
        Id,
        operation::{self, AbsoluteOffset, RelativeOffset},
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    SelectPage(PageVariant),
    DataReceiver(DataReceiver),
    PageMessage(PageMessage),
    KeyboardAndMouse(KeyboardAndMouse),

    Dummy,
}

#[derive(Debug, Clone)]
pub enum DataReceiver {
    GetProcData,
    ProcDataReceived(LoadState<Processors>),

    GetRAMData,
    RAMDataReceived((LoadState<RAM>, LoadState<Swaps>)),

    GetDMIData,
    DMIDataRefresh,
    DMIDataReceived(LoadState<DMIData>),

    GetBatData,
    BatDataReceived(LoadState<BatInfo>),

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
            Self::GetRAMData => {
                crate::pages::mem::MemoryPage::get_data().map(Message::DataReceiver)
            }
            Self::RAMDataReceived(val) => {
                (fx.mem_page.ram_data, fx.mem_page.swap_data) = val;
                Task::none()
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

#[derive(Debug, Clone)]
pub enum PageMessage {
    ExportSingle(PageVariant),
    ProcPage(ProcPageMessage),
    DMIPage(DMIPageMessage),
    MemPage(MemoryPageMessage),
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
            Self::ProcPage(pm) => pm.update(&mut fx.proc_page),
            Self::DMIPage(dp) => dp.update(&mut fx.dmi_page),
            Self::MemPage(mm) => mm.update(&mut fx.mem_page),
        }
    }
}

#[derive(Debug, Clone)]
pub enum KeyboardAndMouse {
    Event(Event),
    LinkButtonPressed(String),
    CopyButtonPressed(String),
}

impl KeyboardAndMouse {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::Event(event) => match event {
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowDown),
                    modifiers,
                    ..
                }) if !modifiers.control() => scroll_down(fx.active_page, modifiers),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowUp),
                    modifiers,
                    ..
                }) if !modifiers.control() => scroll_up(fx.active_page, modifiers),
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
                }) => snap_down(fx.active_page),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::PageUp),
                    ..
                }) => snap_up(fx.active_page),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F1),
                    ..
                }) => fx.select_page(PageVariant::ProgramAbout),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F2),
                    ..
                }) => fx.select_page(PageVariant::ExportData),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F9),
                    ..
                }) => fx.select_page(PageVariant::ProgramSettings),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) if modifiers.control() => fx.select_page(if modifiers.shift() {
                    fx.active_page.prev_page()
                } else {
                    fx.active_page.next_page()
                }),
                _ => Task::none(),
            },
            Self::LinkButtonPressed(_url) => {
                todo!()
            }
            Self::CopyButtonPressed(text) => iced::clipboard::write(text),
        }
    }
}

const SCROLL_UP: f32 = -20.;
const SCROLL_DOWN: f32 = 20.;

fn get_id(page: PageVariant, m: Modifiers) -> Id {
    if m.shift() {
        page.scrolled_id().unwrap_or(Id::new(""))
    } else {
        page.id()
    }
}

fn scroll_up(page: PageVariant, m: Modifiers) -> Task<Message> {
    let id = get_id(page, m);
    operation::scroll_by(
        id,
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_UP,
        },
    )
}

fn scroll_down(page: PageVariant, m: Modifiers) -> Task<Message> {
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

fn snap_up(page: PageVariant) -> Task<Message> {
    let id = page.id();
    operation::snap_to(id, RelativeOffset::START)
}

fn snap_down(page: PageVariant) -> Task<Message> {
    let id = page.id();
    operation::snap_to(id, RelativeOffset::END)
}
