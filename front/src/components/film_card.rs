use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct FilmCardProps {
    pub title: String,
    pub description: String,
    pub image: String,
}

#[component]
pub fn FilmCard(props: FilmCardProps) -> Element {
    rsx! {
        div {
            class: "max-w-sm rounded overflow-hidden shadow-lg",
            img { 
                class: "w-full",
                src: props.image,
                alt: props.title
            }
            div { class: "px-6 py-4" 
                h1 { class: "font-bold
                    text-xl mb-2",
                    {props.title}
                }
                p { class: "text-gray
                    text-base",
                    {props.description}
                }
            }
        }
    }
}