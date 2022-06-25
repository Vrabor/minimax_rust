mod minimax_better_rng;
mod minimax_original;
use rand::prelude::*;
use std::time::{Duration, SystemTime};

fn main() {
    let mut rng = StdRng::from_entropy();

    let now = SystemTime::now();
    println!("better rng: {}", minimax_better_rng::minimax(true, 6, u32::MAX, &mut rng));
    println!("time elapsed: {}", now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    println!("better rng: {}", minimax_original::minimax(true, 6, u32::MAX));
    println!("time elapsed: {}", now.elapsed().unwrap().as_millis());
}
