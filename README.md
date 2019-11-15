# Bustabit Rust

`bustabit` is an utility library for verification of Bustabit games (bets and outcomes).

Bustabit is a provably fair game of luck. You can try the game on the official website: https://bustabit.com/

*“Provably fair”* means that: 1) the outcome of every game round *has not been changed*
after players placed a bet, and 2) this statement could be verified and proven
by any third-party.

Provably fair games rely on properties of cryptographic hash (one-way)
functions. Bustabit’s Proof is more complex. In addition to independence of a game
result, it proves the coefficient of game outcome, called *“a bust”*.

You may use it if you want to get historical busts, or analyze games happened previously.
Also, it may be useful while testing and debugging your auto-playing scripts.

It is a pure Rust implementation of the 3rd party verification script:
https://jsfiddle.net/Dexon95/2fmuxLza/embedded/result/

# Benchmarks
This package also contains performance benchmarks done using Criterion.
Run them using `cargo bench`.

# Examples
For simple examples run `cargo run --example nyans` or `cargo run --example simple`.

# Usage
```
extern crate bustabit;
// extern crate hex;

use bustabit::Game;


fn main() {
    // some game hash
    let s = String::from("b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba");

    // create `Game` object from hash string
    let mut game:Game = Game::new(&s).unwrap();

    // iterate through 20 games occured before game with hash 'b2acd..bdba'
    for g in game.into_iter().take(20) {
        println!("{}", g); // print the Game object
        // println!("{}", game.outcome());  // print game outcome
        // println!("{}", hex::encode(game.hash)); // print game hash of current game
    }
}
```
