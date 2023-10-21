#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::components::SideBarNav::SideBarNav;
use crate::router::PageRouter::Route;

#[inline_props]
pub fn HeadlessPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex",
            SideBarNav {},
            main {
                class: "grow relative p-4 h-auto text-xl lg:ml-72 lg:w-full lg:h-full leading-normal",
                Outlet::<Route> {}
            },
        }
    })
}
