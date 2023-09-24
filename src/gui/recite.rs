use super::{Gui, GuiMessage};
use crate::data::extraction::number_representation;
use iced::widget::{Button, Column, Row, Text};

impl Gui {
    pub(super) fn recite_view(&self) -> iced::Element<'_, GuiMessage> {
        let number_str = match self.number {
            Some(n) => number_representation(n, self.recite_pos),
            None => String::from(""),
        };
        Column::new()
            .push(Text::new(format!("Next position: {}", self.recite_pos)))
            .push(Text::new(number_str))
            .push(self.digit_buttons())
            .padding(5)
            .spacing(5)
            .into()
    }

    fn digit_buttons(&self) -> iced::Element<'_, GuiMessage> {
        let mut row = Row::new();
        for d in 0..10 {
            let digit: char = d.to_string().chars().next().unwrap();
            let mut button = Button::new(Text::new(d.to_string()));
            if !self.wrong_digits.contains(&digit) {
                button = button.on_press(GuiMessage::ReciteDigit(digit));
            }
            row = row.push(button);
        }
        row.padding(5).spacing(5).into()
    }
}
