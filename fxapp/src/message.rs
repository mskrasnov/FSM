use crate::pages::PageVariant;
use ferrix_data::load_state::LoadState;
use ferrix_lib::{cpu::Processors, ram::RAM};

#[derive(Debug, Clone)]
pub enum Message {
    SelectPage(PageVariant),
    DataReceiver(DataReceiver),
    PageMessage(PageMessage),

    Dummy,
}

#[derive(Debug, Clone)]
pub enum DataReceiver {
    GetProcData,
    ProcDataReceived(LoadState<Processors>),

    GetRAMData,
    RAMDataReceived(LoadState<RAM>),
}

#[derive(Debug, Clone)]
pub enum PageMessage {
    ProcPage,
}
