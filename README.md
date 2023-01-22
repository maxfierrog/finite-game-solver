# Playing with finite games

Are finite games really games? Is life deterministic? Do I exist? Answer these and many other questions! Implement your own finite state games and then solve them! This is a small project which gently pokes the surface of computational game theory. 

## Description

**What is a finite state game?** 

Think Tic-Tac-Toe, Connect-4, Chess, Checkers, and many other games in which a finite amount of players take turns knowing all the information about the state of the game, and make deterministic moves to alter said state.

**What does it mean to solve such a game?** 

It boils down to being able to tell, given any possible state of the game, whether you will win, lose, or tie provided that everyone plays perfectly.

This project provides the following:
* A `Game` interface (or trait, as the crabs call it) which comes with a generic `solver`. You can use this solver to solve any game.
* A module with a few games which implement said interface, and may additionally implement more efficient game-specific solvers.
* A (not-yet-existent) UI module, which helps us actually play the games imperfectly as humans.

## TODO:

- Generic solver: Make this function iterative to avoid memory overhead and stack overflows. Improve memory performance. Add option to prune pointless states.
- UI: Implement a basic way to choose which game you want to play. Implement a way to play a generic game. Implement human vs. AI matches.
- Games: Tic-Tac-Toe doesn't seem to bad, I guess.
