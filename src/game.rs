// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2022


/* ---------- AVAILABLE GAMES ---------- */
pub mod zero_by_1_2;
pub mod zero_by_1_3_4;


use uuid::Uuid;
use std::{collections::{HashSet, HashMap}, hash::Hash};


#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win,
    Loss,
    Tie
}


pub trait Game {
    /// Instead of using the generic game solver, we can use each game's
    /// specific solver to efficiently determine the outcome resulting from
    /// playing perfectly from their current state.
    fn solve(&self) -> Outcome;

    /// Instructs the game to play a move with a specified ID.
    fn play(&mut self, mv: Uuid);

    /// Instructs the game to undo the most recently performed move.
    fn undo(&mut self);

    /// Returns a vector with the IDs of all legal moves from the current
    /// state of the game.
    fn possible_moves(&self) -> Vec<Uuid>;

    /// Returns a unique encoding of the current state of the game.
    fn state(&self) -> i32;
        // FIXME: Return hash(state) instead of a UUID in order to interface with
        // more types of games (not just ones whose possible states are injective
        // to the set of possible i32 integers).

    /// Returns the categorical state of the game in an optional, being
    /// of type None if the state is not final.
    fn outcome(&self) -> Option<Outcome>;
}


/// Recursive solver for any GAME which implements the Game trait. Accepts
/// a mutable reference to GAME and a mutable reference SEEN, an empty map.
/// 
/// Once the first call goes out of scope, GAME will again be in its
/// original state, despite being mutated in the process of execution, and
/// SEEN will contain a mapping of all possible states to their corresponding
/// solved outcomes.
/// 
/// Time:       O(# of nodes)
/// Memory:     O(# of nodes * max degree)
/// 
/// TODO: Make this function iterative to avoid memory overhead and stack
/// overflow errors. Improve memory performance. Implement pruning.
pub fn solve(game: &mut dyn Game, seen: &mut HashMap<i32, Outcome>) -> Outcome {
    if let Some(out) = game.outcome() {
        return out
    }
    let mut possible_outcomes: HashSet<Outcome> = HashSet::new();
    for mv in game.possible_moves() {
        game.play(mv);
        let encoded_state = game.state();
        if let Some(out) = seen.get(&encoded_state).copied() {
            possible_outcomes.insert(out);
        } else {
            let out = solve(game, seen);
            possible_outcomes.insert(out);
            seen.insert(encoded_state, out);
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