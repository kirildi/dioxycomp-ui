//! # Label Component
//!
//! Renders a simple label

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::{html::GlobalAttributes, prelude::*};

#[derive(Props, Clone, Default)]
pub struct LabelProps<'a> {
    pub id: Option<&'a str>,
    pub r#for: Option<&'a str>,
    pub value: Option<&'a str>,
    pub styles: Option<&'a str>,
}
#[derive(Props, Clone, Default)]
pub struct AllLabelProps<'a> {
    pub label_props: LabelProps<'a>,
}

pub fn Label<'a>(cx: Scope<'a, AllLabelProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        label{
            id: cx.props.label_props.id,
            r#for: cx.props.label_props.r#for,
            style: cx.props.label_props.styles,
        },
        cx.props.label_props.value
    })
}
