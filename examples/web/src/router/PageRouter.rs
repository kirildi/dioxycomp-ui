#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::app::components::Header::Header;
use crate::app::components::Main::Main;
use crate::pages::headless::components::Button::Button;
use crate::pages::headless::HeadlessPage::HeadlessPage;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        Main {},
        #[nest("/headless")]
            #[layout(HeadlessPage)]                
                #[route("/:name")]
                Button { name: String },
                
            
}
