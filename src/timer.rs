use std::sync::Arc;

use parking_lot::Mutex;
use std::sync::mpsc::{Receiver, Sender};
use thiserror::Error;

use crate::config::Stance;
use crate::notification;

#[derive(Error, Debug, Clone)]
pub enum TimerError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FirstReturn {
    Aborter,
    Sleeper,
}

async fn sleeper(waiting_time: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(waiting_time * 60)).await;
}

async fn aborter(rx: Arc<Mutex<Receiver<bool>>>) {
    let _ = rx.lock().recv();
}

pub async fn start_timer(
    sit_time: u32,
    stand_time: u32,
    rx: Receiver<bool>,
) -> Result<(), TimerError> {
    let start_stance = Stance::Sitting;

    let init_waiting_time;
    let init_prompt;
    if start_stance == Stance::Sitting {
        init_waiting_time = sit_time;
        init_prompt = "Timer starts. Please sit Down.";
    } else {
        init_waiting_time = stand_time;
        init_prompt = "Timer starts. Please stand up.";
    }

    notification::send_notification(init_prompt, init_waiting_time);

    let mut current_stance = start_stance;
    let mut waiting_time = init_waiting_time;
    let packed_rx = Arc::new(Mutex::new(rx));

    loop {
        let res: FirstReturn = tokio::select! {
            _ = aborter(packed_rx.clone()) => {FirstReturn::Aborter},
            _ = sleeper(waiting_time as u64) => {FirstReturn::Sleeper},
        };

        if res == FirstReturn::Aborter {
            break;
        }

        match current_stance {
            Stance::Standing => {
                current_stance = Stance::Sitting;
                waiting_time = sit_time;
                notification::send_notification("Sit Down!", waiting_time);
            }
            Stance::Sitting => {
                current_stance = Stance::Standing;
                waiting_time = stand_time;
                notification::send_notification("Stand Up!", waiting_time);
            }
        }
    }

    Ok(())
}

pub async fn stop_timer(tx: Sender<bool>) -> Result<(), TimerError> {
    let _ = tx.send(true);
    Ok(())
}
