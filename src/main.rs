use iced;

mod app;
mod settings;
mod style;

pub fn main() -> iced::Result {
    iced::application(app::App::new, app::App::update, app::App::view)
        .subscription(app::App::subscription)
        .theme(app::App::theme)
        .title("RustNot")
        .window(iced::window::settings::Settings {
            size: iced::Size {
                width: 400.0,
                height: 600.0,
            },
            ..Default::default()
        })
        .run()
}
