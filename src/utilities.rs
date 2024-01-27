use std::env::args;

use rand::prelude::*;

pub fn generate_16rng_seed() -> [u8; 16] {
    let args = args().collect::<Vec<_>>();

    if args.len() < 2 {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 16];
        rng.fill(&mut seed);
        return seed;
    }

    format!("{:_>16}", args[1])[0..16]
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 16, v.len())
        })
}

pub fn generate_32rng_seed() -> [u8; 32] {
    let args = args().collect::<Vec<_>>();

    if args.len() < 2 {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        return seed;
    }

    format!("{:_>32}", args[1])[0..32]
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 32, v.len())
        })
}
