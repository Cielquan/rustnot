use iced::widget::container;

pub fn tooltip_style(theme: &iced::Theme) -> container::Style {
    container::Style {
        border: iced::Border {
            color: iced::Color::BLACK,
            width: 2.0,
            radius: iced::border::radius(3),
        },
        ..container::rounded_box(theme)
    }
}
