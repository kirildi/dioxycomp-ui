#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

pub mod app;
pub mod pages;
use app::*;

fn main() {
    LaunchBuilder::new(app::App).launch();
}
