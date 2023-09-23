use iced::{
    widget::{Button, Column, Row, Text},
    Sandbox,
};

use crate::data::{e::E, pi::PI};

pub(crate) struct Gui {
    number: Option<&'static str>,
}

impl Sandbox for Gui {
    type Message = GuiMessage;

    fn new() -> Self {
        Gui { number: None }
    }

    fn title(&self) -> String {
        String::from("Irrational Pal")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            GuiMessage::PickedNumber(number) => {
                self.number = Some(number);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let button_e = Button::new(Text::new("e")).on_press(GuiMessage::PickedNumber(E));
        let button_pi = Button::new(Text::new("Pi")).on_press(GuiMessage::PickedNumber(PI));
        let button_browse = Button::new(Text::new("Browse"));
        let button_recite = Button::new(Text::new("Recite"));
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
            col = col.push(self.digit_view());
        }
        col.padding(5).spacing(5).into()
    }
}

impl Gui {
    fn digit_view(&self) -> iced::Element<'_, GuiMessage> {
        let mut row = Row::new();
        let button_left = Button::new(Text::new("<"));
        let button_right = Button::new(Text::new(">"));
        row = row.push(button_left);
        for i in 0..10 {
            row = row.push(Text::new(format!("{}", i)));
        }
        row = row.push(button_right);
        row.padding(5).spacing(5).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    PickedNumber(&'static str),
}
