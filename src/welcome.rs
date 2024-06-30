// Welcome screen for the text editor

use iced::alignment::Horizontal;
use iced::widget::{Button, Text};
use iced::{theme, Length, Theme};
use std::ops::Range;

use crate::texteditor::Message;
use iced::widget::{button, column, container, row, text};
use iced::Element;

pub fn welcome_screen() -> Element<'static, Message> {
    // load image assets/SEIL.png and display on the top
    let image = iced::widget::image::Image::new("assets/EDITOR.png");
    let img = container(image)
        .width(Length::Fill)
        .center_x()
        .height(Length::Fill)
        .center_y();

    let new_file = container(
        Button::new(
            Text::new("New File")
                .style(theme::Text::Color(iced::Color::WHITE))
                .width(150)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::NewFile)
        .style(theme::Button::Primary),
    )
    .width(Length::Fill)
    .center_x();

    let open_file = container(
        Button::new(
            Text::new("Open File")
                .style(theme::Text::Color(iced::Color::WHITE))
                .width(150)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::OpenFile)
        .style(theme::Button::Primary),
    )
    .width(Length::Fill)
    .center_x();

    let welcome = container(
        column![
            container(
                text("SEIL")
                    .size(50)
                    .font(iced::Font::with_name("SERIF"))
                    .style(theme::Text::Color([0.81, 0.96, 0.49].into()))
            )
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

    container(
        row![
            img,
            container(column![welcome, open_file, new_file].spacing(10))
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
