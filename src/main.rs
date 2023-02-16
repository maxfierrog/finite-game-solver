// Max Fierro, maxfierro@berkeley.edu
// Friday January 20th, 2023


pub mod game;


use std::collections::HashMap;
use crate::game::*;


fn main() {
    println!("\n ----------- GAME SOLVER ----------- \n");
    println!("You are playing {}.\n", tic_tac_toe::GAME_NAME);
    println!("{}\n", tic_tac_toe::GAME_DESCRIPTION);
    let mut game = tic_tac_toe::Session::new(4, 3, 3);
    let mut state_map: HashMap<i32, Outcome> = HashMap::new();
    let result = solve(&mut game, &mut state_map);
    analyze(&state_map);
    match result {
        Outcome::Loss(rem) => println!("Loss in {}!", rem),
        Outcome::Tie(rem) => println!("Tie in {}!", rem),
        Outcome::Win(rem) => println!("Win in {}!", rem)
    }
}


fn analyze(state_map: &HashMap<i32, Outcome>) {
    let mut map: HashMap<i32, (i32, i32, i32, i32)> = HashMap::new();
    for (_, out) in state_map {
        match out {
            Outcome::Loss(rem) => { 
                let values = map.entry(*rem)
                    .or_insert((0, 0, 0, 0));
                (*values).0 += 1;
                (*values).3 += 1;
            },
            Outcome::Win(rem) => { 
                let values = map.entry(*rem)
                    .or_insert((0, 0, 0, 0));
                (*values).2 += 1;
                (*values).3 += 1;
            },
            Outcome::Tie(rem) => { 
                let values = map.entry(*rem)
                    .or_insert((0, 0, 0, 0));
                (*values).1 += 1;
                (*values).3 += 1;
            }
        }
    }
    let mut totals = (0, 0, 0, 0);
    let mut collected = Vec::new();
    for (rem, values) in map.iter() {
        collected.push((rem, values));
    }
    collected.sort_by(|a, b| b.0.cmp(a.0));
    println!("Rem\tWin\tLose\tTie\tTotal");
    println!("---------------------------------------");
    for row in collected {
        println!("{}\t{}\t{}\t{}\t{}\t", row.0, row.1.2, row.1.0, row.1.1, row.1.3);
        totals.0 += row.1.0;
        totals.1 += row.1.1;
        totals.2 += row.1.2;
        totals.3 += row.1.3;
    }
    println!("---------------------------------------");
    println!("Tot\t{}\t{}\t{}\t{}\t\n", totals.2, totals.0, totals.1, totals.3);
}