#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (mut _h, mut m) = Self::cal_mins(minutes);
        if m < 0 {
            m = m + 60;
            _h = _h - 1;
        }
        let h = Self::cal_hours(_h + hours);
        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (mut _h, mut m) = Self::cal_mins(self.minutes + minutes);
        if m < 0 {
            m = m + 60;
            _h = _h - 1;
        }
        let h = Self::cal_hours(_h + self.hours);
        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn cal_hours(hours: i32) -> i32 {
        if hours == 0 {
            return 0;
        } else if hours == 24 {
            return 0;
        } else if hours < 0 {
            return 24 + (hours % 24);
        } else if hours < 24 {
            return hours;
        } else {
            return hours % 24;
        }
    }

    pub fn cal_mins(minutes: i32) -> (i32, i32) {
        if minutes < 0 {
            return (Clock::cal_hours(minutes / 60), minutes % 60);
        } else if minutes == 60 {
            return (1, 0);
        } else if minutes > 60 {
            return (minutes / 60, minutes % 60);
        } else {
            return (0, minutes);
        }
    }

    pub fn to_string(&self) -> String {
        let mut result_hour = self.hours.to_string();
        if self.hours <= 9 {
            result_hour = format!("0{}", result_hour);
        }

        let mut result_min = self.minutes.to_string();

        if self.minutes <= 9 {
            result_min = format!("0{}", result_min);
        }

        return format!("{}:{}", result_hour, result_min);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

fn main() {
    println!(" This is the ultimate Clock");

    let clock = Clock::new(100, 120);

    println!("time is : {}", clock.to_string());
}
