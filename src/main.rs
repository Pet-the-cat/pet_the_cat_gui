#![windows_subsystem = "windows"]
use iced::{Settings, Application};
use pet_the_cat_gui::{app::App, localization};

pub fn main() -> iced::Result {
    // Set the current localization to the system's locale.
    localization::set_to_system();

    App::run(Settings {
        //exit_on_close_request: false,
        ..Settings::default()
    })
}
