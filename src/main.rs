#![allow(non_snake_case)]
#![allow(dead_code)]
mod gameParts;
mod utility;

use std::collections::HashMap;
use std::time::Instant;

use clap::Parser;
use utility::{calculate_statistics, high_low_total_counts, print_threshold, start_threads, print_title};

const GAME_NUM: usize = 100_000_000;
const PLAYER_COUNT: usize = 2;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    player_count: Option<usize>,
    #[arg(short, long)]
    game_num: Option<usize>,
}

fn main() {
    let now = Instant::now();
    
    //Get CLI inputs
    let cli = Cli::parse();
    let player_count = cli.player_count.unwrap_or(PLAYER_COUNT);
    let game_num = cli.game_num.unwrap_or(GAME_NUM);

    //Print info about the game
    print_title(game_num, player_count);


    //Run the simulation
    let counts = start_threads(player_count, game_num);

    //get high/low/total count/winning players
    let mut game_stats = high_low_total_counts(&counts, player_count, game_num);

    let mut big_hash_counts = HashMap::new();
    for count in &counts {
        for (key, value) in &count.hash_counts {
            *big_hash_counts.entry(*key).or_insert(0) += value;
        }
    }
    let games_played: i64 = big_hash_counts.values().sum();
    let big_hash_vec: Vec<(i64, i64)> = big_hash_counts.into_iter().collect();


    calculate_statistics(big_hash_vec.clone(), games_played, &mut game_stats);

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
    println!();
    for num_rolls in (10..=100).step_by(10) {
        print_threshold(num_rolls, big_hash_vec.clone(), &game_num, player_count);
    }

    println!("This all took {:.2?}", now.elapsed());
}
