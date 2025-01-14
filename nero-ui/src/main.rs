mod components;
mod macros;
mod pages;
mod types;
mod utils;

use pages::{BaseLayout, WatchPage};
use sycamore::render;

fn main() {
    console_error_panic_hook::set_once();

    render(|| BaseLayout::new(WatchPage).into())
}
