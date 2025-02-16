use nero_extensions::types::Series;
use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{AlignItems, FlexDirection, JustifyContent},
    layout::{Display, ObjectFit, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
    transforms::Rotate,
    typography::TextAlign,
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        document,
        tags::{article, br, div, figure, img, p},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::{
    components::{Button, Icon, IconType, Toolbar},
    tw,
    types::sample_series,
};

pub struct HomePage {
    series: Vec<Series>,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            series: (1..=5).map(|_| sample_series()).collect::<Vec<_>>(),
        }
    }
}

impl HomePage {
    // TODO: Dynamic series poster
    fn render_dynamic_series(series: Vec<Series>) -> View {
        let series = series.into_iter().next().unwrap();

        img()
            .class(tw!(
                Width::WFull,
                Height::HFull,
                ObjectFit::Cover,
                BorderRadius::Xl
            ))
            // TODO: Default image
            .src(series.poster_url.unwrap().to_string())
            .alt(series.title.clone())
            .into()
    }

    // TODO: Progress indicators
    // should this be in a separate function?
    fn render_dynamic_indicators() -> View {
        p().class(tw!(Rotate::Number("90")))
            .children("Indicators...")
            .into()
    }

    fn render_empty_feedback() -> View {
        article()
            .class(tw!(
                Display::Flex,
                FlexDirection::Col,
                Width::WFraction(3, 5),
                Overflow::Auto,
                JustifyContent::Center,
                AlignItems::Center,
                Padding::BNumber("8")
            ))
            .children(
                img()
                    .class(tw!(Width::WNumber("64")))
                    .src("assets/images/shocked_cat.svg"),
            )
            .children(
                p().class(tw!(TextAlign::Center, Padding::BNumber("2")))
                    .children("Whoops...")
                    .children(br())
                    .children("Apparently there's nothing around here."),
            )
            .children(
                Button::icon_label(Icon::new(IconType::Search), "Search series", |_| {
                    if let Some(el) = document().get_element_by_id(Toolbar::SEARCH_INPUT_ID) {
                        el.unchecked_into::<HtmlInputElement>().focus().unwrap();
                    }
                })
                .color(BackgroundColor::Orange200),
            )
            .into()
    }
}

impl From<HomePage> for View {
    fn from(page: HomePage) -> Self {
        div()
            .class(tw!(Display::Flex, Height::HFull))
            .children(
                figure()
                    .class(tw!(
                        Width::WFraction(2, 5),
                        Padding::BNumber("8"),
                        Overflow::Hidden
                    ))
                    .children(HomePage::render_dynamic_series(page.series)),
            )
            .children(
                div()
                    .class(tw!(
                        Width::WNumber("20"),
                        Display::Flex,
                        AlignItems::Center,
                        Padding::BNumber("8")
                    ))
                    .children(HomePage::render_dynamic_indicators()),
            )
            .children(
                // TODO: Series categories if the filter search is available in the extension
                // (series card is needed to display the category with its series)
                HomePage::render_empty_feedback(),
            )
            .into()
    }
}
