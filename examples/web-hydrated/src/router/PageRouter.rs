#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::components::Header::Header;
use crate::app::components::Main::Main;
use crate::pages::headless::components::Button::Button;
use crate::pages::headless::HeadlessPage::HeadlessPage;

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        Main {},
        #[nest("/headless")]
            #[layout(HeadlessPage)]                
                #[route("/:name")]
                Button { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]            
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },       
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
