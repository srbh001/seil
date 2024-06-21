// Welcome screen for the text editor

use iced::alignment::Horizontal;
use iced::Length;
use std::ops::Range;

use crate::texteditor::Message;
use iced::widget::{button, column, container, row, text, Button};
use iced::Element;

pub fn welcome_screen() -> Element<'static, Message> {
    // load image assets/SEIL.png and display on the top
    let image = iced::widget::image::Image::new("assets/EDITOR.png");
    let img = container(image)
        .width(Length::Fill)
        .center_x()
        .height(Length::Fill)
        .center_y();
    let welcome = container(
        column![
            container(text("SEIL").size(50))
                .width(Length::Fill)
                .height(80)
                .center_x(),
            container(text("Simple Editor for IIT B CPU (asm) Language"))
                .width(Length::Fill)
                .center_x(),
            container(text("Welcome!")).width(Length::Fill).center_x(),
        ]
        .spacing(10)
        .padding(10),
    )
    .width(Length::Fill)
    .center_x();

    let open_control = container(
        button("        Open a file")
            .on_press(Message::OpenFile)
            .padding(5)
            .width(150),
    )
    .width(Length::Fill)
    .center_x();

    let new_file = container(
        button("           New file") // FIXME: There has to be a better way
            .on_press(Message::NewFile)
            .padding(5)
            .width(150), // center the text inside button
    )
    .width(Length::Fill)
    .center_x();

    container(
        row![
            img,
            container(column![welcome, open_control, new_file].spacing(10))
                .height(Length::Fill)
                .center_y()
        ]
        .spacing(10),
    )
    .padding(30)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

// Custom Syntax Highlighting
