use super::{Gui, GuiMessage};
use crate::data::extraction::{number_representation, DISPLAYED_DIGITS};
use iced::widget::{Button, Row, Text, TextInput};

impl Gui {
    pub(super) fn first_position_text(&self) -> Text<'_> {
        Text::new(format!("First shown position: {}", self.browse_pos))
    }

    pub(super) fn position_view(&self) -> Row<'_, GuiMessage> {
        let input = TextInput::new("", &self.browse_pos.to_string()).on_input(GuiMessage::TypedPos);
        Row::new()
            .push(Text::new("Jump to position: "))
            .push(input)
            .padding(5)
            .spacing(5)
    }

    pub(super) fn digit_view(&self) -> Row<'_, GuiMessage> {
        let number = self.number.unwrap_or_default();
        let mut button_left = Button::new(Text::new("<"));
        if self.browse_pos > 0 {
            button_left = button_left.on_press(GuiMessage::PosDecrease);
        }
        let button_right = Button::new(Text::new(">")).on_press(GuiMessage::PosIncrease);
        let number_rep = number_representation(number, self.browse_pos + DISPLAYED_DIGITS);

        Row::new()
            .push(button_left)
            .push(Text::new(number_rep))
            .push(button_right)
            .padding(5)
            .spacing(5)
    }
}
