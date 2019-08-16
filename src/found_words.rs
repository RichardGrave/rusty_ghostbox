use std::fmt;

pub struct Word {
    pub time_found: String,
    pub word: String,
    pub chance_num_one: String,
    pub chance_num_two: String,
}

impl fmt::Display for Word {

    pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:10!}, {:10!}, {:10!}, {:10!})", self.time_found, self.word, self.chance_num_one, self.chance_num_two)
    }
}