use std::io;

pub struct Player {
    name: String,
    money: i32,
}

// associated functions
impl Player {
    // This function doesn't need to own its input, as it trims and creates
    // a new string anyway.
    pub fn new(name: &str) -> Player {
        // Creates a new Player object with name, player starts with $0.
        Player {
            name: name.trim().to_string(),
            money: 0,
        }
    }
}

// methods
impl Player {
    pub fn play(&self) -> String {
        // Player can guess a letter or phrase. If correct, updates puzzle_board object.

        // let mut money_earned = 0;
        println!(
            "{}, you have {} points, guess a letter or the phrase: ",
            self.name, self.money
        );
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.to_uppercase();

        guess
    }

    pub fn print_points(&self) {
        println!("{} you now have ${}", self.name, self.money);
    }

    pub fn add_winnings(&mut self, winnings: i32) {
        self.money = self.money + winnings;
        self.print_points();
    }
}
