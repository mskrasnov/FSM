use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::cpu::Processors;
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Length, Task,
    widget::{Column, button, column, container, scrollable, text},
};

use crate::message::{DataReceiver, Message};

use super::{PageData, PageView};

#[derive(Debug, Clone)]
pub struct ProcPage {
    pub proc_data: LoadState<Processors>,
    pub id: usize,
}

impl ProcPage {
    pub fn new() -> Self {
        Self {
            proc_data: LoadState::Loading,
            id: 0,
        }
    }

    fn get_proc_list<'a>(
        &'a self,
        proc: &'a Processors,
        names: Vec<(usize, String)>,
    ) -> Vec<iced::Element<'a, Message>> {
        let mut elements = Vec::with_capacity(proc.entries.len());
        for p in names {
            let b = button(text(p.1))
                .on_press(Message::PageMessage(crate::message::PageMessage::ProcPage(
                    ProcPageMessage::ProcSelected(p.0),
                )))
                .style(if p.0 == self.id {
                    button::subtle
                } else {
                    button::text
                })
                .height(Length::Fill)
                .padding(2)
                .into();
            elements.push(b);
        }
        elements
    }

    fn loaded_view<'a>(&'a self, proc: &'a Processors) -> iced::Element<'a, Message> {
        let proc_names = get_proc_names(proc);
        let proc_list = self.get_proc_list(proc, proc_names);
        let first_panel = container(
            column![
                text("Processors").style(text::secondary),
                Column::from_vec(proc_list),
            ]
            .spacing(5),
        )
        .style(container::rounded_box)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(2);
        let second_panel = self.proc_info(proc, self.id);

        let view = SeparatedView::new(first_panel, second_panel)
            .set_fpane_id("aa")
            .set_spane_id(Self::page_id())
            .set_fpane_max_height(Length::Fixed(170.))
            .set_spane_max_height(Length::Fill);

        view.view().into()
    }

    fn proc_info<'a>(&'a self, proc: &'a Processors, id: usize) -> iced::Element<'a, Message> {
        let proc = &proc.entries[id];
        container(
            scrollable(text(format!("{proc:#?}")))
                .width(Length::Fill)
                .spacing(5)
                .id(Self::page_id()),
        )
        .style(container::rounded_box)
        .into()
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
        match &self.proc_data {
            LoadState::Loaded(proc) => self.loaded_view(proc),
            LoadState::Loading => text("Loading data...").into(),
            LoadState::Error(why) => {
                super::error_page::error(why, DataReceiver::GetProcData).map(Message::DataReceiver)
            }
        }
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

#[derive(Debug, Clone)]
pub enum ProcPageMessage {
    ProcSelected(usize),
}

impl ProcPageMessage {
    pub fn update<'a>(self, page: &'a mut ProcPage) -> Task<Message> {
        match self {
            Self::ProcSelected(id) => {
                page.id = id;
                Task::none()
            }
        }
    }
}

fn get_proc_names<'a>(proc: &'a Processors) -> Vec<(usize, String)> {
    let mut i = 0;
    let mut v = Vec::with_capacity(proc.entries.len());

    for p in &proc.entries {
        v.push((
            i,
            match &p.model_name {
                Some(m) => format!("#{i}: {m}"),
                None => format!("#{i}: Unknown processor"),
            },
        ));
        i += 1;
    }
    v
}
