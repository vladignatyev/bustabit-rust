extern crate bustabit;

use bustabit::Game;


fn main() {
    let s = String::from("b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba");
    let mut counter = 0;

    let mut game:Game = Game::new(&s).unwrap();
    for g in game.into_iter().take(20) {
        println!("{}", g);
    }
}
