extern crate crossterm;

mod found_word;
mod window;

use crossterm::{cursor, input, terminal, ClearType, InputEvent, KeyEvent, RawScreen};
use found_word::Word;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use window::Window;

const QUIT: char = 'q';
const INCREASE_CHANCE: char = 'i';
const DECREASE_CHANCE: char = 'd';
const WORD_SLEEP: char = 'w';
const RANDOM_ONE_SLEEP: char = '1';
const RANDOM_TWO_SLEEP: char = '2';

const DEFAULT_SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
const DEFAULT_CHANGE_RANGE: u16 = 100;
const WORDS_TO_TAKE: u16 = 45;
const FILE_NAME: &str = "src/english_words.txt";

static OPTIONS_LIST: [&'static str; 6] = [
    "i: increase chance to find word",
    "d: decrease chance to find word",
    "w: give more time to search for word",
    "1: give more time to search for random num 1",
    "2: give more time to search for random num 2",
    "q: quit the program",
];

static INFO_LIST: [&'static str; 4] = [
    "Random numbers between: 0 and ",
    "word search time: ",
    "Random number 1 search time: ",
    "Random number 2 search time: ",
];

const OPTIONS_WINDOW: Window = Window {
    begin_row: 0u16,
    begin_column: 0u16,
    end_row: 30u16,
    end_column: 75u16,
};
const FOUND_WORDS_WINDOW: Window = Window {
    begin_row: 0u16,
    begin_column: OPTIONS_WINDOW.end_column + 1,
    end_row: OPTIONS_WINDOW.end_row + 20,
    end_column: OPTIONS_WINDOW.end_column + 85,
};
const INFO_WINDOW: Window = Window {
    begin_row: OPTIONS_WINDOW.end_row + 1,
    begin_column: OPTIONS_WINDOW.begin_column,
    end_row: OPTIONS_WINDOW.end_row + 20,
    end_column: OPTIONS_WINDOW.end_column,
};

//TODO:RG global variables?
//TODO:RG after quiting we need to get control back for the Terminal

fn main() {
    //At start, clear the terminal
    clear_term();
    print_at_pos(0, 0, "Loading file...");

    let mut word_vector: Vec<String> = Vec::new();
    init_word_vec(&mut word_vector);

    //try to create the windows parts
    create_options_window(&word_vector);
    create_found_words_window();
    create_info_window();

    let key_press: Arc<Mutex<char>> = Arc::new(Mutex::new(' '));
    let key_press_clone = key_press.clone();

    thread::spawn(move || {
        key_listener(&key_press_clone);
    });

    //TODO:RG do more
    check_on_chance(&key_press, &word_vector);
}

fn create_options_window(word_vector: &Vec<String>) {
    let writing_position = OPTIONS_WINDOW.get_writing_positon();
    OPTIONS_WINDOW.create_window();

    print_at_pos(
        writing_position.column,
        writing_position.row,
        &format!("Loaded {} English words", word_vector.len()),
    );
    //+2 because the headers are printed first and we need some space between them
    let mut pos_option = writing_position.row + 2;

    print_at_pos(
        writing_position.column,
        pos_option,
        &format!("Use the characters below to change speed and chance to find words"),
    );

    pos_option = pos_option + 2;

    for option in OPTIONS_LIST.iter() {
        print_at_pos(writing_position.column, pos_option, option);
        //get a empty row between them
        pos_option += 1;
    }
}

fn create_found_words_window() {
    let writing_position = FOUND_WORDS_WINDOW.get_writing_positon();
    FOUND_WORDS_WINDOW.create_window();
    print_at_pos(
        writing_position.column,
        writing_position.row,
        &Word::create_header(),
    );
}

fn create_info_window() {
    INFO_WINDOW.create_window();
    create_info_list();
}

fn create_info_list() {
    let writing_position = INFO_WINDOW.get_writing_positon();
    //+2 because the headers are printed first and we need some space between them
    let mut pos_option = writing_position.row + 2;

    for option in INFO_LIST.iter() {
        print_at_pos(writing_position.column, pos_option, option);
        //get a empty row between them
        pos_option += 2;
    }
}

fn init_word_vec(word_vector: &mut Vec<String>) {
    let words_file = File::open(FILE_NAME).expect("opening file");
    let file_reader = BufReader::new(&words_file);

    *word_vector = file_reader.lines().map(|line| line.unwrap()).collect();
}

