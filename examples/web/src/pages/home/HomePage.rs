#![allow(non_snake_case)]
use dioxus::prelude::*;

// use dioxycomp_headless::components::SimpleButton::SimpleButton;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "h-64 mx-auto pt-16 rounded-xl bg-gray-800",
            h1 {
                class: "h-16 text-3xl lg:text-5xl text-center leading-normal font-bold",
                "DIOXYCOMP"
            },
            h2 {
                class: "text-2xl lg:text-4xl text-center leading-normal font-bold",
                "Headless UI component library for the ",
                a {
                    class: "text-blue-300",
                    href: "https://dioxuslabs.com/",
                    "Dioxus framework"
                }
            },
            div {} //TODO add component HeadlessCard {}
        }
    })
}
