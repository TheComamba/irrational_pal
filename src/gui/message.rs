use crate::data::{extraction::get_digits, Number};

use super::{Gui, GuiMode};

#[derive(Debug, Clone)]
pub(crate) enum GuiMessage {
    PickedNumber(&'static Number),
    PickedMode(GuiMode),
    TypedPos(String),
    PosDecrease,
    PosIncrease,
    ReciteDigit(char),
}

impl Gui {
    pub(super) fn handle_message(&mut self, message: GuiMessage) {
        match message {
            GuiMessage::PickedNumber(number) => self.handle_number_pick(number),
            GuiMessage::PickedMode(mode) => self.hanlde_mode_pick(mode),
            GuiMessage::TypedPos(pos) => self.handle_pos_input(pos),
            GuiMessage::PosDecrease => self.handle_pos_decrease(),
            GuiMessage::PosIncrease => self.handle_pos_increase(),
            GuiMessage::ReciteDigit(digit) => self.handle_recite_digit(digit),
        }
    }

    fn handle_number_pick(&mut self, number: &'static Number) {
        self.number = Some(number);
        self.browse_pos = 0;
        self.recite_pos = 0;
    }

    fn hanlde_mode_pick(&mut self, mode: GuiMode) {
        self.mode = mode;
    }

    fn handle_pos_input(&mut self, input: String) {
        let pos = input.parse::<u32>();
        self.browse_pos = pos.unwrap_or(0);
    }

    fn handle_pos_decrease(&mut self) {
        if self.browse_pos > 0 {
            self.browse_pos -= 1;
        }
    }

    fn handle_pos_increase(&mut self) {
        self.browse_pos += 1;
    }

    fn handle_recite_digit(&mut self, digit_input: char) {
        let number = match self.number {
            Some(n) => n,
            None => return,
        };
        let next_digit = get_digits(number, self.recite_pos, 1)[0];
        if next_digit == digit_input {
            self.recite_pos += 1;
            self.wrong_digits.clear();
        } else {
            self.wrong_digits.push(digit_input);
        }
    }
}
