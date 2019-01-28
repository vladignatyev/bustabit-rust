extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

extern crate bustabit;

use bustabit::Game;

fn bench(c: &mut Criterion) {
    let game2 = Game::new(&String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")).unwrap();
    let game3 = Game::new(&String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")).unwrap();

    c.bench_function("game_constructor", |b| {
        b.iter(|| {
            Game::new(&String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2"));
        });
    });

    c.bench_function("game_outcome", |b| {
        let game = Game::new(&String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")).unwrap();
        b.iter(move || {
            game.outcome();
        });
    });

    c.bench_function("game_clone", move |b| {
        b.iter(|| {
            let _g = game2.clone();
        });
    });

    c.bench_function("game_iterator", move |b| {
        let mut _g = game3.clone().into_iter();

        b.iter(|| {
            _g.next();
        });
    });

}

criterion_group!(benches, bench);
criterion_main!(benches);
