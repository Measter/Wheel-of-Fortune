use std::io;

mod puzzle;
use self::puzzle::Puzzle;

mod player;
use self::player::Player;

mod announcer;
use self::announcer::Announcer;

extern crate rand;
use self::rand::Rng;

fn spin_wheel() -> i32 {
    let mut rng = rand::thread_rng();
    // We can just make this an array. Being a vector is unnecessary here, and just allocates
    // for no reason.
    let wheel = [
        2500, 600, 700, 600, 650, 500, 700, 600, 550, 500, 600, 650, 700, 800, 500, 650, 500, 900,
    ];

    // The index isn't being used for anything, so we'll change this to use
    // Rand's Rng::choose();
    // Because we know ahead of time that this is *not* empty, we'll just unwrap.
    // Also note that this function returns a reference to the chose item. In this case
    // it's an int, so we'll just dereference it.
    *rng.choose(&wheel).unwrap()
}

pub struct Game {
    puzzle: Puzzle,
    round: u32,
    players: Vec<Player>,
    // We don't really need to separately keep track of the number of players,
    // as the player Vector keeps track of its own length.
    announcer: Announcer,
}

// initialization of Game
impl Game {
    pub fn new(round: u32) -> Game {
        let players = init_players();

        Game {
            puzzle: Puzzle::new(),
            round,
            players,
            announcer: Announcer::new(round),
        }
    }
}

// Game functions
impl Game {
    pub fn play(&mut self) -> bool {
        // Begins a new round, returns if a player wants to play again.

        let mut solved = false;

        // The welcome message is already a string, so no need for the to_string call.
        println!("{}", self.announcer.welcome);

        while !solved {
            // Instead of going over a range of indices, we'll instead go over
            // the list of players directly.
            // The mutable borrow on self.players is so the loop does not consume
            // the vector, which is not what we want.
            for player in &mut self.players {
                println!("Spin!");

                let wheel_panel = spin_wheel();
                println!("The wheel lands on ${}!", wheel_panel);

                self.puzzle.print();
                // This variable doesn't need to live beyond this loop as a new string
                // is created every time anyway.
                let guess = player.play();

                // Check if a guess has already been made.
                if self.puzzle.guesses.contains(&guess) {
                    println!("That has already been guessed.");
                } else {
                    self.puzzle.guesses.push(guess.clone());
                }

                // Update the puzzle.
                // Instead of declaring a mutable integer and changing it, we can
                // use the if as an expression and assign to it only once.
                let number_in =if self.puzzle.contains(&guess) {
                    self.puzzle.update(&guess)
                } else {
                    0
                };

                player.add_winnings(number_in * wheel_panel);

                solved = self.puzzle.solved();
            }
        }

        println!("Would you like to play again?");
        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect("Failed to read line");

        // Do a little pattern match instead of unwrapping.
        if let Some('y') = replay.chars().next() {
            true
        } else {
            false
        }
    }
}

fn init_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();

    println!("How many players? ");
    let mut num_players = String::new();
    io::stdin()
        .read_line(&mut num_players)
        .expect("Failed to read line");
    let num_players: usize = num_players.trim().parse().expect("Please type a number!");

    for i in 0..num_players {
        let mut player_name = String::new();
        println!("Player {}, what is your name?", i + 1);
        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        let player = Player::new(player_name);

        players.push(player);
    }

    players
}
