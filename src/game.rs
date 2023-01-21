// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2022

use uuid::Uuid;
use std::collections::{HashSet, HashMap};

pub mod coin_game;


/// The possible categorical states of a discrete two-player game.
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win,
    Loss,
    Tie
}


/// Common interface for a game.
pub trait Game {
    /// Instructs the game to play a move with a specified ID.
    fn play(&mut self, mv: Uuid);

    /// Instructs the game to undo the most recently performed move.
    fn undo(&mut self);

    /// Returns a vector with the IDs of all legal moves from the current
    /// state of the game.
    fn possible_moves(&self) -> Vec<Uuid>;

    /// Returns a unique encoding of the current state of the game.
    fn encode_state(&self) -> Uuid;

    /// Returns the categorical state of the game.
    fn evaluate_state(&self) -> Option<Outcome>;
}


fn solve(game: &mut dyn Game) -> Outcome {
    if let Some(out) = game.evaluate_state() {
        return out
    }
    let mut possible_outcomes: HashSet<Outcome> = HashSet::new();
    let mut seen_states: HashMap<Uuid, Outcome> = HashMap::new();
    for mv in game.possible_moves() {
        game.play(mv);
        let encoded_state = game.encode_state();
        if let Some(out) = seen_states.get(&encoded_state).copied() {
            possible_outcomes.insert(out);
        } else {
            let out = solve(game);
            possible_outcomes.insert(out);
            seen_states.insert(game.encode_state(), out);
        }
        game.undo();
    }
    if possible_outcomes.contains(&Outcome::Loss) {
        Outcome::Win
    } else if possible_outcomes.contains(&Outcome::Tie) {
        Outcome::Tie
    } else {
        Outcome::Loss
    }
}