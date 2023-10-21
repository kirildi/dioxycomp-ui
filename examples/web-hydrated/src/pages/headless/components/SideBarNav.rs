#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::router::PageRouter::Route;

#[inline_props]
pub fn SideBarNav(cx: Scope) -> Element {
    let nav_style = "fixed w-full h-2/6 lg:w-72 lg:h-full p-4 bg-zinc-900";
    let link_style = "flex-none hover:bg-gray-600 hover:rounded hover:duration-100";
    cx.render(rsx! {
          nav {
                id: "sidebar",
                class: "{nav_style}",
                h2 {"Components"},
                Link {
                      class: "{link_style}",
                      to: Route::PageLoader { name: String::from("Button")},
                      "Button"
                },
                Link {
                      class: "{link_style}",
                      to: Route::PageLoader { name: String::from("Checkbox")},
                      "Checkbox"
                },
                Link {
                      class: "{link_style}",
                      to: Route::PageLoader { name: String::from("Radio")},
                      "Radio"
                },
          },
    })
}
