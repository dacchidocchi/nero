use nero_extensions::types::Episode;
use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderColor,
    flexbox_grid::{AlignItems, JustifyContent},
    layout::{Display, Position, TopRightBottomLeft, ZIndex},
    sizing::Width,
    tw,
    typography::{FontSize, FontWeight},
};
use sycamore::{
    prelude::ReadSignal,
    web::{
        tags::{div, h2, header, hr, li, p, section, ul},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::utils::ViewBuilder;

use super::{Button, Icon, IconType};

pub struct ListHeader {
    label: &'static str,
    end_slot: Option<View>,
    sticky: bool,
}

impl ListHeader {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            end_slot: None,
            sticky: true,
        }
    }

    pub fn end_slot(mut self, end_slot: impl Into<View>) -> Self {
        self.end_slot = Some(end_slot.into());
        self
    }

    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }
}

impl From<ListHeader> for View {
    fn from(list_header: ListHeader) -> Self {
        header()
            .when(list_header.sticky, |this| {
                this.class(tw!(
                    Position::Sticky,
                    TopRightBottomLeft::TopNumber("0"),
                    ZIndex::Number("10"),
                    BackgroundColor::White
                ))
            })
            .children(
                div()
                    .class(tw!(
                        Display::Flex,
                        Width::WFull,
                        AlignItems::Center,
                        JustifyContent::Between
                    ))
                    .children(
                        h2().class(tw!(FontSize::_2Xl, FontWeight::Semibold))
                            .children(list_header.label),
                    )
                    .when_some(list_header.end_slot, |this, slot| this.children(slot)),
            )
            .children(hr().class(tw!(BorderColor::BorderGray300)))
            .into()
    }
}

pub struct List {
    empty_message: &'static str,
    header: Option<ListHeader>,
    children: View,
}

impl List {
    pub fn new(children: impl Into<View>) -> Self {
        Self {
            empty_message: "no items",
            header: None,
            children: children.into(),
        }
    }

    pub fn header(mut self, header: ListHeader) -> Self {
        self.header = Some(header);
        self
    }
}

impl From<List> for View {
    fn from(list: List) -> Self {
        let content: View = match list.children.as_web_sys().is_empty() {
            true => p().children(list.empty_message).into(),
            false => ul().children(list.children).into(),
        };

        match list.header {
            Some(list_header) => section().children(list_header).children(content).into(),
            None => content,
        }
    }
}

pub struct EpisodesList<T>
where
    T: Fn(Episode) -> View + 'static,
{
    episodes: ReadSignal<Vec<Episode>>,
    card_renderer: T,
}

impl<T> EpisodesList<T>
where
    T: Fn(Episode) -> View,
{
    pub fn new(episodes: ReadSignal<Vec<Episode>>, card_renderer: T) -> Self {
        Self {
            episodes,
            card_renderer,
        }
    }
}

impl<T> From<EpisodesList<T>> for View
where
    T: Fn(Episode) -> View,
{
    fn from(list: EpisodesList<T>) -> Self {
        List::new(move || {
            list.episodes
                .get_clone()
                .into_iter()
                .map(|e| li().children((list.card_renderer)(e)).into())
                .collect::<Vec<_>>()
        })
        .header(ListHeader::new("Episodes").end_slot(Button::new_with_icon(
            Icon::new(IconType::Sort),
            |_| unimplemented!(),
        )))
        .into()
    }
}
