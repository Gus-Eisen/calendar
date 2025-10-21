use pelican_ui::drawable::{Align, Component, Drawable};
use pelican_ui::events::OnEvent;
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Application, Component, Context, Plugin, Plugins, start};
use std::collections::BTreeMap;

use pelican_ui_std::AppPage;
use pelican_ui_std::components::button::Button;
use pelican_ui_std::components::interface::general::{Content, Header, Interface, Page};
use pelican_ui_std::components::{ExpandableText, Text, TextStyle};
use pelican_ui_std::events::NavigateEvent;
use pelican_ui_std::layout::{Column, Offset, Padding, Size};

use crate::event_editor_screen::EventEditorScreen;

#[derive(Debug, Component)]
pub struct YearSelectorScreen(Column, Page, #[skip] String);

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
