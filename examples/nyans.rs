extern crate bustabit;

use bustabit::Game;


fn main() {
    let s = String::from("b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba");
    let mut counter = 0;

    let game:Game = Game::new(&s).unwrap();
    let mut iter = game.into_iter();
    loop {
        let g = iter.next().unwrap();
        if g.outcome() > 1000.0f64 {
            println!("{}", g);
            counter = counter + 1;
        }

        if counter > 100 { break; }
    }
}
