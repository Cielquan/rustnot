use crate::styles::tooltip_style;
use iced::time::milliseconds;
use iced::widget::{
    Button, Svg, Tooltip, button, center, container, mouse_area, opaque, stack, svg, tooltip,
};

pub fn modal<'a, Message>(
    base: impl Into<iced::Element<'a, Message>>,
    content: impl Into<iced::Element<'a, Message>>,
    on_blur: Message,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        iced::Color {
                            a: 0.8,
                            ..iced::Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}

pub fn icon<'a>(icon_file_path: &str, size: Option<u32>) -> Svg<'a, iced::Theme> {
    svg(icon_file_path)
        .content_fit(iced::ContentFit::Contain)
        .style(|theme: &iced::Theme, _style| svg::Style {
            color: Some(theme.palette().text),
        })
        .height(size.unwrap_or(25))
        .width(size.unwrap_or(25))
}

pub fn icon_button<'a, Message>(
    icon_file_path: &str,
) -> Button<'a, Message, iced::Theme, iced::Renderer> {
    button(icon(icon_file_path, None))
        .width(iced::Length::Shrink)
        .height(iced::Length::Shrink)
        .padding(7)
        .style(button::background)
}

pub fn default_tooltip<'a, Message, Renderer>(
    content: impl Into<iced::Element<'a, Message, iced::Theme, Renderer>>,
    tooltip_text: impl Into<iced::Element<'a, Message, iced::Theme, Renderer>>,
) -> Tooltip<'a, Message, iced::Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer,
{
    tooltip(content, tooltip_text, tooltip::Position::Top)
        .gap(5)
        .delay(milliseconds(500))
        .style(tooltip_style)
}
