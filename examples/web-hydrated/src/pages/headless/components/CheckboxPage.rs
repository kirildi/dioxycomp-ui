#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxycomp_headless::components::Checkbox::Checkbox;

#[inline_props]
pub fn CheckboxPage(cx: Scope, name: String) -> Element {
    cx.render(rsx! {
        section {
            id: "main_heading",
            h1 {
                class: "pb-6 text-4xl font-serif font-semibold",
                "Checkbox"
            },
            hr {},
        },
        section {
            id: "description", 
            p {
                class: "text-xl",
                "The checkboxes are typically used to select an item from a list of individual items, or mark one individual item as selected.  
                They consist of a visual select indicator and a label. Checkboxes support three selection states - checked, unchecked and indeterminate. 
                "
            },
        }
        section {
            id: "examples",
            h2 {
                class: "pb-4 text-2xl font-serif font-semibold",
                "Example"
            },
            hr {},
            div {
                class: "flex flex-wrap flex-row my-8 p-4 h-40 bg-neutral-800 rounded-xl",
                div {
                    class: "w-full h-8 pl-4 pb-12 text-white/40 border-b-2 border-white/30 ",
                    "Preview: "
                }
                div {
                    class: "flex justify-center items-center w-full h-24",
                    Checkbox {}
                }
            },
            div {
                class: "flex flex-wrap flex-row p-4 h-96 bg-neutral-800 rounded-xl",
                div {
                    class: "w-full h-8 pl-4 pb-12 text-white/40 border-b-2 border-white/30 ",
                    "Code: "
                },
                div {
                    class: "flex justify-start items-center", 
                    pre {
                        code {
                            class: "text-base",
                            "
                            use dioxycomp_headless::components::Checkbox::Checkbox;
                            //... Some other code here
                            
                            pub fn HomePage(cx: Scope, name: String) -> Element {{    
                                p {{
                                    Checkbox {{}},
                                }}
                            }} 
                            "
                        }
                    }
                }
            }
        }
   })
}
