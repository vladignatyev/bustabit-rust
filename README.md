# Bustabit Rust

`bustabit` is an utility library for verification of Bustabit games (bets and outcomes).

Bustabit is a provably fair game of luck. You can try it on official website: https://bustabit.com/
“Provably fair” means that: 1) the outcome of every game round has not been changed
after players placed a bet, and 2) this statement could be verified and proven
by any third-party.

Provably fair games rely on properties of cryptographical hash (one-way)
functions. Bustabit’s Proof is proving in addition to independence of a game
outcome, the coefficient of game outcome (called *a bust*) itself.

You may use it if you want to get historical busts, or analyze games happened previously.
Also, it may be useful while testing and debugging you autoplaying scripts.

It is a pure Rust implementation of the 3rd party verification script located at
https://jsfiddle.net/Dexon95/2fmuxLza/embedded/result/
