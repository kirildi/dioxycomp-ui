#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::home::HomePage;

pub fn Main(cx: Scope) -> Element {
    cx.render(rsx! {
            main {
            class: "w-full relative pb-24",
            HomePage::HomePage {}
        }
    })
}
