use iced::{
    widget::{Button, Column, Row, Text},
    Sandbox,
};

use crate::data::{e::E, pi::PI};

mod browse;
mod recite;

pub(crate) struct Gui {
    number: Option<&'static str>,
    mode: GuiMode,
    browse_pos: u32,
    recite_pos: u32,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui {
            number: None,
            mode: GuiMode::Browse,
            browse_pos: 0,
            recite_pos: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Irrational Pal")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::PickedNumber(number) => self.number = Some(number),
            GuiMessage::PickedMode(mode) => self.mode = mode,
            GuiMessage::TypedPos(pos) => self.handle_pos_input(pos),
            GuiMessage::PosDecrease => {
                if self.browse_pos > 0 {
                    self.browse_pos -= 1;
                }
            }
            GuiMessage::PosIncrease => self.browse_pos += 1,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let button_e = Button::new(Text::new("e")).on_press(GuiMessage::PickedNumber(E));
        let button_pi = Button::new(Text::new("Pi")).on_press(GuiMessage::PickedNumber(PI));
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
                GuiMode::Browse => col = col.push(self.browse_view()),
                GuiMode::Recite => col = col.push(self.recite_view()),
            }
        }
        col.padding(5).spacing(5).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    PickedNumber(&'static str),
    PickedMode(GuiMode),
    TypedPos(String),
    PosDecrease,
    PosIncrease,
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMode {
    Browse,
    Recite,
}
