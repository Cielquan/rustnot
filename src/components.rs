use iced::widget::{Button, button, center, container, mouse_area, opaque, stack, svg};

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

pub fn create_icon_btn<'a, Message>(
    file_path: &str,
) -> Button<'a, Message, iced::Theme, iced::Renderer> {
    button(svg(file_path).content_fit(iced::ContentFit::Contain).style(
        |theme: &iced::Theme, _style| svg::Style {
            color: Some(theme.palette().text),
        },
    ))
    .width(iced::Length::Shrink)
    .height(iced::Length::Shrink)
    .padding(7)
    .style(button::background)
}
