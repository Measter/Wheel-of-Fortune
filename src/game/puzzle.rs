use std::collections::HashMap;
extern crate rand;
use self::rand::Rng;

pub struct Puzzle {
    category: String,
    solution: String,
    current_puzzle_board: String,
    pub guesses: Vec<String>,
}

fn get_puzzle() -> (String, String) {
    let puzzles: HashMap<&str, Vec<&str>> = [
        ("Fruit", vec!["APPLE", "PEACH"]),
        ("Phrases", vec!["THREES A CROWD."]),
    ].iter()
        .cloned()
        .collect();

    // We're reusing the RNG a couple times, so I would just call thread_rng once.
    // It doesn't really change anything, but makes later lines easier to read
    // in my opinion.
    let mut rng = rand::thread_rng();

    // Instead of explicitly writing out a loop to create the category vector
    // We can just collect the keys() return value directly.
    // We need cloned() here to make sure we get a &str not a &&str.
    let categories: Vec<&str> = puzzles.keys().cloned().collect();

    // As in game::spin_wheel(), we'll use Rng::choose() for selecting the
    // solution and category.
    // Again, we know the input array is not empty, so we'll just unwrap.
    let category = rng.choose(&categories).unwrap();

    // We can just get rid of creating the solution vector completely, as
    // the indexing returns what we need already.
    let solution = rng.choose(&puzzles[category]).unwrap();

    // Due to the above changes, this no longer works as both strings are now
    // of the type &&str, not &str.
    // We could change this to a call to to_string(), but I'll just dereference them.
    (String::from(*category), String::from(*solution))
}

// associated functions
impl Puzzle {
    pub fn new() -> Puzzle {
        // Creates a new Puzzle object.
        let (category, solution) = get_puzzle();
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
    pub fn print(&self) {
        // Prints the category and current state of puzzle.

        println!("\nThe category is {}", self.category);

        // Instead of pushing to a vector, then collecting to a string we can just
        // push onto the string directly.
        // Because we know the length ahead of time, we can create the string with
        // the appropriate capacity.
        // We can further avoid an allocation here by just printing the character
        // directly instead of pushing into a string first.
        for character in self.current_puzzle_board.chars() {
            print!("{} ", character);
        }

        println!();
    }

    // This functions doesn't need to take an owned string to work,
    // so we'll change it to take a string reference.
    pub fn contains(&self, guess: &str) -> bool {
        // Checks if the guess is single char or string and if correct.

        let trimmed_guess = guess.trim().to_string();

        if trimmed_guess.len() == 1 {
            self.check_guess_char(guess.chars().next().unwrap())
        } else {
            self.check_guess_string(trimmed_guess)
        }
    }

    fn check_guess_char(&self, guess: char) -> bool {
        self.solution.contains(guess)
    }

    fn check_guess_string(&self, guess: String) -> bool {
        guess == self.solution
    }

    // As with the contains() function above, this also doesn't need to
    // own its string input.
    pub fn update(&mut self, guess: &str) -> i32 {
        // TASK: This can be a lot cleaner. Probably a Rust matching way of doing this.
        let mut char_indicies: Vec<char> = Vec::new();
        let mut index: usize;
        let mut num_in_solution: i32 = 0;

        for character in self.current_puzzle_board.chars() {
            char_indicies.push(character);
        }

        for character in guess.trim().chars() {
            index = 0;
            for solution_character in self.solution.chars() {
                if character == solution_character {
                    char_indicies[index] = character;
                    num_in_solution += 1;
                }
                index += 1;
            }

            // index = self.solution.chars().position(|c| c == character).unwrap();
            // println!("{}", index);
            // self.current_puzzle_board.replace_range(index..index+1, &character.to_string());
        }

        self.current_puzzle_board = char_indicies.into_iter().collect();

        num_in_solution
    }

    pub fn solved(&self) -> bool {
        self.current_puzzle_board == self.solution
    }
}

fn get_dashes_from_(solution: String) -> (String, String) {
    // As in Puzzle::print(), there's no need to push to a vector first.
    let mut dashes_char = String::new();

    for character in solution.chars() {
        if character == ' ' {
            dashes_char.push(' ');
        } else if character == '.' {
            dashes_char.push('.');
        } else {
            dashes_char.push('_');
        }
    }

    (dashes_char, solution)
}
