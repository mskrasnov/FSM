pub mod error_page;
pub mod todo_page;

pub mod mem;
pub mod proc;

use iced::{
    Alignment::Center,
    Element, Length, Task,
    widget::{Id, column, row, rule, scrollable, space, text},
};

use crate::message::{DataReceiver, Message};

pub trait PageView<'a> {
    fn page_id() -> &'static str;
    fn page_title() -> String;
    fn page_group() -> GroupVariant;

    fn page_title_controls(&'a self) -> Option<Element<'a, Message>> {
        None
    }

    fn page_title_view(&'a self) -> Element<'a, Message> {
        column![
            row![
                text(Self::page_title()).size(26),
                space::horizontal(),
                self.page_title_controls().unwrap_or(row![].into()),
            ]
            .align_y(Center)
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum PageVariant {
    #[default]
    SystemPassport,
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

impl PageVariant {
    pub const ALL: &'static [Self] = &[
        // General
        Self::SystemPassport,
        Self::SystemMonitor,
        // Hardware
        Self::Processors,
        Self::CPUFrequencies,
        Self::CPUVulnerabilities,
        Self::Memory,
        Self::FileSystems,
        Self::DMITables,
        Self::Battery,
        Self::Screens,
        Self::Sensors,
        // Network
        Self::NetworkInterfaces,
        Self::NetworkStatistics,
        // Admin
        Self::Distro,
        Self::Session,
        Self::Users,
        Self::Groups,
        Self::Environment,
        Self::SystemManager,
        Self::Software,
        // System
        Self::Kernel,
        Self::KernelModules,
        Self::FirmwareAttributes,
        Self::SystemMisc,
    ];

    pub fn group(&self) -> GroupVariant {
        match self {
            Self::SystemPassport => GroupVariant::General,
            Self::SystemMonitor => GroupVariant::General,
            Self::Processors => proc::ProcPage::page_group(),
            Self::CPUFrequencies => GroupVariant::Hardware,
            Self::CPUVulnerabilities => GroupVariant::Hardware,
            Self::Memory => mem::MemoryPage::page_group(),
            Self::FileSystems => GroupVariant::Hardware,
            Self::DMITables => GroupVariant::Hardware,
            Self::Battery => GroupVariant::Hardware,
            Self::Screens => GroupVariant::Hardware,
            Self::Sensors => GroupVariant::Hardware,
            Self::NetworkInterfaces => GroupVariant::Network,
            Self::NetworkStatistics => GroupVariant::Network,
            Self::Distro => GroupVariant::Admin,
            Self::Session => GroupVariant::Admin,
            Self::Users => GroupVariant::Admin,
            Self::Groups => GroupVariant::Admin,
            Self::Environment => GroupVariant::Admin,
            Self::SystemManager => GroupVariant::Admin,
            Self::Software => GroupVariant::Admin,
            Self::Kernel => GroupVariant::System,
            Self::KernelModules => GroupVariant::System,
            Self::FirmwareAttributes => GroupVariant::System,
            Self::SystemMisc => GroupVariant::System,
            _ => GroupVariant::Other,
        }
    }

    pub fn id(&self) -> Id {
        Id::new(match self {
            Self::Processors => proc::ProcPage::page_id(),
            Self::Memory => mem::MemoryPage::page_id(),
            _ => "",
        })
    }

    pub fn title(&self) -> String {
        match self {
            Self::Processors => proc::ProcPage::page_title(),
            Self::Memory => mem::MemoryPage::page_title(),
            _ => format!("{:?}", self),
        }
    }

    pub fn view<'a>(&'a self, fx: &'a crate::Ferrix) -> Element<'a, Message> {
        match self {
            Self::Processors => fx.proc_page.view(),
            Self::Memory => fx.mem_page.view(),
            _ => todo_page::todo(),
        }
    }

    fn page_idx(&self) -> usize {
        Self::ALL.iter().position(|p| p == self).unwrap()
    }

    pub fn next_page(&self) -> Self {
        Self::ALL[(self.page_idx() + 1) % Self::ALL.len()]
    }

    pub fn prev_page(&self) -> Self {
        Self::ALL[(self.page_idx() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GroupVariant {
    General,
    Hardware,
    Network,
    Admin,
    System,
    Other,
}

impl GroupVariant {
    pub const ALL: &'static [Self] = &[
        Self::General,
        Self::Hardware,
        Self::Network,
        Self::Admin,
        Self::System,
    ];

    pub fn title(&self) -> String {
        match self {
            Self::General => "General",
            Self::Hardware => "Hardware",
            Self::Network => "Network",
            Self::Admin => "Administration",
            Self::System => "System",
            Self::Other => "Other",
        }
        .to_string()
    }
}
