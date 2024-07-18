use core::panic;
use std::{error, fs::File, io::Read};

use clap::Parser;
use rand::{thread_rng, Rng};
use ratatui::widgets::ListState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    names: Option<String>,
}

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Adding,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Name,
}

#[derive(Debug)]
pub struct Wheel {
    /// Number of times to spin
    pub spin_count: usize,
    /// Angle in degrees
    pub angle: f64,
    /// Change in angle each tick. Variable
    pub d_angle: f64,
    /// Default change in angle
    pub angle_change: f64,
    /// Number of times to spin
    pub num_spins: usize,
}

impl Default for Wheel {
    fn default() -> Self {
        Self {
            angle: 0.0,
            d_angle: 5.0,
            angle_change: 5.0,
            spin_count: 0,
            num_spins: 0,
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Is the application spinning?
    pub spinning: bool,
    /// All winners
    pub all_winners: Vec<String>,
    /// All participants
    pub all_participants: StatefulList<String>,

    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub name_input: String,

    /// Show contestants
    pub show_contestants: bool,

    pub wheel: Wheel,
}

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T: Clone> StatefulList<T> {
    pub fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    self.items.len() - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn select(&mut self, i: Option<usize>) {
        self.state.select(i);
    }

    pub fn get_selected(&mut self) -> Option<T> {
        match self.state.selected() {
            Some(i) => Some(self.items[i].clone()),
            None => None,
        }
    }

    pub fn remove_selected(&mut self) -> Option<T> {
        match self.state.selected() {
            Some(i) => Some(self.items.remove(i)),
            None => None,
        }
    }

    pub fn insert(&mut self, item: T) {
        self.items.push(item);
    }
}

impl Default for App {
    fn default() -> Self {
        let cli = Cli::parse();

        let names = if let Some(names) = cli.names {
            let res = File::open(&names);
            if let Ok(mut file) = res {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("to be able to read from file");
                contents.lines().map(|s| s.to_string()).collect()
            } else {
                panic!("Could not open file: {}", &names);
            }
        } else {
            vec![
                "Lisa".to_string(),
                "John".to_string(),
                "Bob".to_string(),
                "Mary".to_string(),
                "Joe".to_string(),
                "Tom".to_string(),
                "Jane".to_string(),
                "Mark".to_string(),
                "Sally".to_string(),
                "Alice".to_string(),
            ]
        };

        Self {
            running: true,
            all_participants: StatefulList::new(names),
            all_winners: vec![],
            spinning: false,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            name_input: String::new(),
            show_contestants: true,
            wheel: Wheel::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_spin(&mut self) {
        if self.all_participants.items.is_empty() {
            return;
        }

        if self.all_participants.state.selected().is_none() {
            self.all_participants.state.select(Some(0));
        }

        self.wheel.angle = self.index_to_angle(self.all_participants.state.selected().unwrap_or(0));
        let number_of_participants = self.all_participants.items.len();
        let min_spins = (number_of_participants * 50).max(300);
        let max_spins = (number_of_participants * 80).max(500);

        self.wheel.spin_count = thread_rng().gen_range(min_spins..max_spins);
        self.wheel.num_spins = self.wheel.spin_count;

        self.spinning = true;
    }

    pub fn stop_spin(&mut self) {
        self.spinning = false;
        self.wheel.num_spins = 0;
        self.wheel.spin_count = 0;
        self.wheel.d_angle = self.wheel.angle_change;
    }

    pub fn reset_spin(&mut self) {
        self.wheel.spin_count = 0;
        self.wheel.num_spins = 0;
        self.stop_spin();
    }

    pub fn spin_round(&mut self) {
        if !self.spinning {
            return;
        }

        if self.wheel.spin_count > 0 {
            self.wheel.spin_count -= 1;
            self.all_participants
                .select(self.currently_winning_index().into());

            // Slow down the spinner after half of the ticks
            if self.wheel.spin_count < self.wheel.num_spins / 2 {
                self.wheel.d_angle -= self.wheel.angle_change / (self.wheel.num_spins / 2) as f64;
            }
            return;
        }

        if let Some(winner) = self.all_participants.get_selected() {
            self.all_winners.push(winner);
        }
        self.stop_spin();
    }

    pub fn currently_winning_index(&self) -> usize {
        let d = 360.0 / self.all_participants.items.len() as f64;
        (self.wheel.angle / d).floor() as usize
    }

    pub fn index_to_angle(&self, index: usize) -> f64 {
        let d = 360.0 / self.all_participants.items.len() as f64;
        d * index as f64 + d / 2.0
    }

    pub fn save_name(&mut self) {
        if let Some(name) = self.name_input.trim().to_string().into() {
            if !name.is_empty() {
                self.all_participants.insert(name);
            }
            self.name_input = String::new();
        }
    }

    pub fn increase_angle(&mut self) {
        self.wheel.angle += self.wheel.d_angle;
        if self.wheel.angle >= 360.0 {
            self.wheel.angle -= 360.0;
        }
    }

    pub fn toggle_contestants(&mut self) {
        self.show_contestants = !self.show_contestants;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if self.spinning {
            self.increase_angle();
            self.spin_round();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
