use pelican_ui::drawable::{Component, Drawable};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Component, Context};

use pelican_ui_std::AppPage;
use pelican_ui_std::components::TextInput;
use pelican_ui_std::components::button::{
    Button, ButtonSize, ButtonState, ButtonStyle, IconButton,
};
use pelican_ui_std::components::interface::general::{Bumper, Content, Page};
use pelican_ui_std::components::list_item::ListItemSelector;
use pelican_ui_std::events::NavigateEvent;
use pelican_ui_std::layout::{Offset, Stack};

use crate::MonthScreen;
use crate::objects::EventForEES;
use crate::various_date_selector_screens::YearSelectorScreen;

#[derive(Debug, Component)]
pub struct EventEditorScreen(Stack, Page);

impl OnEvent for EventEditorScreen {
    fn on_event(&mut self, _ctx: &mut Context, _event: &mut dyn Event) -> bool {
        // if event.downcast_ref()::<TextInputSelect>.is_some() {
        //
        // }
        true
    }
}

impl AppPage for EventEditorScreen {
    fn has_nav(&self) -> bool {
        false
    }

    fn navigate(
        self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(MonthScreen::new(ctx))),
            1 => Ok(Box::new(YearSelectorScreen::new(ctx))),
            2 => Ok(Box::new(EventEditorScreen::new(ctx))),
            _ => Err(self),
        }
    }
}

impl EventEditorScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let return_to_monthscreen_icon = IconButton::new(
            ctx,
            "back",
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(|ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(0));
                println!("return_to_monthscreen_icon clicked.")
            }),
            None,
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
                &event_title,
                None,
                TextInput::NO_ICON,
                true,
            )
        } else {
            TextInput::new(
                ctx,
                None,
                Some("Event Title"),
                "Enter Event Title here",
                Some("Ex.: Strategy meeting with Satoshi"),
                TextInput::NO_ICON,
                true,
            )
        };

        let year = if event_for_ees.year.is_some() {
            let year = event_for_ees.year.clone().unwrap();
            Button::secondary(
                ctx,
                Some("right"),
                &year,
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
                &month,
                None,
                |_ctx: &mut Context| {
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
                |_ctx: &mut Context| println!("month = !event_for_ees.month.is_some clicked."),
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
            ctx.trigger_event(NavigateEvent(0));
            println!("Save Event button clicked.")
        });

        let bumper = Bumper::single_button(ctx, button);

        EventEditorScreen(Stack::default(), Page::new(None, content, Some(bumper)))
    }
}
