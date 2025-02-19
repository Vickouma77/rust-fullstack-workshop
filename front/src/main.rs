mod components;
use components::{Footer, Header, Button};

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
                    
                    Button{ 
                        text: "Click me", onclick: move |event| println!("Clicked! {event:?}") 
                    }
                }
             }
            Footer{}
         }
    }
}