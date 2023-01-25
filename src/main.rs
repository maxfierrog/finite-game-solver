// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2023


pub mod game;


// Using hash maps to map game states to their solved outcomes.
use std::collections::HashMap;
use crate::game::*;
use std::io;


fn main() {
    println!("\n ------ GAME SOLVER ------ \n");
    println!("You are playing N-to-0-by-1-or-2 \n");
    let num_coins = input_integer();
    let mut game = zero_by_1_2::Session::new(num_coins);
    let mut state_map: HashMap<i32, Outcome> = HashMap::new();
    let result = solve(&mut game, &mut state_map);
    print!("\n If there were {} coins remaining and it was your turn, 
    you could at best turn this into a ", num_coins);
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
    let mut result: i32 = 0;
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
    result
}