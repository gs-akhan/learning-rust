use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_IN_A_DAY: i32 = 1440;
const MINUTES_IN_AN_HOUR: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut all_minutes = hours * MINUTES_IN_AN_HOUR + minutes;
        all_minutes = all_minutes.rem_euclid(MINUTES_IN_A_DAY);
        Clock {
            hours: all_minutes / MINUTES_IN_AN_HOUR,
            minutes: all_minutes % MINUTES_IN_AN_HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn main() {
    println!(" This is the Ultimate & fastest Clock");
    let clock = Clock::new(100, 120);
    println!("time is : {}", clock.to_string());
}
