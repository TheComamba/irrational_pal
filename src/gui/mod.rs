use iced::{
    widget::{Column, Text},
    Sandbox,
};

pub(crate) struct Gui {}

impl Sandbox for Gui {
    type Message = ();

    fn new() -> Self {
        Gui {}
    }

    fn title(&self) -> String {
        String::from("Hello, world!")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new().push(Text::from("value")).into()
    }
}
