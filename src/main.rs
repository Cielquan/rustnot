#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
