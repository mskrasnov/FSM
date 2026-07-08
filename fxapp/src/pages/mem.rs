use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::ram::RAM;
use iced::{
    Task,
    widget::{button, text},
};

use super::{PageData, PageView};
use crate::message::{DataReceiver, Message};

#[derive(Debug, Clone)]
pub struct MemoryPage {
    pub ram_data: LoadState<RAM>,
}

impl MemoryPage {
    pub fn new() -> Self {
        Self {
            ram_data: LoadState::Loading,
        }
    }
}

impl<'a> PageView<'a> for MemoryPage {
    fn page_id() -> &'static str {
        "mem"
    }

    fn page_title() -> String {
        "Memory".to_string()
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::System
    }

    fn page_title_controls(&'a self) -> Option<iced::Element<'a, Message>> {
        Some(
            button("GET")
                .on_press(Message::DataReceiver(DataReceiver::GetRAMData))
                .style(button::subtle)
                .padding(2)
                .into(),
        )
    }

    fn page_contents_view(&'a self) -> iced::Element<'a, Message> {
        text(format!("{:#?}", &self.ram_data)).into()
    }
}

impl PageData for MemoryPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { RAM::new().to_load_state() },
            DataReceiver::RAMDataReceived,
        )
    }
}
