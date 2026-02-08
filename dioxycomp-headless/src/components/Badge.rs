//! # Badge Component
//!
//! Renders a simple badge

#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct BadgeProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
    pub children: Element,
}

pub fn Badge(
    BadgeProps {
        id,
        class_name,
        style,
        children,
    }: BadgeProps,
) -> Element {
    rsx! {
        span {
            id: id,
            class: class_name,
            style: style,
            {children}
        }
    }
}
