use crate::settings::{Settings, Stance};
use crate::style;

use iced::time::{self, Duration, Instant, milliseconds};
use iced::widget::{button, column, row, rule, space, svg, text};
use notify_rust::{Notification, Timeout};

#[derive(Debug, Default)]
pub struct App {
    theme: Option<iced::Theme>,
    settings: Settings,
    current_timer_cycle: Option<TimerCycleInfo>,
}

#[derive(Debug, Clone, Copy)]
struct TimerCycleInfo {
    start_time: Instant,
    duration: Duration,
    stace: Stance,
}

#[derive(Debug, Clone)]
pub enum Message {
    TimerStart,
    TimerStop,
    TimerTick,
    ManualTimerCycleEnd,
    ThemeChanged(Option<iced::Theme>),
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: None,
            settings: Settings {
                sit_duration_as_min: 1,
                stand_duration_as_min: 2,
                start_stance: Stance::default(),
            },
            current_timer_cycle: None,
        }
    }

    pub fn update(&mut self, message: Message) {
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
            Message::ManualTimerCycleEnd => {
                self.start_new_cycle();
            }
            Message::ThemeChanged(new_theme) => self.theme = new_theme,
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

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let tick = match self.current_timer_cycle {
            None => iced::Subscription::none(),
            Some(_) => time::every(milliseconds(100)).map(|_| Message::TimerTick),
        };

        iced::Subscription::batch(vec![tick])
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let main_heading = text("rustnot")
            .width(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .size(style::TEXT_SIZE_HEADING);

        let sit_duration = row![
            text("Sit time [min]:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(style::TEXT_SIZE_NORMAL),
            text!("{}", &self.settings.sit_duration_as_min)
                .align_x(iced::Alignment::End)
                .size(style::TEXT_SIZE_NORMAL),
        ];

        let stand_duration = row![
            text("Stand time [min]:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(style::TEXT_SIZE_NORMAL),
            text!("{}", &self.settings.stand_duration_as_min)
                .align_x(iced::Alignment::End)
                .size(style::TEXT_SIZE_NORMAL),
        ];

        let current_stance_info = row![
            text("Current stance:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(style::TEXT_SIZE_NORMAL),
            text(
                match if let Some(current_cycle) = &self.current_timer_cycle {
                    &current_cycle.stace
                } else {
                    &self.settings.start_stance
                } {
                    Stance::Sitting => "Sitting",
                    Stance::Standing => "Standing",
                },
            )
            .align_x(iced::Alignment::End)
            .size(style::TEXT_SIZE_NORMAL)
        ];

        let next_stance_switch_info = row![
            text("Next cycle in:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(style::TEXT_SIZE_NORMAL),
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
            })
            .align_x(iced::Alignment::End)
            .size(style::TEXT_SIZE_NORMAL)
        ];

        let info_texts = column![
            sit_duration,
            stand_duration,
            current_stance_info,
            next_stance_switch_info,
        ]
        .padding(style::COL_PADDING)
        .spacing(style::COL_SPACING);

        let timer_control_btn = (match &self.current_timer_cycle {
            None => button(text("Start timer").align_x(iced::Center)).on_press(Message::TimerStart),
            Some(_) => button(text("Stop timer").align_x(iced::Center))
                .style(button::danger)
                .on_press(Message::TimerStop),
        })
        .padding(style::BUTTON_PADDING)
        .width(105);

        let stance_switch_btn = button(text("Switch stance now").align_x(iced::Center))
            .on_press_maybe(if self.current_timer_cycle.is_some() {
                Some(Message::ManualTimerCycleEnd)
            } else {
                None
            })
            .padding(style::BUTTON_PADDING);

        let create_icon_btn = |file_path: &str| {
            button(svg(file_path).content_fit(iced::ContentFit::Contain).style(
                |theme: &iced::Theme, _style| svg::Style {
                    color: Some(theme.palette().text),
                },
            ))
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .padding(7)
            .style(button::background)
        };

        let theme_toggle_btn = match self.theme {
            Some(iced::Theme::Dark) => create_icon_btn(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/images/sun.svg"
            ))
            .on_press(Message::ThemeChanged(Some(iced::Theme::Light))),

            Some(iced::Theme::Light) => create_icon_btn(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/images/sun-moon.svg"
            ))
            .on_press(Message::ThemeChanged(None)),

            None | _ => create_icon_btn(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/images/moon.svg"
            ))
            .on_press(Message::ThemeChanged(Some(iced::Theme::Dark))),
        };

        let top_button_bar = row![space().width(iced::Length::Fill), theme_toggle_btn,]
            .width(iced::Length::Fill)
            .padding(iced::Padding {
                top: style::OUTER_PADDING as f32,
                right: style::OUTER_PADDING as f32,
                left: style::OUTER_PADDING as f32,
                bottom: 0.0,
            })
            .spacing(style::ROW_SPACING)
            .align_y(iced::Alignment::Center);

        let main_content = column![
            main_heading,
            rule::horizontal(style::HORIZONTAL_RULE_HEIGHT),
            info_texts,
            rule::horizontal(style::HORIZONTAL_RULE_HEIGHT),
            row![
                timer_control_btn,
                space().width(iced::Length::Fill),
                stance_switch_btn
            ]
            .width(iced::Length::Fill)
            .padding(style::ROW_PADDING)
            .spacing(style::ROW_SPACING)
            .align_y(iced::Alignment::Center),
        ]
        .padding(iced::Padding {
            top: 0.0,
            right: style::OUTER_PADDING as f32,
            left: style::OUTER_PADDING as f32,
            bottom: style::OUTER_PADDING as f32,
        })
        .spacing(style::MAIN_COLUMN_SPACING)
        .align_x(iced::Alignment::Center);

        column![top_button_bar, main_content]
            .padding(0)
            .spacing(0)
            .into()
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        self.theme.clone()
    }
}
