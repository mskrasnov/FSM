pub mod error_page;
pub mod proc;

use iced::{
    Element, Length, Task,
    widget::{column, row, rule, scrollable, text},
};

use crate::message::{DataReceiver, Message};

pub trait PageView<'a> {
    fn page_id() -> &'static str;
    fn page_title() -> String;

    fn page_title_controls(&'a self) -> Option<Element<'a, Message>> {
        None
    }

    fn page_title_view(&'a self) -> Element<'a, Message> {
        column![
            row![
                text(Self::page_title()).size(26),
                self.page_title_controls().unwrap_or(row![].into()),
            ]
            .spacing(5),
            rule::horizontal(1),
        ]
        .spacing(5)
        .into()
    }

    fn page_contents_view(&'a self) -> Element<'a, Message>;

    fn view(&'a self) -> Element<'a, Message> {
        column![
            self.page_title_view(),
            scrollable(self.page_contents_view())
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5)
                .id(Self::page_id()),
        ]
        .spacing(5)
        .into()
    }
}

pub trait PageData {
    fn get_data() -> Task<DataReceiver>;
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum PageVariant {
    #[default]
    Dashboard,
    SystemMonitor,
    Processors,
    CPUFrequencies,
    CPUVulnerabilities,
    Memory,
    FileSystems,
    NetworkInterfaces,
    NetworkStatistics,
    DMITables,
    Battery,
    Screens,
    Sensors,
    Distro,
    Session,
    Users,
    Groups,
    Environment,
    SystemManager,
    Software,
    Kernel,
    KernelModules,
    FirmwareAttributes,
    SystemMisc,
    ExportData,
    ProgramSettings,
    ProgramAbout,
    Todo,
}
