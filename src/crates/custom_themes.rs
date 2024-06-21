// Dracula Theme for Iced
use iced::theme::Palette;
use iced::Color;

pub fn dracula() -> Palette {
    Palette {
        background: Color::from_rgb8(40, 42, 54),

        text: Color::from_rgb8(248, 248, 242),

        primary: Color::from_rgb8(139, 233, 253),

        success: Color::from_rgb8(80, 250, 123),

        danger: Color::from_rgb8(255, 85, 85),
    }
}
