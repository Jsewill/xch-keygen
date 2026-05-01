use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn generate_indices(offset: u32, addresses: u32, skip: usize, random: bool, height: u32, rng: &mut ChaCha20Rng) -> Vec<u32> {
    if !random {
        (offset..offset + addresses * (skip as u32 + 1)).step_by(skip + 1).collect()
    } else {
        let mut indices: Vec<u32> = (offset..height).collect();
        indices.shuffle(rng);
        indices.truncate(addresses as usize);
        indices.sort();
        indices
    }
}
