mod day_view_screen;
mod event_editor_screen;
mod objects;
mod various_date_selector_screens;

use pelican_ui::components::button::{ButtonSize, ButtonStyle, GhostIconButton, PrimaryButton};
use pelican_ui::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent, RootInfo};
use pelican_ui::components::list_item::{ListItem, ListItemGroup};
use pelican_ui::components::{ExpandableText, Text, TextSize, TextStyle};
use pelican_ui::drawable::{Align, Color, Drawable};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::interactions::Button;
use pelican_ui::layouts::Stack;
use pelican_ui::layouts::{Column, Offset};
use pelican_ui::layouts::{Padding, Size};
use pelican_ui::pages::PelicanHome;
use pelican_ui::start;
use pelican_ui::theme::Theme;
use pelican_ui::{Application, Assets, Component, Context, Plugin, drawables, include_dir};
use std::collections::BTreeMap;

use pelican_ui::{ServiceList, Services};

use crate::event_editor_screen::EventEditorScreen;
use crate::objects::EventForEES;

// Define the main application struct. This is our entry point type.
pub struct Calendar;

impl Application for Calendar {
    fn interface(ctx: &mut Context) -> Interface {
        ////TODO: initialize all state objects in interface().
        // ctx.state().set(AllOrders::default());
        // ctx.state().set(AllBikes::default());

        let home = RootInfo::icon("home", "My Calendar", MonthScreen::new(ctx).ok().unwrap());

        Interface::new(ctx, vec![home])
    }

    fn theme(assets: &mut Assets) -> Theme {
        // assets.include_assets(pelican_ui::include_dir!("./resources"));
        Theme::light(assets, Color::from_hex("#ff1f84ff", 255))
    }
}

// Macro to start the application
start!(Calendar);

// Define the first screen of the app
#[derive(Debug, Component)]
pub struct MonthScreen(Column, Page);

// Implement event handling for FirstScreen (empty for now)
impl OnEvent for MonthScreen {}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for MonthScreen {}

impl MonthScreen {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        if ctx.state().get::<EventForEES>().is_none() {
            let event_for_ees = EventForEES::new(None, None, None, None, None);
            ctx.state().set(event_for_ees);
        }

        // Create a header for the page
        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx,        // The text on this header will say "Calendar"
            "Calendar", // There will not be an icon button on this header
            None,
        );

        // Create the main heading text
        let text = Text::new(
            ctx,
            "Hello World!",
            TextSize::Lg,
            TextStyle::Heading,
            Align::Center,
            None,
        );

        // Create subtext.
        let subtext = ExpandableText::new(
            ctx,
            "First project loaded successfully.",
            TextSize::Md,
            TextStyle::Primary,
            // Center the text
            Align::Center,
            // No max lines
            None,
        );

        let primary_button_test = PrimaryButton::new(
            ctx,
            "test",
            |ctx: &mut Context| {
                let page = Box::new(EventEditorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)))
            },
            false,
        );

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            // Vertically center items
            Offset::Center,
            // All items must be boxed as Box<dyn Drawable>
            vec![
                Box::new(text),
                Box::new(subtext),
                Box::new(primary_button_test),
            ],
        );

        Ok(Self(
            Column::new(1.0, Offset::Center, Size::Fit, Padding::new(1.0)),
            Page::new(header, content, None),
        ))
    }
}
