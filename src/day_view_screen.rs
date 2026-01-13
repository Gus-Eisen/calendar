use pelican_ui::components::button::PrimaryButton;
use pelican_ui::components::interface::general::{Content, Header, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent};
use pelican_ui::events::OnEvent;
use pelican_ui::layouts::{Column, Offset};
use pelican_ui::layouts::{Padding, Size};
use pelican_ui::{Component, Context};

use crate::objects::EventRegistry;

#[derive(Debug, Component)]
pub struct DayViewScreen(Column, Page);

impl OnEvent for DayViewScreen {}

impl AppPage for DayViewScreen {}

impl DayViewScreen {
    pub fn new(ctx: &mut Context, year: i32, month: u32, day: u32) -> Result<Self, String> {
        // Query EventRegistry for events on this day
        let events = ctx
            .state()
            .get::<EventRegistry>()
            .map(|registry| registry.events_for_day(year, month, day))
            .unwrap_or_default();

        // Debug: print events found
        for event in &events {
            println!("Event: '{}' on {}", event.title(), event.datetime());
        }

        let month_for_header = Self::month_name(month).unwrap();
        let title = format!("{} {}, {}", month_for_header, day, year);
        println!("DEBUG HEADER: '{}'", title);
        let header = Header::home(ctx, &title, None);

        let ret_to_cal_button = PrimaryButton::new(
            ctx,
            "Return to Calendar",
            |ctx: &mut Context| {
                ctx.trigger_event(NavigationEvent::Pop);
                println!("Return to Calendar button clicked.")
            },
            false,
        );

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(ret_to_cal_button)],
        );

        Ok(Self(
            Column::new(1.0, Offset::Start, Size::Fit, Padding::new(1.0)),
            Page::new(header, content, None),
        ))
    }

    pub fn month_name(month: u32) -> Option<&'static str> {
        const MONTHS: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        MONTHS.get((month - 1) as usize).copied()
    }
}
