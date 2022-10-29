use parking_lot::Mutex;
use thiserror::Error;

use crate::config::Stance;
use crate::notification;

lazy_static! {
    pub static ref TIMER_SIGNAL: Mutex<TimerSignal> = Mutex::new(TimerSignal::Run);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerSignal {
    Run,
    Abort,
}

async fn sleeper(waiting_time: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(waiting_time * 60)).await;
}

async fn aborter() {
    loop {
        if *TIMER_SIGNAL.lock() == TimerSignal::Abort {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

#[derive(Error, Debug, Clone)]
pub enum TimerError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FirstReturn {
    Aborter,
    Sleeper,
}

pub async fn start_timer(
    sit_time: u32,
    stand_time: u32,
    start_stance: Stance,
    toast_duration: u32,
) -> Result<(), TimerError> {
    let init_waiting_time;
    let init_prompt;
    if start_stance == Stance::Sitting {
        init_waiting_time = sit_time;
        init_prompt = "Timer starts. Please sit Down.";
    } else {
        init_waiting_time = stand_time;
        init_prompt = "Timer starts. Please stand up.";
    }

    notification::send_notification(init_prompt, init_waiting_time, toast_duration);

    let mut current_stance = start_stance;
    let mut waiting_time = init_waiting_time;

    loop {
        let res: FirstReturn = tokio::select! {
            _ = sleeper(waiting_time as u64) => {FirstReturn::Sleeper},
            _ = aborter() => {FirstReturn::Aborter},
        };

        if res == FirstReturn::Aborter {
            break;
        }

        match current_stance {
            Stance::Standing => {
                current_stance = Stance::Sitting;
                waiting_time = sit_time;
                notification::send_notification("Sit Down!", waiting_time, toast_duration);
            }
            Stance::Sitting => {
                current_stance = Stance::Standing;
                waiting_time = stand_time;
                notification::send_notification("Stand Up!", waiting_time, toast_duration);
            }
        }
    }

    Ok(())
}
