use nero_extensions::{
    types::{EpisodesPage, Series},
    url::Url,
};
use rustwind::{
    backgrounds::BackgroundColor,
    borders::BorderRadius,
    flexbox_grid::{FlexDirection, Gap, GridTemplateColumns},
    layout::{Display, ObjectFit, Overflow},
    sizing::{Height, Width},
    spacing::Padding,
    typography::{FontSize, FontWeight, LineClamp, TextOverflow},
};
use sycamore::{
    prelude::HtmlImgAttributes,
    web::{
        tags::{article, div, figure, h1, header, img, li, p},
        GlobalProps, HtmlGlobalAttributes, View,
    },
};
use sycamore_router::navigate;

use crate::{
    components::{Button, Icon, IconType, IntoClickableCard, List, ListHeader},
    tw,
    types::{sample_episode, sample_series},
    utils::ViewBuilder,
};

pub struct SeriesPage {
    series: Series,
    episodes: EpisodesPage,
}

impl SeriesPage {
    #[allow(unused_variables)]
    pub fn new(series_id: String) -> Self {
        Self {
            series: sample_series(),
            episodes: EpisodesPage {
                items: (1..=12).map(|_| sample_episode()).collect::<Vec<_>>(),
                has_next_page: false,
            },
        }
    }
}

impl SeriesPage {
    fn render_series_poster(src: Option<Url>, alt: String) -> View {
        img()
            .class(tw!(Width::SizeFull, BorderRadius::Xl, ObjectFit::Cover))
            // TODO: Default image
            .src(src.unwrap().to_string())
            .alt(alt)
            .into()
    }

    /// Renders the series details, including the title, synopsis and a list of quick action buttons.
    ///
    /// # Arguments
    ///
    /// * `id` - The series ID.
    /// * `title` - The series title.
    /// * `synopsis` - The series synopsis.
    /// * `first_episode_id` - The ID of the first episode in the series, used to render a "Watch now" button.
    fn render_series_details(
        id: String,
        title: String,
        synopsis: Option<String>,
        first_episode_id: String,
    ) -> View {
        header()
            .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("4")))
            .children(
                h1().class(tw!(
                    TextOverflow::Truncate,
                    FontSize::_3Xl,
                    FontWeight::Bold
                ))
                .children(title),
            )
            .children(
                div()
                    .class(tw!(Display::Flex, Gap::Number("4")))
                    .children(
                        Button::icon_label(Icon::new(IconType::Play), "Watch now", move |_| {
                            let nav_to = format!("/watch/{}/{}", id, first_episode_id);
                            navigate(&nav_to)
                        })
                        .color(BackgroundColor::Red300),
                    )
                    .children(
                        Button::icon_label(
                            Icon::new(IconType::Share),
                            "Share the series",
                            |_| todo!(),
                        )
                        .color(BackgroundColor::Red300),
                    ),
            )
            .when_some(synopsis, |this, synopsis| {
                this.children(p().class(tw!(LineClamp::Number("5"))).children(synopsis))
            })
            .into()
    }

    fn render_series_episodes(series_id: String, episodes: EpisodesPage) -> View {
        List::new(
            episodes
                .items
                .into_iter()
                .map(|e| {
                    let nav_to = format!("/watch/{}/{}", series_id, e.id);
                    li().children(e.into_clickable_card(move |_| navigate(&nav_to)))
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .header(
            ListHeader::new("Episodes")
                .end_slot(Button::icon(Icon::new(IconType::Sort), |_| todo!())),
        )
        .into()
    }
}

impl From<SeriesPage> for View {
    fn from(page: SeriesPage) -> Self {
        div()
            .class(tw!(
                Display::Grid,
                Height::HFull,
                GridTemplateColumns::Value("2fr_3fr"),
                Gap::Number("20")
            ))
            .children(
                figure()
                    .class(tw!(Overflow::Hidden, Padding::BNumber("8")))
                    .children(SeriesPage::render_series_poster(
                        page.series.poster_url,
                        page.series.title.clone(),
                    )),
            )
            .children(
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::Number("4"),
                        Overflow::Auto
                    ))
                    .children(SeriesPage::render_series_details(
                        page.series.id.clone(),
                        page.series.title,
                        page.series.synopsis,
                        page.episodes.items.first().unwrap().id.clone(),
                    ))
                    .children(SeriesPage::render_series_episodes(
                        page.series.id,
                        page.episodes,
                    )),
            )
            .into()
    }
}
