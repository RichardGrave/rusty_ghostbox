extern crate crossterm;

use crossterm::{cursor, input, terminal, ClearType, InputEvent, KeyEvent, RawScreen};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::sync::{Arc, Mutex};
use std::{thread, time};

const CHANCE_RANGE: u32 = 101;
const SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
const QUIT: char = 'q';

//TODO:RG global variables

//TODO:RG after quiting we need to get control back for the Terminal

fn main() {
    //At start, clear the terminal
    clear_term();
    print_at_pos_zero("Loading file...");

    // let words_file = File::open("src/english_words_alpha.txt").expect("opening file");
    // let words_file = File::open("src/english_words.txt").expect("opening file");
    let words_file = File::open("src/nederlands3.txt").expect("opening file");
    let file_reader = BufReader::new(&words_file);

    let word_vector: Vec<String> = file_reader.lines().map(|line| line.unwrap()).collect();

    // Clear loading;
    clear_term();
    print_at_pos_zero("File loaded.");
    print_at_pos_zero(&format!("Number of English words: {}", word_vector.len()));

    //TODO:RG later
    // let key_press = Arc::new(Mutex::new(' '));
    // let key_press_clone = key_press.clone();

    thread::spawn(move || {
        key_listener();
    });

    //TODO:RG do more
    check_on_chance(&word_vector);
}

#[allow(dead_code)]
fn check_on_chance(word_vector: &Vec<String>) {
    loop {
        //Give time to fetch word
        thread::sleep(SLEEP_DURATION);
        let word_on_line = rand::thread_rng().gen_range(0, word_vector.len());

        //Give time to set next two ranges on the same number
        thread::sleep(SLEEP_DURATION);

        let first_chance = rand::thread_rng().gen_range(0, CHANCE_RANGE);
        let second_chance = rand::thread_rng().gen_range(0, CHANCE_RANGE);

        //If they both generate the same number then show the word on this line
        if first_chance == second_chance {
            print_at_pos_zero(&format!("Word: {}", word_vector.get(word_on_line).unwrap()));
        }
    }
}

//TODO:RG build a use for: key_press_clone: &Arc<Mutex<char>>
fn key_listener() {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self
    // and allows crossterm to read the input and pass it back to you.
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        // enable mouse events to be captured.
        input.enable_mouse_mode().unwrap();

        let mut stdin = input.read_async();

        loop {
            if let Some(key_event) = stdin.next() {
                process_input_event(key_event);

                //End loop if we pressed 'q' (quit)
                // if *key_press_clone.lock().unwrap() == QUIT {
                //     break;
                // }
            }
            thread::sleep(time::Duration::from_millis(50));
        }
    }
}

//TODO:RG build a use for: key_press_clone: &Arc<Mutex<char>>
fn process_input_event(key_event: InputEvent) {
    //TODO:RG do key_events to increase/lower the chance or the speed
    match key_event {
        InputEvent::Keyboard(event) => {
            match event {
                KeyEvent::Char(c) => match c {
                    QUIT => {
                        print_at_pos_zero("Quiting the program");

                        // disable mouse events to be captured.
                        if let Ok(_raw) = RawScreen::disable_raw_mode() {
                            let input = input();
                            input
                                .disable_mouse_mode()
                                .expect("Tried to disable mouse mode");
                        }
                        terminal().exit();
                    }
                    _ => {
                        print_at_pos_zero(&format!("'{}' pressed", c));
                    }
                },
                _ => (),
            }
        }
        _ => (),
    }
}

//Set cursor to the start of the line
fn print_at_pos_zero(message: &str) {
    println!("{}", message);
    cursor()
        .goto(0, cursor().pos().1)
        .expect("tried to goto start of the line");
}

fn clear_term() {
    // Clear loading;
    terminal()
        .clear(ClearType::All)
        .expect("tried to clear terminal");
}
