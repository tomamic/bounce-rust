# bounce-rust

A first dip into rust-lang. A simplistic framework for animating the characters of some generic 2d game.

The `Actor` trait has to be implemented by each character, for acting in the game and handling collisions.

The `Arena` object manages generic characters, calling their `act` method at each turn and their `collide` method when it detects a collision.

<https://tomamic.github.io/bounce-rust/bounce.html>

**Simple exercise.** Try  and implement new actors, for the *Space Invaders* game.
