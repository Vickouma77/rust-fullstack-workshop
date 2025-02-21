mod components;
mod models;

use components::{Footer, Header};

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    static MAIN_CSS: Asset = asset!("/assets/main.css");
    static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        main { 
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header{},

            section { 
                class:  "md:container md:mx-auto md:py-8 flex-1",
                div {  
                    class: "flex justify-center items-center",

                    div { 
                        class: "flex flex-col justify-center items-center",
                        h1 { class: "text-4xl font-bold text-gray-800", "Welcome to Rusty Films" },
                     }
                }
             }

            div { 
                class: "flex justify-center items-center",
                Footer{},
             }
         }
    }
}