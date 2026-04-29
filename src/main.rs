use iced;

#[macro_use]
extern crate serde_derive;

mod app;
mod settings;
mod settings_file;

pub fn main() -> iced::Result {
    iced::application(app::App::new, app::App::update, app::App::view)
        .subscription(app::App::subscription)
        .theme(app::App::theme)
        .title("RustNot")
        .window(iced::window::settings::Settings {
            size: iced::Size {
                width: 400.0,
                height: 400.0,
            },
            ..Default::default()
        })
        .run()
}
