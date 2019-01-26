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

#[derive(Debug)]
pub struct Game {
    pub hash: String,
    _outcome: Option<f64>
}

impl Game {
    pub fn new(s: &String) -> Option<Game> {
        if s.len() != 64 {
            return None
        }

        match hex::decode(s) {
            Ok(val) => {
                Some(Game {
                    hash: s.to_lowercase(),
                    _outcome: Option::None
                })
            }
            Err(err) => {
                None
            }
        }
    }

    pub fn outcome(&mut self) -> Option<f64> {
        // Memoization
        if self._outcome != Option::None {
            return self._outcome;
        }

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

        // truncate and store calculated value
        let o = result.max(1.0f64);
        self._outcome = Some(o);

        // return value of outcome
        self._outcome
    }
}

impl PartialEq for Game {
    fn eq(self: &Self, another: &Game) -> bool {
        self.hash == another.hash
    }
}

impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        let mut hasher = Sha256::new();
        hasher.input(self.hash.clone());
        let result = hasher.result();

        Game::new(&format!("{:x}", result))
    }
}
