# bounce-rust

A first dip into rust-lang. A simplistic framework for animating the characters of some generic 2d game.

The `Actor` trait has to be implemented by each character, for acting in the game and handling collisions.

The `Arena` object manages generic characters, calling their `act` method at each turn and their `collide` method when it detects a collision.

- [Main abstractions](https://github.com/tomamic/bounce-rust/blob/main/src/actor.rs)
- [Example characters](https://github.com/tomamic/bounce-rust/blob/main/src/bounce.rs)
- [Running example (built to WASM)](https://tomamic.github.io/bounce-rust/bounce.html)

## Simple exercise

Try  and implement new actors, for the *Space Invaders* game 👾
