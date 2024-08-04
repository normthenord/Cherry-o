use std::{collections::HashMap, i64::MAX};

pub fn games_above_threshold(threshold: i64, list: Vec<(&i64, &i64)>) -> i64 {
    let mut count = 0;
    for (key, value) in list.into_iter() {
        if *key >= threshold {
            count += value;
        }
    }
    count
}

pub fn print_threshold(num_rolls: i64, big_hash_vec: Vec<(&i64, &i64)>, num_games: &i64) {
    let count = games_above_threshold(num_rolls, big_hash_vec.clone());
    println!(
        "Games with {num_rolls} rolls or more: {} ({:.2}% of the time)",
        count,
        count as f64 / (*num_games) as f64 * 100.0
    );
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

pub fn high_low_total_counts(
    hash_list: Vec<(i64, i64, i64, HashMap<i64, i64>)>,
) -> (i64, i64, i64) {
    let mut high_count = 0;
    let mut low_count = MAX;
    let mut total_count = 0;
    for count in &hash_list {
        if count.0 > high_count {
            high_count = count.0;
        }
        if count.1 < low_count {
            low_count = count.1
        }
        total_count = total_count + count.2;
    }
    (high_count, low_count, total_count)
}

pub fn calculate_statistics(
    big_hash_vec: Vec<(&i64, &i64)>,
    game_played: &i64,
    total_count: &i64,
) -> (f64, i64, (i64, i64)) {
    let mut mode_vec = big_hash_vec.clone();
    mode_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mode = mode_vec[0];

    let mean = *total_count as f64 / *game_played as f64;

    let median = median_calc(game_played, big_hash_vec);

    (mean, median, (*mode.0, *mode.1))
}
