use self::message::GuiMessage;
use crate::data::{e::E, pi::PI, Number};
use iced::{
    widget::{Button, Column, Row, Text},
    Sandbox,
};

mod browse;
mod message;
mod recite;

pub(crate) struct Gui {
    number: Option<&'static Number>,
    mode: GuiMode,
    browse_pos: u32,
    recite_pos: u32,
    wrong_digits: Vec<char>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            number: None,
            mode: GuiMode::Browse,
            browse_pos: 0,
            recite_pos: 0,
            wrong_digits: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Irrational Pal")
    }

    fn update(&mut self, message: Self::Message) {
        self.handle_message(message);
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let button_e = Button::new(Text::new("e")).on_press(GuiMessage::PickedNumber(&E));
        let button_pi = Button::new(Text::new("Pi")).on_press(GuiMessage::PickedNumber(&PI));
        let button_browse =
            Button::new(Text::new("Browse")).on_press(GuiMessage::PickedMode(GuiMode::Browse));
        let button_recite =
            Button::new(Text::new("Recite")).on_press(GuiMessage::PickedMode(GuiMode::Recite));
        let mut col = Column::new().push(
            Row::new()
                .push(button_e)
                .push(button_pi)
                .padding(5)
                .spacing(5),
        );
        if self.number.is_some() {
            col = col.push(
                Row::new()
                    .push(button_browse)
                    .push(button_recite)
                    .padding(5)
                    .spacing(5),
            );
            match self.mode {
                GuiMode::Browse => {
                    col = col
                        .push(self.first_position_text())
                        .push(self.digit_view())
                        .push(self.position_view())
                }
                GuiMode::Recite => {
                    col = col
                        .push(self.next_position_text())
                        .push(self.recited_numbers_text())
                        .push(self.digit_buttons())
                }
            }
        }
        col.padding(5).spacing(5).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMode {
    Browse,
    Recite,
}
