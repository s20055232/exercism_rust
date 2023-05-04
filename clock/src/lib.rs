use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_mins = 60 * (hours % 24) + minutes;
        if total_mins < 0 {
            let times = total_mins / -(60 * 24) + 1;
            total_mins += 60 * 24 * times;
        }
        let hours = total_mins / 60 % 24;
        let minutes = total_mins % 60;
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        let hours = {
            if self.hours % 24 == 0 {
                24
            } else {
                self.hours
            }
        };
        let mut total_mins = hours * 60 + self.minutes + minutes;
        if total_mins < 0 {
            let times = total_mins / -(60 * 24) + 1;
            total_mins += 60 * 24 * times;
        }
        let hours = total_mins / 60 % 24;
        let minutes = total_mins % 60;
        Clock { hours, minutes }
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
