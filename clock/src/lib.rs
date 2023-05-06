use std::fmt::{self, Display};

// 一小時60分鐘
const HOUR: i32 = 60;
//一天1440分鐘
const DAY: i32 = 60 * 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // 由於不用管天，所以可以先去除多餘的天數
        // 然後再加上一天，再去除多餘的天數
        // 1. hours * HOUR + minutes 計算總共的分鐘數
        // 2. 餘數運算，將超出一天的部分去除
        // 3. 加上一天，避免分鐘數為負的情況
        // 4. 再次進行步驟2，將超出一天的部分去除
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
