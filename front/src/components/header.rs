use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let folder = asset!("/assets");

    rsx! {
        div {
            class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",

            div {
                class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
    
                a {
                    class: "flex title-font font-medium items-center text-teal-950 mb-4 md:mb-0",
                    
                    img {
                        class: "w-12 h-12 md:w-16 md:h-16 object-contain bg-transparent p-2 animate-jump",
                        alt: "ferris",
                        src: "{folder}/ferris.png",
                        "loading": "lazy"
                    } 
                    
                    span {
                        class: "ml-3 text-2xl", "Rusty films"
                    }
                }
            }
        }
    }
}
