use std::borrow::BorrowMut;

use iced::{
    button, text_input, Alignment, Application, Button, Column, Command, Container, Element,
    Length, Radio, Row, Rule, Text, TextInput,
};

use crate::config::{self, Stance};
use crate::timer::CycleResult;
use crate::{style, timer};

pub struct App {
    config: config::Config,
    theme: style::Theme,
    state: State,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: config::Config::default(),
            theme: style::Theme::default(),
            state: State::default(),
        }
    }
}

struct State {
    sit_time: text_input::State,
    stand_time: text_input::State,
    toast_duration: text_input::State,
    save_button: button::State,
    config_saved: bool,
    timer_button: button::State,
    timer_running: bool,
    current_stance: Stance,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sit_time: text_input::State::default(),
            stand_time: text_input::State::default(),
            toast_duration: text_input::State::default(),
            save_button: button::State::default(),
            config_saved: false,
            timer_button: button::State::default(),
            timer_running: false,
            current_stance: Stance::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ConfigFileLoaded(Result<config::Config, config::ConfigFileError>),
    SaveConfigToFile,
    ConfigSavedToFile(Result<(), config::ConfigFileError>),
    ConfigValueSitTimeChanged(String),
    ConfigValueStandTimeChanged(String),
    ConfigValueStanceChanged(Stance),
    ConfigValueToastTimeChanged(String),
    StartTimer,
    StopTimer,
    StartTimerCycle(bool),
    TimerCycleFinished(timer::CycleResult),
}

