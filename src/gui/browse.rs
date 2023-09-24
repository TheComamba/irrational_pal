use iced::widget::{Button, Column, Row, Text, TextInput};

use super::{Gui, GuiMessage};
use crate::data::extraction::get_digits;

impl Gui {
    pub(super) fn browse_view(&self) -> iced::Element<'_, GuiMessage> {
        Column::new()
            .push(self.position_view())
            .push(self.digit_view())
            .padding(5)
            .spacing(5)
            .into()
    }

    fn position_view(&self) -> iced::Element<'_, GuiMessage> {
        let input = TextInput::new("", &self.browse_pos.to_string())
            .on_input(|inp| GuiMessage::TypedPos(inp));
        Column::new()
            .push(Text::new("First displayed position: "))
            .push(input)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn digit_view(&self) -> iced::Element<'_, GuiMessage> {
        let number = self.number.unwrap_or_default();
        let shown_digits = get_digits(number, self.browse_pos, 10);
        let mut row = Row::new();
        let mut button_left = Button::new(Text::new("<"));
        if self.browse_pos > 0 {
            button_left = button_left.on_press(GuiMessage::PosDecrease);
        }
        let button_right = Button::new(Text::new(">")).on_press(GuiMessage::PosIncrease);
        row = row.push(button_left);
        for d in shown_digits.iter() {
            row = row.push(Text::new(d.to_string()));
        }
        row = row.push(button_right);
        row.padding(5).spacing(5).into()
    }
}
