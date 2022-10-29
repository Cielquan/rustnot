use parking_lot::Mutex;

use crate::config::Stance;
use crate::notification;

lazy_static! {
    pub static ref TIMER_SIGNAL: Mutex<TimerSignal> = Mutex::new(TimerSignal::Run);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerSignal {
    Run,
    Abort,
    Skip,
}

async fn sleeper(waiting_time: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(waiting_time)).await;
}

async fn aborter() {
    loop {
        if *TIMER_SIGNAL.lock() == TimerSignal::Abort {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

async fn skipper() {
    loop {
        if *TIMER_SIGNAL.lock() == TimerSignal::Skip {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FirstReturn {
    Sleeper,
    Aborter,
    Skipper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CycleResult {
    OK,
    Aborted,
    Skipped,
}

pub async fn run_cycle_timer(
    cycle_stance: Stance,
    cycle_duration: u32,
    is_init_cycle: bool,
    toast_duration: u32,
) -> CycleResult {
    let mut prompt;

    if cycle_stance == Stance::Sitting {
        prompt = String::from("Please sit Down.");
    } else {
        prompt = String::from("Please stand up.");
    }

    if is_init_cycle == true {
        prompt = format!("Timer starts. {}", prompt);
    }

    notification::send_notification(&prompt, cycle_duration, toast_duration);

    match tokio::select! {
        _ = sleeper(cycle_duration as u64) => {FirstReturn::Sleeper},
        _ = aborter() => {FirstReturn::Aborter},
        _ = skipper() => {FirstReturn::Skipper},
    } {
        FirstReturn::Sleeper => {
            return CycleResult::OK;
        }
        FirstReturn::Aborter => {
            return CycleResult::Aborted;
        }
        FirstReturn::Skipper => {
            return CycleResult::Skipped;
        }
    }
}
