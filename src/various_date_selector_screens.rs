use crate::event_editor_screen::EventEditorScreen;
use crate::objects::EventForEES;

use pelican_ui::Component;
use pelican_ui::Context;
use pelican_ui::components::RadioSelector;
use pelican_ui::components::interface::general::{Bumper, Content, Page};
use pelican_ui::components::interface::navigation::AppPage;
use pelican_ui::components::interface::{general::Header, navigation::NavigationEvent};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::layouts::Offset;
use pelican_ui::layouts::Stack;

pub mod year_selector_screen_block {

    use super::*;

    const Y2026: u8 = 1;

    #[derive(Debug, Component)]
    pub struct YearSelectorScreen(Stack, Page);

    impl OnEvent for YearSelectorScreen {
        fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
            vec![event]
        }
    }

    impl AppPage for YearSelectorScreen {}

    impl YearSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let year_radioselector = RadioSelector::new(
                ctx,
                usize::MAX,
                vec![(
                    "2026",
                    "",
                    Box::new(|ctx: &mut Context| {
                        if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                            efees.set_year(Y2026);
                            println!("2026 selected.");
                        }
                    }),
                )],
            );

            let content = Content::new(ctx, Offset::Start, vec![Box::new(year_radioselector)]);
            let bumper = Bumper::stack(ctx, Some("Save Year"), false, |ctx: &mut Context| {
                let page = Box::new(EventEditorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("Save Year bumper clicked.")
            });

            YearSelectorScreen(
                Stack::default(),
                Page::new(Header::stack(ctx, "Select Year"), content, Some(bumper)),
            )
        }
    }
}

pub mod month_selector_screen_block {

    use super::*;

    const JAN: u8 = 1;
    const FEB: u8 = 2;
    const MAR: u8 = 3;
    const APR: u8 = 4;
    const MAY: u8 = 5;
    const JUN: u8 = 6;
    const JUL: u8 = 7;
    const AUG: u8 = 8;
    const SEP: u8 = 9;
    const OCT: u8 = 10;
    const NOV: u8 = 11;
    const DEC: u8 = 12;

    #[derive(Debug, Component)]
    pub struct MonthSelectorScreen(Stack, Page);

    impl OnEvent for MonthSelectorScreen {
        fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
            vec![event]
        }
    }

    impl AppPage for MonthSelectorScreen {}

    impl MonthSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let month_radioselector = RadioSelector::new(
                ctx,
                usize::MAX,
                vec![
                    (
                        "January",
                        "1",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(JAN);
                                println!("Selected January.")
                            }
                        }),
                    ),
                    (
                        "February",
                        "2",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(FEB);
                                println!("Selected February.")
                            }
                        }),
                    ),
                    (
                        "March",
                        "3",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(MAR);
                                println!("Selected March.")
                            }
                        }),
                    ),
                    (
                        "April",
                        "4",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(APR);
                                println!("Selected April.")
                            }
                        }),
                    ),
                    (
                        "May",
                        "5",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(MAY);
                                println!("Selected May.")
                            }
                        }),
                    ),
                    (
                        "June",
                        "6",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(JUN);
                                println!("Selected June.")
                            }
                        }),
                    ),
                    (
                        "July",
                        "7",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(JUL);
                                println!("Selected July.")
                            }
                        }),
                    ),
                    (
                        "August",
                        "8",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(AUG);
                                println!("Selected August.")
                            }
                        }),
                    ),
                    (
                        "September",
                        "9",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(SEP);
                                println!("Selected September.")
                            }
                        }),
                    ),
                    (
                        "October",
                        "10",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(OCT);
                                println!("Selected October.")
                            }
                        }),
                    ),
                    (
                        "November",
                        "11",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(NOV);
                                println!("Selected November.")
                            }
                        }),
                    ),
                    (
                        "December",
                        "12",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_month(DEC);
                                println!("Selected December.")
                            }
                        }),
                    ),
                ],
            );

            let content = Content::new(ctx, Offset::Start, vec![Box::new(month_radioselector)]);
            let bumper = Bumper::stack(ctx, Some("Save Month"), false, |ctx: &mut Context| {
                let page = Box::new(EventEditorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("Save Month button clicked.")
            });

            MonthSelectorScreen(
                Stack::default(),
                Page::new(Header::stack(ctx, "Select Month"), content, Some(bumper)),
            )
        }
    }
}

pub mod day_selector_screen_block {
    use crate::objects::Month;
    use chrono::{Datelike, NaiveDate, Weekday};

    use super::*;

    #[derive(Component, Debug)]
    pub struct DaySelectorScreen(Stack, Page);

