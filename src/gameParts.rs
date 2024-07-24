use std::{i64::MAX, sync::Arc};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::BTreeMap;

#[derive(Debug)]
enum RollOption {
    OneCherry,
    TwoCherry,
    ThreeCherry,
    FourCherry,
    Bird,
    Dog,
    OopsNoCherries,
}

pub struct Game {
    count: i64,
    cherries: i64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            count: 0,
            cherries: 10,
        }
    }

    fn game(&mut self) -> i64 {
        loop {
            let roll: RollOption = rand::random();
            self.count = self.count + 1;
            match roll {
                RollOption::OneCherry => self.cherries = self.cherries - 1,
                RollOption::TwoCherry => self.cherries = self.cherries - 2,
                RollOption::ThreeCherry => self.cherries = self.cherries - 3,
                RollOption::FourCherry => self.cherries = self.cherries - 4,
                RollOption::Bird => self.cherries = self.cherries + 2,
                RollOption::Dog => self.cherries = self.cherries + 2,
                RollOption::OopsNoCherries => self.cherries = 10,
            }

            if self.cherries > 10 {
                self.cherries = 10;
            }
            if self.cherries <= 0 {
                break;
            }
        }
        self.count
    }
}

impl Distribution<RollOption> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RollOption {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=7) {
            // rand 0.8
            0 => RollOption::OopsNoCherries,
            1 => RollOption::OneCherry,
            2 => RollOption::TwoCherry,
            3 => RollOption::ThreeCherry,
            4 => RollOption::FourCherry,
            5 => RollOption::Bird,
            _ => RollOption::Dog,
        }
    }
}

// fn game() -> i64 {
//     let mut cherries: i64 = 10;
//     let mut count = 0;
//     loop {
//         let roll: RollOption = rand::random();
//         // println!("{:#?}", roll);
//         count = count + 1;
//         match roll {
//             RollOption::OneCherry => cherries = cherries - 1,
//             RollOption::TwoCherry => cherries = cherries - 2,
//             RollOption::ThreeCherry => cherries = cherries - 3,
//             RollOption::FourCherry => cherries = cherries - 4,
//             RollOption::Bird => cherries = cherries + 2,
//             RollOption::Dog => cherries = cherries + 2,
//             RollOption::OopsNoCherries => cherries = 10,
//         }

//         if cherries > 10 {
//             cherries = 10;
//         }
//         if cherries <= 0 {
//             break;
//         }
//     }
//     count
// }

pub fn threaded_games(num: i64) -> (i64, i64, i64, BTreeMap<i64, i64>) {
    let mut high_count = 0;
    let mut low_count = MAX;
    let mut total_count = 0;
    let mut hash_counts = BTreeMap::new();

    for _ in 1..num {
        // let game_count = game();
        let mut game = Game::new();
        let game_count = game.game();
        if hash_counts.contains_key(&game_count) {
            hash_counts.insert(game_count, hash_counts.get(&game_count).unwrap() + 1);
        } else {
            hash_counts.insert(game_count, 1 as i64);
        }
        total_count = total_count + game_count;
        if game_count > high_count {
            high_count = game_count;
        }
        if game_count < low_count {
            low_count = game_count;
        }
    }
    // println!("{:?}", hash_counts);
    (high_count, low_count, total_count, hash_counts)
}
