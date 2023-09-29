#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages;

pub fn Main(cx: Scope) -> Element {
    cx.render(rsx! {
            main {
            class: "",
            pages::Home::Home {}
        }
    })
}
