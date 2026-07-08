use iced::widget::{center, text};

pub fn todo<'a>() -> iced::Element<'a, crate::message::Message> {
    center(
        text("This function is not implemented yet")
            .size(20)
            .style(text::secondary),
    )
    .into()
}