impl<'a> Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(config::Config::load_from_file(), Message::ConfigFileLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("rustnot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConfigFileLoaded(res) => {
                if let Ok(new_conf) = res {
                    self.config = new_conf;
                    self.state.config_saved = true;
                }
            }
            Message::SaveConfigToFile => {
                return Command::perform(
                    config::Config::save_to_file(self.config.clone()),
                    Message::ConfigSavedToFile,
                );
            }
            Message::ConfigSavedToFile(res) => {
                if res.is_ok() {
                    self.state.config_saved = true;
                } else {
                    self.state.config_saved = false;
                }
            }
            Message::ConfigValueSitTimeChanged(input) => {
                if let Ok(dur) = input.parse::<u32>() {
                    self.config.sit_time = dur;
                    self.state.config_saved = false;
                }
            }
            Message::ConfigValueStandTimeChanged(input) => {
                if let Ok(dur) = input.parse::<u32>() {
                    self.config.stand_time = dur;
                    self.state.config_saved = false;
                }
            }
            Message::ConfigValueStanceChanged(new_stance) => {
                self.config.start_stance = new_stance;
                self.state.config_saved = false;
            }
            Message::ConfigValueToastTimeChanged(input) => {
                if let Ok(dur) = input.parse::<u32>() {
                    self.config.toast_duration = dur;
                    self.state.config_saved = false;
                }
            }
            Message::StartTimer => {
                self.state.timer_running = true;
                *timer::TIMER_SIGNAL.lock() = timer::TimerSignal::Run;
                self.state.current_stance = self.config.start_stance.clone();
                return Command::perform(async { true }, Message::StartTimerCycle);
            }
            Message::StartTimerCycle(is_init) => {
                return Command::perform(
                    timer::run_cycle_timer(
                        self.state.current_stance.clone(),
                        if self.state.current_stance == Stance::Sitting {
                            self.config.sit_time
                        } else {
                            self.config.stand_time
                        },
                        is_init,
                        self.config.toast_duration.clone(),
                    ),
                    Message::TimerCycleFinished,
                );
            }
            Message::TimerCycleFinished(res) => match res {
                CycleResult::Aborted => self.state.timer_running = false,
                CycleResult::OK => {
                    match self.state.current_stance {
                        Stance::Sitting => self.state.current_stance = Stance::Standing,
                        Stance::Standing => self.state.current_stance = Stance::Sitting,
                    };
                    return Command::perform(async { false }, Message::StartTimerCycle);
                }
            },
            Message::StopTimer => {
                *timer::TIMER_SIGNAL.lock() = timer::TimerSignal::Abort;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        const LABLE_WIDTH: u16 = 240;
        const TEXT_SIZE: u16 = 30;
        const PADDING: u16 = 10;

        let sit_time = Row::new()
            .padding(PADDING)
            .push(
                Text::new("Sit Time (min):")
                    .width(Length::Units(LABLE_WIDTH))
                    .size(TEXT_SIZE),
            )
            .push(
                TextInput::new(
                    self.state.sit_time.borrow_mut(),
                    "Sit time",
                    &self.config.sit_time.to_string()[..],
                    Message::ConfigValueSitTimeChanged,
                )
                .size(TEXT_SIZE)
                .style(self.theme),
            );

        let stand_time = Row::new()
            .padding(PADDING)
            .push(
                Text::new("Stand Time (min):")
                    .width(Length::Units(LABLE_WIDTH))
                    .size(TEXT_SIZE),
            )
            .push(
                TextInput::new(
                    self.state.stand_time.borrow_mut(),
                    "Stand time",
                    &self.config.stand_time.to_string()[..],
                    Message::ConfigValueStandTimeChanged,
                )
                .size(TEXT_SIZE)
                .style(self.theme),
            );

        let stance_switch = Column::new()
            .spacing(PADDING / 2)
            .padding(PADDING)
            .push(Text::new("Choose a starting stance:"))
            .push(
                Radio::new(
                    Stance::Sitting,
                    format!("{:?}", Stance::Sitting),
                    Some(self.config.start_stance),
                    Message::ConfigValueStanceChanged,
                )
                .style(self.theme),
            )
            .push(
                Radio::new(
                    Stance::Standing,
                    format!("{:?}", Stance::Standing),
                    Some(self.config.start_stance),
                    Message::ConfigValueStanceChanged,
                )
                .style(self.theme),
            );

        let toast_duration = Row::new()
            .padding(PADDING)
            .push(
                Text::new("Notification duration (sec):")
                    .width(Length::Units(LABLE_WIDTH))
                    .size(TEXT_SIZE),
            )
            .push(
                TextInput::new(
                    self.state.toast_duration.borrow_mut(),
                    "Notification duration",
                    &self.config.toast_duration.to_string()[..],
                    Message::ConfigValueToastTimeChanged,
                )
                .size(TEXT_SIZE)
                .style(self.theme),
            );

        let save_state_text;
        if self.state.config_saved {
            save_state_text = "Config saved";
        } else {
            save_state_text = "Unsaved config";
        }

        let save_btn = Column::new()
            .padding(PADDING)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    &mut self.state.save_button,
                    Text::new("Save config to file"),
                )
                .padding(PADDING)
                .style(self.theme)
                .on_press(Message::SaveConfigToFile),
            )
            .push(Text::new(save_state_text).size(TEXT_SIZE / 2));

        let btn_text;
        let btn_on_press;
        let timer_state_text;
        if self.state.timer_running {
            btn_text = "Stop Timer";
            btn_on_press = Message::StopTimer;
            timer_state_text = "Running ...";
        } else {
            btn_text = "Start Timer";
            btn_on_press = Message::StartTimer;
            timer_state_text = "Stopped";
        }

        let timer_btn = Column::new()
            .padding(PADDING)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.state.timer_button, Text::new(btn_text))
                    .padding(PADDING)
                    .style(self.theme)
                    .on_press(btn_on_press),
            )
            .push(Text::new(timer_state_text).size(TEXT_SIZE / 2));

        let content = Column::new()
            .spacing(PADDING)
            .padding(PADDING)
            .align_items(Alignment::Center)
            .push(sit_time)
            .push(stand_time)
            .push(Rule::horizontal(PADDING).style(self.theme))
            .push(stance_switch)
            .push(Rule::horizontal(PADDING).style(self.theme))
            .push(toast_duration)
            .push(Rule::horizontal(PADDING).style(self.theme))
            .push(save_btn)
            .push(Rule::horizontal(PADDING).style(self.theme))
            .push(timer_btn);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
