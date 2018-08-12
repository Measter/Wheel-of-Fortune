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

    let mut categories: Vec<&str> = Vec::new();
    for key in puzzles.keys() {
        categories.push(key);
    }

    // As in game::spin_wheel(), we'll use Rng::choose() for selecting the
    // solution and category.
    // Again, we know the input array is not empty, so we'll just unwrap.
    let category = rng.choose(&categories).unwrap();

    let mut solutions: Vec<&str> = Vec::new();
    for val in &puzzles[category] {
        solutions.push(val);
    }
    let solution = rng.choose(&solutions).unwrap();

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

        let mut puzzle_board = Vec::new();
        for character in self.current_puzzle_board.chars() {
            puzzle_board.push(character);
            puzzle_board.push(' ');
        }

        let puzzle_board_string: String = puzzle_board.into_iter().collect();
        println!("{}\n", puzzle_board_string);
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
    let mut dashes_char_vec = vec![];

    for character in solution.chars() {
        if character == ' ' {
            dashes_char_vec.push(' ');
        } else if character == '.' {
            dashes_char_vec.push('.');
        } else {
            dashes_char_vec.push('_');
        }
    }

    let dashes: String = dashes_char_vec.into_iter().collect();

    (dashes, solution)
}
