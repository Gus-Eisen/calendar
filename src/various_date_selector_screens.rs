use pelican_ui::drawable::{Component, Drawable};
use pelican_ui::events::OnEvent;
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Component, Context};

use pelican_ui_std::AppPage;
use pelican_ui_std::components::button::{
    Button, ButtonSize, ButtonState, ButtonStyle, IconButton,
};
use pelican_ui_std::components::interface::general::{Bumper, Content, Page};
use pelican_ui_std::components::list_item::ListItemSelector;
use pelican_ui_std::events::NavigateEvent;
use pelican_ui_std::layout::{Offset, Stack};

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
        let year = ListItemSelector::new(ctx, ("2025", "", None), ("2026", "", None), None, None);

        let content = Content::new(
            ctx,
            Offset::Start,
            vec![Box::new(return_to_eventeditorscreen_icon), Box::new(year)],
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
