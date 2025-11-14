use pelican_ui::components::TextInput;
use pelican_ui::components::button::{GhostIconButton, SecondaryButton};
use pelican_ui::components::interface::general::{Bumper, Content, Header, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent};
use pelican_ui::events::{Event, KeyboardEvent, OnEvent};
use pelican_ui::layouts::Offset;
use pelican_ui::layouts::Stack;
use pelican_ui::{Component, Context};

use crate::objects::EventForEES;
use crate::various_date_selector_screens::month_selector_screen_block::MonthSelectorScreen;
use crate::various_date_selector_screens::year_selector_screen_block::YearSelectorScreen;

#[derive(Debug, Component)]
pub struct EventEditorScreen(Stack, Page);

impl OnEvent for EventEditorScreen {
    fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        if event.downcast_ref::<KeyboardEvent>().is_some()
            && let Some(input) = self.1.content().find::<TextInput>()
        {
            let event_for_ees = ctx
                .state()
                .get_named_mut::<EventForEES>("event_for_ees")
                .unwrap();

            event_for_ees.event_title = Some(input.value().clone());
            println!(
                "Event Title: {:?}",
                event_for_ees.event_title.clone().unwrap()
            );
        }
        vec![]
    }
}

impl AppPage for EventEditorScreen {}

impl EventEditorScreen {
    pub fn new(ctx: &mut Context) -> Self {
        // let return_to_monthscreen_icon = GhostIconButton::new(
        //     ctx,
        //     "back",
        //     Box::new(|ctx: &mut Context| {
        //         ctx.trigger_event(NavigationEvent::Reset);
        //         println!("return_to_monthscreen_icon clicked.")
        //     }),
        // );

        let event_for_ees = ctx.state().get::<EventForEES>().unwrap().to_owned();

        let event_title = if event_for_ees.event_title.is_some() {
            let event_title = event_for_ees.event_title.clone().unwrap();
            TextInput::new(
                ctx,
                None,
                Some("Event Title"),
                Some(&event_title),
                None,
                None,
            )
        } else {
            TextInput::new(
                ctx,
                None,
                Some("Event Title"),
                Some("Enter Event Title here"),
                Some("Ex.: Strategy meeting with Satoshi"),
                None,
            )
        };

        let year = if event_for_ees.year.is_some() {
            let year = event_for_ees.year.clone().unwrap();
            SecondaryButton::medium(ctx, "right", year.as_str(), None, |ctx: &mut Context| {
                let page = Box::new(YearSelectorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("year = event_for_ees.year.is_some clicked.")
            })
        } else {
            SecondaryButton::medium(
                ctx,
                "right",
                "Select year here",
                None,
                |ctx: &mut Context| {
                    let page = Box::new(YearSelectorScreen::new(ctx));
                    ctx.trigger_event(NavigationEvent::Push(Some(page)));
                    println!("year = !event_for_ees.year.is_some clicked.")
                },
            )
        };

        let month = if event_for_ees.month.is_some() {
            let month = event_for_ees.month.clone().unwrap();
            SecondaryButton::medium(ctx, "right", month.as_str(), None, |ctx: &mut Context| {
                let page = Box::new(MonthSelectorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("month = event_for_ees.month.is_some clicked.");
            })
        } else {
            SecondaryButton::medium(
                ctx,
                "right",
                "Select month here",
                None,
                |ctx: &mut Context| {
                    println!("month = !event_for_ees.month.is_some clicked.");
                    let page = Box::new(MonthSelectorScreen::new(ctx));
                    ctx.trigger_event(NavigationEvent::Push(Some(page)));
                },
            )
        };

        let day = if event_for_ees.day.is_some() {
            let day = event_for_ees.day.clone().unwrap();
            SecondaryButton::medium(ctx, "right", &day, None, |_ctx: &mut Context| {
                println!("day = event_for_ees.day.is_some clicked.")
            })
        } else {
            SecondaryButton::medium(
                ctx,
                "right",
                "Select day here",
                None,
                |_ctx: &mut Context| println!("day = !event_for_ees.day.is_some clicked."),
            )
        };

        let time = if event_for_ees.time.is_some() {
            let time = event_for_ees.time.clone().unwrap();
            SecondaryButton::medium(ctx, "right", &time, None, |_ctx: &mut Context| {
                println!("time = event_for_ees.time.is_some clicked.")
            })
        } else {
            SecondaryButton::medium(
                ctx,
                "right",
                "Select time here",
                None,
                |_ctx: &mut Context| println!("time = !event_for_ees.time.is_some clicked."),
            )
        };

        let content = Content::new(
            ctx,
            Offset::Start,
            vec![
                Box::new(event_title),
                Box::new(year),
                Box::new(month),
                Box::new(day),
                Box::new(time),
            ],
        );

        // let bumper = Bumper::single_button(ctx, button);
        let bumper = Bumper::stack(ctx, Some("Save Event"), false, |ctx: &mut Context| {
            let event_for_ees = ctx
                .state()
                .get_named::<EventForEES>("event_for_ees")
                .unwrap();
            event_for_ees.all_some();
            ctx.trigger_event(NavigationEvent::Reset);
            println!("Save Event button clicked.")
        });

        EventEditorScreen(
            Stack::default(),
            Page::new(Header::stack(ctx, "Stack Test"), content, Some(bumper)),
        )
    }
}
