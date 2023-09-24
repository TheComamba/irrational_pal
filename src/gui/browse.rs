use iced::widget::{Button, Row, Text, TextInput};

use crate::data::extraction::get_digits;

use super::{Gui, GuiMessage};

impl Gui {
    pub(super) fn position_view(&self) -> iced::Element<'_, GuiMessage> {
        let input =
            TextInput::new("", &self.pos.to_string()).on_input(|inp| GuiMessage::TypedPos(inp));
        Row::new()
            .push(Text::new("Starting at: "))
            .push(input)
            .padding(5)
            .spacing(5)
            .into()
    }

    pub(super) fn handle_pos_input(&mut self, input: String) {
        self.pos = 0;
    }

    pub(super) fn digit_view(&self) -> iced::Element<'_, GuiMessage> {
        let number = self.number.unwrap_or("");
        let shown_digits = get_digits(number, self.pos, 10);
        let mut row = Row::new();
        let mut button_left = Button::new(Text::new("<"));
        if self.pos > 0 {
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
