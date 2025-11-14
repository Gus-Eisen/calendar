use crate::objects::EventForEES;
use pelican_ui::Component;
use pelican_ui::Context;
use pelican_ui::components::RadioSelector;
use pelican_ui::components::button::GhostIconButton;
use pelican_ui::components::interface::general::{Bumper, Content, Page};
use pelican_ui::components::interface::navigation::AppPage;
use pelican_ui::components::interface::{general::Header, navigation::NavigationEvent};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::layouts::Offset;
use pelican_ui::layouts::Stack;

pub mod year_selector_screen_block {
    use crate::event_editor_screen::EventEditorScreen;

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
                                //TODO: delete this printf when done debugging !event_for_ees.year.
                                println!("{:?}", efees)
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
            let return_to_eventeditorscreen_icon = GhostIconButton::new(
                ctx,
                "backspace",
                Box::new(|ctx: &mut Context| {
                    ctx.trigger_event(NavigationEvent::Pop);
                    println!("return_to_eventeditorscreen_icon clicked.")
                }),
            );
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

            let content = Content::new(
                ctx,
                Offset::Start,
                vec![
                    Box::new(return_to_eventeditorscreen_icon),
                    Box::new(month_radioselector),
                ],
            );
            let bumper = Bumper::stack(ctx, Some("Save Year"), false, |ctx: &mut Context| {
                ctx.trigger_event(NavigationEvent::Pop);
                println!("Save Year button clicked.")
            });

            MonthSelectorScreen(
                Stack::default(),
                Page::new(Header::stack(ctx, "Select Year"), content, Some(bumper)),
            )
        }
    }
}

// pub fn month_radioselector_builder(ctx: &mut Context) -> RadioSelector {
//     RadioSelector::new(
//         ctx,
//         0,
//         vec![
//             ("January", "", Box::new(|ctx: &mut Context| if let Some(ees) = ctx.state().get_mut::<EventForEES>() {
//                 ees.set_month(JAN);
//                 println!("Selected January.")
//         })),
//             ("February", "", Box::new(|_| if let Some(ees) = ctx.state().get_mut::<EventForEES>() {
//                 ees.set_month(FEB);
//                 println!("Selected February.")
//         })),
//             ("March", "", Box::new(|_| println!("Selected March."))),
//             ("April", "", Box::new(|_| println!("Selected April."))),
//             ("May", "", Box::new(|_| println!("Selected May."))),
//             ("June", "", Box::new(|_| println!("Selected June."))),
//             ("July", "", Box::new(|_| println!("Selected July."))),
//             ("August", "", Box::new(|_| println!("Selected August."))),
//             (
//                 "September",
//                 "",
//                 Box::new(|_| println!("Selected September.")),
//             ),
//             ("October", "", Box::new(|_| println!("Selected October."))),
//             ("November", "", Box::new(|_| println!("Selected November."))),
//             ("December", "", Box::new(|_| println!("Selected December."))),
//         ],
//     )
// }
