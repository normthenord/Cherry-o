#![allow(non_snake_case)]
#[allow(dead_code)]
mod gameParts;
mod utility;

use std::collections::HashMap;
use std::time::Instant;

use utility::{calculate_statistics, high_low_total_counts, print_threshold, start_threads};

const GAME_NUM: i64 = 1_000_000;
const PLAYER_COUNT: usize = 4;

fn main() {
    let now = Instant::now();

    let counts = start_threads();

    //get high/low/total count/winning players
    let (high_count, low_count, total_count, winning_players, avg_min) = high_low_total_counts(counts.clone());

    let mut big_hash_counts = HashMap::new();
    for count in &counts {
        for (key, value) in count.3.clone().into_iter() {
            *big_hash_counts.entry(key).or_insert(0) += value;
        }
    }
    let big_hash_vec: Vec<(&i64, &i64)> = big_hash_counts.iter().collect();

    let games_played: i64 = big_hash_counts.values().sum();
    let (mean, median, mode) =
        calculate_statistics(big_hash_vec.clone(), &games_played, &total_count);

    println!(
        "Total Games Played: {}\nNumber of Players {}\nMax Rolls: {}\nFewest Rolls: {}\nAvg Rolls: {:.1}\nMedian: {}\nMost Common Result: {}: {} ({:.2}% of the time)\nAvg rolls for game to end: {}\n",
        GAME_NUM,
        PLAYER_COUNT,
        high_count,
        low_count,
        mean,
        median,
        mode.0,
        mode.1,
        mode.1 as f64/GAME_NUM as f64 * 100.0,
        avg_min);

    for (player_num, count) in winning_players.iter().enumerate(){
        println!("Player #{}: {} wins: {:.2}% of the time", player_num + 1, count, *count as f64/GAME_NUM as f64 * 100.0)
    }
    println!("");
    for num_rolls in (10..=100).step_by(10) {
        print_threshold(num_rolls, big_hash_vec.clone(), &GAME_NUM);
    }

    println!("This all took {:.2?}\n", now.elapsed());
}