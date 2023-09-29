#![allow(non_snake_case)]
use dioxus::prelude::*;

// use dioxycomp_headless::components::SimpleButton::SimpleButton;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {section{
        class: "bg-gray-900",
    }})
}
