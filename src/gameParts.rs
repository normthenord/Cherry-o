use std::collections::HashMap;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub struct ThreadedGame {
    pub high_count: i64,
    pub low_count: i64,
    pub total_count: i64,
    pub hash_counts: HashMap<i64, i64>,
    pub player_winners: Vec<(isize, i64)>,
    pub min_rolls_to_win: Vec<i64>,
}
// use crate::PLAYER_COUNT;

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
#[derive(Debug, Clone, Copy)]
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
            self.count += 1;
            match rand::random::<RollOption>() {
                RollOption::OneCherry => self.cherries  -= 1,
                RollOption::TwoCherry => self.cherries -= 2,
                RollOption::ThreeCherry => self.cherries -= 3,
                RollOption::FourCherry => self.cherries -= 4,
                RollOption::Bird => self.cherries += 2,
                RollOption::Dog => self.cherries += 2,
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

pub fn threaded_games(num: usize, player_count: usize) -> ThreadedGame {
    let mut high_count = 0;
    let mut low_count = i64::MAX;
    let mut total_count = 0;
    let mut hash_counts = HashMap::new();
    let mut winner_counts = HashMap::new();
    let mut min_rolls_to_win = Vec::new();

    for _ in 0..num {
        let mut player_vec = Vec::new();
        for _ in 0..player_count {
            player_vec.push(Game::new().game());
        }

        for player in player_vec.clone() {
            *hash_counts.entry(player).or_insert(0) += 1;
            total_count += player;
            if player > high_count {
                high_count = player;
            }
            if player < low_count {
                low_count = player;
            }
        }
        let player_vec = player_vec.clone().into_iter().collect::<Vec<i64>>();
        min_rolls_to_win.push(*player_vec.iter().min().unwrap());

        *winner_counts
            .entry(crate::utility::calcuate_winner(&player_vec[..]).expect("No winner? Bug!"))
            .or_insert(0) += 1;
    }
    // println!("{:?}", winner_counts);
    let mut player_winners: Vec<(isize, i64)> = winner_counts.into_iter().collect();
    player_winners.sort();
    ThreadedGame {
        high_count,
        low_count,
        total_count,
        hash_counts,
        player_winners,
        min_rolls_to_win,
    }
}
