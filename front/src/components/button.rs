use dioxus::prelude::*;
use crate::models::ButtonType;

#[component]
pub fn Button(button_type: ButtonType, onclick: EventHandler<MouseEvent>) -> Element {
    rsx!{
        button {
            class: format!("text-white px-4 py-2 rounded-md transition-colors duration-300 ease-in-out {}",
                button_type),
            onclick: move |event| {
                onclick.emit(event);
            },
            children: {
                "Click me"
            }
        }
    } 
}