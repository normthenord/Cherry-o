use indicatif::ProgressBar;

use crate::gameParts::threaded_games;
use crate::gameParts::ThreadedGame;
use std::sync::Arc;
use std::sync::Mutex;
use std::{fmt::Display, thread};

#[derive(Default)]
pub struct GameStats {
    pub game_num: usize,
    pub player_count: usize,
    pub high_count: i64,
    pub low_count: i64,
    pub total_count: i64,
    pub total_winners: Vec<i64>,
    pub avg_min: f64,
    pub mean: f64,
    pub median: i64,
    pub mode: (i64, i64),
}

impl Display for GameStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Total Games Played: {}", self.game_num)?;
        writeln!(f, "Number of Players {}", self.player_count)?;
        writeln!(f, "Max Rolls: {}", self.high_count)?;
        writeln!(f, "Fewest Rolls: {}", self.low_count)?;
        writeln!(f, "Avg Rolls: {}", self.mean)?;
        writeln!(f, "Median: {}", self.median)?;
        writeln!(
            f,
            "Most Common Result: {}: {} ({:.2}% of the time)",
            self.mode.0,
            self.mode.1,
            self.mode.1 as f64 / (self.game_num * self.player_count) as f64 * 100.0
        )?;
        writeln!(f, "Avg rolls for game to end: {}", self.avg_min)
    }
}

pub fn games_above_threshold(threshold: i64, list: Vec<(&i64, &i64)>) -> i64 {
    let mut count = 0;
    for (key, value) in list.into_iter() {
        if *key >= threshold {
            count += value;
        }
    }
    count
}

pub fn print_threshold(
    num_rolls: i64,
    big_hash_vec: Vec<(&i64, &i64)>,
    num_games: &usize,
    player_count: usize,
) {
    let count = games_above_threshold(num_rolls, big_hash_vec.clone());
    println!(
        "Games with {num_rolls} rolls or more: {} ({:.2}% of the time)",
        count,
        count as f64 / (*num_games) as f64 * 100.0 / player_count as f64
    );
}

pub fn high_low_total_counts(
    hash_list: &Vec<ThreadedGame>,
    player_count: usize,
    game_num: usize,
) -> GameStats {
    let mut high_count = 0;
    let mut low_count = i64::MAX;
    let mut total_count = 0;
    let mut total_winners = vec![0i64; player_count];
    let mut avg_min: i64 = 0;
    for count in hash_list {
        if count.high_count > high_count {
            high_count = count.high_count;
        }
        if count.low_count < low_count {
            low_count = count.low_count;
        }
        total_count  += count.total_count;
        
        for (player_num, game) in count.player_winners.iter().enumerate() {
            total_winners[player_num] += game.1;
        }
        for min in &count.min_rolls_to_win {
            avg_min += min;
        }
    }
    let avg_min = avg_min as f64 / game_num as f64;

    GameStats {
        high_count,
        low_count,
        total_count,
        total_winners,
        avg_min,
        player_count,
        game_num,
        ..Default::default()
    }
}

pub fn calculate_statistics(
    big_hash_vec: Vec<(&i64, &i64)>,
    game_played: &i64,
    game_stats: &mut GameStats,
) {
    let mut mode_vec = big_hash_vec.clone();
    mode_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mode = mode_vec[0];

    game_stats.mode.0 = *mode.0;
    game_stats.mode.1 = *mode.1;
    game_stats.mean = game_stats.total_count as f64 / *game_played as f64;

    game_stats.median = median_calc(game_played, big_hash_vec);
}

fn median_calc(num_games: &i64, mut list: Vec<(&i64, &i64)>) -> i64 {
    let mut c = 0;
    let mut median: i64 = 0;
    list.sort_by_key(|k| *k);
    for (k, v) in list.into_iter() {
        c += v;
        if c > num_games / 2 {
            median = *k;
            break;
        }
    }
    median
}

pub fn calcuate_winner(player_vec: &[i64]) -> Option<isize> {
    let min_value = player_vec.iter().min().unwrap();
    for (index, score) in player_vec.iter().enumerate() {
        if score == min_value {
            return Some(index as isize);
        }
    }
    None
}

pub fn start_threads(player_count: usize, num_games: &usize) -> Vec<ThreadedGame> {
    let num_cores = thread::available_parallelism().unwrap().get();
    let num_games_per_thread = num_games / num_cores;
    let extra_games = num_games % num_cores;

    let pb = ProgressBar::new(*num_games as u64);
    let pb_mutex = Arc::new(Mutex::new(pb));
    let mut handles = vec![];
    for x in 0..num_cores {
        let threaded_pb = Arc::clone(&pb_mutex);
        if x == 0 {
            let handle = thread::spawn(move || {
                threaded_games(
                    num_games_per_thread + extra_games,
                    player_count,
                    &threaded_pb,
                )
            });
            handles.push(handle);
        } else {
            let handle = thread::spawn(move || {
                threaded_games(num_games_per_thread, player_count, &threaded_pb)
            });
            handles.push(handle);
        }
    }

    let mut counts = vec![];
    for handle in handles {
        counts.push(handle.join().expect("Oops"));
    }
    counts
}
