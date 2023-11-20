#![windows_subsystem = "windows"]
use iced::{Settings, Application};
use pet_the_cat_gui::app::App;

pub fn main() -> iced::Result {
    App::run(Settings {
        //exit_on_close_request: false,
        ..Settings::default()
    })
}
