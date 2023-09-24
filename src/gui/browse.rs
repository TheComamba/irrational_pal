use iced::widget::{Button, Column, Row, Text, TextInput};

use super::{Gui, GuiMessage};
use crate::data::extraction::number_representation;

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
        let input = TextInput::new("", &self.browse_pos.to_string()).on_input(GuiMessage::TypedPos);
        Column::new()
            .push(Text::new("First displayed position: "))
            .push(input)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn digit_view(&self) -> iced::Element<'_, GuiMessage> {
        let number = self.number.unwrap_or_default();
        let mut button_left = Button::new(Text::new("<"));
        if self.browse_pos > 0 {
            button_left = button_left.on_press(GuiMessage::PosDecrease);
        }
        let button_right = Button::new(Text::new(">")).on_press(GuiMessage::PosIncrease);

        let number_rep = number_representation(number, self.browse_pos + 10);
        Row::new()
            .push(button_left)
            .push(Text::new(number_rep))
            .push(button_right)
            .padding(5)
            .spacing(5)
            .into()
    }
}
