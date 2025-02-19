use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    pub text: String,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button { 
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
            onclick: props.onclick,
            {props.text}
         }
    }
}