use pelican_ui::components::button::{ButtonSize, ButtonStyle, GhostIconButton};
use pelican_ui::components::interface::general::{Bumper, Content, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigateEvent, PelicanError};
use pelican_ui::components::list_item::{ListItem, ListItemGroup};
use pelican_ui::components::{RadioSelector, TextInput};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::interactions::Button;
use pelican_ui::layouts::Offset;
use pelican_ui::layouts::Stack;
use pelican_ui::{Component, Context};

use crate::MonthScreen;
use crate::objects::EventForEES;
use crate::various_date_selector_screens::month_selector_screen_block::MonthSelectorScreen;
use crate::various_date_selector_screens::year_selector_screen_block::YearSelectorScreen;

#[derive(Debug, Component)]
pub struct EventEditorScreen(Stack, Page);

impl OnEvent for EventEditorScreen {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if event.downcast_ref::<InputEditedEvent>().is_some()
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
        true
    }
}

impl AppPage for EventEditorScreen {
    fn has_navigator(&self) -> bool {
        false
    }

    fn navigate(
        self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, PelicanError> {
        match index {
            0 => Ok(Box::new(MonthScreen::new(ctx))),
            1 => Ok(Box::new(YearSelectorScreen::new(ctx))),
            2 => Ok(Box::new(MonthSelectorScreen::new(ctx))),
            _ => Err(PelicanError::InvalidPage(Some(self))),
        }
    }
}

impl EventEditorScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let return_to_monthscreen_icon = GhostIconButton::new(
            ctx,
            "back",
            Box::new(|ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(0));
                println!("return_to_monthscreen_icon clicked.")
            }),
        );

        let event_for_ees = ctx
            .state()
            .get_named::<EventForEES>("event_for_ees")
            .unwrap()
            .to_owned();

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
            Button::secondary(
                ctx,
                Some("right"),
                year.as_str(),
                None,
                |ctx: &mut Context| {
                    ctx.trigger_event(NavigateEvent(1));
                    println!("year = event_for_ees.year.is_some clicked.")
                },
                None,
            )
        } else {
            Button::secondary(
                ctx,
                Some("right"),
                "Select year here",
                None,
                |ctx: &mut Context| {
                    ctx.trigger_event(NavigateEvent(1));
                    println!("year = !event_for_ees.year.is_some clicked.")
                },
                None,
            )
        };

        let month = if event_for_ees.month.is_some() {
            let month = event_for_ees.month.clone().unwrap();
            Button::secondary(
                ctx,
                Some("right"),
                month.as_str(),
                None,
                |ctx: &mut Context| {
                    ctx.trigger_event(NavigateEvent(2));
                    println!("month = event_for_ees.month.is_some clicked.");
                },
                None,
            )
        } else {
            Button::secondary(
                ctx,
                Some("right"),
                "Select month here",
                None,
                |ctx: &mut Context| {
                    println!("month = !event_for_ees.month.is_some clicked.");
                    ctx.trigger_event(NavigateEvent(2));
                },
                None,
            )
        };

        let day = if event_for_ees.day.is_some() {
            let day = event_for_ees.day.clone().unwrap();
            Button::secondary(
                ctx,
                Some("right"),
                &day,
                None,
                |_ctx: &mut Context| println!("day = event_for_ees.day.is_some clicked."),
                None,
            )
        } else {
            Button::secondary(
                ctx,
                Some("right"),
                "Select day here",
                None,
                |_ctx: &mut Context| println!("day = !event_for_ees.day.is_some clicked."),
                None,
            )
        };

        let time = if event_for_ees.time.is_some() {
            let time = event_for_ees.time.clone().unwrap();
            Button::secondary(
                ctx,
                Some("right"),
                &time,
                None,
                |_ctx: &mut Context| println!("time = event_for_ees.time.is_some clicked."),
                None,
            )
        } else {
            Button::secondary(
                ctx,
                Some("right"),
                "Select time here",
                None,
                |_ctx: &mut Context| println!("time = !event_for_ees.time.is_some clicked."),
                None,
            )
        };

        let content = Content::new(
            ctx,
            Offset::Start,
            vec![
                Box::new(return_to_monthscreen_icon),
                Box::new(event_title),
                Box::new(year),
                Box::new(month),
                Box::new(day),
                Box::new(time),
            ],
        );

        let button = Button::primary(ctx, "Save Event", |ctx: &mut Context| {
            let event_for_ees = ctx
                .state()
                .get_named::<EventForEES>("event_for_ees")
                .unwrap();
            event_for_ees.all_some();
            ctx.trigger_event(NavigateEvent(0));
            println!("Save Event button clicked.")
        });

        let bumper = Bumper::single_button(ctx, button);

        EventEditorScreen(Stack::default(), Page::new(None, content, Some(bumper)))
    }
}
