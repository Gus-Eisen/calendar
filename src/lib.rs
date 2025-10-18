mod day_view_screen;
mod event_editor_screen;
mod objects;

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

// Define the main application struct. This is our entry point type.
pub struct MyApp;

// Implement the Services trait for MyApp
impl Services for MyApp {
    // Provide a list of services used by the app. Here, it's empty.
    fn services() -> ServiceList {
        ServiceList(BTreeMap::new())
    }
}

// Implement the Plugins trait for MyApp
impl Plugins for MyApp {
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
        let interface = Interface::new(ctx, Box::new(home), None, None);
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
            0 => return Ok(Box::new(EventEditorScreen::new(ctx))),
            _ => Err(self),
        }
    }
}

impl MonthScreen {
    pub fn new(ctx: &mut Context) -> Self {
        // Create a header for the page
        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx,        // The text on this header will say "Calendar"
            "Calendar", // There will not be an icon button on this header
            None,
        );

        let font_size = ctx.theme.fonts.size;
        let color = ctx.theme.colors.text.heading;

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
