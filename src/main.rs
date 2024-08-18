#![allow(non_snake_case)]
#[allow(dead_code)]
mod gameParts;
mod utility;

use std::collections::HashMap;
use std::time::Instant;

use std::thread;

use gameParts::threaded_games;

use utility::{calculate_statistics, high_low_total_counts, print_threshold};

const GAME_NUM: i64 = 1_000_000_00;
const PLAYER_COUNT: usize = 8;

fn main() {
    let num_cores = thread::available_parallelism().unwrap().get() as i64;
    let now = Instant::now();
    let num_games = GAME_NUM;
    let num_games_per_thread = num_games / num_cores;
    let extra_games = num_games % num_cores;

    let mut handles = vec![];

    for x in 0..num_cores {
        if x == 0 {
            let handle = thread::spawn(move || threaded_games(num_games_per_thread + extra_games));
            handles.push(handle);
        } else {
            let handle = thread::spawn(move || threaded_games(num_games_per_thread));
            handles.push(handle);
        }
    }

    //bring the threads back together
    let mut counts = vec![];
    for handle in handles {
        counts.push(handle.join().unwrap());
    }



    //get high/low/total count/winning players
    let (high_count, low_count, total_count, winning_players) = high_low_total_counts(counts.clone());

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

    assert_eq!(num_games * PLAYER_COUNT as i64, games_played);
    println!(
        "Total Games Played: {}\nNumber of Players {}\nMax Rolls: {}\nFewest Rolls: {}\nAvg Rolls: {:.1}\nMedian: {}\nMost Common Result: {}: {} ({:.2}% of the time)\n",
        GAME_NUM,
        PLAYER_COUNT,
        high_count,
        low_count,
        mean,
        median,
        mode.0,
        mode.1,
        mode.1 as f64/num_games as f64 * 100.0,);

    for (player_num, count) in winning_players.iter().enumerate(){
        println!("Player #{}: {} wins: {:.2}% of the time", player_num, count, *count as f64/GAME_NUM as f64 * 100.0)
    }
    println!("");
    for num_rolls in (10..=100).step_by(10) {
        print_threshold(num_rolls, big_hash_vec.clone(), &num_games);
    }

    println!("This all took {:.2?}\n", now.elapsed());

    // gameParts::multiples();
}