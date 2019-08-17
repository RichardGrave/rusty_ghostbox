extern crate chrono;
extern crate crossterm;

mod found_word;
mod options;
mod window;

use chrono::Local;
use crossterm::{cursor, input, terminal, ClearType, RawScreen};
use found_word::Word;
use options::Options;
use rand::Rng;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
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

const KEY_LISTENER_SLEEP: u64 = 100;
const LOWEST_RANGE: u16 = 10;
const RANGE_STEPS: u16 = 10;
const LOWEST_SLEEP: u64 = 50;
const SLEEP_STEPS: u64 = 50;
const WORDS_TO_TAKE: u16 = 45;
// const FILE_NAME: &str = "english_words.txt";
static WORDS_FILE: &'static str = include_str!("english_words.txt");

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
    end_column: OPTIONS_WINDOW.end_column + 110,
};
const INFO_WINDOW: Window = Window {
    begin_row: OPTIONS_WINDOW.end_row + 1,
    begin_column: OPTIONS_WINDOW.begin_column,
    end_row: OPTIONS_WINDOW.end_row + 20,
    end_column: OPTIONS_WINDOW.end_column,
};

fn main() {
    let options: Arc<RwLock<Options>> = Arc::new(RwLock::new(Options::default()));
    let options_clone = options.clone();

    //At start, clear the terminal
    clear_term();
    print_at_pos(0, 0, "Loading file...");

    let word_vector: Vec<String> = WORDS_FILE.lines().map(|line| line.to_string()).collect();

    //try to create the windows parts
    create_options_window(&word_vector);
    create_found_words_window();
    //only need to do this for info
    INFO_WINDOW.create_window();
    create_info_list(&options);

    thread::spawn(move || {
        key_listener(options_clone);
    });

    //go find words
    get_words_by_chance(&options, &word_vector);
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

fn create_info_list(options: &Arc<RwLock<Options>>) {
    let writing_position = INFO_WINDOW.get_writing_positon();
    //+2 because the headers are printed first and we need some space between them
    let mut pos_option = writing_position.row + 2;

    for option in options.read().unwrap().get_all_info().iter() {
        print_at_pos(writing_position.column, pos_option, option);
        //get a empty row between them
        pos_option += 2;
    }
}

fn get_words_by_chance(options: &Arc<RwLock<Options>>, word_vector: &Vec<String>) {
    let mut found_words = Vec::<Word>::new();

    loop {
        //Give time to fetch word
        thread::sleep(options.read().unwrap().word_sleep);
        //use number of words in vector as the range
        let word_on_line = rand::thread_rng().gen_range(0, word_vector.len());

        //Give time to set next two ranges on the same number
        thread::sleep(options.read().unwrap().random_one_sleep);
        let first_chance = rand::thread_rng().gen_range(0, options.read().unwrap().chance_range);

        //Give time to set next second equal to the first
        thread::sleep(options.read().unwrap().random_two_sleep);
        let second_chance = rand::thread_rng().gen_range(0, options.read().unwrap().chance_range);

        //If they both generate the same number then show the word on this line
        if first_chance == second_chance {
            let word = Word {
                time_found: Local::now().format("%H:%M:%S").to_string(),
                word: word_vector.get(word_on_line).unwrap().clone(),
                chance_num_one: first_chance.to_string(),
                chance_num_two: second_chance.to_string(),
                chance_range: options.read().unwrap().chance_range.to_string(),
            };
            found_words.push(word);
            print_found_words(&found_words);
        }
    }
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

fn key_listener(options: Arc<RwLock<Options>>) {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self
    // and allows crossterm to read the input and pass it back to you.
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        // enable mouse events to be captured.
        input.enable_mouse_mode().unwrap();

        //We dont want the input queued while keeping the keyboard char pressed down.
        //So when we release the key, nothing should happen in the background
        loop {
            match input.read_char() {
                Ok(c) => process_input_event(&options, c),
                Err(e) => println!("error: {}", e),
            }

            thread::sleep(Duration::from_millis(KEY_LISTENER_SLEEP));
        }
    }
}

//TODO:RG build a use for: key_press_clone: &Arc<Mutex<char>>
fn process_input_event(options: &Arc<RwLock<Options>>, key_event: char) {
    match key_event {
        QUIT => {
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
        INCREASE_CHANCE => {
            increase_decrease_chance(&mut options.write().unwrap().chance_range, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_CHANCE => {
            increase_decrease_chance(&mut options.write().unwrap().chance_range, false);
            //change info
            create_info_list(&options);
        }
        INCREASE_WORD_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().word_sleep, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_WORD_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().word_sleep, false);
            //change info
            create_info_list(&options);
        }
        INCREASE_RANDOM_ONE_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_one_sleep, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_RANDOM_ONE_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_one_sleep, false);
            //change info
            create_info_list(&options);
        }
        INCREASE_RANDOM_TWO_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_two_sleep, true);
            //change info
            create_info_list(&options);
        }
        DECREASE_RANDOM_TWO_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_two_sleep, false);
            //change info
            create_info_list(&options);
        }
        RESET_OPTIONS => {
            *options.write().unwrap() = Options::default();
            create_info_list(&options);
        }
        //ignore the rest
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
