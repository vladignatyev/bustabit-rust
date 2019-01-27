extern crate bustabit;

use bustabit::Game;


fn main() {
    let s = String::from("b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba");
    let mut counter = 0;

    let mut game:Game = Game::new(&s).unwrap();
    loop {
        if game.outcome() > 1000.0f64 {
            println!("{}", game);
            counter = counter + 1;
        }

        game = game.next().unwrap();

        if counter > 100 { break; }
    }
}
