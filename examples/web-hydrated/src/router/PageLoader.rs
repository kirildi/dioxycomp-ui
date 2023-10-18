#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::headless::components::ButtonPage::ButtonPage;
use crate::pages::headless::components::CheckboxPage::CheckboxPage;

#[inline_props]
pub fn PageLoader(cx: Scope, name: String) -> Element {
    match name.as_str() {
        "Button" => cx.render(rsx! {
            p { "{name}"},
            ButtonPage { name: String::from("Button")}
        }),
        "Checkbox" => cx.render(rsx! {
            p { "{name}"},
            CheckboxPage { name: String::from("Checkbox")}
        }),
        _ => cx.render(rsx! {
            p { "no page to render" }
        }),
    }
}
