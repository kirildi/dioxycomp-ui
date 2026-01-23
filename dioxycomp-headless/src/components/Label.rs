//! # Label Component
//!
//! Renders a simple label

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct LabelProps {
    pub id: Option<String>,
    pub r#for: Option<String>,
    pub value: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
    pub children: Element,
}

pub fn Label(
    LabelProps {
        id,
        r#for,
        value,
        class_name,
        style,
        children,
    }: LabelProps,
) -> Element {
    match id {
        Some(_) => rsx! {
            label {
                id: id,
                r#for: r#for,
                class: class_name,
                style: style,
                {value},
                {children}
            },
        },
        None => rsx! {
            label {
                r#for: r#for,
                class: class_name,
                style: style,
                {value}
                {children}
            }
        },
    }
}
