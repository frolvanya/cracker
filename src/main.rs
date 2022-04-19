use std::{sync::Arc, time::Instant};

use colored::Colorize;

mod hash;
mod initial_params;
mod word_generator;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HashType {
    Sha256,
    Sha512,
}

struct HashCrack {
    hash_type: HashType,
    hash_sum: String,
    word_generation_preferences: Vec<bool>,
}

impl HashCrack {
    fn start(self: &Arc<Self>) {
        let crack_start_time = Instant::now();
        let preferences = &self.word_generation_preferences;

        for possible_word in word_generator::generate(preferences.to_vec()) {
            if self.clone().hash_type == HashType::Sha256
                && hash::sha256(possible_word.clone()) == self.clone().hash_sum
            {
                println!("{}", format!("Found: {}", possible_word).green());
                println!(
                    "{}",
                    format!(
                        "Hash was cracked in {:.2}s",
                        crack_start_time.elapsed().as_secs_f32()
                    )
                    .green()
                );

                std::process::exit(1);
            } else if self.hash_type == HashType::Sha512
                && hash::sha512(possible_word.clone()) == self.clone().hash_sum
            {
                println!("{}", possible_word.green());
                println!(
                    "{}",
                    format!(
                        "Hash was cracked in {:.2}s",
                        crack_start_time.elapsed().as_secs_f32()
                    )
                    .green()
                );

                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let recieve_params = initial_params::get();

    let init_hash_crack = HashCrack {
        hash_type: recieve_params.hash_type,
        hash_sum: recieve_params.hash_sum,
        word_generation_preferences: recieve_params.word_generation_preferences,
    };

    HashCrack::start(&Arc::new(init_hash_crack));
}
