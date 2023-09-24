use std::cmp::min;

use super::{Gui, GuiMessage};
use crate::data::extraction::get_digits;
use iced::widget::{Column, Text};

impl Gui {
    pub(super) fn recite_view(&self) -> iced::Element<'_, GuiMessage> {
        Column::new()
            .push(Text::new(format!("Next position: {}", self.recite_pos)))
            .push(Text::new(self.number_text()))
            .padding(5)
            .spacing(5)
            .into()
    }

    fn number_text(&self) -> String {
        let number = match self.number {
            Some(n) => n,
            None => return String::from(""),
        };

        let name = number.name;
        let amount = min(self.recite_pos, 10);
        let start = self.recite_pos - amount;
        let mut recited_digits = get_digits(number, start, amount);
        if start == 0 && !recited_digits.is_empty() {
            recited_digits.insert(1, '.');
        } else if start > 0 {
            for _ in 0..3 {
                recited_digits.insert(0, '.');
            }
        }
        let recited_digits = String::from(recited_digits.iter().collect::<String>());
        format!("{}={}", name, recited_digits)
    }
}
