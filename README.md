# Bustabit Rust

`bustabit` is an utility library for verification of Bustabit games (bets and outcomes).

Bustabit is a provably fair game of luck. You can try the game on the official website: https://bustabit.com/

*“Provably fair”* means that: 1) the outcome of every game round *has not been changed*
after players placed a bet, and 2) this statement could be verified and proven
by any third-party.

Provably fair games rely on properties of cryptographic hash (one-way)
functions. Bustabit’s Proof is more complex. In addition to independence of a game
result, it proves the coefficient of game outcome *a bust*.

You may use it if you want to get historical busts, or analyze games happened previously.
Also, it may be useful while testing and debugging you autoplaying scripts.

It is a pure Rust implementation of the 3rd party verification script:
https://jsfiddle.net/Dexon95/2fmuxLza/embedded/result/

# Package

For examples try `cargo run --example nyans` or `cargo run --example simple`.
This package also contains performance benchmarks. To run them, consider using a *nightly build* of _rustc_.
