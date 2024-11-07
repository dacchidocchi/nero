use leptos::{
    html::{header, hr, ul, ElementChild},
    prelude::{AnyView, ClassAttribute, IntoAny},
    IntoView,
};
use typewind::{
    backgrounds::BackgroundColor,
    flexbox_grid::{FlexDirection, Gap},
    layout::{Display, Overflow, Position, TopRightBottomLeft},
    sizing::Width,
    ToClasses,
};

use crate::{layout::Layout, IntoComponent};

/// List header component that can be used with the `List` component.
#[derive(ToClasses)]
pub struct ListHeader {
    background_color: Option<BackgroundColor>,
    top_right_bottom_left: Option<TopRightBottomLeft>,
    position: Option<Position>,
    #[tw(skip)]
    children: AnyView,
}

impl ListHeader {
    /// Creates a new `ListHeader` with the specified children element.
    ///
    /// By default the list header is created with the `TopRightBottomLeft::Top0` and `Position::Sticky` properties.
    pub fn new(children: impl IntoView + 'static) -> Self {
        Self {
            background_color: None,
            top_right_bottom_left: Some(TopRightBottomLeft::Top0),
            position: Some(Position::Sticky),
            children: children.into_any(),
        }
    }

    /// Sets the background color of the list header.
    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = Some(background_color);
        self
    }

    /// Sets the top, right, bottom, or left of the list header.
    pub fn top_right_bottom_left(mut self, top_right_bottom_left: TopRightBottomLeft) -> Self {
        self.top_right_bottom_left = Some(top_right_bottom_left);
        self
    }

    /// Sets the position of the list header.
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
}

impl IntoComponent for ListHeader {
    fn into_component(self) -> impl IntoView {
        header()
            .class(format!("{} {}", self.classes(), Width::Full))
            .child((self.children, hr()))
    }
}

/// Represents a list of items with an optional header.
#[derive(ToClasses)]
pub struct List {
    #[tw(skip)]
    header: Option<ListHeader>,
    direction: Option<FlexDirection>,
    gap: Option<Gap>,
    overflow: Option<Overflow>,
    #[tw(skip)]
    children: AnyView,
}

impl List {
    /// Creates a new `List` with the specified children element.
    pub fn new(children: impl IntoView + 'static) -> Self {
        Self {
            header: None,
            direction: None,
            gap: None,
            overflow: None,
            children: children.into_any(),
        }
    }

    /// Sets the header of the list.
    pub fn header(mut self, header: ListHeader) -> Self {
        self.header = Some(header);
        self
    }

    /// Sets the flex direction of the list.
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Sets the gap between the list items.
    pub fn gap(mut self, gap: Gap) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Sets the overflow behavior of the list.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }
}

impl IntoComponent for List {
    fn into_component(self) -> impl IntoView {
        let ul = ul()
            .class(format!("{} {}", Display::Flex, self.classes()))
            .child(self.children);

        match self.header {
            Some(header) => Layout::v_stack((header.into_component(), ul))
                .gap(self.gap.unwrap_or(Gap::_0))
                .into_component()
                .into_any(),
            None => ul.into_any(),
        }
    }
}