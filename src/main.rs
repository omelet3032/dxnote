use dioxus::desktop::{Config, WindowBuilder, use_window};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {

    let window_attrs = WindowBuilder::new()
        .with_always_on_top(false) 
        .with_title("dxnote")
        .with_visible(true)
        .with_focused(true); // 포커스 동작 보정 1순위 정적


    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(window_attrs)
        )
        .launch(App);
}

#[component]
fn App() -> Element {
//    let window = use_window();

//     // 앱이 처음 렌더링될 때 딱 한 번 실행됩니다.
//     use_effect(move || {
//         // 창을 전면으로 가져오고 포커스를 요청합니다.
//         window.set_focus();
//     }); // 포커스 동작 보정 2순위

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