use gui::Gui;
use iced::Sandbox;

mod data;
mod gui;

fn main() -> iced::Result {
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = (300, 400);
    let mut settings = iced::settings::Settings::default();
    settings.window = window_settings;
    Gui::run(settings)
}
