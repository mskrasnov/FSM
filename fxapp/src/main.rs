use iced::{Element, Task, widget::row};

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
            _ => Task::none(),
        }
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
    iced::application(Ferrix::new, Ferrix::message, Ferrix::view).run()
}
