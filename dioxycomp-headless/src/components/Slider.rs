//! # Slider Component
//!
//! Renders a Slider component, which is usually used to control values in a specific range

#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SliderProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
    pub track_class_name: Option<String>,
    pub handle_class_name: Option<String>,
    pub track_style: Option<String>,
    pub handle_style: Option<String>,
    pub children: Element,
}
#[derive(PartialEq, Props, Clone)]
pub struct SliderTrackProps {
    pub id: Option<String>,
    pub track_class_name: Option<String>,
    pub track_style: Option<String>,
    pub handle_class_name: Option<String>,
    pub handle_style: Option<String>,
    pub children: Element,
}
#[derive(PartialEq, Props, Clone)]
pub struct SliderHandleProps {
    pub id: Option<String>,
    pub handle_class_name: Option<String>,
    pub handle_style: Option<String>,
    pub children: Element,
}

pub fn Slider(
    SliderProps {
        id,
        class_name,
        style,
        track_class_name,
        handle_class_name,
        track_style,
        handle_style,
        children,
    }: SliderProps,
) -> Element {
    let is_open = use_signal(|| false);

    rsx! {
        div {
            id: id,
            class: class_name,
            style: style,
            SliderTrack {
                id: id.clone(),
                track_class_name: track_class_name,
                track_style: track_style,
                handle_class_name: handle_class_name,
                handle_style: handle_style,
                {children}
            },
        }
    }
}

pub fn SliderTrack(
    SliderTrackProps {
        id,
        track_class_name,
        track_style,
        handle_class_name,
        handle_style,
        children,
    }: SliderTrackProps,
) -> Element {
    rsx! {
        div {
            id: id.clone(),
            class: track_class_name,
            style: track_style,
            SliderHandle {handle_class_name: handle_class_name, handle_style: handle_style},
            {children}
        }
    }
}

pub fn SliderHandle(
    SliderHandleProps {
        id,
        handle_class_name,
        handle_style,
        children,
    }: SliderHandleProps,
) -> Element {
    rsx! {
        span {
            id: id.clone(),
            class: handle_class_name,
            style: handle_style,
            {children}
        }
    }
}
