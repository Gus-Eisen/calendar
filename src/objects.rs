use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn as_str(&self) -> &str {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Year {
    Y2025,
    Y2026,
}

impl Year {
    pub fn as_str(&self) -> &str {
        match self {
            Year::Y2025 => "2025",
            Year::Y2026 => "2026",
        }
    }
    pub fn as_i32(&self) -> i32 {
        match self {
            Year::Y2025 => 2025,
            Year::Y2026 => 2026,
        }
    }
}

//Event(s) are stored in UTC, then converted to display appropriate TZ for User.
#[derive(Debug)]
pub struct Event {
    title: String,
    datetime: DateTime<Utc>,
}

#[derive(Debug)]
pub struct EventRegistry {
    vec_of_events: Option<Vec<Event>>,
}

impl EventRegistry {
    pub fn new(vec_of_events: Option<Vec<Event>>) -> Self {
        Self { vec_of_events }
    }
}

#[derive(Debug, Clone)]
pub struct EventForEES {
    pub event_title: Option<String>,
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub day: Option<String>,
    pub time: Option<String>,
}

impl EventForEES {
    pub fn new(
        event_title: Option<String>,
        year: Option<Year>,
        month: Option<Month>,
        day: Option<String>,
        time: Option<String>,
    ) -> Self {
        Self {
            event_title,
            year,
            month,
            day,
            time,
        }
    }

    pub fn get_title(&self) -> Option<String> {
        self.event_title.clone()
    }

    pub fn get_year_as_i32(&self) -> i32 {
        self.year.clone().unwrap().as_i32()
    }

    pub fn get_month_as_u32(&self) -> u32 {
        self.month.clone().unwrap().as_u32()
    }

    pub fn get_day_as_u32(&self) -> u32 {
        self.day
            .clone()
            .unwrap()
            .parse::<u32>()
            .expect("Unable to parse Day from EventForEES")
    }

    pub fn get_time(&self) -> Option<String> {
        self.time.clone()
    }

    pub fn all_some(&self) -> bool {
        self.event_title.is_some()
            && self.year.is_some()
            && self.month.is_some()
            && self.day.is_some()
            && self.time.is_some()
    }

    pub fn set_year(&mut self, index: u8) {
        match index {
            0 => {
                self.year = Some(Year::Y2025);
                println!("Year: {:?}", self.year);
            }
            1 => {
                self.year = Some(Year::Y2026);
                println!("Year: {:?}", self.year);
            }
            _ => panic!("Invalid year index: {}", index),
        }
    }

    pub fn set_month(&mut self, index: u8) {
        match index {
            0 => {
                self.month = Some(Month::January);
                println!("Month: {:?}", self.month);
            }
            1 => {
                self.month = Some(Month::February);
                println!("Month: {:?}", self.month);
            }
            2 => {
                self.month = Some(Month::March);
                println!("Month: {:?}", self.month);
            }
            3 => {
                self.month = Some(Month::April);
                println!("Month: {:?}", self.month);
            }
            4 => {
                self.month = Some(Month::May);
                println!("Month: {:?}", self.month);
            }
            5 => {
                self.month = Some(Month::June);
                println!("Month: {:?}", self.month);
            }
            6 => {
                self.month = Some(Month::July);
                println!("Month: {:?}", self.month);
            }
            7 => {
                self.month = Some(Month::August);
                println!("Month: {:?}", self.month);
            }
            8 => {
                self.month = Some(Month::September);
                println!("Month: {:?}", self.month);
            }
            9 => {
                self.month = Some(Month::October);
                println!("Month: {:?}", self.month);
            }
            10 => {
                self.month = Some(Month::November);
                println!("Month: {:?}", self.month);
            }
            11 => {
                self.month = Some(Month::December);
                println!("Month: {:?}", self.month);
            }
            _ => panic!("Invalid month index: {}", index),
        }
    }

    pub fn set_day(&mut self, day: String) {
        self.day = Some(day)
    }

    pub fn set_time(&mut self, time: String) {
        self.time = Some(time)
    }

    pub fn get_month(&self) -> Option<Month> {
        self.month.clone()
    }
}
