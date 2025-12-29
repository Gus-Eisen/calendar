mod day_view_screen;
mod event_editor_screen;
mod objects;
mod various_date_selector_screens;

use chrono::{Datelike, Local, Weekday};
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
pub struct MonthOfMyWeekdayRow(
    Row,
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
    pub fn new(ctx: &mut Context, days: [Option<u32>; 7], today: u32) -> Self {
        MyWeekdayRow(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            MonthScreen::make_day_cell(ctx, days[0], today),
            MonthScreen::make_day_cell(ctx, days[1], today),
            MonthScreen::make_day_cell(ctx, days[2], today),
            MonthScreen::make_day_cell(ctx, days[3], today),
            MonthScreen::make_day_cell(ctx, days[4], today),
            MonthScreen::make_day_cell(ctx, days[5], today),
            MonthScreen::make_day_cell(ctx, days[6], today),
        )
    }
}

#[derive(Debug, Component)]
pub struct MyWeekday(Stack, Rectangle, Text);
impl OnEvent for MyWeekday {}

impl MyWeekday {
    pub fn new(ctx: &mut Context, label: &str, border: Option<(f32, Color)>) -> Self {
        let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, border);
        let text = Text::new(
            ctx,
            label,
            TextSize::Md,
            TextStyle::Primary,
            Align::Center,
            None,
        );
        let layout = Stack(
            Offset::Center,
            Offset::Center,
            Size::Fit,
            Size::Fit,
            Padding::default(),
        );
        Self(layout, rect, text)
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

    pub fn weekday_row_builder(ctx: &mut Context) -> MyWeekdayRow {
        let mon = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        let tue = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        let wed = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        let thu = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        let fri = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        let sat = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        let sun = {
            let rect = Rectangle::new(Color(0, 0, 0, 1), 8.0, None);
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
                Size::Fit,
                Size::Fit,
                Padding::default(),
            );
            MyWeekday(layout, rect, label)
        };

        MyWeekdayRow(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            mon,
            tue,
            wed,
            thu,
            fri,
            sat,
            sun,
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
                if Self::leap_year_determiner() {
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

        let row1 = Self::build_week_row(ctx, &mut day, num_days, today, 0, offset);
        let row2 = Self::build_week_row(ctx, &mut day, num_days, today, 1, offset);
        let row3 = Self::build_week_row(ctx, &mut day, num_days, today, 2, offset);
        let row4 = Self::build_week_row(ctx, &mut day, num_days, today, 3, offset);
        let row5 = if num_of_rows >= 5 {
            Some(Self::build_week_row(
                ctx, &mut day, num_days, today, 4, offset,
            ))
        } else {
            None
        };
        let row6 = if num_of_rows >= 6 {
            Some(Self::build_week_row(
                ctx, &mut day, num_days, today, 5, offset,
            ))
        } else {
            None
        };

        MonthOfMyWeekdayRow(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
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

    fn leap_year_determiner() -> bool {
        let current_year = Local::now().year();
        (current_year % 4 == 0) && (current_year % 100 != 0 || current_year % 400 == 0)
    }

    fn make_day_cell(ctx: &mut Context, day_opt: Option<u32>, today: u32) -> MyWeekday {
        match day_opt {
            Some(day) => {
                let is_today = day == today;
                let border = if is_today {
                    Some((2.0, Color::from_hex("#ff1f84ff", 255)))
                } else {
                    Some((1.0, Color::BLACK))
                };
                MyWeekday::new(ctx, &day.to_string(), border)
            }
            None => MyWeekday::new(ctx, "", Some((1.0, Color::BLACK))),
        }
    }

    fn build_week_row(
        ctx: &mut Context,
        day: &mut u32,
        num_days: u32,
        today: u32,
        row_idx: i32,
        offset: usize,
    ) -> MyWeekdayRow {
        let mut days: [Option<u32>; 7] = [None; 7];
        for col in 0..7 {
            if row_idx == 0 && col < offset {
                days[col] = None;
            } else if *day <= num_days {
                days[col] = Some(*day);
                *day += 1;
            } else {
                days[col] = None;
            }
        }
        MyWeekdayRow::new(ctx, days, today)
    }
}
