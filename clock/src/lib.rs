use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl Clock {
    fn process(start_hour: i32, start_minute: i32, hours: i32, minutes: i32) -> (i32, i32) {
        let hour_calc = ((start_minute as f32 + minutes as f32) / 60 as f32).floor();
        let minute: i32 = (start_minute + (60 + (minutes % 60)) % 60) % 60;
        let hour: i32 = (start_hour + (24 + (hours + hour_calc as i32) % 24) % 24) % 24;
        (hour, minute)
    }
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let (hour, minute) = Clock::process(0, 0, hours, minutes);
        Clock {
            minute: minute,
            hour: hour,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Clock {
        let (hour, minute) = Clock::process(self.hour, self.minute, 0, minutes);
        self.minute = minute;
        self.hour = hour;
        self.clone()
    }
}
