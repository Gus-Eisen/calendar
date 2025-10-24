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

        let event_title = TextInput::new(
            ctx,
            None,
            Some("Event Title"),
            "Enter Event Title",
            Some("Ex.: Strategy meeting with Satoshi"),
            TextInput::NO_ICON,
            true,
        );

        let year = Button::secondary(
            ctx,
            Some("right"),
            "Select year here",
            None,
            |ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(1));
                println!("Year button clicked.")
            },
            None,
        );

        let month = Button::secondary(
            ctx,
            Some("right"),
            "Select month here",
            None,
            |_ctx: &mut Context| println!("Month"),
            None,
        );

        let day = Button::secondary(
            ctx,
            Some("right"),
            "Select day here",
            None,
            |_ctx: &mut Context| println!("Day"),
            None,
        );

        let time = Button::secondary(
            ctx,
            Some("right"),
            "Select time here",
            None,
            |_ctx: &mut Context| println!("Time"),
            None,
        );

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

    //deprecated. Review and delete.
    pub fn year(ctx: &mut Context) -> Self {
        let return_to_eventeditorscreen_new = IconButton::new(
            ctx,
            "back",
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(|ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(2));
                println!("return_to_eventeditorscreen_new button clicked.");
            }),
            None,
        );
        let year = ListItemSelector::new(ctx, ("2025", "", None), ("2026", "", None), None, None);

        let content = Content::new(
            ctx,
            Offset::Start,
            vec![Box::new(return_to_eventeditorscreen_new), Box::new(year)],
        );
        let button = Button::primary(ctx, "Save Year", |ctx: &mut Context| {
            let event_for_ees = ctx
                .state()
                .get_named::<EventForEES>("event_for_ees")
                .unwrap();
            // println!("{}", year.index().unwrap());

            ctx.trigger_event(NavigateEvent(2));
            println!("Save Year clicked.")
        });
        let bumper = Bumper::single_button(ctx, button);
        EventEditorScreen(Stack::default(), Page::new(None, content, Some(bumper)))
    }
}
