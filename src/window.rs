use crossterm::cursor;

pub struct Window {
    pub begin_row: u16,
    pub begin_column: u16,
    pub end_row: u16,
    pub end_column: u16,
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

    fn create_corners(&self) {
        cursor()
            .goto(self.begin_column, self.begin_row)
            .expect("tried to goto upper_left");
        print!("{}", UPPER_LEFT_CORNER);
        cursor()
            .goto(self.begin_column, self.end_row)
            .expect("tried to goto lower_left");
        print!("{}", LOWER_LEFT_CORNER);
        cursor()
            .goto(self.end_column, self.begin_row)
            .expect("tried to goto upper_righ");
        print!("{}", UPPER_RIGHT_CORNER);
        cursor()
            .goto(self.end_column, self.end_row)
            .expect("tried to goto lower_right");
        print!("{}", LOWER_RIGHT_CORNER);
    }

    fn create_left_right(column: u16, begin: u16, end: u16) {
        for row in begin..end {
            cursor()
                .goto(column, row)
                .expect("tried to goto left_right");
            print!("{}", LINE_UP_DOWN);
        }
    }

    fn create_up_down(row: u16, begin: u16, end: u16) {
        for column in begin..end {
            cursor().goto(column, row).expect("tried to goto up_down");
            print!("{}", LINE_LEFT_RIGHT);
        }
    }
}

// fn create_borders(start_x: u16, start_y: u16, end_x: u16, end_y: u16){
//         .expect("tried to goto start of the line");
// }
