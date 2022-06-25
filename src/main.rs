mod minimax_better_rng;
mod minimax_original;
mod minimax_traditional;
mod alpha_beta;
use rand::prelude::*;
use std::time::SystemTime;

fn main() {
    let seed : [u8; 32]= [2, 42, 13, 15, 55, 75, 99, 34, 76, 63, 77, 49, 33, 55, 37, 19,
                          128, 127, 133, 149, 99, 145, 233, 207, 108, 77, 37, 233, 197, 1, 5, 7];
    let mut rng = StdRng::from_seed(seed);

    let now = SystemTime::now();
    println!("original better rng: {}", minimax_better_rng::minimax(true, 6, u32::MAX, &mut rng));
    println!("time elapsed: {}", now.elapsed().unwrap().as_millis());

    rng = StdRng::from_seed(seed); // reseed to get reproducable results

    let now = SystemTime::now();
    println!("alpha-beta: {}", alpha_beta::alpha_beta(true, 6, u32::MIN, u32::MAX, &mut rng));
    println!("time elapsed: {}", now.elapsed().unwrap().as_millis());

    rng = StdRng::from_seed(seed); // reseed to get reproducable results

    let now = SystemTime::now();
    println!("reimplementation: {}", minimax_traditional::minimax(true, 6, &mut rng));
    println!("time elapsed: {}", now.elapsed().unwrap().as_millis());

    /* Takes forever on my system
    let now = SystemTime::now();
    println!("original: {}", minimax_original::minimax(true, 6, u32::MAX));
    println!("time elapsed: {}", now.elapsed().unwrap().as_millis());
    */
}
