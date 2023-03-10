# Playing with finite games

Are finite games really games? Is life deterministic? Do I exist? Answer these and many other questions! Implement your own finite state games and then solve them! This is a small project which gently pokes the surface of computational game theory.

## Description

**What is a finite state game?** 

Think Tic-Tac-Toe, Connect-4, Chess, Checkers, and many other games in which a finite amount of players take turns knowing all the information about the state of the game, and make deterministic moves to alter said state.

**What does it mean to solve such a game?** 

It boils down to being able to tell, given any possible state of the game, whether you will win, lose, or tie provided that everyone plays perfectly.

This project provides the following:
* A `Game` interface (or trait, as the crabs call it) which comes with a generic `solver`. You can use this solver to solve any game.
* A module with a few games which implement said interface.
* A (not-yet-existent) UI module, which helps us actually play the games imperfectly as humans.

## Solved games

- N-to-0-by-1-or-2: Two players take turns removing one or two coins from a pool of N coins. The person who removes the last coin(s) wins.
- N-to-0-by-1-3-or-4: Two players take turns removing one, three, or four coins from a pool of N coins. The person who removes the last coin(s) wins.
- NxM Tic-Tac-Toe: The same thing as Tic-Tac-Toe, but with variable length and width. The regular 3x3 tic-tac-toe is a special case of this one. Performing full analyses starts getting impractical for N and M above 6 or 7.

## Development notes

- This project is currently barebones, and has no UI

## TODO

- Generic solver: Make this function iterative to avoid memory overhead and stack overflows. Improve memory performance. Add option to prune pointless states.
- UI: Implement a basic way to choose which game you want to play. Implement a generic graphic user interface. Implement human-involved matches.
