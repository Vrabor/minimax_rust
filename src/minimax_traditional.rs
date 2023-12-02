use rand::rngs::StdRng;
use rand::RngCore;

pub fn minimax(is_maximizing: bool, depth: u32, prng: &mut StdRng) -> u32 {
    if depth == 0 {
        return prng.next_u32();
    }
    
    if is_maximizing {
        let mut value = u32::MIN;
        for _ in 0..32 {
            value = u32::max(value, minimax(false, depth - 1, prng));
        }
        value
    }
    else {
        let mut value = u32::MAX;
        for _ in 0..32 {
            value = u32::min(value, minimax(true, depth - 1, prng));
        }
        value
    }
}