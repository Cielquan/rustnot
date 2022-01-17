#[cfg(unix)]
use notify_rust::{Notification, Timeout};
#[cfg(target_os = "windows")]
use winrt_notification::{Duration, Sound, Toast};

#[cfg(unix)]
pub fn send_notification(prompt: &str, next_toast_in: u32) {
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

#[cfg(target_os = "windows")]
pub fn send_notification(prompt: &str, next_toast_in: u32) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(prompt)
        .text1("It's time to change your stance.")
        .text2(&format!("Next reminder in: {} min.", next_toast_in))
        .sound(Some(Sound::Default))
        .duration(Duration::Long)
        .show()
        .expect("unable to toast");
}
