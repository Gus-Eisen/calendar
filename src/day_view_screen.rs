use chrono::{DateTime, Local, Timelike, Utc};
use pelican_ui::components::Text;
use pelican_ui::components::button::PrimaryButton;
use pelican_ui::components::interface::general::{Bumper, Content, Header, Page};
use pelican_ui::components::interface::navigation::{AppPage, NavigationEvent};
use pelican_ui::drawable::Drawable;
use pelican_ui::events::OnEvent;
use pelican_ui::layouts::{Column, Offset};
use pelican_ui::layouts::{Padding, Size};
use pelican_ui::{Component, Context};

use crate::event_editor_screen::EventEditorScreen;
use crate::objects::EventRegistry;

#[derive(Debug, Component)]
pub struct DayViewScreen(Column, Page);

impl OnEvent for DayViewScreen {}

impl AppPage for DayViewScreen {}

impl DayViewScreen {
    pub fn new(ctx: &mut Context, year: i32, month: u32, day: u32) -> Result<Self, String> {
        // Query EventRegistry for events on this day (clone data to defeat borrow checker)
        let events: Vec<(String, DateTime<Utc>)> = ctx
            .state()
            .get::<EventRegistry>()
            .map(|registry| {
                registry
                    .events_for_day(year, month, day)
                    .iter()
                    .map(|e| (e.title().to_string(), e.datetime()))
                    .collect()
            })
            .unwrap_or_default();

        // Debug: print events found
        for (title, datetime) in &events {
            println!("Event: '{}' on {}", title, datetime);
        }

        let month_for_header = Self::month_name(month).unwrap();
        let title = format!("{} {}, {}", month_for_header, day, year);
        println!("DEBUG HEADER: '{}'", title);
        let header = Header::stack(ctx, &title);
        let mut content_items: Vec<Box<dyn Drawable>> = Self::vec_of_text(ctx, events)
            .into_iter()
            .map(|t| Box::new(t) as Box<dyn Drawable>)
            .collect();

        let bumper = Bumper::stack(ctx, Some("Create New Event"), false, |ctx: &mut Context| {
            // TODO: pass in Year, Month and Day to EES::new().
            let page = Box::new(EventEditorScreen::new(ctx));
            ctx.trigger_event(NavigationEvent::Push(Some(page)));
        });

        // Combine icon, heading, and subtext into page content
        let content = Content::new(ctx, Offset::Start, content_items);

        Ok(Self(
            Column::new(1.0, Offset::Start, Size::Fit, Padding::new(1.0)),
            Page::new(header, content, Some(bumper)),
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

    pub fn vec_of_text(ctx: &mut Context, events: Vec<(String, DateTime<Utc>)>) -> Vec<Text> {
        let mut texts = Vec::new();
        for (title, datetime) in events {
            let local_datetime = datetime.with_timezone(&Local);
            let label = format!("{} - {}", local_datetime.format("%I:%M %p"), title);
            let text = Text::new(
                ctx,
                &label,
                pelican_ui::components::TextSize::Md,
                pelican_ui::components::TextStyle::Primary,
                pelican_ui::drawable::Align::Left,
                None,
            );
            texts.push(text);
        }
        texts
    }
}
