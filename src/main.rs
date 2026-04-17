mod app;
mod settings;

pub fn main() -> iced::Result {
    iced::application(app::App::new, app::App::update, app::App::view)
        .subscription(app::App::subscription)
        .run()
}
