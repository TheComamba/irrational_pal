use iced::{
    widget::{Button, Column, Row, Text},
    Sandbox,
};

pub(crate) struct Gui {}

impl Sandbox for Gui {
    type Message = ();

    fn new() -> Self {
        Gui {}
    }

    fn title(&self) -> String {
        String::from("Irrational Pal")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let button_e = Button::new(Text::new("e"));
        let button_pi = Button::new(Text::new("Pi"));
        let button_browse = Button::new(Text::new("Browse"));
        let button_recite = Button::new(Text::new("Recite"));
        Column::new()
            .push(
                Row::new()
                    .push(button_e)
                    .push(button_pi)
                    .padding(5)
                    .spacing(5),
            )
            .push(
                Row::new()
                    .push(button_browse)
                    .push(button_recite)
                    .padding(5)
                    .spacing(5),
            )
            .push(self.digit_view())
            .padding(5)
            .spacing(5)
            .into()
    }
}

impl Gui {
    fn digit_view(&self) -> iced::Element<'_, ()> {
        let mut row = Row::new();
        let button_left = Button::new(Text::new("<"));
        let button_right = Button::new(Text::new(">"));
        row = row.push(button_left);
        for i in 0..10 {
            row = row.push(Text::new(format!("{}", i)));
        }
        row = row.push(button_right);
        row.padding(5).spacing(5).into()
    }
}
