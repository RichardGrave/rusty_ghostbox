extern crate chrono;
extern crate crossterm;

mod found_word;
mod options;
mod window;

use chrono::Local;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute, style};
use found_word::Word;
use options::Options;
use rand::Rng;
use std::fmt::Display;
use std::io::{stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
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

const EXIT_PROGRAM: bool = true;
const DONT_EXIT_PROGRAM: bool = false;

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
    //Try to enter alternate screen or print error
    if let Err(err_mess) = execute!(stdout(), terminal::EnterAlternateScreen) {
        println!("{}", err_mess);
    }

    //At start, clear the terminal
    clear_term();
    print_at_pos(0, 0, "Loading file...");

    let options: Arc<RwLock<Options>> = Arc::new(RwLock::new(Options::default()));
    let options_thread_one = options.clone();
    let options_thread_two = options.clone();

    let found_words: Arc<RwLock<Vec<Word>>> = Arc::new(RwLock::new(Vec::<Word>::default()));
    let found_words_thread_one = found_words.clone();
    let found_words_thread_two = found_words.clone();

    //All the words from a file
    let word_vector: Vec<String> = WORDS_FILE.lines().map(|line| line.to_string()).collect();
    let word_vector_size = word_vector.len();

    let (chan_sender, chan_receiver): (Sender<bool>, Receiver<bool>) = std::sync::mpsc::channel();
    let chan_sender_key_listener = chan_sender.clone();

    thread::spawn(move || {
        key_listener(options_thread_one, chan_sender_key_listener);
    });
    thread::spawn(move || {
        creat_all_windows(
            options_thread_two,
            word_vector_size,
            found_words_thread_one,
            chan_receiver,
        );
    });

    //go find words
    get_words_by_chance(options, word_vector, found_words_thread_two, chan_sender);
}

fn creat_all_windows(
    options: Arc<RwLock<Options>>,
    word_vector_size: usize,
    found_words: Arc<RwLock<Vec<Word>>>,
    chan_receiver: Receiver<bool>,
) {
    loop {
        //Show everything on the screen
        create_options_window(word_vector_size);
        create_found_words_window();
        //only need to do this for info
        INFO_WINDOW.create_window();
        create_info_list(&options);
        print_found_words(&found_words.read().unwrap());

        //Keep blocking till we receive something or print error if something went wrong
        if let Err(err_mess) = chan_receiver.recv() {
            println!("{}", err_mess);
        }
    }
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

fn create_options_window(word_vector_size: usize) {
    let writing_position = OPTIONS_WINDOW.get_writing_positon();
    OPTIONS_WINDOW.create_window();

    print_at_pos(
        writing_position.column,
        writing_position.row,
        &format!("Loaded {} English words", word_vector_size),
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
        &format!("Do not hold the keys down for too long! (CrossTerm spams key events)"),
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
    //Yep, this does what is says ;) Creates a window for the words
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
        print_at_pos(writing_position.column, pos_option, &option);
        //get a empty row between them
        pos_option += 2;
    }
}

fn get_words_by_chance(
    options: Arc<RwLock<Options>>,
    word_vector: Vec<String>,
    found_words: Arc<RwLock<Vec<Word>>>,
    chan_sender: Sender<bool>,
) {
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
            found_words.write().unwrap().push(word);

            //Doesn't matter if we send true or false. We just need to send something.
            //Receiver blocks till we send
            if let Err(err_msg) = chan_sender.send(true) {
                println!("{}", err_msg);
            }
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
        print_at_pos(writing_position.column, pos_word, &found_word);
        pos_word += 1;
    }
}

fn key_listener(options: Arc<RwLock<Options>>, chan_sender: Sender<bool>) {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self
    // and allows crossterm to read the input and pass it back to you.
    if let Ok(_raw) = terminal::enable_raw_mode() {
        // enable mouse events to be captured.

        //Enable mouse event capture or print error if it fails
        if let Err(error_mess) = execute!(stdout(), EnableMouseCapture) {
            println!("{}", error_mess);
        }

        //We dont want the input queued while keeping the keyboard char pressed down.
        //So when we release the key, nothing should happen in the background
        loop {
            //get char so we can act on it
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
                //Not to happy about this event::read() because the key press spamming is back.
                //We can't seem to do anything about it.
                //I wish there was a DisableKeyboardCapture and DisableKeyboardCapture so we could
                //temporarily disable key events
            })) = event::read()
            {
                if process_input_event(&options, c) {
                    //Cleanup if we want to quit the program
                    cleanup_on_exit();
                }

                //Doesn't matter if we send true or false. We just need to send something.
                //Receiver blocks till we send
                if let Err(err_msg) = chan_sender.send(true) {
                    println!("{}", err_msg);
                }
            }
            //This stops the keypress spamming a bit
            thread::sleep(Duration::from_millis(KEY_LISTENER_SLEEP));
        }
    }
}

fn process_input_event(options: &Arc<RwLock<Options>>, key_code: char) -> bool {
    match key_code {
        QUIT => return EXIT_PROGRAM,
        INCREASE_CHANCE => {
            increase_decrease_chance(&mut options.write().unwrap().chance_range, true);
        }
        DECREASE_CHANCE => {
            increase_decrease_chance(&mut options.write().unwrap().chance_range, false);
        }
        INCREASE_WORD_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().word_sleep, true);
        }
        DECREASE_WORD_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().word_sleep, false);
        }
        INCREASE_RANDOM_ONE_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_one_sleep, true);
        }
        DECREASE_RANDOM_ONE_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_one_sleep, false);
        }
        INCREASE_RANDOM_TWO_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_two_sleep, true);
        }
        DECREASE_RANDOM_TWO_SLEEP => {
            increase_decrease_sleep(&mut options.write().unwrap().random_two_sleep, false);
        }
        RESET_OPTIONS => {
            *options.write().unwrap() = Options::default();
        }
        //ignore the rest
        _ => (),
    }
    DONT_EXIT_PROGRAM
}

fn print_at_pos<T>(column: u16, row: u16, message: T)
where
    T: Copy + Display,
{
    //Print to screen or give error message if it fails
    if let Err(err_mess) = execute!(stdout(), cursor::MoveTo(column, row), style::Print(message)) {
        println!("{}", err_mess);
    }
}

//Clear terminal
fn clear_term() {
    //Print to screen or give error message if it fails
    if let Err(err_mess) = execute!(
        stdout(),
        cursor::MoveTo(1, 1),
        terminal::Clear(ClearType::All),
    ) {
        println!("{}", err_mess);
    }
}

//Do all this if we want to exit the program
fn cleanup_on_exit() {
    print_at_pos(0, cursor::position().unwrap().1, "Quiting the program");

    // disable mouse events to be captured or print error if it fails
    if let Ok(_raw) = terminal::disable_raw_mode() {
        if let Err(error_mess) = execute!(stdout(), DisableMouseCapture) {
            println!("{}", error_mess);
        }
    }

    clear_term();

    //Try to leave this screen and go back to the one we started this program in or print error
    if let Err(err_mess) = execute!(stdout(), terminal::LeaveAlternateScreen) {
        println!("{}", err_mess);
    }

    //Stop the program
    std::process::exit(1);
}
