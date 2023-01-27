// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2023


pub mod game;


// Using hash maps to map game states to their solved outcomes.
use std::collections::HashMap;
use crate::game::*;
use std::io;


fn main() {
    println!("\n -------------------- GAME SOLVER -------------------- \n");
    println!("You are playing {}.\n", tic_tac_toe::GAME_NAME);
    println!("{}\n", tic_tac_toe::GAME_DESCRIPTION);
    let mut game = tic_tac_toe::Session::new();
    let mut state_map: HashMap<i32, Outcome> = HashMap::new();
    let result = solve(&mut game, &mut state_map);
    count_outcomes(&state_map);
    match result {
        Outcome::Loss => println!("loss."),
        Outcome::Tie => println!("tie."),
        Outcome::Win => println!("win.")
    }
}

/// Asks user to input an integer and returns it, re-prompting if
/// the user inputs something unexpected.
fn input_integer() -> i32 {
    let mut failed: bool = true;
    let mut result: u64 = 0;
    while failed {
        println!("Input desired amount of coins (N):\n");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        result = match input.trim().parse() {
            Ok(num) => { failed = false; num },
            Err(_) => { 
                println!("\nPlease type in a positive integer!\n");
                continue
            }
        };
    }
    result as i32
}

fn count_outcomes(state_map: &HashMap<i32, Outcome>) {
    let mut ties = 0;
    let mut wins = 0;
    let mut losses = 0;
    for (_, out) in state_map {
        match out {
            Outcome::Loss => { losses += 1; },
            Outcome::Win => { wins += 1; },
            Outcome::Tie => { ties += 1; }
        }
    }
    println!("Wins: {}", wins);
    println!("Losses: {}", losses);
    println!("Ties: {}", ties);
}