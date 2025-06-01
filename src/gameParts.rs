use std::{
    collections::HashMap,
    i64,
    sync::{Arc, Mutex},
};

use indicatif::{self, ProgressBar};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

pub struct ThreadedGame {
    pub high_count: i64,
    pub low_count: i64,
    pub total_count: i64,
    pub hash_counts: HashMap<i64, i64>,
    pub player_winners: Vec<(isize, i64)>,
    pub min_rolls_to_win: Vec<i64>,
}

impl Default for ThreadedGame {
    fn default() -> Self {
        Self {
            high_count: Default::default(),
            low_count: i64::MAX,
            total_count: Default::default(),
            hash_counts: Default::default(),
            player_winners: Default::default(),
            min_rolls_to_win: Default::default(),
        }
    }
}

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
        let mut rng = thread_rng();
        loop {
            self.count += 1;
            match rng.gen::<RollOption>() {
                RollOption::OneCherry => self.cherries -= 1,
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

pub fn threaded_games(
    num: usize,
    player_count: usize,
    pb: &Arc<Mutex<ProgressBar>>,
) -> ThreadedGame {
    let mut threaded_game = ThreadedGame::default();
    let mut winner_counts = HashMap::new();
    threaded_game.min_rolls_to_win = Vec::with_capacity(num);

    let mut count = 0;
    for _idx in 0..num {
        let mut player_vec = Vec::new();
        for _ in 0..player_count {
            player_vec.push(Game::new().game());
        }

        for &player in &player_vec {
            *threaded_game.hash_counts.entry(player).or_insert(0) += 1;
            threaded_game.total_count += player;

            threaded_game.high_count = threaded_game.high_count.max(player);

            threaded_game.low_count = threaded_game.low_count.min(player);
        }

        if let Some(min) = player_vec.iter().min() {
            threaded_game.min_rolls_to_win.push(*min);
        }

        *winner_counts
            .entry(crate::utility::calculate_winner(&player_vec[..]).expect("No winner? Bug!"))
            .or_insert(0) += 1;

        count += 1;
        if count % 1000 == 0 {
            if let Ok(pb) = pb.try_lock() {
                pb.inc(count);
                count = 0;
            }
        }
    }
    // println!("{:?}", winner_counts);
    threaded_game.player_winners = winner_counts.into_iter().collect();
    threaded_game.player_winners.sort();
    pb.lock().unwrap().inc(count);
    threaded_game
}
