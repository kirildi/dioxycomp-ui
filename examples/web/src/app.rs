#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;

pub mod components;
use components::*;

pub fn App(cx: Scope) -> Element {
    // let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div{
            class:"bg-gray-800",
            Header::Header {},
            Main::Main {},
            //Outlet::<PageRouter::Route> { }
        }
    })
}
