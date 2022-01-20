#![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod app;
mod config;
mod notification;
mod style;
mod timer;

use iced::{window, Application, Settings};

pub fn main() -> iced::Result {
    app::App::run(Settings {
        // antialiasing: true,
        window: window::Settings {
            size: (400, 650),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
