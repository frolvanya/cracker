use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::Arc,
    time::Instant,
};

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
    file_name: String,
}

impl HashCrack {
    fn crack(self: &Arc<Self>, crack_start_time: Instant, possible_word: String) {
        if (self.hash_type == HashType::Sha256
            && hash::sha256(possible_word.clone()) == self.clone().hash_sum)
            || (self.hash_type == HashType::Sha512
                && hash::sha512(possible_word.clone()) == self.clone().hash_sum)
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
        }
    }

    fn start(self: &Arc<Self>) {
        let crack_start_time = Instant::now();
        let file_name = &self.file_name;

        if file_name.is_empty() {
            let preferences = &self.word_generation_preferences;

            for possible_word in word_generator::generate(preferences.to_vec()) {
                self.crack(crack_start_time, possible_word)
            }
        } else if let Ok(words) = read_lines(file_name) {
            for possible_word in words.flatten() {
                self.crack(crack_start_time, possible_word);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let recieve_params = initial_params::get();

    let init_hash_crack = HashCrack {
        hash_type: recieve_params.hash_type,
        hash_sum: recieve_params.hash_sum,
        word_generation_preferences: recieve_params.word_generation_preferences,
        file_name: recieve_params.file_name,
    };

    HashCrack::start(&Arc::new(init_hash_crack));
}
