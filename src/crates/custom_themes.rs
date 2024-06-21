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

pub fn black() -> Palette {
    Palette {
        background: Color::BLACK,

        text: Color::WHITE,

        primary: Color::from_rgb8(255, 213, 3),

        success: Color::from_rgb8(80, 250, 123),

        danger: Color::from_rgb8(255, 85, 85),
    }
}

pub fn white() -> Palette {
    Palette {
        background: Color::WHITE,

        text: Color::BLACK,

        primary: Color::from_rgb8(255, 213, 3),

        success: Color::from_rgb8(80, 250, 123),

        danger: Color::from_rgb8(255, 85, 85),
    }
}

pub fn exotic() -> Palette {
    Palette {
        background: Color::from_rgb8(12, 12, 12), // Very dark grey background

        text: Color::from_rgb8(173, 255, 47), // Green-yellow text

        primary: Color::from_rgb8(0, 100, 0), // Dark green primary

        success: Color::from_rgb8(50, 205, 50), // Lime green success

        danger: Color::from_rgb8(255, 69, 0), // Red-orange danger
    }
}
