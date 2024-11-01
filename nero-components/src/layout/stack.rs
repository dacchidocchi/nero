use leptos::IntoView;
use typewind::{
    flexbox_grid::{AlignItems, FlexDirection, FlexWrap, Gap, JustifyContent},
    layout::Display,
    ToClasses,
};

use super::Layout;

/// Flex layout container that arranges child elements
/// in either a horizontal (row) or vertical (column) stack. It provides properties
/// for alignment, wrapping, and spacing between child elements.
#[derive(ToClasses)]
pub struct StackLayout {
    display: Display,
    align: Option<AlignItems>,
    justify: Option<JustifyContent>,
    wrap: Option<FlexWrap>,
    direction: Option<FlexDirection>,
}

impl Layout<StackLayout> {
    /// Creates a horizontal stack layout with child elements aligned in a row.
    ///
    /// By default the horizontal stack is created with the `AlignItems::Center` alignment and
    /// `Gap::_0_5` spacing between child elements.
    pub fn h_stack(children: impl IntoView + 'static) -> Self {
        Layout::new(
            StackLayout {
                display: Display::Flex,
                align: Some(AlignItems::Center),
                justify: None,
                wrap: None,
                direction: Some(FlexDirection::Row),
            },
            children,
        )
        .gap(Gap::_0_5)
    }

    /// Creates a vertical stack layout with child elements aligned in a column.
    ///
    /// By default the vertical stack items is created with the `AlignItems::Center` alignment and
    /// `Gap::_0_5` spacing between child elements.
    pub fn v_stack(children: impl IntoView + 'static) -> Self {
        Layout::new(
            StackLayout {
                display: Display::Flex,
                align: Some(AlignItems::Center),
                justify: None,
                wrap: None,
                direction: Some(FlexDirection::Col),
            },
            children,
        )
        .gap(Gap::_0_5)
    }

    /// Sets the `align-items` property to align children along the cross-axis in the layout.
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.layout.align = Some(align);
        self
    }

    /// Sets the `justify-content` property to control the alignment of children along the main axis.
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.layout.justify = Some(justify);
        self
    }

    /// Sets the `flex-wrap` property, allowing children to wrap within the container if necessary.
    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.layout.wrap = Some(wrap);
        self
    }
}
