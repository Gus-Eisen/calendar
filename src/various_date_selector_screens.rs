use pelican_ui::drawable::{Component, Drawable};
use pelican_ui::events::OnEvent;
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Component, Context};

use crate::objects::EventForEES;
use pelican_ui::events::Event;
use pelican_ui_std::AppPage;
use pelican_ui_std::components::button::{
    Button, ButtonSize, ButtonState, ButtonStyle, IconButton,
};
use pelican_ui_std::components::interface::general::{Bumper, Content, Page};
use pelican_ui_std::components::list_item::ListItemSelector;
use pelican_ui_std::events::ListItemSelect;
use pelican_ui_std::events::NavigateEvent;
use pelican_ui_std::layout::{Offset, Stack};

use crate::event_editor_screen::EventEditorScreen;

pub mod year_selector_screen_block {
    use super::*;

    #[derive(Debug, Component)]
    pub struct YearSelectorScreen(Stack, Page, #[skip] String);

    impl OnEvent for YearSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
            if event.downcast_ref::<ListItemSelect>().is_some() {
                let index = self
                    .1
                    .content()
                    //TODO: ListItemSelector is broken within RAMP. Fix when updated.
                    .find::<ListItemSelector>()
                    .unwrap()
                    .index()
                    .unwrap();
                let event_for_ees = ctx
                    .state()
                    .get_named_mut::<EventForEES>("event_for_ees")
                    .unwrap();
                match index {
                    0 => {
                        event_for_ees.year = Some("2025".to_string());
                        println!("Year: 2025");
                    }
                    1 => {
                        event_for_ees.year = Some("2026".to_string());
                        println!("Year: 2026");
                    }
                    _ => (),
                }
            }
            true
        }
    }

    impl AppPage for YearSelectorScreen {
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
                0 => Ok(Box::new(EventEditorScreen::new(ctx))),
                _ => Err(self),
            }
        }
    }

    impl YearSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let return_to_eventeditorscreen_icon = IconButton::new(
                ctx,
                "backspace",
                ButtonSize::Medium,
                ButtonStyle::Secondary,
                ButtonState::Default,
                Box::new(|ctx: &mut Context| {
                    ctx.trigger_event(NavigateEvent(0));
                    println!("return_to_eventeditorscreen_icon clicked.")
                }),
                None,
            );
            let year =
                ListItemSelector::new(ctx, ("2025", "", None), ("2026", "", None), None, None);

            let content = Content::new(
                ctx,
                Offset::Start,
                vec![Box::new(return_to_eventeditorscreen_icon), Box::new(year)],
            );
            let button = Button::primary(ctx, "Save Year", |ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(0));
                println!("Save Year button clicked.")
            });
            let bumper = Bumper::single_button(ctx, button);

            YearSelectorScreen(
                Stack::default(),
                Page::new(None, content, Some(bumper)),
                String::default(),
            )
        }
    }
}

pub mod month_selector_screen_block {
    use crate::objects::MonthOfYear;

    use super::*;

    #[derive(Debug, Component)]
    pub struct MonthSelectorScreen(Stack, Page);

    impl OnEvent for MonthSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
            if event.downcast_ref::<ListItemSelect>().is_some() {
                let index = self
                    .1
                    .content()
                    //TODO: ListItemSelector is broken within RAMP. Fix when updated.
                    .find::<ListItemSelector>()
                    .unwrap()
                    .index()
                    .unwrap();
                let event_for_ees = ctx
                    .state()
                    .get_named_mut::<EventForEES>("event_for_ees")
                    .unwrap();
                match index {
                    0 => {
                        event_for_ees.month = Some(MonthOfYear::January.as_str().to_string());
                        println!("Month: January");
                    }
                    1 => {
                        event_for_ees.month = Some(MonthOfYear::February.as_str().to_string());
                        println!("Month: February");
                    }
                    2 => {
                        event_for_ees.month = Some(MonthOfYear::March.as_str().to_string());
                        println!("Month: March");
                    }
                    3 => {
                        event_for_ees.month = Some(MonthOfYear::April.as_str().to_string());
                        println!("Month: April");
                    }
                    4 => {
                        event_for_ees.month = Some(MonthOfYear::May.as_str().to_string());
                        println!("Month: May");
                    }
                    5 => {
                        event_for_ees.month = Some(MonthOfYear::June.as_str().to_string());
                        println!("Month: June");
                    }
                    6 => {
                        event_for_ees.month = Some(MonthOfYear::July.as_str().to_string());
                        println!("Month: July");
                    }
                    7 => {
                        event_for_ees.month = Some(MonthOfYear::August.as_str().to_string());
                        println!("Month: August");
                    }
                    8 => {
                        event_for_ees.month = Some(MonthOfYear::September.as_str().to_string());
                        println!("Month: September");
                    }
                    9 => {
                        event_for_ees.month = Some(MonthOfYear::October.as_str().to_string());
                        println!("Month: October");
                    }
                    10 => {
                        event_for_ees.month = Some(MonthOfYear::November.as_str().to_string());
                        println!("Month: November");
                    }
                    11 => {
                        event_for_ees.month = Some(MonthOfYear::December.as_str().to_string());
                        println!("Month: December");
                    }
                    _ => (),
                }
            }
            true
        }
    }
}
