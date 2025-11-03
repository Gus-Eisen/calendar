mod day_view_screen;
mod event_editor_screen;
mod objects;
mod various_date_selector_screens;

use pelican::components::button::{ButtonSize, ButtonStyle, GhostIconButton};
use pelican::components::interface::general::{Bumper, Content, Header, Interface, Page};
use pelican::components::interface::navigation::{AppPage, NavigateEvent, PelicanError};
use pelican::components::list_item::{ListItem, ListItemGroup};
use pelican::components::{ExpandableText, Text, TextStyle};
use pelican::interactions::Button;
use roost::drawable::{Align, Drawable};
use roost::events::{Event, OnEvent};
use roost::layouts::Stack;
use roost::layouts::{Column, Offset};
use roost::layouts::{Padding, Size};
use roost::maverick_os::start;
use roost::{Application, Component, Context, Plugin};
use std::collections::BTreeMap;

use roost::{ServiceList, Services};

use crate::event_editor_screen::EventEditorScreen;
use crate::objects::EventForEES;

// Define the main application struct. This is our entry point type.
pub struct MyApp;

// Implement the Services trait for MyApp
impl Services for MyApp {
    // Provide a list of services used by the app. Here, it's empty.
    fn services() -> ServiceList {
        ServiceList(BTreeMap::new())
    }
}

//FIX: fn plugins needs reconciliation.
impl Plugin for MyApp {
    // Provide a list of plugins used by the app. Currently, there are none.
    fn plugins(_ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![]
    }
}

// Implement the Application trait for MyApp
impl Application for MyApp {
    // Asynchronously create the main drawable UI component
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        // Create the first screen
        let home = MonthScreen::new(ctx);
        // Create the main interface with the first screen as the starting page
        let interface = Interface::new(ctx, Box::new(home));
        // Return the interface wrapped in a Box
        Box::new(interface)
    }
}

// Macro to start the application
start!(MyApp);

// Define the first screen of the app
#[derive(Debug, Component)]
pub struct MonthScreen(Column, Page);

// Implement event handling for FirstScreen (empty for now)
impl OnEvent for MonthScreen {}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for MonthScreen {
    // This screen does not have a navigation bar
    fn has_navigator(&self) -> bool {
        false
    }

    fn navigate(
        self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, PelicanError> {
        match index {
            0 => Ok(Box::new(EventEditorScreen::new(ctx))),
            _ => Err(PelicanError::InvalidPage(Some(self))),
        }
    }
}

impl MonthScreen {
    pub fn new(ctx: &mut Context) -> Self {
        if ctx
            .state()
            .get_named::<EventForEES>("event_for_ees")
            .is_none()
        {
            let event_for_ees = EventForEES::new(None, None, None, None, None);
            ctx.state()
                .set_named(String::from("event_for_ees"), event_for_ees);
        }

        // Create a header for the page
        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx,        // The text on this header will say "Calendar"
            "Calendar", // There will not be an icon button on this header
            None,
        );

        let font_size = ctx.theme.fonts.size;

        // Create the main heading text
        let text = Text::new(
            ctx,
            // This text will say "Hello World!"
            "Hello World!",
            // The style of this text will be heading
            TextStyle::Heading,
            // The size will be h2
            font_size.h2,
            // The text alignment
            Align::Center,
        );

        // Create subtext.
        let subtext = ExpandableText::new(
            ctx,
            "First project loaded successfully.",
            // This text will have primary text style.
            TextStyle::Primary,
            // Medium font size
            font_size.md,
            // Center the text
            Align::Center,
            // No max lines
            None,
        );

        let button = Button::primary(ctx, "test", |ctx: &mut Context| {
            ctx.trigger_event(NavigateEvent(0));
            println!("Test button clicked.")
        });

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            // Vertically center items
            Offset::Center,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text), Box::new(subtext), Box::new(button)],
        );

        // Return the FirstScreen with a default Stack and a
        // new Page containinhg our header, content, and no bumper.
        MonthScreen(
            Column::new(1.0, Offset::Center, Size::Fit, Padding::new(1.0)),
            Page::new(Some(header), content, None),
        )
    }
}
