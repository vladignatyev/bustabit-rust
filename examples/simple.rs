extern crate bustabit;

use bustabit::Game;

fn main() {
    let hash = String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2");
    let game = bustabit::Game::new(&hash).unwrap();
    println!("{}", game.outcome()); // will print 2055.9
}
