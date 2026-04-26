use crate::settings::{Settings, Stance};

use iced::keyboard::{self, key};
use iced::time::{self, Duration, Instant, milliseconds};
use iced::widget::{
    button, center, column, container, mouse_area, opaque, operation, radio, row, rule, space,
    stack, svg, text,
};
use iced_aw;
use notify_rust::{Notification, Timeout};

#[derive(Debug, Default)]
pub struct App {
    theme: Option<iced::Theme>,
    settings_modal_show: bool,
    settings_modal_fields: Settings,
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
    KeyBoardEvent(keyboard::Event),
    TimerStart,
    TimerStop,
    TimerTick,
    ManualTimerCycleEnd,
    ThemeChanged(Option<iced::Theme>),
    SettingsModalShow,
    SettingsModalHide,
    SettingsSaveAndModalHide,
    SettingSitTimeChanged(u64),
    SettingStandTimeChanged(u64),
    SettingStartStanceChanged(Stance),
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: None,
            settings_modal_show: false,
            settings: Settings {
                sit_duration_as_min: 45,
                stand_duration_as_min: 20,
                start_stance: Stance::default(),
            },
            settings_modal_fields: Settings {
                sit_duration_as_min: 45,
                stand_duration_as_min: 20,
                start_stance: Stance::default(),
            },
            current_timer_cycle: None,
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::KeyBoardEvent(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                } => {
                    if self.settings_modal_show {
                        self.hide_modal();
                    }
                    iced::Task::none()
                }
                _ => iced::Task::none(),
            },
            Message::TimerStart => {
                self.start_new_cycle();
                iced::Task::none()
            }
            Message::TimerStop => {
                self.current_timer_cycle = None;
                iced::Task::none()
            }
            Message::TimerTick => {
                if let Some(cycle_info) = &self.current_timer_cycle {
                    let run_duration = Instant::now() - cycle_info.start_time;
                    if run_duration >= cycle_info.duration {
                        self.start_new_cycle();
                    };
                };
                iced::Task::none()
            }
            Message::ManualTimerCycleEnd => {
                self.start_new_cycle();
                iced::Task::none()
            }
            Message::ThemeChanged(new_theme) => {
                self.theme = new_theme;
                iced::Task::none()
            }
            Message::SettingsModalShow => {
                self.settings_modal_show = true;
                self.reset_modal_fields();
                operation::focus_next()
            }
            Message::SettingsModalHide => {
                self.hide_modal();
                iced::Task::none()
            }
            Message::SettingSitTimeChanged(new_sit_time) => {
                self.settings_modal_fields.sit_duration_as_min = new_sit_time;
                iced::Task::none()
            }
            Message::SettingStandTimeChanged(new_stand_time) => {
                self.settings_modal_fields.stand_duration_as_min = new_stand_time;
                iced::Task::none()
            }
            Message::SettingStartStanceChanged(new_start_stance) => {
                self.settings_modal_fields.start_stance = new_start_stance;
                iced::Task::none()
            }
            Message::SettingsSaveAndModalHide => {
                self.settings.sit_duration_as_min = self.settings_modal_fields.sit_duration_as_min;
                self.settings.stand_duration_as_min =
                    self.settings_modal_fields.stand_duration_as_min;
                self.settings.start_stance = self.settings_modal_fields.start_stance;
                self.hide_modal();
                iced::Task::none()
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let tick = match self.current_timer_cycle {
            None => iced::Subscription::none(),
            Some(_) => time::every(milliseconds(100)).map(|_| Message::TimerTick),
        };

        iced::Subscription::batch(vec![tick, keyboard::listen().map(Message::KeyBoardEvent)])
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        pub const OUTER_PADDING: u16 = 20;
        pub const MAIN_COLUMN_SPACING: u32 = 10;

        pub const HORIZONTAL_RULE_HEIGHT: u32 = 2;

        pub const BUTTON_PADDING: u16 = 10;

        pub const COL_PADDING: u16 = 15;
        pub const COL_SPACING: u32 = 5;

        pub const ROW_PADDING: u16 = 15;
        pub const ROW_SPACING: u32 = 10;

        pub const TEXT_SIZE_HEADING: u32 = 45;
        pub const TEXT_SIZE_NORMAL: u32 = 25;

        let main_heading = text("rustnot")
            .width(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .size(TEXT_SIZE_HEADING);

        let sit_duration = row![
            text("Sit time [min]:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(TEXT_SIZE_NORMAL),
            text!("{}", &self.settings.sit_duration_as_min)
                .align_x(iced::Alignment::End)
                .size(TEXT_SIZE_NORMAL),
        ];

        let stand_duration = row![
            text("Stand time [min]:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(TEXT_SIZE_NORMAL),
            text!("{}", &self.settings.stand_duration_as_min)
                .align_x(iced::Alignment::End)
                .size(TEXT_SIZE_NORMAL),
        ];

        let current_stance_info = row![
            text("Current stance:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(TEXT_SIZE_NORMAL),
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
            .size(TEXT_SIZE_NORMAL)
        ];

        let next_stance_switch_info = row![
            text("Next cycle in:")
                .width(iced::Length::Fill)
                .align_x(iced::Alignment::Start)
                .size(TEXT_SIZE_NORMAL),
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
            .size(TEXT_SIZE_NORMAL)
        ];

        let info_texts = column![
            sit_duration,
            stand_duration,
            current_stance_info,
            next_stance_switch_info,
        ]
        .padding(COL_PADDING)
        .spacing(COL_SPACING);

        let timer_control_btn = (match &self.current_timer_cycle {
            None => button(text("Start timer").align_x(iced::Center)).on_press(Message::TimerStart),
            Some(_) => button(text("Stop timer").align_x(iced::Center))
                .style(button::danger)
                .on_press(Message::TimerStop),
        })
        .padding(BUTTON_PADDING)
        .width(105);

        let stance_switch_btn = button(text("Switch stance now").align_x(iced::Center))
            .on_press_maybe(if self.current_timer_cycle.is_some() {
                Some(Message::ManualTimerCycleEnd)
            } else {
                None
            })
            .padding(BUTTON_PADDING);

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

        let settings_btn = create_icon_btn(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/images/settings.svg"
        ))
        .on_press(Message::SettingsModalShow);

        let top_button_bar = row![space::horizontal(), theme_toggle_btn, settings_btn]
            .width(iced::Length::Fill)
            .padding(iced::Padding {
                top: OUTER_PADDING as f32,
                right: OUTER_PADDING as f32,
                left: OUTER_PADDING as f32,
                bottom: 0.0,
            })
            .spacing(ROW_SPACING)
            .align_y(iced::Alignment::Center);

        let main_content = column![
            main_heading,
            rule::horizontal(HORIZONTAL_RULE_HEIGHT),
            info_texts,
            rule::horizontal(HORIZONTAL_RULE_HEIGHT),
            row![timer_control_btn, space::horizontal(), stance_switch_btn]
                .width(iced::Length::Fill)
                .padding(ROW_PADDING)
                .spacing(ROW_SPACING)
                .align_y(iced::Alignment::Center),
        ]
        .padding(iced::Padding {
            top: 0.0,
            right: OUTER_PADDING as f32,
            left: OUTER_PADDING as f32,
            bottom: OUTER_PADDING as f32,
        })
        .spacing(MAIN_COLUMN_SPACING)
        .align_x(iced::Alignment::Center);

        let main_content = container(column![top_button_bar, main_content].padding(0).spacing(0));

        if self.settings_modal_show {
            let modal_content = container(
                column![
                    text("Settings").size(TEXT_SIZE_HEADING),
                    rule::horizontal(HORIZONTAL_RULE_HEIGHT),
                    column![
                        row![
                            text("Sit time [min]:").size(TEXT_SIZE_NORMAL),
                            iced_aw::number_input(
                                &self.settings_modal_fields.sit_duration_as_min,
                                0..=1440,
                                Message::SettingSitTimeChanged
                            )
                            .step(1)
                            .on_input(Message::SettingSitTimeChanged)
                            .on_submit(Message::SettingsSaveAndModalHide),
                        ],
                        row![
                            text("Stand time [min]:").size(TEXT_SIZE_NORMAL),
                            iced_aw::number_input(
                                &self.settings_modal_fields.stand_duration_as_min,
                                0..=1440,
                                Message::SettingStandTimeChanged
                            )
                            .step(1)
                            .on_input(Message::SettingStandTimeChanged)
                            .on_submit(Message::SettingsSaveAndModalHide),
                        ],
                        column![
                            text("Choose start stance:"),
                            radio(
                                "Sitting",
                                Stance::Sitting,
                                Some(self.settings_modal_fields.start_stance),
                                Message::SettingStartStanceChanged
                            ),
                            radio(
                                "Standing",
                                Stance::Standing,
                                Some(self.settings_modal_fields.start_stance),
                                Message::SettingStartStanceChanged
                            ),
                        ],
                        rule::horizontal(HORIZONTAL_RULE_HEIGHT),
                        row![
                            button(text("Save"))
                                .style(button::success)
                                .on_press(Message::SettingsSaveAndModalHide),
                            space::horizontal(),
                            button(text("Cancel"))
                                .style(button::danger)
                                .on_press(Message::SettingsModalHide),
                        ]
                        .width(iced::Length::Fill)
                        .padding(ROW_PADDING)
                        .spacing(ROW_SPACING)
                        .align_y(iced::Alignment::Center),
                    ]
                    .spacing(10)
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(container::rounded_box);

            modal(main_content, modal_content, Message::SettingsModalHide)
        } else {
            main_content.into()
        }
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        self.theme.clone()
    }
}

impl App {
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

    fn reset_modal_fields(&mut self) {
        self.settings_modal_fields.sit_duration_as_min = self.settings.sit_duration_as_min;
        self.settings_modal_fields.stand_duration_as_min = self.settings.stand_duration_as_min;
        self.settings_modal_fields.start_stance = self.settings.start_stance;
    }

    fn hide_modal(&mut self) {
        self.settings_modal_show = false;
        self.reset_modal_fields();
    }
}

fn modal<'a, Message>(
    base: impl Into<iced::Element<'a, Message>>,
    content: impl Into<iced::Element<'a, Message>>,
    on_blur: Message,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        iced::Color {
                            a: 0.8,
                            ..iced::Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
