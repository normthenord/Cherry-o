use std::{collections::HashMap, i64::MAX};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::GAME_NUM;

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

pub fn multiple_players(num_players: usize) {
    let game_num = GAME_NUM.clamp(0, 1_000_000);
    let mut winners_counts = HashMap::new();
    for _ in 0..game_num {
        let mut player_vec = vec![Game::new(); num_players];

        let player_vec = player_vec
            .iter_mut()
            .map(|player| player.game())
            .collect::<Vec<_>>();

        *winners_counts
            .entry(crate::utility::calcuate_winner(&player_vec[..]).expect("No winner? Bug!"))
            .or_insert(0) += 1;
    }

    let mut winners_counts = winners_counts
        .iter()
        .map(|(index, count)| (format!("Player {}", index + 1), count))
        .collect::<Vec<_>>();

    winners_counts.sort();
    for (name, count) in winners_counts {
        println!(
            "{}: {}  -> Wins {:.2}% of the time",
            name,
            count,
            *count as f64 / game_num as f64 * 100.0
        );
    }

    // println!("{:?}", winners_counts);
}
