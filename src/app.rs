use std::time::{Instant, Duration};

use iced::{Element, window, Application, Subscription, Command, executor, Theme, Event, theme, time, widget::{column, text, button}, Length, alignment, Alignment, font::load};
use pet_the_cat::game::{MULTIPLIER_COST, PETTING_MACHINE_COST};

use crate::views::game::Game;

/// Application widget.
#[derive(Default)]
pub struct App {
    // Game instance
    pub game: pet_the_cat::game::Game,
    // Game widget
    pub game_view: Game,
    // Game is loaded
    loaded: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Window events
    EventOccurred(Event),
    // Application events
    Tick(Instant),
    Exit,
    // Game events
    PetCat,
    BuyMultiplier,
    BuyPettingMachine,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        t!("title")
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        // Load the game if it hasn't been loaded yet
        // TODO: Find a better way to do this
        if self.loaded == false {
            self.load();
            self.loaded = true;
        }

        match message {
            // Window events
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    //self.save();
                    window::close()
                } else {
                    Command::none()                
                }
            }
            // Application events
            Message::Exit => window::close(),
            Message::Tick(_) => {
                // Machines pet cat every second
                self.game.pet_cat_with_machine();
                self.save(); // TODO: Find a better way to do this
                Command::none()
            }
            // Game events
            Message::PetCat => {
                self.game.pet_cat();
                Command::none()
            },
            Message::BuyMultiplier => {
                self.game.buy_multiplier();
                Command::none()
            },
            Message::BuyPettingMachine => {
                self.game.buy_petting_machine();
                Command::none()
            },
            _ => Command::none(),
        }
        
    }

    fn subscription(&self) -> Subscription<Message> {
        //event::listen().map(Message::EventOccurred)
        // match self {
        //     crate::app::App { .. } => {
                
        //     },
        //     crate::game::Game { .. } => {
        //         // Tick every second
        //         time::every(Duration::from_millis(1000))
        //             .map(Message::Tick)
        //     }
        // }

        // match self {
        //     crate::game::Game => self
        //         .game_view
        //         .subscription()
        //         .map(Message::Tick),
        // }

        time::every(Duration::from_millis(1000))
                    .map(Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        //self.game_view.view().map(Message::EventOccurred)

        let title = text(t!("title"))
                    .width(Length::Fill)
                    .size(50)
                    .horizontal_alignment(alignment::Horizontal::Center);

        if self.loaded == false {
            let loading_text = text(t!("loading"))
                    .width(Length::Fill)
                    .size(30)
                    .horizontal_alignment(alignment::Horizontal::Center);

            return column!(title, loading_text).into();
        }
        
        let stats = text(t!("stats",
                        cat_petted = self.game.cat_petted,
                        multiplier = self.game.multiplier,
                        petting_machine = self.game.petting_machine))
                    .width(Length::Fill)
                    .size(30)
                    .horizontal_alignment(alignment::Horizontal::Center);

        let pet_cat_button = button(text(t!("pet")))
                    .on_press(Message::PetCat)
                    .style(theme::Button::Primary)
                    .padding(10);

        let buy_multiplier_button = button(text(t!("buy_multiplier")))
                    .on_press(Message::BuyMultiplier)
                    .style(theme::Button::Secondary)
                    .padding(10);

        // TODO: Make this button disabled if the player doesn't have enough cat pets
        let buy_petting_machine_button = button(text(t!("buy_petting_machine")))
                    .on_press(Message::BuyPettingMachine)
                    // Style disabled button if the player doesn't have enough cat pets
                    .style(theme::Button::Secondary)
                    .padding(10);

        let mut content = column![title, stats, pet_cat_button]
                    .spacing(20)
                    .max_width(800)
                    .align_items(Alignment::Center);

        if self.game.cat_petted >= MULTIPLIER_COST {
            content = content.push(buy_multiplier_button);
        }

        if self.game.cat_petted >= PETTING_MACHINE_COST {
            content = content.push(buy_petting_machine_button);
        }

        content.into()
    }
}

impl App {    
    // Load the `Game` save file in TOML format
    pub fn load(&mut self) {
        let game_str_result = std::fs::read_to_string("save.toml");

        let game_str = match game_str_result {
            Ok(game_str) => game_str,
            Err(_) => {
                // Check if save file exists
                let save_exists = std::path::Path::new("save.toml").exists();

                if save_exists {
                    // If it does, but we can't read it, then something is wrong
                    panic!("{}", t!("fail_read_save"));
                }

                // If it doesn't, then create a new save file
                self.save();
                return;
            }
        };

        self.game = toml::from_str(&game_str).unwrap();
    }

    // Save the `Game` to a TOML file
    pub fn save(&self) {
        let save: String = toml::to_string(&self.game).unwrap();
        std::fs::write("save.toml", save).unwrap();
    }    
}