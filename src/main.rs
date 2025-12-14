use gui::Gui;
use iced::{window::Settings, Size};

mod data;
mod gui;

fn main() -> iced::Result {
    let window_settings = Settings {
        size: Size::new(300.0, 400.0),
        ..Default::default()
    };
    iced::application(Gui::default, Gui::update, Gui::view)
        .title("Irrational Pal")
        .window(window_settings)
        .theme(Gui::theme)
        .run()
}
