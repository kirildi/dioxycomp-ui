//! # Label Component
//!
//! Renders a simple label

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::{html::label, prelude::*};

#[derive(Props, Clone, Default)]
pub struct LabelProps<'a> {
    pub id: Option<&'a str>,
    pub r#for: Option<&'a str>,
    pub value: Option<&'a str>,
    pub class_name: Option<&'a str>,
    pub styles: Option<&'a str>,
}
#[derive(Props, Clone, Default)]
pub struct AllLabelProps<'a> {
    #[props(!optional)]
    pub label_props: Option<LabelProps<'a>>,
    pub r#for: Option<&'a str>,
    pub children: Element<'a>,
}

pub fn Label<'a>(cx: Scope<'a, AllLabelProps<'a>>) -> Element<'a> {
    match cx.props.label_props {
        Some(_) => cx.render(rsx! {
            label {
                id: cx.props.label_props.as_ref().and_then(|lp| lp.id),
                r#for: cx.props.label_props.as_ref().and_then(|lp| lp.r#for),
                class: cx.props.label_props.as_ref().and_then(|lp| lp.class_name),
                style: cx.props.label_props.as_ref().and_then(|lp| lp.styles),
                cx.props.label_props.as_ref().and_then(|lp| lp.value)
            },
        }),
        None => cx.render(rsx! { label { cx.props.children.as_ref().unwrap() } }),
    }
}
