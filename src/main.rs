#![windows_subsystem = "windows"]
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{thread, time};

use anyhow::Result;
#[cfg(unix)]
use notify_rust::{Notification, Timeout};
use serde_derive::Deserialize;
use toml;
#[cfg(target_os = "windows")]
use winrt_notification::{Duration, Sound, Toast};

enum CurrentStance {
    Standing,
    Sitting,
}

#[derive(Deserialize)]
struct Config {
    sit_time: u64,
    stand_time: u64,
}

const DEFAULT_CONFIG: Config = Config {
    sit_time: 45,
    stand_time: 15,
};

const DEFAULT_CONFIG_FILE_CONTENT: &str = r#"sit_time = 45
stand_time = 15
"#;

const CONFIG_FILE: &str = "rustnot_config.toml";

fn main() {
    let mut current_stance = CurrentStance::Standing;
    let config = load_config();
    let mut waiting_time: u64;

    loop {
        match current_stance {
            CurrentStance::Standing => {
                current_stance = CurrentStance::Sitting;
                waiting_time = config.sit_time;
                send_notification("Sit Down!", waiting_time);
            }
            CurrentStance::Sitting => {
                current_stance = CurrentStance::Standing;
                waiting_time = config.stand_time;
                send_notification("Stand Up!", waiting_time);
            }
        }
        thread::sleep(time::Duration::from_secs(waiting_time * 60));
    }
}

fn load_config() -> Config {
    let file_content: String;
    if Path::new(CONFIG_FILE).exists() {
        file_content = match load_config_from_file() {
            Ok(config) => config,
            Err(_) => String::from(DEFAULT_CONFIG_FILE_CONTENT),
        };
    } else {
        file_content = String::from(DEFAULT_CONFIG_FILE_CONTENT);
        create_new_config_file();
    }
    parse_config(&file_content)
}

fn load_config_from_file() -> Result<String> {
    let mut s = String::new();
    File::open(CONFIG_FILE)?.read_to_string(&mut s)?;
    Ok(s)
}

fn create_new_config_file() {
    match File::create(CONFIG_FILE) {
        Ok(mut file) => match file.write_all(DEFAULT_CONFIG_FILE_CONTENT.as_bytes()) {
            Ok(_) => return,
            Err(_) => panic!("Could not write to config file."),
        },
        Err(_) => panic!("Could not create config file."),
    };
}

fn parse_config(config: &str) -> Config {
    match toml::from_str(config) {
        Ok(conf) => conf,
        Err(_) => DEFAULT_CONFIG,
    }
}

#[cfg(target_os = "windows")]
fn send_notification(prompt: &str, next_toast_in: u64) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(prompt)
        .text1("It's time to change your stance.")
        .text2(&format!("Next reminder in: {} min.", next_toast_in))
        .sound(Some(Sound::Default))
        .duration(Duration::Long)
        .show()
        .expect("unable to toast");
}

#[cfg(unix)]
fn send_notification(prompt: &str, next_toast_in: u64) {
    Notification::new()
        .summary(prompt)
        .body(&format!(
            "It's time to change your stance.\nNext reminder in: {} min.",
            next_toast_in
        ))
        .sound_name("dialog-information")
        .timeout(Timeout::Never)
        .show()
        .expect("unable to toast");
}
