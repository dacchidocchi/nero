use std::rc::Rc;

use nero_extensions::types::{Episode, SeriesVideo};
use rustwind::{
    flexbox_grid::{FlexDirection, Gap, GridTemplateColumns},
    layout::{Display, Overflow},
    sizing::Height,
    tw,
    typography::{FontSize, FontWeight},
};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::{
    prelude::ReadSignal,
    web::{
        tags::{article, aside, div, h1, p, section},
        window, GlobalProps, HtmlGlobalAttributes, Resource, View,
    },
};
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    components::{EpisodesList, IntoSmallClickableCard, OnReachBottom, VideoPlayer},
    hooks::{use_episode_videos, use_infinite_episodes, InfinitePage},
    utils::{navigate_with_state, ViewBuilder},
};

pub struct WatchPage {
    // TODO: Change to Resource<Series>?
    series_id: String,
    episode: Episode,
    videos: Resource<Vec<SeriesVideo>>,
    episodes_page: Rc<InfinitePage<Episode>>,
}

impl WatchPage {
    pub fn new(series_id: String, episode_id: String) -> Self {
        Self {
            series_id: series_id.clone(),
            episode: {
                let state = window().history().unwrap_throw().state().unwrap_throw();
                from_value::<Episode>(state).unwrap_throw()
            },
            videos: use_episode_videos(series_id.clone(), episode_id),
            episodes_page: Rc::new(use_infinite_episodes(series_id)),
        }
    }
}

impl WatchPage {
    fn render_episode_details(title: String, synopsis: Option<String>) -> View {
        section()
            .class(tw!(Display::Flex, FlexDirection::Col, Gap::Number("2")))
            .children(
                h1().class(tw!(FontSize::_2Xl, FontWeight::Semibold))
                    .children(title),
            )
            .when_some(synopsis, |this, synopsis| {
                this.children(p().children(synopsis))
            })
            .into()
    }

    fn render_episodes(series_id: String, episodes: ReadSignal<Vec<Episode>>) -> View {
        EpisodesList::new(episodes, move |e| {
            let nav_to = format!("/watch/{}/{}", series_id, e.id);
            let state = to_value(&e).unwrap_throw();
            e.into_small_clickable_card(move |_| navigate_with_state(&nav_to, &state))
        })
        .into()
    }
}

impl From<WatchPage> for View {
    fn from(page: WatchPage) -> Self {
        div()
            .class(tw!(
                Display::Grid,
                Height::HFull,
                GridTemplateColumns::Value("4fr_2fr"),
                Gap::Number("12"),
                Overflow::Hidden
            ))
            .children(
                article()
                    .class(tw!(
                        Display::Flex,
                        FlexDirection::Col,
                        Gap::Number("4"),
                        Overflow::YAuto
                    ))
                    .children(move || match page.videos.get_clone() {
                        Some(videos) => {
                            View::from(VideoPlayer::new(videos[0].video_url.clone()).when_some(
                                page.episode.thumbnail_url.clone(),
                                |this, thumbnail_url| this.poster_url(thumbnail_url),
                            ))
                        }
                        None => "Loading player...".into(),
                    })
                    .children(WatchPage::render_episode_details(
                        page.episode
                            .title
                            .unwrap_or(format!("Episode {}", page.episode.number)),
                        page.episode.description,
                    )),
            )
            .children(
                aside()
                    .on_reach_bottom({
                        let episodes_page = page.episodes_page.clone();
                        move || episodes_page.load_more()
                    })
                    .class(tw!(Overflow::YAuto))
                    .children(WatchPage::render_episodes(
                        page.series_id,
                        page.episodes_page.items(),
                    )),
            )
            .into()
    }
}
