extern crate bustabit;
use bustabit::Game;

#[cfg(test)]
#[test]
fn should_equal_to_provided() {
    let test = String::from("1e09741e2d57ffdf7c8620b22585416adef89898ff1903eefd7e41460ed9e628");
    let hs = Game::new(&test).unwrap();
    assert_eq!(hs.hash, test);
}

#[test]
fn should_return_none_short() {
    let test = String::from("asd");
    let hs = Game::new(&test);
    assert_eq!(hs, Option::None);
}

#[test]
fn should_return_none_invalid_symbols() {
    let test = String::from("1g09741e2h57ffdf7c8620b22585416adef89898ff1903eefd7e41460ed9e628");
    let hs = Game::new(&test);
    assert_eq!(hs, Option::None);
}

#[test]
fn should_equal_to_lowercased() {
    let test = String::from("1E09741E2d57ffdf7c8620b22585416adef89898ff1903eefd7e41460ed9e628");
    let hs = Game::new(&test).unwrap();
    assert_eq!(hs.hash, test.to_lowercase());
}

#[test]
fn should_have_next_item() {
    let s = String::from("b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba");
    let mut hs = Game::new(&s).unwrap();

    let s2 = String::from("82886b71b3b26e4b162bbdf4e7024f50f6a7250c207fb9ce497ad56a3e7e700a");
    let expected = Game::new(&s2).unwrap();
    let n = hs.next().unwrap();
    assert_eq!(expected, n);
}

#[test]
fn should_return_correct_outcome() {
    let s = vec!["b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba",
    "82886b71b3b26e4b162bbdf4e7024f50f6a7250c207fb9ce497ad56a3e7e700a",
    "cb7da3ec6cb68d4c0fccd9640641cabcac641f12ff97e7314ec3e074ac0981e0",
    "b459e199bac1342bf22d7aa6d19180aff35ab69f453d809949aab3e8d5e545aa"];

    let o = vec![1.26f64, 1.18f64, 1.26f64, 3.77f64];

    for i in 0..3 {
        let mut h = Game::new(&String::from(s[i])).unwrap();
        assert_eq!(h.outcome().unwrap(), o[i]);
    }
}
