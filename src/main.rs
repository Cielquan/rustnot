use std::fmt::Display;

use iced::time::{self, Duration, Instant, milliseconds};
use iced::widget::{Column, button, column, row, text};

pub fn main() -> iced::Result {
    iced::application(RustNot::new, RustNot::update, RustNot::view)
        .subscription(RustNot::subscription)
        .run()
}

const CYCLE_DURATION_AS_MIN: u64 = 1;

#[derive(Debug, Default)]
struct RustNot {
    timer_state: TimerState,
}

#[derive(Debug, Default, Clone, Copy)]
enum TimerState {
    #[default]
    Stopped,
    Running(TimerCycleInfo),
}

impl Display for TimerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerState::Stopped => write!(f, "{}", "Stopped"),
            TimerState::Running(_) => write!(f, "{}", "Running"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TimerCycleInfo {
    start_time: Instant,
    duration: Duration,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TimerStart,
    TimerStop,
    TimerTick,
}

impl RustNot {
    fn new() -> Self {
        Self {
            timer_state: TimerState::default(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TimerStart => {
                self.start_new_cycle();
            }
            Message::TimerStop => {
                self.timer_state = TimerState::Stopped;
            }
            Message::TimerTick => match &self.timer_state {
                TimerState::Running(cycle_info) => {
                    let run_duration = Instant::now() - cycle_info.start_time;
                    if run_duration >= cycle_info.duration {
                        self.start_new_cycle();
                    };
                }
                TimerState::Stopped => {}
            },
        }
    }

    fn start_new_cycle(&mut self) {
        self.timer_state = TimerState::Running(TimerCycleInfo {
            start_time: Instant::now(),
            duration: Duration::from_mins(CYCLE_DURATION_AS_MIN),
        });
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let tick = match self.timer_state {
            TimerState::Stopped => iced::Subscription::none(),
            TimerState::Running(_) => time::every(milliseconds(100)).map(|_| Message::TimerTick),
        };

        iced::Subscription::batch(vec![tick])
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            row![
                text("Timer status: "),
                text(format!("{}", &self.timer_state)),
            ],
            row![
                text("Time till cycle end: "),
                text(match &self.timer_state {
                    TimerState::Running(cycle_info) => {
                        const MINUTE: u64 = 60;
                        const HOUR: u64 = 60 * MINUTE;

                        let run_duration = Instant::now() - cycle_info.start_time;
                        let displayed_duration_as_sec = if run_duration < cycle_info.duration {
                            (cycle_info.duration - run_duration).as_secs()
                        } else {
                            0
                        };
                        format!(
                            "{:0>2}:{:0>2}:{:0>2}",
                            displayed_duration_as_sec / HOUR,
                            (displayed_duration_as_sec % HOUR) / MINUTE,
                            displayed_duration_as_sec % MINUTE,
                        )
                    }
                    TimerState::Stopped => "-".to_string(),
                }),
            ],
            match &self.timer_state {
                TimerState::Running(_) => button(text("Stop").align_x(iced::Center))
                    .on_press(Message::TimerStop)
                    .padding(10)
                    .width(80),
                TimerState::Stopped => button(text("Start").align_x(iced::Center))
                    .on_press(Message::TimerStart)
                    .padding(10)
                    .width(80),
            },
        ]
    }
}
