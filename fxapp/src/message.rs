use crate::{
    Ferrix,
    pages::{PageData, PageVariant, proc::ProcPageMessage},
};
use ferrix_data::load_state::LoadState;
use ferrix_lib::{cpu::Processors, ram::RAM};
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
    Keyboard(Keyboard),

    Dummy,
}

#[derive(Debug, Clone)]
pub enum DataReceiver {
    GetProcData,
    ProcDataReceived(LoadState<Processors>),

    GetRAMData,
    RAMDataReceived(LoadState<RAM>),
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
                fx.mem_page.ram_data = val;
                Task::none()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PageMessage {
    ProcPage(ProcPageMessage),
}

impl PageMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::ProcPage(pm) => pm.update(&mut fx.proc_page),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Keyboard {
    Event(Event),
}

impl Keyboard {
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
        }
    }
}

const SCROLL_UP: f32 = -20.;
const SCROLL_DOWN: f32 = 20.;

fn get_id(page: PageVariant, m: Modifiers) -> Id {
    if m.shift() {
        Id::new("") // TODO
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
