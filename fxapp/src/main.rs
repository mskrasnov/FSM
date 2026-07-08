use iced::{Element, Task};

use crate::{
    message::Message,
    pages::{PageData, PageVariant, PageView},
};

pub mod message;
pub mod pages;

#[derive(Debug)]
pub struct Ferrix {
    pub active_page: PageVariant,

    pub proc_page: pages::proc::ProcPage,
}

impl Ferrix {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                active_page: PageVariant::Processors,
                proc_page: pages::proc::ProcPage::new(),
            },
            pages::proc::ProcPage::get_data().map(Message::DataReceiver),
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
            },
            _ => Task::none(),
        }
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        match self.active_page {
            PageVariant::Processors => self.proc_page.view(),
            _ => iced::widget::text("TODO").into(),
        }
    }
}

fn main() -> iced::Result {
    iced::application(Ferrix::new, Ferrix::message, Ferrix::view).run()
}
