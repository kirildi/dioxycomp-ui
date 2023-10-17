#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::headless::components::Button::Button;

#[inline_props]
pub fn PageLoader(cx: Scope, name: String) -> Element {
    match name {
        Button => cx.render(rsx! {
            self::Button { name: String::from("Button")}
        }),
    }
}
