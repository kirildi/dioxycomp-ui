#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::router::PageRouter::Route;

pub fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "h-24 p-8 flex gap-8 text-xl mr-0",

            Link{
                to: Route::Main {},
                "Home"
            },
            Link{
                to: Route::HeadlessPage {},
                "Headless"
            },

        }
    })
}
