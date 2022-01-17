#![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod config;
mod notification;

use std::{thread, time};

use config::CONFIG;

enum CurrentStance {
    Standing,
    Sitting,
}

fn main() {
    let mut current_stance = CurrentStance::Standing;
    let mut waiting_time: u32;
    config::load_config().unwrap();
    config::save_config().unwrap();

    loop {
        match current_stance {
            CurrentStance::Standing => {
                current_stance = CurrentStance::Sitting;
                waiting_time = (*CONFIG.lock()).settings.sit_time;
                notification::send_notification("Sit Down!", waiting_time);
            }
            CurrentStance::Sitting => {
                current_stance = CurrentStance::Standing;
                waiting_time = (*CONFIG.lock()).settings.stand_time;
                notification::send_notification("Stand Up!", waiting_time);
            }
        }
        thread::sleep(time::Duration::from_secs(waiting_time as u64 * 60));
    }
}
