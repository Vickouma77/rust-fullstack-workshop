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
        Hero{}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            "Hello, Vick"
        }
    }
}
