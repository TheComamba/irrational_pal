use super::{Gui, GuiMessage};
use crate::data::extraction::number_representation;
use iced::widget::{Button, Row, Text};

impl Gui {
    pub(super) fn next_position_text(&self) -> Text<'_> {
        Text::new(format!("Next position: {}", self.recite_pos))
    }

    pub(super) fn recited_numbers_text(&self) -> Text<'_> {
        let number_str = match self.number {
            Some(n) => number_representation(n, self.recite_pos),
            None => String::from(""),
        };
        Text::new(number_str)
    }

    pub(super) fn digit_buttons(&self) -> Row<'_, GuiMessage> {
        let mut row = Row::new();
        for d in 0..10 {
            let digit: char = d.to_string().chars().next().unwrap();
            let mut button = Button::new(Text::new(d.to_string()));
            if !self.wrong_digits.contains(&digit) {
                button = button.on_press(GuiMessage::ReciteDigit(digit));
            }
            row = row.push(button);
        }
        row.padding(5).spacing(5)
    }
}
