use super::{Gui, GuiMessage};
use crate::data::extraction::number_representation;
use iced::{
    mouse::Cursor,
    widget::{
        canvas::{self, path::lyon_path::geom::euclid::Trig},
        Button, Canvas, Text,
    },
    Element, Rectangle, Renderer, Theme,
};

impl Gui {
    pub(super) fn next_position_text(&self) -> Text {
        Text::new(format!("Next position: {}", self.recite_pos))
    }

    pub(super) fn recited_numbers_text(&self) -> Text {
        let number_str = match self.number {
            Some(n) => number_representation(n, self.recite_pos),
            None => String::from(""),
        };
        Text::new(number_str)
    }

    pub(super) fn digit_buttons(&self) -> Element<'_, GuiMessage> {
        iced::widget::canvas(DigitButtons::new()).into()
    }
}

pub(super) struct DigitButtons {
    cache: canvas::Cache,
}

impl DigitButtons {
    pub(super) fn new() -> Self {
        Self {
            cache: canvas::Cache::new(),
        }
    }
}

impl<Message> canvas::Program<Message> for DigitButtons {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        let buttons = self.cache.draw(renderer, bounds.size(), |frame| {
            //arrange buttons in circle
            let center = iced::Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let radius = bounds.width.min(bounds.height) / 2.0;
            let button_count = 10;
            let angle_step = 2.0 * std::f32::consts::PI / button_count as f32;
            let mut angle = 0.0;
            for i in 0..button_count {
                let button_center = center + iced::Vector::new(angle.cos(), angle.sin()) * radius;
                let digit: char = std::char::from_digit(i, 10).unwrap();
                let button = Button::new(Text::new(digit.to_string()))
                    .on_press(GuiMessage::ReciteDigit(digit));

                // add button to frame.

                angle += angle_step;
            }
        });
        vec![buttons]
    }
}
