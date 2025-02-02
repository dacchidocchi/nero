use rustwind::{
    active,
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, FlexDirection, Gap, JustifyContent},
    hover,
    interactivity::Cursor,
    layout::{AspectRatio, Display, ObjectFit},
    sizing::Width,
    spacing::Padding,
    transforms::Scale,
    transitions_animation::TransitionDuration,
    typography::{Color, FontSize, FontWeight, LineClamp, TextOverflow},
};
use sycamore::{
    prelude::{HtmlAAttributes, HtmlImgAttributes},
    web::{
        tags::{a, div, h3, img, p, span, HtmlA},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};

use crate::{tw, types::Episode, utils::ViewBuilder};

pub trait IntoSmallCard<T: Into<View>> {
    fn into_small_card(self) -> T;
}

pub trait IntoCard<T: Into<View>> {
    fn into_card(self) -> T;
}

const BASE_EPISODE_CARD_CLASSES: &str = tw!(
    Display::Flex,
    AlignItems::Center,
    Gap::Number("4"),
    Padding::Number("1"),
    Cursor::Pointer,
    BorderRadius::Md,
    TransitionDuration::Number("300"),
    hover!(BackgroundColor::Gray100),
    active!(Scale::Number("95"))
);

impl IntoSmallCard<HtmlA> for Episode {
    fn into_small_card(self) -> HtmlA {
        a().href("/watch")
            .class(BASE_EPISODE_CARD_CLASSES)
            .children(
                img()
                    .class(tw!(
                        Width::WFraction(1, 2),
                        BorderRadius::Lg,
                        AspectRatio::Video,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.thumbnail_url.unwrap_or_default())
                    .alt(
                        self.title
                            .clone()
                            .unwrap_or(format!("Episode {}", self.number)),
                    ),
            )
            .children(
                div()
                    .class(tw!(
                        Width::WFraction(1, 2),
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::Number("1")
                    ))
                    .children(
                        h3().class(tw!(TextOverflow::Truncate, FontWeight::Semibold))
                            .children(format!("Episode {}", self.number)),
                    )
                    .when_some(self.title, |this, title| {
                        this.children(
                            p().class(tw!(LineClamp::Number("2"), Color::Gray500, FontSize::Sm))
                                .children(title),
                        )
                    }),
            )
    }
}

impl IntoCard<HtmlA> for Episode {
    fn into_card(self) -> HtmlA {
        let title = self.title.unwrap_or(format!("Episode {}", self.number));

        a().href("/watch")
            .class(BASE_EPISODE_CARD_CLASSES)
            .children(
                span()
                    .class(tw!(
                        Width::WFraction(1, 12),
                        Display::Flex,
                        JustifyContent::Center,
                        FontWeight::Semibold
                    ))
                    .children(self.number),
            )
            .children(
                img()
                    .class(tw!(
                        Width::WFraction(4, 12),
                        BorderRadius::Lg,
                        AspectRatio::Video,
                        ObjectFit::Cover
                    ))
                    // TODO: use a default thumbnail if none is provided
                    .src(self.thumbnail_url.unwrap_or_default())
                    .alt(title.clone()),
            )
            .children(
                div()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Width::WFraction(7, 12)
                    ))
                    .children(h3().class(tw!(FontWeight::Semibold)).children(title))
                    .when_some(self.description, |this, description| {
                        this.children(
                            p().class(tw!(Color::Gray500, FontSize::Sm, LineClamp::Number("3")))
                                .children(description),
                        )
                    }),
            )
    }
}
