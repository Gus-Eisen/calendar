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

    const Y2025: u8 = 0;
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
                0,
                vec![
                    (
                        "2025",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_year(Y2025);
                                println!("2025 selected.")
                            }
                        }),
                    ),
                    (
                        "2026",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_year(Y2026);
                                println!("2026 selected.");
                            }
                        }),
                    ),
                ],
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

    const JAN: u8 = 0;
    const FEB: u8 = 1;
    const MAR: u8 = 2;
    const APR: u8 = 3;
    const MAY: u8 = 4;
    const JUN: u8 = 5;
    const JUL: u8 = 6;
    const AUG: u8 = 7;
    const SEP: u8 = 8;
    const OCT: u8 = 9;
    const NOV: u8 = 10;
    const DEC: u8 = 11;

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
                0,
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

            RadioSelector::new(ctx, 0, vec_of_radioselector)
        }
    }
}

pub mod time_selector_screen_block {
    use super::*;
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
            let time_radioselector = RadioSelector::new(
                ctx,
                0,
                vec![
                    (
                        "12:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0000".to_string());
                                println!("12:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0015".to_string());
                                println!("12:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0030".to_string());
                                println!("12:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0045".to_string());
                                println!("12:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0100".to_string());
                                println!("1:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0115".to_string());
                                println!("1:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0130".to_string());
                                println!("1:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0145".to_string());
                                println!("1:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0200".to_string());
                                println!("2:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0215".to_string());
                                println!("2:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0230".to_string());
                                println!("2:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0245".to_string());
                                println!("2:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0300".to_string());
                                println!("3:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0315".to_string());
                                println!("3:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0330".to_string());
                                println!("3:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0345".to_string());
                                println!("3:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0400".to_string());
                                println!("4:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0415".to_string());
                                println!("4:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0430".to_string());
                                println!("4:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0445".to_string());
                                println!("4:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0500".to_string());
                                println!("5:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0515".to_string());
                                println!("5:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0530".to_string());
                                println!("5:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0545".to_string());
                                println!("5:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0600".to_string());
                                println!("6:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0615".to_string());
                                println!("6:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0630".to_string());
                                println!("6:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0645".to_string());
                                println!("6:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0700".to_string());
                                println!("7:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0715".to_string());
                                println!("7:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0730".to_string());
                                println!("7:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0745".to_string());
                                println!("7:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0800".to_string());
                                println!("8:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0815".to_string());
                                println!("8:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0830".to_string());
                                println!("8:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0845".to_string());
                                println!("8:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0900".to_string());
                                println!("9:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0915".to_string());
                                println!("9:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0930".to_string());
                                println!("9:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("0945".to_string());
                                println!("9:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1000".to_string());
                                println!("10:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1015".to_string());
                                println!("10:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1030".to_string());
                                println!("10:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1045".to_string());
                                println!("10:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:00 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1100".to_string());
                                println!("11:00 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:15 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1115".to_string());
                                println!("11:15 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:30 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1130".to_string());
                                println!("11:30 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:45 a.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1145".to_string());
                                println!("11:45 a.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1200".to_string());
                                println!("12:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1215".to_string());
                                println!("12:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1230".to_string());
                                println!("12:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "12:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1245".to_string());
                                println!("12:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1300".to_string());
                                println!("1:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1315".to_string());
                                println!("1:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1330".to_string());
                                println!("1:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "1:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1345".to_string());
                                println!("1:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1400".to_string());
                                println!("2:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1415".to_string());
                                println!("2:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1430".to_string());
                                println!("2:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "2:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1445".to_string());
                                println!("2:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1500".to_string());
                                println!("3:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1515".to_string());
                                println!("3:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1530".to_string());
                                println!("3:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "3:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1545".to_string());
                                println!("3:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1600".to_string());
                                println!("4:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1615".to_string());
                                println!("4:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1630".to_string());
                                println!("4:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "4:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1645".to_string());
                                println!("4:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1700".to_string());
                                println!("5:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1715".to_string());
                                println!("5:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1730".to_string());
                                println!("5:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "5:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1745".to_string());
                                println!("5:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1800".to_string());
                                println!("6:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1815".to_string());
                                println!("6:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1830".to_string());
                                println!("6:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "6:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1845".to_string());
                                println!("6:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1900".to_string());
                                println!("7:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1915".to_string());
                                println!("7:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1930".to_string());
                                println!("7:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "7:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("1945".to_string());
                                println!("7:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2000".to_string());
                                println!("8:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2015".to_string());
                                println!("8:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2030".to_string());
                                println!("8:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "8:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2045".to_string());
                                println!("8:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2100".to_string());
                                println!("9:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2115".to_string());
                                println!("9:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2130".to_string());
                                println!("9:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "9:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2145".to_string());
                                println!("9:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2200".to_string());
                                println!("10:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2215".to_string());
                                println!("10:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2230".to_string());
                                println!("10:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "10:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2245".to_string());
                                println!("10:45 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:00 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2300".to_string());
                                println!("11:00 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:15 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2315".to_string());
                                println!("11:15 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:30 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2330".to_string());
                                println!("11:30 p.m. selected.");
                            }
                        }),
                    ),
                    (
                        "11:45 p.m.",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_time("2345".to_string());
                                println!("11:45 p.m. selected.");
                            }
                        }),
                    ),
                ],
            );
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
}