fn check_on_chance(key_press_clone: &Arc<Mutex<char>>, word_vector: &Vec<String>) {
    let mut found_words = Vec::<Word>::new();
    let mut chance_range = DEFAULT_CHANGE_RANGE;
    let mut word_sleep = DEFAULT_SLEEP_DURATION;
    let mut random_one_sleep = DEFAULT_SLEEP_DURATION;
    let mut random_two_sleep = DEFAULT_SLEEP_DURATION;

    loop {
        match *key_press_clone.lock().unwrap() {
            QUIT => {
                //End loop if we pressed 'q' (quit)
                break;
            }
            INCREASE_CHANCE => {
                increase_decrease_chance(&mut chance_range, &true);
            }
            DECREASE_CHANCE => {
                increase_decrease_chance(&mut chance_range, &false);
            }
            WORD_SLEEP => {}
            RANDOM_ONE_SLEEP => {}
            RANDOM_TWO_SLEEP => {}
            _ => {}
        }

        //reset key_press
        *key_press_clone.lock().unwrap() = ' ';

        //Give time to fetch word
        thread::sleep(word_sleep);
        let word_on_line = rand::thread_rng().gen_range(0, word_vector.len());

        //Give time to set next two ranges on the same number
        thread::sleep(random_one_sleep);
        let first_chance = rand::thread_rng().gen_range(0, chance_range);

        //Give time to set next second equal to the first
        thread::sleep(random_two_sleep);
        let second_chance = rand::thread_rng().gen_range(0, chance_range);

        //If they both generate the same number then show the word on this line
        if first_chance == second_chance {
            let word = Word {
                time_found: String::from("time_test"),
                word: word_vector.get(word_on_line).unwrap().clone(),
                chance_num_one: String::from("change_one_test"),
                chance_num_two: String::from("change_two_test"),
            };
            found_words.push(word);
            print_found_words(&found_words);
        }
    }
}

fn increase_decrease_chance(chance: &mut u16, increase: &bool) {
    if *increase {
        //increase. The random numbers have to be lower so you'll get the same number faster
        if chance != &10u16 {
            *chance -= 10;
        } else {
            *chance = 1;
        }
    } else {
        //decrease. The random numbers have to be higher so it's less likely they match
        if chance != &1u16 {
            *chance += 10;
        } else {
            *chance = 10;
        }
    }
}

fn print_found_words(found_words: &Vec<Word>) {
    let writing_position = FOUND_WORDS_WINDOW.get_writing_positon();
    //get last 20 found
    let last_twenty_words: Vec<_> = found_words
        .iter()
        .rev()
        .take(WORDS_TO_TAKE as usize)
        .collect::<Vec<_>>();

    //+2 because the headers are printed first and we need some space between them
    let mut pos_word = writing_position.row + 2;

    //Max 20 words
    for found_word in last_twenty_words {
        print_word_at_pos(writing_position.column, pos_word, &found_word);
        pos_word += 1;
    }
}

fn key_listener(key_press_clone: &Arc<Mutex<char>>) {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self
    // and allows crossterm to read the input and pass it back to you.
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        // enable mouse events to be captured.
        input.enable_mouse_mode().unwrap();

        let mut stdin = input.read_async();

        loop {
            if let Some(key_event) = stdin.next() {
                process_input_event(key_press_clone, &key_event);

                //End loop if we pressed 'q' (quit)
                if *key_press_clone.lock().unwrap() == QUIT {
                    break;
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

//TODO:RG build a use for: key_press_clone: &Arc<Mutex<char>>
fn process_input_event(key_press_clone: &Arc<Mutex<char>>, key_event: &InputEvent) {
    //TODO:RG do key_events to increase/lower the chance or the speed
    match key_event {
        InputEvent::Keyboard(event) => {
            match event {
                KeyEvent::Char(c) => match c {
                    &QUIT => {
                        print_at_pos(0, cursor().pos().1, "Quiting the program");

                        // disable mouse events to be captured.
                        if let Ok(_raw) = RawScreen::disable_raw_mode() {
                            let input = input();
                            input
                                .disable_mouse_mode()
                                .expect("Tried to disable mouse mode");
                        }
                        //Disabled terminal().exit() because it sometimes causes panics.
                        //Maybe because of the thread??
                        // clear_term();
                        terminal().exit();
                    }
                    //ignore the rest
                    _ => {
                        *key_press_clone.lock().unwrap() = *c;
                    }
                },
                _ => (),
            }
        }
        _ => (),
    }
}

//Set cursor to the start of the line
fn print_at_pos(column: u16, row: u16, message: &str) {
    cursor()
        .goto(column, row)
        .expect("tried to goto start of the line");

    println!("{}", message);
}

//Set cursor to the start of the line
fn print_word_at_pos(column: u16, row: u16, message: &Word) {
    cursor()
        .goto(column, row)
        .expect("tried to goto start of the line");

    println!("{}", message);
}

fn clear_term() {
    // Clear loading;
    terminal()
        .clear(ClearType::All)
        .expect("tried to clear terminal");
}
