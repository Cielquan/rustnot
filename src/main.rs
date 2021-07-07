#[cfg(target_os = "windows")]
extern crate winrt_notification;
#[cfg(unix)]
use notify_rust::{Notification, Timeout};
#[cfg(target_os = "windows")]
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    send_notification("Stand Up!", "10")
}

#[cfg(target_os = "windows")]
fn send_notification(prompt: &str, next_toast: &str) {
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
