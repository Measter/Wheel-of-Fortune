use std::io;
use std::collections;

struct Player {
    name: String,
    points: i32,
}


// associated functions
impl Player {
    fn new(name: String) -> Player {
        // Creates a new Player object with name, player starts with 0 points.
        Player {
            name: name.trim().to_string(),
            points: 0,
        }
    }
}

// methods
impl Player {
    fn play(&self, mut puzzle: Puzzle) -> Puzzle {
        // Player can guess a letter or phrase. If correct, updates puzzle_board object.

        println!("{}, guess a letter or the phrase: ", self.name);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        guess = guess.to_uppercase();

        if puzzle.guesses.contains(&guess) {
            println!("That has already been guessed.");
        }
        else {
            puzzle.guesses.push(guess.clone());
        }

        if puzzle.contains(guess.clone()) {
            puzzle.update(guess);
        }

        puzzle
    }
}

struct Puzzle {
    category: String,
    solution: String,
    current_puzzle_board: String,
    guesses: Vec<String>,
}

// associated functions
impl Puzzle {
    fn new() -> Puzzle {
        // Creates a new Puzzle object.

        let category: String = String::from("Fruit");
        let solution: String = String::from("APPLE");
        let (current_puzzle_board, solution) = get_dashes_from_(solution);
        let guesses = Vec::new();

        Puzzle {
            category,
            solution,
            current_puzzle_board,
            guesses,
        }
    }
}

// methods
impl Puzzle {
    fn print(&self) {
        // Prints the category and current state of puzzle.

        println!("\nThe category is {}", self.category);

        let mut puzzle_board = Vec::new();
        for character in self.current_puzzle_board.chars() {
            puzzle_board.push(character);
            puzzle_board.push(' ');
        }

        let puzzle_board_string: String = puzzle_board.into_iter().collect();
        println!("{}\n", puzzle_board_string);
    }

    fn contains(&self, guess: String) -> bool {
        // Checks if the guess is single char or string and if correct.

        let trimmed_guess = guess.trim().to_string();
        
        if trimmed_guess.len() == 1 {
            self.check_guess_char(guess.chars().next().unwrap())
        }
        else {
            self.check_guess_string(trimmed_guess)
        }
    }

    fn check_guess_char(&self, guess: char) -> bool {
        self.solution.contains(guess)
    }

    fn check_guess_string(&self, guess: String) -> bool {
        guess == self.solution
    }

    fn update(&mut self, guess: String) {
        // TASK: This can be a lot cleaner. Probably a Rust matching way of doing this.
        let mut char_indicies: Vec<char> = Vec::new();
        let mut index: usize;

        for character in self.current_puzzle_board.chars() {
            char_indicies.push(character);
        }

        for character in guess.trim().chars() {
            index = 0;
            for solution_character in self.solution.chars() {
                if character == solution_character {
                    char_indicies[index] = character;
                }
                index += 1;
            }

            // index = self.solution.chars().position(|c| c == character).unwrap();
            // println!("{}", index);
            // self.current_puzzle_board.replace_range(index..index+1, &character.to_string());
        }

        self.current_puzzle_board = char_indicies.into_iter().collect();
    }

    fn solved(&self) -> bool {
        self.current_puzzle_board == self.solution
    }
}

fn main() {
    let mut continue_playing = true;
    while continue_playing {
        continue_playing = play_game();
    }
}

fn play_game() -> bool {

    let mut solved = false;

    print_intro_screen();
    let players = build_players(get_number_of_players());
    let mut puzzle = Puzzle::new();

    while !solved {
        for player in &players {
            puzzle.print();
            puzzle = player.play(puzzle);
            solved = puzzle.solved();
        }
    }
    
    false
}

fn print_intro_screen() {
    println!("Welcome to Wheel of Fortune!");
}

fn build_players(n: u32) -> Vec<Player> {
    // instantiates n Player objects 

    let mut players: Vec<Player> = Vec::new();

    for i in 0..n {
        let mut player_name = String::new();
        println!("Player {}, what is your name?", i +1);
        io::stdin().read_line(&mut player_name).expect("Failed to read line");

        let player = Player::new(player_name);

        players.push(player);
    }

    players
}

fn get_number_of_players() -> u32 {
    // asks how many players are playing the game.

    println!("How many players? ");
    let mut num_players = String::new();
    io::stdin().read_line(&mut num_players).expect("Failed to read line");
    let num_players: u32 = num_players.trim().parse().expect("Please type a number!");

    // TASK: Limit to 3 players

    num_players
}

fn get_dashes_from_(solution: String) -> (String, String) {
    let mut dashes_char_vec = vec![];

    for character in solution.chars() {
        if character == ' ' {
            dashes_char_vec.push(' ');
        }
        else {
            dashes_char_vec.push('_');
        }
    };

    let dashes: String = dashes_char_vec.into_iter().collect();

    (dashes, solution)
}

// fn spinWheel -> integer value of pts. (eventually a lose a turn result)

// fn getPhrase() returns a phrase from a JSON file

// include reference to tests
#[cfg(test)]
mod test;