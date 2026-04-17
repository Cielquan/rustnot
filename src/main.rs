use iced::time::{self, Duration, Instant, milliseconds};
use iced::widget::{button, column, row, text};
use notify_rust::{Notification, Timeout};

pub fn main() -> iced::Result {
    iced::application(RustNot::new, RustNot::update, RustNot::view)
        .subscription(RustNot::subscription)
        .run()
}

#[derive(Debug, Default)]
struct RustNot {
    settings: Settings,
    current_timer_cycle: Option<TimerCycleInfo>,
}

#[derive(Debug, Default)]
struct Settings {
    sit_duration_as_min: u64,
    stand_duration_as_min: u64,
    start_stance: Stance,
}

impl Settings {
    fn get_duration_for_stance(&self, stance: &Stance) -> u64 {
        match stance {
            Stance::Sitting => self.sit_duration_as_min,
            Stance::Standing => self.stand_duration_as_min,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Stance {
    #[default]
    Sitting,
    Standing,
}

impl Stance {
    fn inverted(current: Stance) -> Self {
        match current {
            Stance::Sitting => Stance::Standing,
            Stance::Standing => Stance::Sitting,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TimerCycleInfo {
    start_time: Instant,
    duration: Duration,
    stace: Stance,
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
            settings: Settings {
                sit_duration_as_min: 1,
                stand_duration_as_min: 2,
                start_stance: Stance::default(),
            },
            current_timer_cycle: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TimerStart => {
                self.start_new_cycle();
            }
            Message::TimerStop => {
                self.current_timer_cycle = None;
            }
            Message::TimerTick => {
                if let Some(cycle_info) = &self.current_timer_cycle {
                    let run_duration = Instant::now() - cycle_info.start_time;
                    if run_duration >= cycle_info.duration {
                        self.start_new_cycle();
                    };
                }
            }
        }
    }

    fn start_new_cycle(&mut self) {
        match self.current_timer_cycle {
            None => {
                self.current_timer_cycle = Some(TimerCycleInfo {
                    start_time: Instant::now(),
                    duration: Duration::from_mins(
                        self.settings
                            .get_duration_for_stance(&self.settings.start_stance),
                    ),
                    stace: self.settings.start_stance,
                });
            }
            Some(cycle_info) => {
                let new_cycle_stance = Stance::inverted(cycle_info.stace);
                let new_cycle_duration =
                    Duration::from_mins(self.settings.get_duration_for_stance(&new_cycle_stance));
                self.current_timer_cycle = Some(TimerCycleInfo {
                    start_time: Instant::now(),
                    duration: new_cycle_duration,
                    stace: new_cycle_stance,
                });

                Notification::new()
                    .summary(match new_cycle_stance {
                        Stance::Sitting => "Please sit Down.",
                        Stance::Standing => "Please stand up.",
                    })
                    .body(&format!(
                        "It's time to change your stance.\nNext reminder in: {} min.",
                        new_cycle_duration.as_secs() / 60
                    ))
                    .sound_name("dialog-information")
                    .timeout(Timeout::Milliseconds(10 * 1000))
                    .show()
                    .expect("unable to toast");
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let tick = match self.current_timer_cycle {
            None => iced::Subscription::none(),
            Some(_) => time::every(milliseconds(100)).map(|_| Message::TimerTick),
        };

        iced::Subscription::batch(vec![tick])
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            row![
                text("Sit duration [min]: "),
                text(&self.settings.sit_duration_as_min),
            ],
            row![
                text("Stand duration [min]: "),
                text(&self.settings.stand_duration_as_min),
            ],
            row![
                text("Timer status: "),
                text(if self.current_timer_cycle.is_some() {
                    "Running"
                } else {
                    "Stopped"
                }),
            ],
            row![
                text("Time till cycle end: "),
                text(match &self.current_timer_cycle {
                    Some(cycle_info) => {
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
                    None => "-".to_string(),
                }),
            ],
            match &self.current_timer_cycle {
                Some(_) => button(text("Stop").align_x(iced::Center))
                    .on_press(Message::TimerStop)
                    .padding(10)
                    .width(80),
                None => button(text("Start").align_x(iced::Center))
                    .on_press(Message::TimerStart)
                    .padding(10)
                    .width(80),
            },
        ]
        .into()
    }
}
