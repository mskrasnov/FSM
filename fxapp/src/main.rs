use std::time::Duration;

use iced::{Element, Subscription, Task, widget::row};

use crate::{
    message::Message,
    pages::{PageData, PageVariant},
};

pub mod message;
pub mod navigation;
pub mod pages;

#[derive(Debug)]
pub struct Ferrix {
    pub active_page: PageVariant,

    pub proc_page: pages::proc::ProcPage,
    pub mem_page: pages::mem::MemoryPage,
}

impl Ferrix {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                active_page: PageVariant::Memory,
                proc_page: pages::proc::ProcPage::new(),
                mem_page: pages::mem::MemoryPage::new(),
            },
            Task::batch([
                pages::proc::ProcPage::get_data().map(Message::DataReceiver),
                pages::mem::MemoryPage::get_data().map(Message::DataReceiver),
            ]),
        )
    }

    fn select_page(&mut self, page: PageVariant) -> Task<Message> {
        self.active_page = page;
        Task::none()
    }

    fn message(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::DataReceiver(drm) => match drm {
                message::DataReceiver::GetProcData => {
                    pages::proc::ProcPage::get_data().map(Message::DataReceiver)
                }
                message::DataReceiver::ProcDataReceived(val) => {
                    self.proc_page.proc_data = val;
                    Task::none()
                }
                message::DataReceiver::GetRAMData => {
                    pages::mem::MemoryPage::get_data().map(Message::DataReceiver)
                }
                message::DataReceiver::RAMDataReceived(val) => {
                    self.mem_page.ram_data = val;
                    Task::none()
                }
            },
            Message::SelectPage(page) => {
                self.active_page = page;
                Task::none()
            }
            Message::Keyboard(key) => key.update(self),
            _ => Task::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let scripts = vec![
            iced::event::listen().map(|event| Message::Keyboard(message::Keyboard::Event(event))),
            iced::time::every(Duration::from_secs_f32(1.))
                .map(|_| Message::DataReceiver(message::DataReceiver::GetProcData)),
            iced::time::every(Duration::from_secs_f32(1.))
                .map(|_| Message::DataReceiver(message::DataReceiver::GetRAMData)),
        ];
        Subscription::batch(scripts)
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        let page = self.active_page.view(&self);

        row![navigation::sidebar(self.active_page), page,]
            .spacing(5)
            .padding(5)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(Ferrix::new, Ferrix::message, Ferrix::view)
        .subscription(Ferrix::subscription)
        .run()
}
