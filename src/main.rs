use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {

    let window_attrs = WindowBuilder::new()
        .with_always_on_top(false) 
        .with_title("dxnote");

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(window_attrs)
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    
    rsx! {
        Note {}
        Button {}
    }
   
}

#[component]
fn Note() -> Element {
   let mut text_value = use_signal(|| String::new());
    
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS,}
        div {
            textarea {
                class: "textarea",
                oninput: move |event| {
                text_value.set(event.value());
                },
            }
        }
    } 
}

#[component]
fn Button() -> Element {
    rsx! {
         div {
            class: "save-button-container",

            button {
                onclick: move |_| {
                    println!("click");
                    /* 
                        1. save를 누르면
                        2. insert 쿼리가 실행되면서
                        3.
                     */
                },
                "save",
            }
         }
         /* 
            onclick save
                "insert ~"
         */
    }
}