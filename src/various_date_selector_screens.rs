use pelican_ui::drawable::{Align, Component, Drawable};
use pelican_ui::events::OnEvent;
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Application, Component, Context, Plugin, Plugins, start};
use std::collections::BTreeMap;

use pelican_ui_std::AppPage;
use pelican_ui_std::components::button::{
    Button, ButtonSize, ButtonState, ButtonStyle, IconButton,
};
use pelican_ui_std::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican_ui_std::components::{ExpandableText, Text, TextStyle};
use pelican_ui_std::events::NavigateEvent;
use pelican_ui_std::layout::{Column, Offset, Padding, Size, Stack};

use crate::event_editor_screen::EventEditorScreen;

#[derive(Debug, Component)]
pub struct YearSelectorScreen(Stack, Page, #[skip] String);

impl OnEvent for YearSelectorScreen {}

impl AppPage for YearSelectorScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool {
        false
    }

    // Handle page navigation. Always returns Err(self) because this page cannot navigate.
    fn navigate(
        self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(EventEditorScreen::new(ctx))),
            _ => Err(self),
        }
    }
}

impl YearSelectorScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let return_to_eventeditorscreen_icon = IconButton::new(
            ctx,
            "backspace",
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(|ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(0));
                println!("return_to_eventeditorscreen_icon clicked.")
            }),
            None,
        );
        let content = Content::new(
            ctx,
            Offset::Start,
            vec![Box::new(return_to_eventeditorscreen_icon)],
        );
        let button = Button::primary(ctx, "Save Year", |ctx: &mut Context| {
            ctx.trigger_event(NavigateEvent(0));
            println!("Save Year button clicked.")
        });
        let bumper = Bumper::single_button(ctx, button);

        YearSelectorScreen(
            Stack::default(),
            Page::new(None, content, Some(bumper)),
            String::default(),
        )
    }
}
