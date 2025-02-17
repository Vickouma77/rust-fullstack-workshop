use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let folder = asset!("/assets");

    rsx! {
        footer {
            class: "flex justify-center items-center bg-gray-900 p-4",

            a {
                class: "w-16 h-16 md:w-20 md:h-20",
                href: "",
                target: "_blank",
                img {
                    class: "w-full h-full object-contain",
                    alt: "prestone",
                    src: "{folder}/prestone.png",
                    "loading": "lazy"
                }
            }

            svg {
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "w-6 h-6 mx-4 text-white",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M6 18L18 6M6 6l12 12"
                }
            }

            a {
                class: "w-16 h-16 md:w-20 md:h-20",
                href: "",
                target: "_blank",
                img {
                    class: "w-full h-full object-contain",
                    alt: "rust",
                    src: "{folder}/bcnrust.png",
                    "loading": "lazy"
                }
            }
        }
    }
}
