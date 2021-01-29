# Development Guide

Some notes on how this library is structured/built.

## Structure

Most of the functions related to players taking their turn are not in Player,
as might be expected, but rather in Game. The main reason for this is because
the borrow checker in Rust makes it impossible to call a method on a Player
in a Game that also takes a reference to the Game as a parameter. Making the
functions part of Game and instead passing in an index referencing the player
makes the function signatures cleaner and makes it easier to implement effects
involving other players (e.g. Attack cards).

## Player Turn Order

Turn order follows increasing player index, i.e. Player 2 goes after Player 1
and sits to their left

For example, the following code loops over every player in the game, starting
with the player whose turn it is:

```rust
let player_count = game.players.len();
for i in 0..player_count {
    let index = (i + current_player_index) % player_count;

    // Do something with the player here
}
```

To skip the current player, simply change the `0..player_count` to
`1..player_count`
