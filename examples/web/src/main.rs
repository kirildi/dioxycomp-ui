#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

pub mod common_components;
use common_components::*;
pub mod pages;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    // let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div{
            class:"bg-gray-800",
            Header::Header {}
            Main::Main {}
        }
    })
}