    impl OnEvent for DaySelectorScreen {
        fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
            vec![event]
        }
    }

    impl AppPage for DaySelectorScreen {}

    impl DaySelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let day_radioselector = Self::day_radioselector_builder(ctx);
            let content = Content::new(ctx, Offset::Start, vec![Box::new(day_radioselector)]);
            let bumper = Bumper::stack(ctx, Some("Save Day"), false, |ctx: &mut Context| {
                let page = Box::new(EventEditorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("Save Day button clicked.")
            });
            DaySelectorScreen(
                Stack::default(),
                Page::new(Header::stack(ctx, "Select Day"), content, Some(bumper)),
            )
        }

        // Pulls Month from EventForEES and returns number of days as u8.
        pub fn amt_of_days(ctx: &mut Context) -> u8 {
            let efees = ctx.state().get::<EventForEES>().unwrap();
            match efees.get_month().unwrap() {
                Month::January => 31,
                //TODO: code in logic for leap year.
                Month::February => 28,
                Month::March => 31,
                Month::April => 30,
                Month::May => 31,
                Month::June => 30,
                Month::July => 31,
                Month::August => 31,
                Month::September => 30,
                Month::October => 31,
                Month::November => 30,
                Month::December => 31,
            }
        }
        pub fn day_u8_to_stringliteral(day: u8) -> &'static str {
            match day {
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                10 => "10",
                11 => "11",
                12 => "12",
                13 => "13",
                14 => "14",
                15 => "15",
                16 => "16",
                17 => "17",
                18 => "18",
                19 => "19",
                20 => "20",
                21 => "21",
                22 => "22",
                23 => "23",
                24 => "24",
                25 => "25",
                26 => "26",
                27 => "27",
                28 => "28",
                29 => "29",
                30 => "30",
                31 => "31",
                _ => panic!("Out of bounds day."),
            }
        }

        // Pulls Year and Month from EventForEES and returns an ordered vec of Weekday(s).
        pub fn days_of_week_determiner(ctx: &mut Context) -> Vec<Weekday> {
            let ees = ctx.state().get::<EventForEES>().unwrap();
            let mut date =
                NaiveDate::from_ymd_opt(ees.get_year_as_i32(), ees.get_month_as_u32(), 1).unwrap();
            let mut weekdays = Vec::new();

            while date.month() == ees.get_month_as_u32() {
                weekdays.push(date.weekday());
                date = date.succ_opt().unwrap();
            }
            weekdays
        }
        pub fn day_radioselector_builder(ctx: &mut Context) -> RadioSelector {
            let amt_of_days = Self::amt_of_days(ctx);
            let vec_of_weekday = Self::days_of_week_determiner(ctx);

            let vec_of_radioselector: Vec<(&str, &str, Box<dyn FnMut(&mut Context) + 'static>)> =
                (1..=amt_of_days)
                    .map(|day| {
                        let day_str = Self::day_u8_to_stringliteral(day);
                        let weekday_str = vec_of_weekday
                            .get((day - 1) as usize)
                            .map(|wd| match wd {
                                Weekday::Mon => "Mon",
                                Weekday::Tue => "Tue",
                                Weekday::Wed => "Wed",
                                Weekday::Thu => "Thu",
                                Weekday::Fri => "Fri",
                                Weekday::Sat => "Sat",
                                Weekday::Sun => "Sun",
                            })
                            .expect("Weekday variant not found.");

                        (
                            day_str,
                            weekday_str,
                            Box::new(move |ctx: &mut Context| {
                                if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                    efees.set_day(day.to_string());
                                    println!("Selected day {}", day);
                                }
                            }) as Box<dyn FnMut(&mut Context)>,
                        )
                    })
                    .collect();

            RadioSelector::new(ctx, usize::MAX, vec_of_radioselector)
        }
    }
}

