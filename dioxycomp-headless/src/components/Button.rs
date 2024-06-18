//! # Button Component
//!
//! Renders a simple button which can be clicked to toggle

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::prelude::*;
use dioxus_elements::button;

#[component]
pub fn Button(
    onpress: EventHandler<MouseEvent>,
    children: Element,
    id: Option<String>,
    styles: Option<String>,
    name: Option<String>,
    value: Option<String>,
    disabled: Option<bool>,
    r#type: Option<String>,
    autofocus: Option<bool>,
) -> Element {
    let mut state = use_signal(|| false);
    let setState = |mut st: Signal<bool>| st.toggle();
    rsx! {
        button {
                id: id,
                autofocus: autofocus,
                disabled: disabled,
                name: name,
                r#type: r#type,
                value: value,
                style: styles,
                onclick: move |event| onpress.call(event),
                {children}
        }
    }
}
