use crate::event_editor_screen::EventEditorScreen;
use crate::objects::EventForEES;
use pelican_ui::Context;
use pelican_ui::components::RadioSelector;
use pelican_ui::components::button::{ButtonSize, ButtonStyle, GhostIconButton};
use pelican_ui::components::interface::general::{Bumper, Content, Page};
use pelican_ui::components::interface::navigation::AppPage;
use pelican_ui::components::list_item::{ListItem, ListItemGroup};
use pelican_ui::drawable;
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::interactions::Button;
use pelican_ui::layout::Layout;
use pelican_ui::layouts::Offset;
use pelican_ui::layouts::Stack;
use pelican_ui::{Application, Component, Plugin, drawables, include_dir};

pub mod year_selector_screen_block {

    use pelican_ui::components::interface::navigation::NavigationEvent;

    use super::*;

    #[derive(Debug, Component)]
    pub struct YearSelectorScreen(Stack, Page, #[skip] String);

    impl OnEvent for YearSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
            //FIX: Figure out event type emitter for RadioSelector.
            if event.downcast_ref::<ListItem>().is_some() {
                let index = self
                    .1
                    .content()
                    .find::<RadioSelector>()
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
        // fn has_navigator(&self) -> bool {
        //     true
        // }
        //
        // fn navigate(
        //     self: Box<Self>,
        //     ctx: &mut Context,
        //     index: usize,
        // ) -> Result<Box<dyn AppPage>, PelicanError> {
        //     match index {
        //         0 => Ok(Box::new(EventEditorScreen::new(ctx))),
        //         _ => Err(PelicanError::InvalidPage(Some(self))),
        //     }
        // }
    }

    impl YearSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            let return_to_eventeditorscreen_icon = GhostIconButton::new(
                ctx,
                "backspace",
                Box::new(|ctx: &mut Context| {
                    ctx.trigger_event(NavigationEvent::Pop);
                    println!("return_to_eventeditorscreen_icon clicked.")
                }),
            );
            let year =
                ListItemSelector::new(ctx, ("2025", "", None), ("2026", "", None), None, None);

            let content = Content::new(
                ctx,
                Offset::Start,
                vec![Box::new(return_to_eventeditorscreen_icon), Box::new(year)],
            );
            // let button = Button::primary(ctx, "Save Year", |ctx: &mut Context| {
            //     ctx.trigger_event(NavigationEvent::Pop);
            //     println!("Save Year button clicked.")
            // });
            // let bumper = Bumper::single_button(ctx, button);
            let bumper = Bumper::stack(ctx, Some("Save Year"), false, |ctx: &mut Context| {
                ctx.trigger_event(NavigationEvent::Pop);
                println!("Save Year bumper clicked.")
            });

            YearSelectorScreen(
                Stack::default(),
                Page::new(None, content, Some(bumper)),
                String::default(),
            )
        }
    }
}

pub mod month_selector_screen_block {

    use pelican_ui::components::{button::PrimaryButton, interface::navigation::NavigationEvent};

    use crate::objects::Month;

    use super::*;

    #[derive(Debug, Component)]
    pub struct MonthSelectorScreen(Stack, Page);

    impl OnEvent for MonthSelectorScreen {
        fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
            // if event.downcast_ref::<ListItemSelect>().is_some() {
            //     let index = self
            //         .1
            //         .content()
            //         .find::<ListItemGroup>()
            //         .unwrap()
            //         //TODO: figure out how to correlate ListItem.
            //         .inner()
            //         .iter();
            //
            //     let event_for_ees = ctx
            //         .state()
            //         .get_named_mut::<EventForEES>("event_for_ees")
            //         .unwrap();
            //     event_for_ees.set_month(index);
            // }
            true
        }
    }

    impl AppPage for MonthSelectorScreen {
        // // This screen does not have a navigation bar
        // fn has_navigator(&self) -> bool {
        //     false
        // }
        //
        // fn navigate(
        //     self: Box<Self>,
        //     ctx: &mut Context,
        //     index: usize,
        // ) -> Result<Box<dyn AppPage>, PelicanError> {
        //     match index {
        //         0 => Ok(Box::new(EventEditorScreen::new(ctx))),
        //         _ => Err(PelicanError::InvalidPage(Some(self))),
        //     }
        // }
    }

    impl MonthSelectorScreen {
        pub fn new(ctx: &mut Context) -> Self {
            //FIX: IconButton needs reconciliation.
            let return_to_eventeditorscreen_icon = GhostIconButton::new(
                ctx,
                "backspace",
                Box::new(|ctx: &mut Context| {
                    ctx.trigger_event(NavigationEvent::Pop);
                    println!("return_to_eventeditorscreen_icon clicked.")
                }),
            );
            let vec_month_listitem = Self::vec_month_listitem_builder(ctx);

            let month_listitemgroup = ListItemGroup::new(vec_month_listitem);
            let content = Content::new(
                ctx,
                Offset::Start,
                vec![
                    Box::new(return_to_eventeditorscreen_icon),
                    Box::new(month_listitemgroup),
                ],
            );
            let button = PrimaryButton::new(
                ctx,
                "Save Year",
                |ctx: &mut Context| {
                    ctx.trigger_event(NavigationEvent::Pop);
                    println!("Save Year button clicked.")
                },
                false,
            );
            let bumper = Bumper::single_button(ctx, button);

            MonthSelectorScreen(Stack::default(), Page::new(None, content, Some(bumper)))
        }

        pub fn vec_month_listitem_builder(ctx: &mut Context) -> Vec<ListItem> {
            vec![
                //FIX: ListItem needs reconciliation.
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
