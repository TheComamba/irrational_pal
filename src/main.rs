use gui::Gui;
use iced::Size;

mod data;
mod gui;

fn main() -> iced::Result {
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = Size::new(300.0, 400.0);
    iced::application("Irrational Pal", Gui::update, Gui::view)
        .window(window_settings)
        .theme(Gui::theme)
        .run()
}
