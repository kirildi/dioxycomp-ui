#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::router::PageRouter::Route;

#[inline_props]
pub fn HeadlessPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex",
            nav { // TODO Extract to component
                id: "sidebar",
                class: "fixed w-full h-2/6 lg:w-72 lg:h-full p-4 bg-zinc-900",
                h2 {"Components"},
                Link {
                    class: "flex-none hover:bg-gray-600 hover:rounded hover:duration-100",
                    to: Route::Button { name: String::from("Button")},
                    "Button"
                },
            },
            main {
                class: "grow relative p-4 h-auto text-xl lg:ml-72 lg:w-full lg:h-full leading-normal",
                Outlet::<Route> {}
            },
        }
    })
}
