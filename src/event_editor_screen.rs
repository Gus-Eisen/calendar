use pelican_ui::drawable::{Component, Drawable};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Component, Context};
use pelican_ui_std::{
    AppPage, ButtonSize, ButtonState, ButtonStyle, ClearActiveInput, Content, Header, IconButton,
    InputEditedEvent, NavigateEvent, Offset, Page, Stack, TextInput,
};

use crate::MonthScreen;

#[derive(Debug, Component)]
pub struct EventEditorScreen(Stack, Page);

impl OnEvent for EventEditorScreen {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        true
    }
}

impl AppPage for EventEditorScreen {
    fn has_nav(&self) -> bool {
        false
    }

    fn navigate(
        mut self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => return Ok(Box::new(MonthScreen::new(ctx))),
            _ => Err(self),
        }
    }
}

impl EventEditorScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let return_to_monthscreen_icon = IconButton::new(
            ctx,
            "backspace",
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(|ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(0));
                println!("return_to_monthscreen_icon clicked.")
            }),
            None,
        );

        let content = Content::new(
            ctx,
            Offset::Start,
            vec![Box::new(return_to_monthscreen_icon)],
        );

        EventEditorScreen(Stack::default(), Page::new(None, content, None))
    }
}
