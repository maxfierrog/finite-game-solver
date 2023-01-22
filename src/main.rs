// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2022

pub mod game;

use std::io;
use crate::game::Game;
use crate::game::coin_game;
use crate::game::Outcome;
use crate::game::solve;

fn main() {
    println!("GAME OF COINS!\n");
    let num_coins = input_integer();
    let mut game = coin_game::CoinGame::new(num_coins);
    game.play(game.move_uuid(coin_game::Move::Two));
    let result = solve(&mut game);
    match result {
        Outcome::Loss => print!("Loss."),
        Outcome::Tie => print!("Tie."),
        Outcome::Win => print!("Win.")
    }
}

// Asks user to input an integer.
fn input_integer() -> i32 {
    println!("Input desired amount of coins:");
    let mut input = String::new();
    let mut result = 0;
    let mut failed = true;
    while failed {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        result = match input.trim().parse() {
            Ok(num) => { failed = false; num },
            Err(_) => continue
        };
    }
    result
}