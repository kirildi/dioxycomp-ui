//! # Button Component
//!
//! Renders a simple button which can be clicked to toggle

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::{html::GlobalAttributes, prelude::*};

#[derive(Props, Clone, Default)]
pub struct ButtonProps<'a> {
    pub id: Option<&'a str>,
    pub label: Option<&'a str>,
    pub autofocus: Option<bool>,
    pub disabled: Option<bool>,
    pub name: Option<&'a str>,
    pub r#type: Option<&'a str>,
    pub value: Option<&'a str>,
    pub styles: Option<&'a str>,
}
#[derive(Props, Clone, Default)]
pub struct AllButtonProps<'a> {
    pub button_props: ButtonProps<'a>,
}

pub fn Button<'a>(cx: Scope<'a, AllButtonProps<'a>>) -> Element<'a> {
    let state = use_state(&cx, || false);
    cx.render(rsx! {
        button{
            id: cx.props.button_props.id,
            autofocus: cx.props.button_props.autofocus,
            disabled: cx.props.button_props.disabled,
            name: cx.props.button_props.name,
            r#type: cx.props.button_props.r#type,
            value: cx.props.button_props.value,
            style: cx.props.button_props.styles,
            onclick: move |_| state.set(!state),
            cx.props.button_props.label
        }
    })
}

// pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
//     let state = use_state(&cx, || false);

//     cx.render(rsx! {button {
//         id: cx.props.id,
//         autofocus: cx.props.autofocus,
//         disabled: cx.props.disabled,
//         name: cx.props.name,
//         r#type: cx.props.r#type,
//         value: cx.props.value,
//         style: cx.props.styles,
//         onclick: move |_| state.set(!state),
//     }})
// }
