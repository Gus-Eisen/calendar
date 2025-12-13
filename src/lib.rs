mod day_view_screen;
mod event_editor_screen;
mod objects;
mod various_date_selector_screens;

use pelican_ui::components::button::PrimaryButton;
use pelican_ui::components::interface::general::{Content, Header, Interface, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent, RootInfo};
use pelican_ui::components::{ExpandableText, Rectangle, Text, TextSize, TextStyle};
use pelican_ui::drawable::{Align, Color};
use pelican_ui::events::OnEvent;
use pelican_ui::layouts::{Bin, Column, Offset, Row, Stack};
use pelican_ui::layouts::{Padding, Size};
use pelican_ui::start;
use pelican_ui::theme::Theme;
use pelican_ui::{Application, Assets, Component, Context};

use crate::event_editor_screen::EventEditorScreen;
use crate::objects::{EventForEES, EventForER, EventRegistry};

// Define the main application struct. This is our entry point type.
pub struct Calendar;

impl Application for Calendar {
    fn interface(ctx: &mut Context) -> Interface {
        ////TODO: initialize all state objects in interface().
        // ctx.state().set(AllOrders::default());

        let home = RootInfo::icon("home", "My Calendar", MonthScreen::new(ctx).ok().unwrap());

        Interface::new(ctx, vec![home])
    }

    fn theme(assets: &mut Assets) -> Theme {
        Theme::light(assets, Color::from_hex("#ff1f84ff", 255))
    }
}

// Macro to start the application
start!(Calendar);

#[derive(Debug, Component)]
pub struct WeekdayRow(
    Row,
    Weekday,
    Weekday,
    Weekday,
    Weekday,
    Weekday,
    Weekday,
    Weekday,
);
impl OnEvent for WeekdayRow {}

#[derive(Debug, Component)]
pub struct Weekday(Stack, Rectangle, Text);
impl OnEvent for Weekday {}

// Define the first screen of the app
#[derive(Debug, Component)]
pub struct MonthScreen(Row, Page);

impl OnEvent for MonthScreen {}

impl AppPage for MonthScreen {}

impl MonthScreen {
    pub fn new(ctx: &mut Context) -> Result<Self, String> {
        if ctx.state().get::<EventForEES>().is_none() {
            let event_for_ees = EventForEES::new(None, None, None, None, None);
            ctx.state().set(event_for_ees);
        }

        if ctx.state().get::<EventRegistry>().is_none() {
            let event_registry = EventRegistry::new(vec![None]);
            ctx.state().set(event_registry);
        }

        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx, // The text on this header will say "Calendar"
            "Calendar", None, // There will not be an icon button on this header
        );

        let new_event_button = PrimaryButton::new(
            ctx,
            "Create New Event",
            |ctx: &mut Context| {
                let page = Box::new(EventEditorScreen::new(ctx));
                ctx.trigger_event(NavigationEvent::Push(Some(page)));
                println!("Create New Event button clicked.")
            },
            false,
        );

        let weekday_row = Self::weekday_row_builder(ctx);

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(weekday_row), Box::new(new_event_button)],
        );

        Ok(Self(
            Row::new(1.0, Offset::Start, Size::Fit, Padding::new(1.0)),
            Page::new(header, content, None),
        ))
    }

    pub fn weekday_row_builder(ctx: &mut Context) -> WeekdayRow {
        let mon = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Mon",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        let tue = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Tue",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        let wed = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Wed",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        let thu = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Thu",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        let fri = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Fri",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        let sat = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Sat",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        let sun = {
            let rect = Rectangle::new(
                Color(255, 255, 255, 1),
                8.0,
                Some((2.0, Color(0, 0, 0, 255))),
            );
            let label = Text::new(
                ctx,
                "Sun",
                TextSize::Md,
                TextStyle::Primary,
                Align::Center,
                None,
            );
            let layout = Stack(
                Offset::Center,
                Offset::Center,
                Size::Static(100.0),
                Size::Static(100.0),
                Padding::default(),
            );
            Weekday(layout, rect, label)
        };

        WeekdayRow(
            Row::new(12.0, Offset::Start, Size::Fit, Padding::default()),
            mon,
            tue,
            wed,
            thu,
            fri,
            sat,
            sun,
        )
    }
}
