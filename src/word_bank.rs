use itertools::Itertools;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Word {
    pub text: String,
}

pub fn read_word_bank() -> serde_json::Result<Vec<String>> {
    let word_bank_path = env::var("WORD_BANK_PATH")
        .expect("Be sure to add wordbank file path to environment variables");

    let contents = fs::read_to_string(word_bank_path).expect("Failed to read wordbank");

    serde_json::from_str(&contents)
}

pub fn sample_word_bank(n: usize) -> Vec<String> {
    let word_bank = read_word_bank().unwrap();
    let mut rng = &mut rand::thread_rng();
    word_bank
        .choose_multiple(&mut rng, n)
        .cloned()
        .collect_vec()
}
