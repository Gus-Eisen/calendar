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
                    //FIX: ListItemSelector is broken within RAMP. Fix when updated.
                    .find::<ListItemSelector>()
                    .unwrap()
                    .index()
                    .unwrap();
                let event_for_ees = ctx
                    .state()
                    .get_named_mut::<EventForEES>("event_for_ees")
                    .unwrap();
                event_for_ees.set_year(index);
            }
            true
        }
    }

    impl AppPage for YearSelectorScreen {
        // This screen does not have a navigation bar
        fn has_nav(&self) -> bool {
            false
        }

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
    use pelican_ui_std::{
        ElementID,
        components::list_item::{ListItem, ListItemGroup},
    };

    use crate::objects::Month;

    use super::*;

    #[derive(Debug, Component)]
    pub struct MonthSelectorScreen(Stack, Page);

    impl OnEvent for MonthSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
            if event.downcast_ref::<ListItemSelect>().is_some() {
                let index = self
                    .1
                    .content()
                    //FIX: ListItemSelector is broken within RAMP. Fix when updated.
                    .find::<ListItemSelector>()
                    .unwrap()
                    .index()
                    .unwrap();
                let event_for_ees = ctx
                    .state()
                    .get_named_mut::<EventForEES>("event_for_ees")
                    .unwrap();
                event_for_ees.set_month(index);
            }
            true
        }
    }

    impl AppPage for MonthSelectorScreen {
        // This screen does not have a navigation bar
        fn has_nav(&self) -> bool {
            false
        }

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

    impl MonthSelectorScreen {
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
            // let month_jan_to_apr = ListItemSelector::new(
            //     ctx,
            //     ("January", "", None),
            //     ("February", "", None),
            //     Some(("March", "", None)),
            //     Some(("April", "", None)),
            // );

            let vec_month_listitem = vec![
                ListItem::new(
                    ctx,
                    false,
                    "January",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("January ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "February",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("February ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "March",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("March ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "April",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("April ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "May",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("May ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "June",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("June ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "July",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("July ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "August",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("August ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "September",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("September ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "October",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("October ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "November",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("November ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "December",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("December ListItem clicked."),
                ),
            ];
            let month_listitemgroup = ListItemGroup::new(vec_month_listitem);
            let content = Content::new(
                ctx,
                Offset::Start,
                vec![
                    Box::new(return_to_eventeditorscreen_icon),
                    Box::new(month_listitemgroup),
                ],
            );
            let button = Button::primary(ctx, "Save Year", |ctx: &mut Context| {
                ctx.trigger_event(NavigateEvent(0));
                println!("Save Year button clicked.")
            });
            let bumper = Bumper::single_button(ctx, button);

            MonthSelectorScreen(Stack::default(), Page::new(None, content, Some(bumper)))
        }

        pub fn vec_month_listitem_builder(ctx: &mut Context) -> Vec<ListItem> {
            vec![
                ListItem::new(
                    ctx,
                    false,
                    "January",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("January ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "February",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("February ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "March",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("March ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "April",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("April ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "May",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("May ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "June",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("June ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "July",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("July ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "August",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("August ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "September",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("September ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "October",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("October ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "November",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("November ListItem clicked."),
                ),
                ListItem::new(
                    ctx,
                    false,
                    "December",
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(false),
                    None,
                    Some(ElementID::new()),
                    true,
                    |ctx: &mut Context| println!("December ListItem clicked."),
                ),
            ]
        }
    }
}
