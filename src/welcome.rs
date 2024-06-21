// Welcome screen for the text editor

use iced::Length;
use std::ops::Range;

use crate::texteditor::Message;
use iced::widget::{button, column, container, text};
use iced::Element;

pub fn welcome_screen() -> Element<'static, Message> {
    let welcome = container(text("Welcome to SEIL").size(50))
        .width(1000)
        .center_x();

    let open_control = container(button("Open").on_press(Message::OpenFile).padding(5))
        .width(1000)
        .center_x();

    container(column![welcome, open_control].spacing(10))
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

// Custom Syntax Highlighting
