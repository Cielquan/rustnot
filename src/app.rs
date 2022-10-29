use std::borrow::BorrowMut;

use iced::{
    alignment, button, text_input, Alignment, Application, Button, Column, Command, Container,
    Element, Length, Radio, Row, Rule, Text, TextInput,
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
    stance_switch_button: button::State,
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
            stance_switch_button: button::State::default(),
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
    SwitchStance,
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
                    self.state.current_stance = new_conf.start_stance;
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
                if self.state.timer_running == false {
                    self.state.current_stance = new_stance;
                }
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
                CycleResult::OK | CycleResult::Skipped => {
                    if res == CycleResult::Skipped {
                        *timer::TIMER_SIGNAL.lock() = timer::TimerSignal::Run;
                    }
                    match self.state.current_stance {
                        Stance::Sitting => self.state.current_stance = Stance::Standing,
                        Stance::Standing => self.state.current_stance = Stance::Sitting,
                    };
                    return Command::perform(async { false }, Message::StartTimerCycle);
                }
                CycleResult::Aborted => {
                    self.state.timer_running = false;
                    self.state.current_stance = self.config.start_stance;
                }
            },
            Message::StopTimer => {
                *timer::TIMER_SIGNAL.lock() = timer::TimerSignal::Abort;
            }
            Message::SwitchStance => {
                if self.state.timer_running == true {
                    *timer::TIMER_SIGNAL.lock() = timer::TimerSignal::Skip;
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        const MAIN_COLUMNS_WIDTH: u16 = 400;
        const MAIN_COLUMNS_PADDING: u16 = 20;
        const MAIN_COLUMNS_SPACING: u16 = 10;

        /// Space left and right of rule
        const VERTICAL_RULE_PADDING: u16 = 0;
        /// Space above and below of rule
        const HORIZONTAL_RULE_PADDING: u16 = 10;

        const BUTTON_PADDING: u16 = 10;

        const ROW_PADDING: u16 = 15;
        const ROW_SPACING: u16 = 10;
        const COL_PADDING: u16 = 15;
        const COL_SPACING: u16 = 10;

        const TEXTBOX_WIDTH: u16 = 50;

        const TEXT_SIZE_HEADING: u16 = 45;
        const TEXT_SIZE_NORMAL: u16 = 30;
        const TEXT_SIZE_SMALL: u16 = 15;

        let config_heading = Text::new("Config")
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(TEXT_SIZE_HEADING);

        let sit_time = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Fill)
            .push(
                Text::new("Sit time [min]:")
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Left)
                    .size(TEXT_SIZE_NORMAL),
            )
            .push(
                TextInput::new(
                    self.state.sit_time.borrow_mut(),
                    "...",
                    &self.config.sit_time.to_string()[..],
                    Message::ConfigValueSitTimeChanged,
                )
                .width(Length::Units(TEXTBOX_WIDTH))
                .size(TEXT_SIZE_NORMAL)
                .style(self.theme),
            );

        let stand_time = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Fill)
            .push(
                Text::new("Stand time [min]:")
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Left)
                    .size(TEXT_SIZE_NORMAL),
            )
            .push(
                TextInput::new(
                    self.state.stand_time.borrow_mut(),
                    "...",
                    &self.config.stand_time.to_string()[..],
                    Message::ConfigValueStandTimeChanged,
                )
                .width(Length::Units(TEXTBOX_WIDTH))
                .size(TEXT_SIZE_NORMAL)
                .style(self.theme),
            );

        let set_times = Column::new()
            .padding(COL_PADDING)
            .spacing(COL_SPACING)
            .width(Length::Fill)
            .push(sit_time)
            .push(stand_time);

        let stance_switch = Column::new()
            .padding(COL_PADDING)
            .spacing(COL_SPACING)
            .push(
                Text::new("Choose a starting stance:")
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Left)
                    .size(TEXT_SIZE_NORMAL),
            )
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
            .padding(ROW_PADDING)
            .width(Length::Fill)
            .align_items(Alignment::Fill)
            .push(
                Text::new("Notification time [sec]:")
                    .width(Length::Fill)
                    .size(TEXT_SIZE_NORMAL),
            )
            .push(
                TextInput::new(
                    self.state.toast_duration.borrow_mut(),
                    "...",
                    &self.config.toast_duration.to_string()[..],
                    Message::ConfigValueToastTimeChanged,
                )
                .width(Length::Units(TEXTBOX_WIDTH))
                .size(TEXT_SIZE_NORMAL)
                .style(self.theme),
            );

        let save_state_text;
        if self.state.config_saved {
            save_state_text = "Config saved";
        } else {
            save_state_text = "Unsaved config";
        }

        let save_btn = Column::new()
            .padding(COL_PADDING)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    &mut self.state.save_button,
                    Text::new("Save config to file"),
                )
                .padding(BUTTON_PADDING)
                .style(self.theme)
                .on_press(Message::SaveConfigToFile),
            )
            .push(Text::new(save_state_text).size(TEXT_SIZE_SMALL));

        let config_column = Column::new()
            .padding(MAIN_COLUMNS_PADDING)
            .spacing(MAIN_COLUMNS_SPACING)
            .width(Length::Units(MAIN_COLUMNS_WIDTH))
            .align_items(Alignment::Center)
            .push(config_heading)
            .push(Rule::horizontal(HORIZONTAL_RULE_PADDING).style(self.theme))
            .push(set_times)
            .push(Rule::horizontal(HORIZONTAL_RULE_PADDING).style(self.theme))
            .push(stance_switch)
            .push(Rule::horizontal(HORIZONTAL_RULE_PADDING).style(self.theme))
            .push(toast_duration)
            .push(Rule::horizontal(HORIZONTAL_RULE_PADDING).style(self.theme))
            .push(save_btn);

        let timer_heading = Text::new("Timer")
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(TEXT_SIZE_HEADING);

        let timer_state_text_content;
        if self.state.timer_running {
            timer_state_text_content = "Running ...";
        } else {
            timer_state_text_content = "Stopped";
        }

        let timer_state_text = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Text::new("Timer:")
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Left)
                    .size(TEXT_SIZE_NORMAL),
            )
            .push(
                Text::new(timer_state_text_content)
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .size(TEXT_SIZE_NORMAL),
            );

        let stance_text = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Text::new("Current stance:")
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Left)
                    .size(TEXT_SIZE_NORMAL),
            )
            .push(
                Text::new(format!("{}", self.state.current_stance))
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .size(TEXT_SIZE_NORMAL),
            );

        let timer_time_text = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Text::new("Next switch in [min]:")
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Left)
                    .size(TEXT_SIZE_NORMAL),
            )
            .push(
                Text::new("69")
                    .horizontal_alignment(alignment::Horizontal::Right)
                    .size(TEXT_SIZE_NORMAL),
            );

        let info_texts = Column::new()
            .padding(COL_PADDING)
            .spacing(COL_SPACING)
            .width(Length::Fill)
            .push(timer_state_text)
            .push(stance_text)
            .push(timer_time_text);

        let timer_controll_btn_text;
        let timer_controll_btn_on_press;
        if self.state.timer_running {
            timer_controll_btn_text = "Stop Timer";
            timer_controll_btn_on_press = Message::StopTimer;
        } else {
            timer_controll_btn_text = "Start Timer";
            timer_controll_btn_on_press = Message::StartTimer;
        }

        let timer_controll_btn = Column::new()
            .padding(COL_PADDING)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    &mut self.state.timer_button,
                    Text::new(timer_controll_btn_text),
                )
                .padding(BUTTON_PADDING)
                .width(Length::Units(105))
                .style(self.theme)
                .on_press(timer_controll_btn_on_press),
            );

        let stance_switch_btn = Column::new()
            .padding(COL_PADDING)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    &mut self.state.stance_switch_button,
                    Text::new("Switch stance now"),
                )
                .padding(BUTTON_PADDING)
                .style(self.theme)
                .on_press(Message::SwitchStance),
            );

        let timer_buttons = Row::new()
            .padding(ROW_PADDING)
            .spacing(ROW_SPACING)
            .width(Length::Fill)
            .push(timer_controll_btn)
            .push(stance_switch_btn);

        let timer_column = Column::new()
            .padding(MAIN_COLUMNS_PADDING)
            .spacing(MAIN_COLUMNS_SPACING)
            .width(Length::Units(MAIN_COLUMNS_WIDTH))
            .align_items(Alignment::Center)
            .push(timer_heading)
            .push(Rule::horizontal(HORIZONTAL_RULE_PADDING).style(self.theme))
            .push(info_texts)
            .push(Rule::horizontal(HORIZONTAL_RULE_PADDING).style(self.theme))
            .push(timer_buttons);

        let content = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Start)
            .push(timer_column)
            .push(Rule::vertical(VERTICAL_RULE_PADDING).style(self.theme))
            .push(config_column);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
