#![allow(non_snake_case)]
mod gameParts;
mod utility;

use std::collections::HashMap;
use std::time::Instant;

use std::thread;

use gameParts::threaded_games;

use utility::{games_above_threshold, high_low_total_counts, median_calc, print_threshold};

fn main() {
    let num_cores = thread::available_parallelism().unwrap().get() as i64;
    let now = Instant::now();
    let num_games = 100_000_000;
    let num_games_per_thread = num_games / num_cores;
    let extra_games = num_games % num_cores;

    let mut handles = vec![];

    for x in 1..num_cores + 1 {
        if x == 1 {
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

    //get high/low/total count
    let (high_count, low_count, total_count) = high_low_total_counts(counts.clone());

    let mut big_hash_counts = HashMap::new();
    for count in &counts {
        for (key, value) in count.3.clone().into_iter() {
            *big_hash_counts.entry(key).or_insert(value) += value;
        }
    }

    let mut big_hash_vec: Vec<(&i64, &i64)> = big_hash_counts.iter().collect();

    let mut mode_vec = big_hash_vec.clone();
    mode_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mode = mode_vec[0];
    big_hash_vec.sort_by_key(|k| *k);

    // let hundred_count = games_above_threshold(100, big_hash_vec.clone());
    // let fifty_count = games_above_threshold(50, big_hash_vec.clone());
    // let twentyfive_count = games_above_threshold(25, big_hash_vec.clone());
    // let fifteen_count = games_above_threshold(15, big_hash_vec.clone());

    let median = median_calc(&num_games, big_hash_vec.clone());

    println!(
        "Max Rolls: {}\nFewest Rolls: {}\nAvg Rolls: {}\nMedian: {}\nMost Common Result: {}: {} ({}% of the time)\n",
        high_count,
        low_count,
        total_count as f64 / num_games as f64,
        median,
        mode.0,
        mode.1,
        *mode.1 as f64/num_games as f64 * 100.0,);

    for num_rolls in (10..=100).step_by(10){
        print_threshold(num_rolls, big_hash_vec.clone(), &num_games);
    }
    
    println!("This all took {:.2?}", now.elapsed())
}
