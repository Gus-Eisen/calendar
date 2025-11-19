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

        pub fn day_of_week_determiner(ctx: &mut Context) -> &'static str {
            let ees = ctx.state().get::<EventForEES>().unwrap();
            let date = NaiveDate::from_ymd_opt(
                ees.get_year_as_i32(),
                ees.get_month_as_u32(),
                ees.get_day_as_u32(),
            )
            .unwrap();
            match date.weekday() {
                Weekday::Mon => "Mon",
                Weekday::Tue => "Tue",
                Weekday::Wed => "Wed",
                Weekday::Thu => "Thu",
                Weekday::Fri => "Fri",
                Weekday::Sat => "Sat",
                Weekday::Sun => "Sun",
            }
        }
        pub fn day_radioselector_builder(ctx: &mut Context) -> RadioSelector {
            let amt_of_days = Self::amt_of_days(ctx);
            let vec_radioselector: Vec<(&str, &str, Box<dyn FnMut(&mut Context) + 'static>)> = (1
                ..=amt_of_days)
                .map(|day| {
                    let day_str = Self::day_u8_to_stringliteral(day);
                    (
                        day_str,
                        //TODO: code in day of week logic here.
                        "",
                        Box::new(move |ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_day(day.to_string());
                                println!("Selected day {}", day);
                            }
                        }) as Box<dyn FnMut(&mut Context)>,
                    )
                })
                .collect();
            RadioSelector::new(ctx, 0, vec_radioselector)
        }
    }
}
