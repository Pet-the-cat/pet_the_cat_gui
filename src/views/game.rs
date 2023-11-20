use iced::{Element, Subscription, Command, time};
use std::time::{Instant, Duration};

#[derive(Default)]
pub struct Game;

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
}

impl Game {
    fn new() -> Self {
        Self::default()
    }

    // fn update(&mut self, message: Message) -> Command<Message> {
    //     // match message {
    //     //     Message::Tick(_) => {
    //     //         // Machines pet cat every second
    //     //         self.game.pet_cat_with_machine();
    //     //         Command::none()
    //     //     }
    //     // }
    // }

    // fn subscription(&self) -> Subscription<Message> {
    //     time::every(Duration::from_millis(1000))
    //             .map(Message::Tick)
    // }

    pub fn view(&self) -> Element<Message> {
        "Hello, world!".into()
    }
}