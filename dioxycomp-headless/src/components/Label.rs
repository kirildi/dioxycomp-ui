//! # Label Component
//!
//! Renders a simple label

#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::{html::label, prelude::*};

#[derive(PartialEq, Props, Clone)]
pub struct LabelProps {
    pub id: Option<String>,
    pub r#for: Option<String>,
    pub value: Option<String>,
    pub class_name: Option<String>,
    pub styles: Option<String>,
}
pub fn Label(label_props: LabelProps) -> Element {
    match label_props.id {
        Some(_) => rsx! {
            label {
                id: label_props.id,
                r#for:  label_props.r#for,
                class:  label_props.class_name,
                style:  label_props.styles,
                label_props.value,
            },
        },
        None => rsx! { label { "" } },
    }
}
