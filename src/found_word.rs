use std::fmt;

pub struct Word {
    pub time_found: String,
    pub word: String,
    pub chance_num_one: String,
    pub chance_num_two: String,
    pub chance_range: String,
}

impl Word {
    //Longest word is 45 chars long
    pub fn create_header() -> String {
        format!(
            //Apparently we can't use constants????
            "{:15}{:20}{:20}{:47}",
            "Time", "Chance range", "Random numbers", "Word found is"
        )
    }
}

impl fmt::Display for Word {
    //Longest word is 45 chars long
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            //Apparently we can't use constants????
            "{:15}{:20}{:20}{:47}",
            //only print chance_num_one because both chance_num's should be the same.
            self.time_found,
            self.chance_range,
            self.chance_num_one,
            self.word
        )
    }
}
