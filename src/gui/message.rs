use super::{Gui, GuiMode};

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    PickedNumber(&'static str),
    PickedMode(GuiMode),
    TypedPos(String),
    PosDecrease,
    PosIncrease,
}

impl Gui {
    pub(super) fn handle_message(&mut self, message: GuiMessage) {
        match message {
            GuiMessage::PickedNumber(number) => self.handle_number_pick(number),
            GuiMessage::PickedMode(mode) => self.hanlde_mode_pick(mode),
            GuiMessage::TypedPos(pos) => self.handle_pos_input(pos),
            GuiMessage::PosDecrease => self.handle_pos_decrease(),
            GuiMessage::PosIncrease => self.handle_pos_increase(),
        }
    }

    fn handle_number_pick(&mut self, number: &'static str) {
        self.number = Some(number);
        self.browse_pos = 0;
        self.recite_pos = 0;
    }

    fn hanlde_mode_pick(&mut self, mode: GuiMode) {
        self.mode = mode;
    }

    fn handle_pos_input(&mut self, input: String) {
        self.browse_pos = 0;
    }

    fn handle_pos_decrease(&mut self) {
        if self.browse_pos > 0 {
            self.browse_pos -= 1;
        }
    }

    fn handle_pos_increase(&mut self) {
        self.browse_pos += 1;
    }
}
