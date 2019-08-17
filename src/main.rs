extern crate crossterm;
extern crate chrono;

mod found_word;
mod options;
mod window;

use crossterm::{cursor, input, terminal, ClearType, InputEvent, KeyEvent, RawScreen};
use found_word::Word;
use options::Options;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;
use window::Window;

const INCREASE_CHANCE: char = 'a';
const DECREASE_CHANCE: char = 's';
const INCREASE_WORD_SLEEP: char = 'w';
const DECREASE_WORD_SLEEP: char = 'e';
const INCREASE_RANDOM_ONE_SLEEP: char = '1';
const DECREASE_RANDOM_ONE_SLEEP: char = '2';
const INCREASE_RANDOM_TWO_SLEEP: char = '3';
const DECREASE_RANDOM_TWO_SLEEP: char = '4';
const RESET_OPTIONS: char = 'r';
const QUIT: char = 'q';

const LOWEST_RANGE: u16 = 10;
const RANGE_STEPS: u16 = 10;
const LOWEST_SLEEP: u64 = 50;
const SLEEP_STEPS: u64 = 50;
const WORDS_TO_TAKE: u16 = 45;
const FILE_NAME: &str = "src/english_words.txt";

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

    //go find words
    get_words_by_chance(&key_press, &word_vector);
}

fn create_options_array() -> Vec<String> {
    //Need to create Vec<String> here, apparently it can't be a static global

    //!! Auto format isn't that great in here
    let mut option_list = Vec::<String>::new();
    option_list.push(format!("{}: increase chance to find word", INCREASE_CHANCE));
    option_list.push(format!("{}: decrease chance to find word", DECREASE_CHANCE));
    option_list.push(format!(
        "{}: increase time to search for word",
        INCREASE_WORD_SLEEP
    ));
    option_list.push(format!(
        "{}: decrease time to search for word",
        DECREASE_WORD_SLEEP
    ));
    option_list.push(format!(
        "{}: increase time to search for random num 1",
        INCREASE_RANDOM_ONE_SLEEP
    ));
    option_list.push(format!(
        "{}: decrease time to search for random num 1",
        DECREASE_RANDOM_ONE_SLEEP
    ));
    option_list.push(format!(
        "{}: increase time to search for random num 2",
        INCREASE_RANDOM_TWO_SLEEP
    ));
    option_list.push(format!(
        "{}: decrease time to search for random num 2",
        DECREASE_RANDOM_TWO_SLEEP
    ));
    option_list.push(format!("{}: reset options to default", RESET_OPTIONS));
    option_list.push(format!("{}: quit the program", QUIT));

    option_list
}

