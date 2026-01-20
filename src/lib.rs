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

#[derive(Debug, Component)]
pub struct MonthOfMyWeekdayRow(
    Column,
    MyWeekdayRow,
    MyWeekdayRow,
    MyWeekdayRow,
    MyWeekdayRow,
    Option<MyWeekdayRow>,
    Option<MyWeekdayRow>,
);

impl OnEvent for MonthOfMyWeekdayRow {}

#[derive(Debug, Component)]
pub struct MyWeekdayRow(
    Row,
    MyWeekday,
    MyWeekday,
    MyWeekday,
    MyWeekday,
    MyWeekday,
    MyWeekday,
    MyWeekday,
);
impl OnEvent for MyWeekdayRow {}

impl MyWeekdayRow {
    pub fn new(
        ctx: &mut Context,
        days: [Option<u32>; 7],
        today: u32,
        events: &HashSet<u32>,
        year: i32,
        month: u32,
    ) -> Self {
        MyWeekdayRow(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            MonthScreen::make_day_cell(
                ctx,
                days[0],
                today,
                days[0].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
            MonthScreen::make_day_cell(
                ctx,
                days[1],
                today,
                days[1].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
            MonthScreen::make_day_cell(
                ctx,
                days[2],
                today,
                days[2].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
            MonthScreen::make_day_cell(
                ctx,
                days[3],
                today,
                days[3].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
            MonthScreen::make_day_cell(
                ctx,
                days[4],
                today,
                days[4].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
            MonthScreen::make_day_cell(
                ctx,
                days[5],
                today,
                days[5].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
            MonthScreen::make_day_cell(
                ctx,
                days[6],
                today,
                days[6].is_some_and(|d| events.contains(&d)),
                year,
                month,
            ),
        )
    }
}

#[derive(Debug, Component)]
pub struct DayCellContent(Column, Text, Option<Shape>);
impl OnEvent for DayCellContent {}

#[derive(Debug, Component)]
pub struct MyWeekday(
    Stack,
    Rectangle,
    DayCellContent,
    #[skip] Option<(i32, u32, u32)>,
);
impl OnEvent for MyWeekday {
    fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        // handles logic for User clicking on day cell.
        if let Some(MouseEvent {
            state: MouseState::Pressed,
            position: Some(_),
        }) = event.downcast_ref::<MouseEvent>()
            && let Some((year, month, day)) = self.3
        {
            println!("Day {}-{}-{} clicked.", year, month, day);
            let page = Box::new(DayViewScreen::new(ctx, year, month, day).unwrap());
            ctx.trigger_event(NavigationEvent::Push(Some(page)));
        }
        vec![event]
    }
}

impl MyWeekday {
    pub fn new(
        ctx: &mut Context,
        label: &str,
        border: Option<(f32, Color)>,
        has_event: bool,
        is_today: bool,
        year: i32,
        month: u32,
        day: Option<u32>,
    ) -> Self {
        let background_color = if is_today {
            Color::from_hex("#b3e0f2", 235)
        } else {
            Color(255, 255, 255, 255)
        };
        let rect = Rectangle::new(background_color, 8.0, border);
        let text = Text::new(
            ctx,
            label,
            TextSize::Md,
            TextStyle::Primary,
            Align::Center,
            None,
        );
        let dot = if has_event {
            Some(Circle::new(6.0, Color::from_hex("#BF360C", 255), false))
        } else {
            None
        };
        let content = DayCellContent(
            Column::new(2.0, Offset::Center, Size::Fit, Padding::default()),
            text,
            dot,
        );
        let layout = Stack(
            Offset::Center,
            Offset::Center,
            Size::Static(40.0),
            Size::Static(40.0),
            Padding::default(),
        );
        let date = day.map(|d| (year, month, d));
        Self(layout, rect, content, date)
    }
}

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

        let now = Local::now();
        let current_month = now.format("%B").to_string();
        let current_year = now.year().to_string();
        let month_and_year = format!("{current_month} {current_year}");

        let header = Header::home(ctx, "Calendar", None);

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
        let month_grid = Self::monthofweekdayrow_builder(ctx);

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![
                Box::new(weekday_row),
                Box::new(month_grid),
                Box::new(new_event_button),
            ],
        );

        Ok(Self(
            Row::new(1.0, Offset::Start, Size::Fit, Padding::new(1.0)),
            Page::new(header, content, None),
        ))
    }

    pub fn weekday_row_builder(ctx: &mut Context) -> MyWeekdayRow {
        MyWeekdayRow(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            MyWeekday::new(ctx, "Mon", None, false, false, 0, 0, None),
            MyWeekday::new(ctx, "Tue", None, false, false, 0, 0, None),
            MyWeekday::new(ctx, "Wed", None, false, false, 0, 0, None),
            MyWeekday::new(ctx, "Thu", None, false, false, 0, 0, None),
            MyWeekday::new(ctx, "Fri", None, false, false, 0, 0, None),
            MyWeekday::new(ctx, "Sat", None, false, false, 0, 0, None),
            MyWeekday::new(ctx, "Sun", None, false, false, 0, 0, None),
        )
    }

    pub fn monthofweekdayrow_builder(ctx: &mut Context) -> MonthOfMyWeekdayRow {
        let now = Local::now();
        let today = now.day();
        let current_month = now.month();
        let first_of_month_as_weekday = now.with_day(1).unwrap().weekday();

        let num_of_days_in_month = match current_month {
            1 => 31,
            2 => {
                if Self::is_leap_year() {
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
            _ => panic!("Invalid month number."),
        };

        let num_of_rows =
            Self::num_of_row_determiner(num_of_days_in_month, first_of_month_as_weekday);

        let offset = match first_of_month_as_weekday {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };

        let mut day = 1u32;
        let num_days = num_of_days_in_month as u32;

        let current_year = now.year();
        let events = ctx
            .state()
            .get::<EventRegistry>()
            .map(|registry| registry.days_with_events(current_year, current_month))
            .unwrap_or_default();

        let row1 = Self::build_week_row(
            ctx,
            &mut day,
            num_days,
            today,
            0,
            offset,
            &events,
            current_year,
            current_month,
        );
        let row2 = Self::build_week_row(
            ctx,
            &mut day,
            num_days,
            today,
            1,
            offset,
            &events,
            current_year,
            current_month,
        );
        let row3 = Self::build_week_row(
            ctx,
            &mut day,
            num_days,
            today,
            2,
            offset,
            &events,
            current_year,
            current_month,
        );
        let row4 = Self::build_week_row(
            ctx,
            &mut day,
            num_days,
            today,
            3,
            offset,
            &events,
            current_year,
            current_month,
        );
        let row5 = if num_of_rows >= 5 {
            Some(Self::build_week_row(
                ctx,
                &mut day,
                num_days,
                today,
                4,
                offset,
                &events,
                current_year,
                current_month,
            ))
        } else {
            None
        };
        let row6 = if num_of_rows >= 6 {
            Some(Self::build_week_row(
                ctx,
                &mut day,
                num_days,
                today,
                5,
                offset,
                &events,
                current_year,
                current_month,
            ))
        } else {
            None
        };

        MonthOfMyWeekdayRow(
            Column::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            row1,
            row2,
            row3,
            row4,
            row5,
            row6,
        )
    }

    fn num_of_row_determiner(num_of_days_in_month: i32, first_of_month_as_weekday: Weekday) -> i32 {
        let offset = match first_of_month_as_weekday {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };

        (offset + num_of_days_in_month + 6) / 7
    }

    fn is_leap_year() -> bool {
        let current_year = Local::now().year();
        (current_year % 4 == 0) && (current_year % 100 != 0 || current_year % 400 == 0)
    }

    fn make_day_cell(
        ctx: &mut Context,
        day_opt: Option<u32>,
        today: u32,
        has_event: bool,
        year: i32,
        month: u32,
    ) -> MyWeekday {
        match day_opt {
            Some(day) => {
                let is_today = day == today;
                let border = Some((1.0, Color::BLACK));
                MyWeekday::new(
                    ctx,
                    &day.to_string(),
                    border,
                    has_event,
                    is_today,
                    year,
                    month,
                    Some(day),
                )
            }
            None => MyWeekday::new(
                ctx,
                "",
                Some((1.0, Color::BLACK)),
                false,
                false,
                year,
                month,
                None,
            ),
        }
    }

    fn build_week_row(
        ctx: &mut Context,
        day: &mut u32,
        num_days: u32,
        today: u32,
        row_idx: i32,
        offset: usize,
        events: &HashSet<u32>,
        year: i32,
        month: u32,
    ) -> MyWeekdayRow {
        let mut days: [Option<u32>; 7] = [None; 7];
        for (col, day_slot) in days.iter_mut().enumerate() {
            if row_idx == 0 && col < offset {
                *day_slot = None;
            } else if *day <= num_days {
                *day_slot = Some(*day);
                *day += 1;
            } else {
                *day_slot = None;
            }
        }
        MyWeekdayRow::new(ctx, days, today, events, year, month)
    }
}
