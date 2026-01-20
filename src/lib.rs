mod day_view_screen;
mod event_editor_screen;
mod objects;
mod various_date_selector_screens;

use std::collections::HashSet;

use chrono::{Datelike, Local, Weekday};
use pelican_ui::components::button::PrimaryButton;
use pelican_ui::components::interface::general::{Content, Header, Interface, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent, RootInfo};
use pelican_ui::components::{Circle, Rectangle, Text, TextSize, TextStyle};
use pelican_ui::drawable::{Align, Color, Shape};
use pelican_ui::events::{Event, MouseEvent, MouseState, OnEvent};
use pelican_ui::layouts::{Column, Offset, Row, Stack};
use pelican_ui::layouts::{Padding, Size};
use pelican_ui::start;
use pelican_ui::theme::Theme;
use pelican_ui::{Application, Assets, Component, Context};

use crate::day_view_screen::DayViewScreen;
use crate::event_editor_screen::EventEditorScreen;
use crate::objects::{
    CustomHeaderForMonthScreen, CustomPageForMonthScreen, EventForEES, EventRegistry,
};

// Define the main application struct. This is our entry point type.
pub struct Calendar;

impl Application for Calendar {
    fn interface(ctx: &mut Context) -> Interface {
        let home = RootInfo::icon("home", "My Calendar", MonthScreen::new(ctx).ok().unwrap());

        Interface::new(ctx, vec![home])
    }

    fn theme(assets: &mut Assets) -> Theme {
        Theme::light(assets, Color::from_hex("#BF360C", 255))
    }
}

// Macro to start the application
start!(Calendar);

// Define the first screen of the app
#[derive(Debug, Component)]
pub struct MonthScreen(Column, Page);

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

        let header = Header::home(ctx, "Calendar", None);

        // let now = Local::now();
        // let current_month = now.format("%B").to_string();
        // let current_year = now.year().to_string();
        // let month_and_year = format!("{current_month} {current_year}");

        let placeholder = Text::new(
            ctx,
            "placeholder",
            TextSize::H2,
            TextStyle::Secondary,
            Align::Left,
            None,
        );

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(placeholder)],
        );

        Ok(Self(
            Column::new(1.0, Offset::Start, Size::Fit, Padding(1.0, 1.0, 1.0, 1.0)),
            Page::new(header, content, None),
        ))
    }

    fn is_leap_year() -> bool {
        let current_year = Local::now().year();
        (current_year % 4 == 0) && (current_year % 100 != 0 || current_year % 400 == 0)
    }
}
