#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

pub mod components;
use components::*;
pub mod pages;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    // let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        Header::Header {}
        Main::Main {}
    })
}
