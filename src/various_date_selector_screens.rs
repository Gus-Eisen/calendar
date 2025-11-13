use crate::event_editor_screen::EventEditorScreen;
use crate::objects::EventForEES;
use pelican_ui::Context;
use pelican_ui::components::RadioSelector;
use pelican_ui::components::button::{ButtonSize, ButtonStyle, GhostIconButton};
use pelican_ui::components::interface::general::{Bumper, Content, Page};
use pelican_ui::components::interface::navigation::AppPage;
use pelican_ui::components::list_item::{ListItem, ListItemGroup};
use pelican_ui::drawable;
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::interactions::Button;
use pelican_ui::layout::Layout;
use pelican_ui::layouts::Offset;
use pelican_ui::layouts::Stack;
use pelican_ui::{Application, Component, Plugin, drawables, include_dir};

pub mod year_selector_screen_block {

    use pelican_ui::{
        components::interface::{general::Header, navigation::NavigationEvent},
        events::MouseEvent,
    };

    use super::*;

    #[derive(Debug, Component)]
    pub struct YearSelectorScreen(Stack, Page, #[skip] String);

    impl OnEvent for YearSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
            ////FIX: Figure out event type emitter for RadioSelector.
            //if event.downcast_ref::<MouseEvent>().is_some() {
            //    let index = self
            //        .1
            //        .content()
            //        .find::<RadioSelector>()
            //        .unwrap()
            //        .
            //        .unwrap();
            //    let event_for_ees = ctx
            //        .state()
            //        .get_mut::<EventForEES>()
            //        .unwrap();
            //    event_for_ees.set_year(index);
            //}
            vec![]
        }
    }

    impl AppPage for YearSelectorScreen {
        // fn has_navigator(&self) -> bool {
        //     true
        // }
        //
        // fn navigate(
        //     self: Box<Self>,
        //     ctx: &mut Context,
        //     index: usize,
        // ) -> Result<Box<dyn AppPage>, PelicanError> {
        //     match index {
        //         0 => Ok(Box::new(EventEditorScreen::new(ctx))),
        //         _ => Err(PelicanError::InvalidPage(Some(self))),
        //     }
        // }
    }

    impl YearSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let return_to_eventeditorscreen_icon = GhostIconButton::new(
                ctx,
                "backspace",
                Box::new(|ctx: &mut Context| {
                    ctx.trigger_event(NavigationEvent::Pop);
                    println!("return_to_eventeditorscreen_icon clicked.")
                }),
            );
            let year_radioselector = RadioSelector::new(
                ctx,
                0,
                vec![
                    (
                        "2025",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_year(0);
                                println!("2025 selected.")
                            }
                        }),
                    ),
                    (
                        "2026",
                        "",
                        Box::new(|ctx: &mut Context| {
                            if let Some(efees) = ctx.state().get_mut::<EventForEES>() {
                                efees.set_year(1);
                                println!("2026 selected.")
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
                    Box::new(year_radioselector),
                ],
            );
            // let button = Button::primary(ctx, "Save Year", |ctx: &mut Context| {
            //     ctx.trigger_event(NavigationEvent::Pop);
            //     println!("Save Year button clicked.")
            // });
            // let bumper = Bumper::single_button(ctx, button);
            let bumper = Bumper::stack(ctx, Some("Save Year"), false, |ctx: &mut Context| {
                ctx.trigger_event(NavigationEvent::Pop);
                println!("Save Year bumper clicked.")
            });

            YearSelectorScreen(
                Stack::default(),
                Page::new(Header::stack(ctx, "Select Year"), content, Some(bumper)),
                String::default(),
            )
        }
    }
}

pub mod month_selector_screen_block {

    use pelican_ui::components::{
        button::PrimaryButton,
        interface::{general::Header, navigation::NavigationEvent},
    };

    use crate::objects::Month;

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
    const DEC: u8 = 12;

    #[derive(Debug, Component)]
    pub struct MonthSelectorScreen(Stack, Page);

    impl OnEvent for MonthSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
            // if event.downcast_ref::<ListItemSelect>().is_some() {
            //     let index = self
            //         .1
            //         .content()
            //         .find::<ListItemGroup>()
            //         .unwrap()
            //         //TODO: figure out how to correlate ListItem.
            //         .inner()
            //         .iter();
            //
            //     let event_for_ees = ctx
            //         .state()
            //         .get_named_mut::<EventForEES>("event_for_ees")
            //         .unwrap();
            //     event_for_ees.set_month(index);
            // }
            vec![]
        }
    }

    impl AppPage for MonthSelectorScreen {
        // // This screen does not have a navigation bar
        // fn has_navigator(&self) -> bool {
        //     false
        // }
        //
        // fn navigate(
        //     self: Box<Self>,
        //     ctx: &mut Context,
        //     index: usize,
        // ) -> Result<Box<dyn AppPage>, PelicanError> {
        //     match index {
        //         0 => Ok(Box::new(EventEditorScreen::new(ctx))),
        //         _ => Err(PelicanError::InvalidPage(Some(self))),
        //     }
        // }
    }

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
