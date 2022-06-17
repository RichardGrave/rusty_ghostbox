use crossterm::{cursor, execute, style};
use std::io::stdout;

pub struct Window {
    pub begin_row: u16,
    pub begin_column: u16,
    pub end_row: u16,
    pub end_column: u16,
}

pub struct CursorPosition {
    pub column: u16,
    pub row: u16,
}

const UPPER_LEFT_CORNER: char = '╔';
const LOWER_LEFT_CORNER: char = '╚';
const UPPER_RIGHT_CORNER: char = '╗';
const LOWER_RIGHT_CORNER: char = '╝';
const LINE_UP_DOWN: char = '║';
const LINE_LEFT_RIGHT: char = '═';

impl Window {
    pub fn create_window(&self) {
        Self::create_corners(self);
        Self::create_left_right(self.begin_column, self.begin_row + 1, self.end_row);
        Self::create_left_right(self.end_column, self.begin_row + 1, self.end_row);

        Self::create_up_down(self.begin_row, self.begin_column + 1, self.end_column);
        Self::create_up_down(self.end_row, self.begin_column + 1, self.end_column);
    }

    pub fn get_writing_positon(&self) -> CursorPosition {
        CursorPosition {
            column: self.begin_column + 2,
            row: self.begin_row + 2,
        }
    }

    fn create_corners(&self) {
        //Print to screen or give error message if it fails
        if let Err(err_message) = execute!(
            stdout(),
            cursor::MoveTo(self.begin_column, self.begin_row),
            style::Print(UPPER_LEFT_CORNER),
            cursor::MoveTo(self.begin_column, self.end_row),
            style::Print(LOWER_LEFT_CORNER),
            cursor::MoveTo(self.end_column, self.begin_row),
            style::Print(UPPER_RIGHT_CORNER),
            cursor::MoveTo(self.end_column, self.end_row),
            style::Print(LOWER_RIGHT_CORNER)
        ) {
            println!("{}", err_message);
        }
    }

    fn create_left_right(column: u16, begin: u16, end: u16) {
        for row in begin..end {
            Self::create_line(column, row, LINE_UP_DOWN);
        }
    }

    fn create_up_down(row: u16, begin: u16, end: u16) {
        for column in begin..end {
            Self::create_line(column, row, LINE_LEFT_RIGHT);
        }
    }

    fn create_line(column: u16, row: u16, character: char) {
        if let Err(err_message) = execute!(
            stdout(),
            cursor::MoveTo(column, row),
            style::Print(character)
        ) {
            println!("{}", err_message);
        }
    }
}
