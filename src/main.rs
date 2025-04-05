#![allow(non_snake_case)]
#[allow(dead_code)]
mod gameParts;
mod utility;

use std::collections::HashMap;
use std::time::Instant;

use utility::{calculate_statistics, high_low_total_counts, print_threshold, start_threads};

const GAME_NUM: usize = 100_000_000;
const PLAYER_COUNT: usize = 2;
fn main() {
    let now = Instant::now();
    //get player count from args. Default to const in file if not supplied
    let player_count = match std::env::args().nth(1) {
        Some(count) => count.parse::<usize>().unwrap(),
        None => PLAYER_COUNT,
    };

    let game_num = match std::env::args().nth(2) {
        Some(count) => count.parse::<usize>().unwrap(),
        None => GAME_NUM,
    };

    let counts = start_threads(player_count.clone(), &game_num);

    //get high/low/total count/winning players
    let mut game_stats = high_low_total_counts(&counts, player_count.clone(), game_num);

    let mut big_hash_counts = HashMap::new();
    for count in &counts {
        for (key, value) in count.hash_counts.clone().into_iter() {
            *big_hash_counts.entry(key).or_insert(0) += value;
        }
    }
    let big_hash_vec: Vec<(&i64, &i64)> = big_hash_counts.iter().collect();

    let games_played: i64 = big_hash_counts.values().sum();
    calculate_statistics(
        big_hash_vec.clone(),
        &games_played,
        &mut game_stats,
        player_count,
    );

    // PRINT STATS
    println!("{game_stats}");
    for (player_num, count) in game_stats.total_winners.iter().enumerate() {
        println!(
            "Player #{}: {} wins: {:.2}% of the time",
            player_num + 1,
            count,
            *count as f64 / game_num as f64 * 100.0
        )
    }
    println!("");
    for num_rolls in (10..=100).step_by(10) {
        print_threshold(num_rolls, big_hash_vec.clone(), &game_num, player_count);
    }

    println!("This all took {:.2?}", now.elapsed());
}
