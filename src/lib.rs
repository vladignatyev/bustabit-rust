//! # Bustabit
//!
//! `bustabit` is an utility library for verification of Bustabit games.
//!
//! Bustabit is a provably fair game of luck. “Provably fair” means that: 1) the outcome of every
//! game round has not been changed after players placed a bet, and 2) this statement
//! could be verified and proven by any third-party.
//! Provably fair games rely on properties of cryptographical hash (one-way) functions.
//! Bustabit’s Proof is proving additionally to independence of game outcome, the coefficient of outcome
//! (called *a bust*) itself.
//!
//! You may use it if you want to get historical busts, or analyze games happened previously.
//! Also, it may be useful while testing and debugging you autoplaying scripts.
//!
//! It is a pure Rust implementation of the 3rd party verification script: https://jsfiddle.net/Dexon95/2fmuxLza/embedded/result/


#![allow(unused)]

extern crate sha2;
extern crate hmac;
extern crate hex;
extern crate byteorder;

use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
type HmacSha256 = Hmac<Sha256>;

use std::i32;
use std::fmt;


#[derive(Debug)]
pub struct Game {
    pub hash: String
}

impl Game {
    /// Creates new game from Game's hash
    /// # Examples
    ///
    /// ```
    /// let hash = String::from("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2");
    /// let game = bustabit::Game::new(&hash).unwrap();
    /// println!("{}", game.outcome()) // will print 2055.9
    /// ```
    pub fn new(s: &String) -> Option<Game> {
        if s.len() != 64 {
            return None
        }

        match hex::decode(s) {
            Ok(val) => {
                Some(Game {
                    hash: s.to_lowercase()
                })
            }
            Err(err) => {
                None
            }
        }
    }

    /// Calculates and returns and outcome of the game
    pub fn outcome(&self) -> f64 {
        // Bustabit seeding data
        const SEED:&'static [u8; 64] = b"0000000000000000004d6ec16dafe9d8370958664c1dc422f452892264c59526";
        const NUM_BITS:u32 = 52;

        // size of u64 type in bits
        const U64_SIZE:u32 = 64;

        // create HMAC-SHA256 with salt = SEED
        let mut mac = HmacSha256::new_varkey(SEED).expect("HMAC should accept Seed as key");
        // push Game hash as input
        mac.input(&hex::decode(&self.hash).unwrap());
        let result = mac.result();
        let code_bytes = result.code();

        // retrieving 52 most significant bits
        let mut rdr = Cursor::new(code_bytes);
        let mut r:u64 = (rdr.read_u64::<BigEndian>().unwrap() & 0b11111111_11111111_11111111_11111111_11111111_11111111_11110000_00000000) >> (U64_SIZE - NUM_BITS);

        // calculate outcome of the game
        let x1 = r as f64 / 2u64.pow(NUM_BITS) as f64;
        let x = 99.0f64 / (1.0f64 - x1);
        let result:f64 = x.floor() * 0.01f64;

        // truncate and return calculated value
        result.max(1.0f64)
    }
}

impl PartialEq for Game {
    fn eq(self: &Self, another: &Game) -> bool {
        self.hash == another.hash
    }
}

/// Iterator implementation that helps to navigate previous Bustabit games,
/// so you can verify multiple games!
/// # Examples
/// ```
/// # let hash = String::from("b2acd37fbdb5509926ab5d7329704c840f8467266c90019682f3b260a029bdba");
/// let mut latest_game = bustabit::Game::new(&hash).unwrap();
/// let mut previous_game = latest_game.next().unwrap();
/// let one_before_previous = previous_game.next().unwrap();
/// ```
impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        let mut hasher = Sha256::new();
        hasher.input(self.hash.clone());
        let result = hasher.result();

        Game::new(&format!("{:x}", result))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game’s hash: {} Bust: {}", self.hash, self.outcome())
    }
}
