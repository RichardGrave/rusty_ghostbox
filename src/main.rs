use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};

const CHANCE_RANGE: u32 = 101;
const SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
fn main() {
    println!("Loading file...");
    let words_file = File::open("src/english_words_alpha.txt").expect("opening file");
    let file_reader = BufReader::new(&words_file);

    let word_vector: Vec<String> = file_reader.lines().map(|line| line.unwrap()).collect();

    println!("File loaded.");
    println!("Number of words: {}", word_vector.len());

    loop {
        //TODO:RG do more
        check_on_chance(&word_vector);
        // check_on_word_range(&word_vector);
    }
}

#[allow(dead_code)]
fn check_on_chance(word_vector: &Vec<String>) {
    //Give time to fetch word
    thread::sleep(SLEEP_DURATION);
    let word_on_line = rand::thread_rng().gen_range(0, word_vector.len());

    //Give time to set next two ranges on the same number
    thread::sleep(SLEEP_DURATION);

    let first_chance = rand::thread_rng().gen_range(0, CHANCE_RANGE);
    let second_chance = rand::thread_rng().gen_range(0, CHANCE_RANGE);

    //If they both generate the same number then show the word on this line
    if first_chance == second_chance {
        println!("Word: {}", word_vector.get(word_on_line).unwrap());
    }
}

#[allow(dead_code)]
fn check_on_word_range(word_vector: &Vec<String>) {
    let word_on_line_one = rand::thread_rng().gen_range(0, word_vector.len());
    let word_on_line_two = rand::thread_rng().gen_range(0, word_vector.len());

    //If they both generate the same number then show the word on this line
    if word_on_line_one == word_on_line_two {
        println!("Word: {}", word_vector.get(word_on_line_one).unwrap());
    }
}
