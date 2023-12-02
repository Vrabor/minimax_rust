use rand::rngs::StdRng;
use rand::RngCore;

pub fn minimax(is_maximizing: bool, depth: u32, best_seen: u32, prng: &mut StdRng) -> u32 {
    if depth == 0 {
        return prng.next_u32();
    }
    
    if is_maximizing {
        let mut max = 0;
        for _ in 0..32 {
            let eval = minimax(false, depth - 1, max, prng);
            if eval >= best_seen {
                return eval;
            }
            if eval > max {
                max = eval;
            }
        }
        max
    }
    else {
        let mut min = u32::MAX;
        for _ in 0..32 {
            let eval = minimax(true, depth - 1, min, prng);
            if eval <= best_seen {
                return eval;
            }
            if eval < min {
                min = eval;
            }
        }
        min
    }
}