fn create_options_window(word_vector: &Vec<String>) {
    let writing_position = OPTIONS_WINDOW.get_writing_positon();
    OPTIONS_WINDOW.create_window();

    print_at_pos(
        writing_position.column,
        writing_position.row,
        &format!("Loaded {} English words", word_vector.len()),
    );
    let mut pos_option = writing_position.row + 2;
    print_at_pos(
        writing_position.column,
        pos_option,
        &format!("Use the characters below to change speed and chance to find words"),
    );
    pos_option += 1;
    print_at_pos(
        writing_position.column,
        pos_option,
        &format!("You can keep the keys pressed to chance the values"),
    );

    pos_option += 2;
    let mut use_empty_row = false;

    for option in create_options_array().iter() {
        print_at_pos(writing_position.column, pos_option, option);

        //We use this to group the options
        if use_empty_row {
            pos_option += 2;
            use_empty_row = false;
        } else {
            //get a empty row between them
            pos_option += 1;
            use_empty_row = true;
        }
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
    create_info_list(&Options::default());
}

fn create_info_list(options: &Options) {
    let writing_position = INFO_WINDOW.get_writing_positon();
    //+2 because the headers are printed first and we need some space between them
    let mut pos_option = writing_position.row + 2;

    for option in options.get_all_info().iter() {
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

fn get_words_by_chance(key_press_clone: &Arc<Mutex<char>>, word_vector: &Vec<String>) {
    let mut found_words = Vec::<Word>::new();
    let mut options = Options::default();

    loop {
        if change_option_values(key_press_clone, &mut options) {
            break;
        }

        //reset key_press
        *key_press_clone.lock().unwrap() = ' ';

        //Give time to fetch word
        thread::sleep(options.word_sleep);
        //use number of words in vector as the range
        let word_on_line = rand::thread_rng().gen_range(0, word_vector.len());

        //Give time to set next two ranges on the same number
        thread::sleep(options.random_one_sleep);
        let first_chance = rand::thread_rng().gen_range(0, options.chance_range);

        //Give time to set next second equal to the first
        thread::sleep(options.random_two_sleep);
        let second_chance = rand::thread_rng().gen_range(0, options.chance_range);

        //If they both generate the same number then show the word on this line
        if first_chance == second_chance {
            let word = Word {
                time_found: Local::now().format("%H:%M:%S").to_string(),
                word: word_vector.get(word_on_line).unwrap().clone(),
                chance_num_one: first_chance.to_string(),
                chance_num_two: second_chance.to_string(),
                chance_range: options.chance_range.to_string(),
            };
            found_words.push(word);
            print_found_words(&found_words);
        }
    }
}

fn change_option_values(key_press_clone: &Arc<Mutex<char>>, options: &mut Options) -> bool {
    let mut quit_program = false;

    match *key_press_clone.lock().unwrap() {
        QUIT => {
            //Inform the loop that we pressed 'q' and quit the program
            quit_program = true;
        }
        INCREASE_CHANCE => {
            increase_decrease_chance(&mut options.chance_range, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_CHANCE => {
            increase_decrease_chance(&mut options.chance_range, false);
            //change info
            create_info_list(&options);
        }
        INCREASE_WORD_SLEEP => {
            increase_decrease_sleep(&mut options.word_sleep, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_WORD_SLEEP => {
            increase_decrease_sleep(&mut options.word_sleep, false);
            //change info
            create_info_list(&options);
        }
        INCREASE_RANDOM_ONE_SLEEP => {
            increase_decrease_sleep(&mut options.random_one_sleep, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_RANDOM_ONE_SLEEP => {
            increase_decrease_sleep(&mut options.random_one_sleep, false);
            //change info
            create_info_list(&options);
        }
        INCREASE_RANDOM_TWO_SLEEP => {
            increase_decrease_sleep(&mut options.random_two_sleep, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_RANDOM_TWO_SLEEP => {
            increase_decrease_sleep(&mut options.random_two_sleep, false);
            //change info
            create_info_list(&options);
        }
        RESET_OPTIONS => {
            *options = Options::default();
            create_info_list(&options);
        }
        _ => {}
    }

    quit_program
}

fn increase_decrease_sleep(sleep_time: &mut Duration, increase_sleep: bool) {
    let mut sleep_time_millis = sleep_time.as_millis() as u64;

    if increase_sleep {
        //increase sleep time
        sleep_time_millis += SLEEP_STEPS;

        *sleep_time = Duration::from_millis(sleep_time_millis);
    } else {
        //increase. The random numbers have to be lower so you'll get the same number faster
        if sleep_time_millis != LOWEST_SLEEP {
            sleep_time_millis -= SLEEP_STEPS;

            *sleep_time = Duration::from_millis(sleep_time_millis);
        }
    }
}

fn increase_decrease_chance(chance: &mut u16, increase_chance: bool) {
    if increase_chance {
        //increase chance to find word.
        //The random numbers have to be lower so you'll get the same number faster
        if *chance != LOWEST_RANGE {
            *chance -= RANGE_STEPS;
        }
    } else {
        //decrease chance to find word.
        //The random numbers have to be higher so it's less likely they match
        *chance += RANGE_STEPS;
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
            thread::sleep(Duration::from_millis(10));
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
                        clear_term();
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
