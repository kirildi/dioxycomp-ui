#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Nav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "h-24 p-8 flex gap-8 text-xl mr-0",
            a {
                href: "/",
                "Home"
            },
            a {
                href: "#",
                "Components"

            }
        }
    })
}
