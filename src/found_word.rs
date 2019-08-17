use std::fmt;

pub struct Word {
    pub time_found: String,
    pub word: String,
    pub chance_num_one: String,
    pub chance_num_two: String,
    pub chance_range: String,
}

impl Word {
    pub fn create_header() -> String {
        format!(
            //Apparently we can't use constants????
            "{:10}{:25}{:10}{:10}{:20}",
            "Time", "Word", "Random 1", "Random 2", "Chance range"
        )
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            //Apparently we can't use constants????
            "{:10}{:25}{:10}{:10}{:20}",
            self.time_found, self.word, self.chance_num_one, self.chance_num_two, self.chance_range
        )
    }
}
