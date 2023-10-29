#![allow(dead_code)]

use data::node::AppState;
use iced::{self, executor, widget::Text, Application, Command};

pub fn run_ui(app_state: AppState) -> Result<(), iced::Error> {
    let settings = iced::Settings::with_flags(app_state);

    Ui::run(settings)
}
pub struct Ui {
    app_state: AppState,
}

#[derive(Debug, Clone)]
pub enum AppMsg {}

impl Application for Ui {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Theme = iced::Theme;
    type Flags = AppState;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let ui_state = Ui { app_state: flags };

        (ui_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("App")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Text::new("hello").into()
    }
}
