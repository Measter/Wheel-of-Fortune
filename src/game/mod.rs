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
    let index = rng.gen_range(0, wheel.len());

    wheel[index]
}

pub struct Game {
    puzzle: Puzzle,
    round: u32,
    players: Vec<Player>,
    num_players: usize,
    announcer: Announcer,
}

// initialization of Game
impl Game {
    pub fn new(round: u32) -> Game {
        let (players, num_players) = init_players();

        Game {
            puzzle: Puzzle::new(),
            round,
            players,
            num_players,
            announcer: Announcer::new(round),
        }
    }
}

// Game functions
impl Game {
    pub fn play(&mut self) -> bool {
        // Begins a new round, returns if a player wants to play again.

        let mut solved = false;

        println!("{}", self.announcer.welcome.to_string());

        let mut guess: String;

        while !solved {
            for i in 0..self.num_players {
                let ref mut player = self.players[i];

                println!("Spin!");

                let wheel_panel = spin_wheel();
                println!("The wheel lands on ${}!", wheel_panel);

                self.puzzle.print();
                guess = player.play();

                // Check if a guess has already been made.
                if self.puzzle.guesses.contains(&guess) {
                    println!("That has already been guessed.");
                } else {
                    self.puzzle.guesses.push(guess.clone());
                }

                let mut number_in: i32 = 0;
                // Update the puzzle.
                if self.puzzle.contains(guess.clone()) {
                    number_in = self.puzzle.update(guess);
                }

                player.add_winnings(number_in * wheel_panel);

                solved = self.puzzle.solved();
            }
        }

        println!("Would you like to play again?");
        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect("Failed to read line");

        if replay.chars().next().unwrap() == 'y' {
            true
        } else {
            false
        }
    }
}

fn init_players() -> (Vec<Player>, usize) {
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

    (players, num_players)
}
