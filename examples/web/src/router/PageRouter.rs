#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::app::components::Header::Header;
use crate::app::components::Main::Main;
use crate::pages::headless::components::Button::Button;
use crate::pages::headless::HeadlessPage::HeadlessPage;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        Main {},
        #[nest("/headless")]
            #[layout(HeadlessPage)]                
                #[route("/:name")]
                PageLoader { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]            
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },       
}

#[inline_props]
fn PageLoader(cx: Scope, name: String) -> Element {
    match name {
        Button => cx.render(rsx! {
            self::Button { name: String::from("Button")}
        }),
    }
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
