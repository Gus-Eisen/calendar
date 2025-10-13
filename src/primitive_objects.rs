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
