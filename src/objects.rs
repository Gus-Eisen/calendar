use chrono::NaiveDateTime;

pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub enum MonthOfYear {
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

pub enum Year {
    Y2025,
}

pub struct Event {
    title: String,
    time: NaiveDateTime,
}

pub struct EventRegistry {
    vec_of_events: Vec<Event>,
}

#[derive(Debug, Clone)]
pub struct EventForEES {
    pub event_title: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub time: Option<String>,
}

impl EventForEES {
    pub fn new(
        event_title: Option<String>,
        year: Option<String>,
        month: Option<String>,
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
}
