use gui::Gui;
use iced::{Sandbox, Settings};

mod data;
mod gui;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
