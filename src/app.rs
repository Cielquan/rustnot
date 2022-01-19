use std::borrow::BorrowMut;

use iced::{
    button, text_input, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, Radio, Row, Rule, Text, TextInput,
};

use parking_lot::Mutex;
use std::sync::mpsc::{channel, Sender};

use crate::config::{self, Stance};
use crate::{style, timer};

pub struct App {
    config: config::Config,
    theme: style::Theme,
    state: State,
    timer_handle: Mutex<Option<Sender<bool>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: config::Config::default(),
            theme: style::Theme::default(),
            state: State::default(),
            timer_handle: Mutex::new(None),
        }
    }
}

struct State {
    sit_time: text_input::State,
    stand_time: text_input::State,
    save_button: button::State,
    config_saved: bool,
    timer_button: button::State,
    timer_running: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sit_time: text_input::State::default(),
            stand_time: text_input::State::default(),
            save_button: button::State::default(),
            config_saved: false,
            timer_button: button::State::default(),
            timer_running: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SitTimeChanged(String),
    StandTimeChanged(String),
    StanceChanged(Stance),
    ConfigLoaded(Result<config::Config, config::ConfigFileError>),
    SaveConfig,
    ConfigSaved(Result<(), config::ConfigFileError>),
    StartTimer,
    TimerStopped(Result<(), timer::TimerError>),
    StopTimer,
}

impl<'a> Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(config::Config::load_from_file(), Message::ConfigLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("rustnot")
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
        match message {
            Message::SitTimeChanged(input) => {
                if let Ok(dur) = input.parse::<u32>() {
                    self.config.sit_time = dur;
                    self.state.config_saved = false;
                }
            }
            Message::StandTimeChanged(input) => {
                if let Ok(dur) = input.parse::<u32>() {
                    self.config.stand_time = dur;
                    self.state.config_saved = false;
                }
            }
            Message::StanceChanged(new_stance) => {
                self.config.start_stance = new_stance;
                self.state.config_saved = false;
            }
            Message::ConfigLoaded(res) => {
                if let Ok(new_conf) = res {
                    self.config = new_conf;
                    self.state.config_saved = true;
                }
            }
            Message::SaveConfig => {
                return Command::perform(
                    config::Config::save_to_file(self.config.clone()),
                    Message::ConfigSaved,
                );
            }
            Message::ConfigSaved(res) => {
                if res.is_ok() {
                    self.state.config_saved = true;
                } else {
                    self.state.config_saved = false;
                }
            }
            Message::StartTimer => {
                let (tx, rx) = channel();
                *self.timer_handle.lock() = Some(tx);
                self.state.timer_running = true;
                return Command::perform(
                    timer::start_timer(
                        self.config.sit_time.clone(),
                        self.config.stand_time.clone(),
                        self.config.start_stance.clone(),
                        rx,
                    ),
                    Message::TimerStopped,
                );
            }
            Message::TimerStopped(res) => {
                if res.is_ok() {
                    self.state.timer_running = false;
                }
            }
            Message::StopTimer => {
                let mut timer_handle_guard = self.timer_handle.lock();
                if timer_handle_guard.is_some() {
                    let tx = std::mem::replace(&mut *timer_handle_guard, None).unwrap();
                    return Command::perform(timer::stop_timer(tx), Message::TimerStopped);
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        const LABLE_WIDTH: u16 = 170;
        const TEXT_SIZE: u16 = 30;
        const PADDING: u16 = 10;

        let sit_time = Row::new()
            .padding(PADDING)
            .push(
                Text::new("Sit Time:")
                    .width(Length::Units(LABLE_WIDTH))
                    .size(TEXT_SIZE),
            )
            .push(
                TextInput::new(
                    self.state.sit_time.borrow_mut(),
                    "Sit time",
                    &self.config.sit_time.to_string()[..],
                    Message::SitTimeChanged,
                )
                .size(TEXT_SIZE)
                .style(self.theme),
            );

        let stand_time = Row::new()
            .padding(PADDING)
            .push(
                Text::new("Stand Time:")
                    .width(Length::Units(LABLE_WIDTH))
                    .size(TEXT_SIZE),
            )
            .push(
                TextInput::new(
                    self.state.stand_time.borrow_mut(),
                    "Stand time",
                    &self.config.stand_time.to_string()[..],
                    Message::StandTimeChanged,
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
                    Message::StanceChanged,
                )
                .style(self.theme),
            )
            .push(
                Radio::new(
                    Stance::Standing,
                    format!("{:?}", Stance::Standing),
                    Some(self.config.start_stance),
                    Message::StanceChanged,
                )
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
            .align_items(Align::Center)
            .push(
                Button::new(
                    &mut self.state.save_button,
                    Text::new("Save config to file"),
                )
                .padding(PADDING)
                .style(self.theme)
                .on_press(Message::SaveConfig),
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
            .align_items(Align::Center)
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
            .align_items(Align::Center)
            .push(sit_time)
            .push(stand_time)
            .push(Rule::horizontal(PADDING).style(self.theme))
            .push(stance_switch)
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
