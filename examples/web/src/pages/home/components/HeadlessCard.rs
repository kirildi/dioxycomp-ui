#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn HeadlessCard(cx: Scope) -> Element {
    cx.render(rsx! {
        // Card container
        div {
            class: "relative w-96 h-[24em] mx-auto rounded-2xl bg-gray-800 border-2 border-gray-300 border-solid shadow-md shadow-indigo-500 hover:shadow-lg hover:shadow-indigo-500",
            // Card main content
            div {
                class: "h-64 p-4 border-b-2 border-gray-300 text-center",
                span {
                    class: "material-symbols-outlined p-4 !text-[12em] opacity-50  text-gray-300",
                    "restart_alt"
                }
            },
            //Card description
            div {
                class: "h-24 p-4",
                h1{ 
                    class:"text-xl text-left font-bold",
                    "Headless UI components"
                },
                p {
                    class: "text-lg text-left break-words",
                    "Simple unstyled, open-source, accessible UI components, to use in your applications "
                }
            }
        }
    })
}
