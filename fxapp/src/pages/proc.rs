use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::cpu::Processors;
use iced::{Task, widget::text};

use crate::message::{DataReceiver, Message};

use super::{PageData, PageView};

#[derive(Debug, Clone)]
pub struct ProcPage {
    pub proc_data: LoadState<Processors>,
    _selected_proc: usize,
}

impl ProcPage {
    pub fn new() -> Self {
        Self {
            proc_data: LoadState::Loading,
            _selected_proc: 0,
        }
    }
}

impl<'a> PageView<'a> for ProcPage {
    fn page_id() -> &'static str {
        "proc"
    }

    fn page_title() -> String {
        "Processors".to_string()
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_contents_view(&'a self) -> iced::Element<'a, Message> {
        text(format!("{:?}", &self.proc_data)).into()
    }
}

impl PageData for ProcPage {
    fn get_data() -> iced::Task<DataReceiver> {
        Task::perform(
            async move { Processors::new().to_load_state() },
            DataReceiver::ProcDataReceived,
        )
    }
}
