use std::{collections::HashMap, i64::MAX};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
    pub fn new() -> Self {
        Self {
            count: 0,
            cherries: 10,
        }
    }

    fn game(&mut self) -> i64 {
        loop {
            self.count = self.count + 1;
            match rand::random::<RollOption>() {
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
        match rng.gen_range(0..7) {
            0 => RollOption::OopsNoCherries,
            1 => RollOption::OneCherry,
            2 => RollOption::TwoCherry,
            3 => RollOption::ThreeCherry,
            4 => RollOption::FourCherry,
            5 => RollOption::Bird,
            6 => RollOption::Dog,
            _ => unreachable!("Shouldn't ever roll anything else"),
        }
    }
}

pub fn threaded_games(num: i64) -> (i64, i64, i64, HashMap<i64, i64>) {
    let mut high_count = 0;
    let mut low_count = MAX;
    let mut total_count = 0;
    let mut hash_counts = HashMap::new();

    for _ in 0..num {
        let mut game = Game::new();
        let game_count = game.game();
        *hash_counts.entry(game_count).or_insert(0) += 1;
        total_count = total_count + game_count;
        if game_count > high_count {
            high_count = game_count;
        }
        if game_count < low_count {
            low_count = game_count;
        }
    }

    (high_count, low_count, total_count, hash_counts)
}
