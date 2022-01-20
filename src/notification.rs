#[cfg(unix)]
use notify_rust::{Notification, Timeout};
#[cfg(target_os = "windows")]
use winrt_notification::{Duration, Sound, Toast};

#[cfg(unix)]
pub fn send_notification(prompt: &str, next_toast_in: u32, toast_duration: u32) {
    Notification::new()
        .summary(prompt)
        .body(&format!(
            "It's time to change your stance.\nNext reminder in: {} min.",
            next_toast_in
        ))
        .sound_name("dialog-information")
        .timeout(Timeout::Milliseconds(toast_duration * 1000))
        .show()
        .expect("unable to toast");
}

#[cfg(target_os = "windows")]
pub fn send_notification(prompt: &str, next_toast_in: u32, toast_duration: u32) {
    let duration = sec_to_duration(toast_duration);

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(prompt)
        .text1("It's time to change your stance.")
        .text2(&format!("Next reminder in: {} min.", next_toast_in))
        .sound(Some(Sound::Default))
        .duration(duration)
        .show()
        .expect("unable to toast");
}

#[cfg(target_os = "windows")]
pub fn sec_to_duration(sec: u32) -> Duration {
    if sec == 7 {
        Duration::Short
    } else if sec == 25 {
        Duration::Long
    } else if 25 - sec > sec - 7 {
        Duration::Short
    } else {
        Duration::Long
    }
}
