use iced::widget::{Column, Text};

use super::{Gui, GuiMessage};

impl Gui {
    pub(super) fn recite_view(&self) -> iced::Element<'_, GuiMessage> {
        Column::new()
            .push(Text::new(format!("Next position: {}", self.browse_pos)))
            .push(Text::new(self.number_text()))
            .padding(5)
            .spacing(5)
            .into()
    }

    fn number_text(&self) -> String {
        format!("TODO=TODO")
    }
}
