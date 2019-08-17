use std::time;

const DEFAULT_SLEEP_DURATION: time::Duration = time::Duration::from_millis(250);
const DEFAULT_CHANGE_RANGE: u16 = 100;

pub struct Options {
    pub chance_range: u16,
    pub word_sleep: time::Duration,
    pub random_one_sleep: time::Duration,
    pub random_two_sleep: time::Duration,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            chance_range: DEFAULT_CHANGE_RANGE,
            word_sleep: DEFAULT_SLEEP_DURATION,
            random_one_sleep: DEFAULT_SLEEP_DURATION,
            random_two_sleep: DEFAULT_SLEEP_DURATION,
        }
    }
}

impl Options {
    //Use extra space at end of Strings to make sure the all of the previous characters are overwritten

    fn info_random_number_range(&self) -> String {
        format!("Random numbers between: 0 and {}   ", self.chance_range)
    }
    fn info_word(&self) -> String {
        format!(
            "Word search wait time: {} ms   ",
            self.word_sleep.as_millis()
        )
    }
    fn info_random_number_one(&self) -> String {
        format!(
            "Random number 1 search wait time: {} ms   ",
            self.random_one_sleep.as_millis()
        )
    }
    fn info_random_number_two(&self) -> String {
        format!(
            "Random number 2 search wait time: {} ms    ",
            self.random_two_sleep.as_millis()
        )
    }

    pub fn get_all_info(&self) -> Vec<String> {
        let mut info = Vec::<String>::new();
        info.push(Self::info_random_number_range(&self));
        info.push(Self::info_word(&self));
        info.push(Self::info_random_number_one(&self));
        info.push(Self::info_random_number_two(&self));

        info
    }
}
