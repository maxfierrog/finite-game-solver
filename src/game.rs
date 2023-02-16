// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2023


/* ---------- AVAILABLE GAMES ---------- */
pub mod zero_by_1_2;
pub mod zero_by_1_3_4;
pub mod tic_tac_toe;


use uuid::Uuid;
use std::{collections::HashMap, hash::Hash};


#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win(i32),
    Loss(i32),
    Tie(i32)
}


pub trait Game {
    fn play(&mut self, mv: Uuid);
    fn undo(&mut self);
    fn state(&self) -> i32;
    fn outcome(&self) -> Option<Outcome>;
    fn possible_moves(&self) -> Vec<Uuid>;
}


pub fn solve(game: &mut dyn Game, seen: &mut HashMap<i32, Outcome>) -> Outcome {
    if let Some(out) = game.outcome() {
        return out
    }
    let mut possible_outcomes: Vec<Outcome> = Vec::new();
    for mv in game.possible_moves() {
        game.play(mv);
        let encoded_state = game.state();
        if let Some(out) = seen.get(&encoded_state).copied() {
            possible_outcomes.push(out);
        } else {
            let out = solve(game, seen);
            possible_outcomes.push(out);
            seen.insert(encoded_state, out);
        }
        game.undo();
    }
    get_outcome(possible_outcomes)
}


fn get_outcome(available: Vec<Outcome>) -> Outcome {
    let mut w_rem = i32::MAX;
    let mut l_rem = i32::MAX;
    let mut t_rem = i32::MAX;
    let mut win = false;
    let mut tie = false;
    for out in available {
        match out {
            Outcome::Loss(rem) => {
                win = true;
                if (rem + 1) < w_rem {
                    w_rem = rem + 1;
                }
            },
            Outcome::Tie(rem) => {
                tie = true;
                if (rem + 1) < t_rem {
                    t_rem = rem + 1;
                }
            },
            Outcome::Win(rem) => {
                if (rem + 1) < l_rem {
                    l_rem = rem + 1;
                }
            }
        }
    }
    if win {
        Outcome::Win(w_rem)
    } else if tie {
        Outcome::Tie(t_rem)
    } else {
        Outcome::Loss(l_rem)
    }
}
