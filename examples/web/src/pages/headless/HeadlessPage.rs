#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn HeadlessPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
           class: "h-16 text-3xl lg:text-5xl text-center leading-normal",
           "Under construction"
        },
    })
}
