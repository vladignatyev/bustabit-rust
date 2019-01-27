#![feature(test)]

extern crate test;
extern crate bustabit;

use bustabit::Game;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_game(b: &mut Bencher) {
        b.iter(|| {
            Game::new(&String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")).unwrap();
        });
    }

    #[bench]
    fn bench_outcome(b: &mut Bencher) {
        let game = Game::new(&String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")).unwrap();
        b.iter(|| {
            game.outcome();
        });
    }
}