pub mod time_selector_screen_block {
    use super::*;
    const TIME_DISPLAYS: [&str; 96] = [
        "12:00 a.m.",
        "12:15 a.m.",
        "12:30 a.m.",
        "12:45 a.m.",
        "1:00 a.m.",
        "1:15 a.m.",
        "1:30 a.m.",
        "1:45 a.m.",
        "2:00 a.m.",
        "2:15 a.m.",
        "2:30 a.m.",
        "2:45 a.m.",
        "3:00 a.m.",
        "3:15 a.m.",
        "3:30 a.m.",
        "3:45 a.m.",
        "4:00 a.m.",
        "4:15 a.m.",
        "4:30 a.m.",
        "4:45 a.m.",
        "5:00 a.m.",
        "5:15 a.m.",
        "5:30 a.m.",
        "5:45 a.m.",
        "6:00 a.m.",
        "6:15 a.m.",
        "6:30 a.m.",
        "6:45 a.m.",
        "7:00 a.m.",
        "7:15 a.m.",
        "7:30 a.m.",
        "7:45 a.m.",
        "8:00 a.m.",
        "8:15 a.m.",
        "8:30 a.m.",
        "8:45 a.m.",
        "9:00 a.m.",
        "9:15 a.m.",
        "9:30 a.m.",
        "9:45 a.m.",
        "10:00 a.m.",
        "10:15 a.m.",
        "10:30 a.m.",
        "10:45 a.m.",
        "11:00 a.m.",
        "11:15 a.m.",
        "11:30 a.m.",
        "11:45 a.m.",
        "12:00 p.m.",
        "12:15 p.m.",
        "12:30 p.m.",
        "12:45 p.m.",
        "1:00 p.m.",
        "1:15 p.m.",
        "1:30 p.m.",
        "1:45 p.m.",
        "2:00 p.m.",
        "2:15 p.m.",
        "2:30 p.m.",
        "2:45 p.m.",
        "3:00 p.m.",
        "3:15 p.m.",
        "3:30 p.m.",
        "3:45 p.m.",
        "4:00 p.m.",
        "4:15 p.m.",
        "4:30 p.m.",
        "4:45 p.m.",
        "5:00 p.m.",
        "5:15 p.m.",
        "5:30 p.m.",
        "5:45 p.m.",
        "6:00 p.m.",
        "6:15 p.m.",
        "6:30 p.m.",
        "6:45 p.m.",
        "7:00 p.m.",
        "7:15 p.m.",
        "7:30 p.m.",
        "7:45 p.m.",
        "8:00 p.m.",
        "8:15 p.m.",
        "8:30 p.m.",
        "8:45 p.m.",
        "9:00 p.m.",
        "9:15 p.m.",
        "9:30 p.m.",
        "9:45 p.m.",
        "10:00 p.m.",
        "10:15 p.m.",
        "10:30 p.m.",
        "10:45 p.m.",
        "11:00 p.m.",
        "11:15 p.m.",
        "11:30 p.m.",
        "11:45 p.m.",
    ];

    const TIME_CODES: [&str; 96] = [
        "0000", "0015", "0030", "0045", "0100", "0115", "0130", "0145", "0200", "0215", "0230",
        "0245", "0300", "0315", "0330", "0345", "0400", "0415", "0430", "0445", "0500", "0515",
        "0530", "0545", "0600", "0615", "0630", "0645", "0700", "0715", "0730", "0745", "0800",
        "0815", "0830", "0845", "0900", "0915", "0930", "0945", "1000", "1015", "1030", "1045",
        "1100", "1115", "1130", "1145", "1200", "1215", "1230", "1245", "1300", "1315", "1330",
        "1345", "1400", "1415", "1430", "1445", "1500", "1515", "1530", "1545", "1600", "1615",
        "1630", "1645", "1700", "1715", "1730", "1745", "1800", "1815", "1830", "1845", "1900",
        "1915", "1930", "1945", "2000", "2015", "2030", "2045", "2100", "2115", "2130", "2145",
        "2200", "2215", "2230", "2245", "2300", "2315", "2330", "2345",
    ];

    #[derive(Component, Debug)]
    pub struct TimeSelectorScreen(Stack, Page);

    impl OnEvent for TimeSelectorScreen {
        fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
            vec![event]
        }
    }

    impl AppPage for TimeSelectorScreen {}

    impl TimeSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let time_builder = vec_for_timeselector_builder(ctx);
            let time_radioselector = RadioSelector::new(ctx, usize::MAX, time_builder);
            let content = Content::new(ctx, Offset::Start, vec![Box::new(time_radioselector)]);
            let bumper = Bumper::stack(ctx, Some("Save Year"), false, |ctx: &mut Context| {
                let page = Box::new(EventEditorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("Save Year bumper clicked.")
            });

            TimeSelectorScreen(
                Stack::default(),
                Page::new(Header::stack(ctx, "Select Time"), content, Some(bumper)),
            )
        }
    }

    fn vec_for_timeselector_builder(
        _ctx: &mut Context,
    ) -> Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)> {
        (0..96)
            .map(|i| {
                let display_str = TIME_DISPLAYS[i];
                let time_code = TIME_CODES[i];

                (
                    display_str,
                    "",
                    Box::new(move |ctx: &mut Context| {
                        if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                            efees.set_time(time_code.to_string());
                            println!("{} selected.", display_str);
                        }
                    }) as Box<dyn FnMut(&mut Context)>,
                )
            })
            .collect()
    }
}
