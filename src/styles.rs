use iced::widget::container;

pub const OUTER_PADDING: u16 = 20;
pub const MAIN_COLUMN_SPACING: u32 = 20;

pub const HORIZONTAL_RULE_HEIGHT: u32 = 2;

pub const BUTTON_PADDING: [u16; 2] = [8, 10];

pub const COL_SPACING: u32 = 5;

pub const ROW_PADDING: u16 = 15;
pub const ROW_SPACING: u32 = 10;

pub const TEXT_SIZE_HEADING: u32 = 45;
pub const TEXT_SIZE_NORMAL: u32 = 20;

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
