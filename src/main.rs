#[cfg(target_os = "windows")]
extern crate winrt_notification;
#[cfg(unix)]
use notify_rust::{Notification, Timeout};
use std::{thread, time};
#[cfg(target_os = "windows")]
use winrt_notification::{Duration, Sound, Toast};

enum CurrentStance {
    Standing,
    Sitting,
}

fn main() {
    let mut current_stance = CurrentStance::Standing;
    let mut waiting_time: u64;

    loop {
        match current_stance {
            CurrentStance::Standing => {
                current_stance = CurrentStance::Sitting;
                waiting_time = 45;
                send_notification("Sit Down!", waiting_time);
            }
            CurrentStance::Sitting => {
                current_stance = CurrentStance::Standing;
                waiting_time = 15;
                send_notification("Stand Up!", waiting_time);
            }
        }
        thread::sleep(time::Duration::from_secs(waiting_time));
    }
}

#[cfg(target_os = "windows")]
fn send_notification(prompt: &str, next_toast: u64) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(prompt)
        .text1("It's time to change your stance.")
        .text2(&format!("Next reminder in: {} min.", next_toast))
        .sound(Some(Sound::Default))
        .duration(Duration::Long)
        .show()
        .expect("unable to toast");
}

#[cfg(unix)]
fn send_notification(prompt: &str, next_toast: &str) {
    Notification::new()
        .summary(prompt)
        .body(&format!(
            "It's time to change your stance.\nNext reminder in: {} min.",
            next_toast
        ))
        .sound_name("dialog-information")
        .timeout(Timeout::Never)
        .show()
        .expect("unable to toast");
}
