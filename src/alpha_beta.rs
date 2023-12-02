use rand::rngs::StdRng;
use rand::RngCore;

pub fn alpha_beta(is_maximizing: bool, depth: u32, alpha: u32, beta: u32, prng: &mut StdRng) -> u32 {
    if depth == 0 {
        return prng.next_u32();
    }
    
    if is_maximizing {
        let mut value = u32::MIN;
        let mut alpha = alpha;
        for _ in 0..32 {
            value = u32::max(value, alpha_beta(false, depth - 1, alpha, beta, prng));
            alpha = u32::max(alpha, value);
            if value >= beta {
                return value
            }
        }
        value
    }
    else {
        let mut value = u32::MAX;
        let mut beta = beta;
        for _ in 0..32 {
            value = u32::min(value, alpha_beta(true, depth - 1, alpha, beta, prng));
            beta = u32::min(beta, value);
            if value <= alpha{
                return value;
            }
        }
        value
    }
}