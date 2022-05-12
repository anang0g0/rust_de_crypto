//use rand_chacha;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;
use rand::rngs::OsRng;
use rand::rngs::adapter::ReseedingRng;
use rand::prelude::*;
use rand_chacha::ChaCha20Core; 

fn generate_random_numbers_with_a_seed(seed : u64) {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    println!("Deterministic Random numbers with seed:{}", rng.next_u32());
}

fn main(){
generate_random_numbers_with_a_seed(200); 
//Result : -416273517
let prng = ChaCha20Core::from_entropy();
let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng); //Reseeding
println!("Random number: {}", reseeding_rng.gen::<u64>());
}