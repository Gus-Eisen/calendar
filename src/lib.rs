mod day_view_screen;
mod event_editor_screen;
mod objects;
mod various_date_selector_screens;

use chrono::{Datelike, Local};
use pelican_ui::components::interface::general::{Content, Header, Interface, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent, RootInfo};
use pelican_ui::components::list_item::{ListItem, ListItemGroup, ListItemInfoLeft};
use pelican_ui::drawable::Color;
use pelican_ui::events::OnEvent;
use pelican_ui::layouts::{Column, Offset};
use pelican_ui::layouts::{Padding, Size};
use pelican_ui::start;
use pelican_ui::theme::Theme;
use pelican_ui::utils::TitleSubtitle;
use pelican_ui::{Application, Assets, Component, Context};

use crate::day_view_screen::DayViewScreen;
use crate::objects::{EventForEES, EventForER, EventRegistry};

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

        let event_registry: EventRegistry = ctx.state().get::<EventRegistry>().unwrap().clone();

        let now = Local::now();
        // let current_month = now.format("%B").to_string();
        // let current_year = now.year().to_string();
        // let month_and_year = format!("{current_month} {current_year}");

        let header = Header::home(ctx, "Calendar", None);

        let listitemgroup = ListItemGroup::new(Self::listitem_builder(ctx, now, event_registry));

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(listitemgroup)],
        );

        Ok(Self(
            Column::new(1.0, Offset::Start, Size::Fit, Padding::new(1.0)),
            Page::new(header, content, None),
        ))
    }

    // iterate over a month range (1-31), create a ListItem, then collect into a vec.
    fn listitem_builder(
        ctx: &mut Context,
        now: chrono::DateTime<chrono::Local>,
        event_registry: EventRegistry,
    ) -> Vec<ListItem> {
        // create range for use in vec_of_listitem.
        let range = Self::num_of_days_in_month(now);

        let vec_of_listitem: Vec<ListItem> = (1..=range)
            .map(|d| {
                let day_of_week = now.with_day(d as u32).unwrap().weekday();
                let day_of_month = d as u32;
                let month = now.month();
                let year = now.year();
                let events_with_days = event_registry.days_with_events(year, month);
                let day_events: Option<&Vec<&EventForER>> =
                    if events_with_days.contains(&(d as u32)) {
                        Some(&event_registry.events_for_day(year, month, d as u32))
                    } else {
                        None
                    };
                ListItem::new(
                    ctx,
                    None,
                    ListItemInfoLeft::new(&d.to_string(), &day_of_week.to_string(), None, None),
                    day_events
                        .and_then(|events| events.first())
                        .map(|e| TitleSubtitle::new(e.title(), "")),
                    None,
                    None,
                    move |ctx: &mut Context| {
                        let page =
                            Box::new(DayViewScreen::new(ctx, year, month, day_of_month).unwrap());
                        ctx.trigger_event(NavigationEvent::Push(Some(page)));
                    },
                )
            })
            .collect();

        vec_of_listitem
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn num_of_days_in_month(now: chrono::DateTime<chrono::Local>) -> i32 {
        match now.month() {
            1 => 31,
            2 => {
                if Self::is_leap_year(now.year()) {
                    29
                } else {
                    28
                }
            }
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => panic!("Something went wrong with num_of_days_in_month()."),
        }
    }

    //     fn get_event_registry(ctx: &mut Context)
}
