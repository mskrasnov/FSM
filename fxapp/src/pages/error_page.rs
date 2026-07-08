use crate::message::DataReceiver;
use iced::widget::{button, center, column, text};

pub fn error<'a>(etext: &'a str, message: DataReceiver) -> iced::Element<'a, DataReceiver> {
    let update_btn = button("Update").on_press(message).style(button::danger);

    center(column![text("Error!").size(26), text(etext), update_btn,].spacing(5)).into()
}
