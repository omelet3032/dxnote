use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

fn main() {
    let window_attrs = WindowBuilder::new()
        .with_always_on_top(false) // 여기서 명시적으로 false 설정
        .with_title("dxnote");

    // 2. Config에 해당 설정을 넣어서 실행합니다.
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window_attrs).with_custom_head(r#"<link rel="stylesheet" href="assets/main.css">"#.to_string()))
        .launch(App);
}

#[component]
fn App() -> Element {
    // 입력된 텍스트를 저장할 상태 (Signal)
    let mut text_value = use_signal(|| String::new());

    rsx! {
        div {
            textarea {
                class: "textarea",
                // value: "{text_value}",
                // 값이 변할 때마다 상태 업데이트
                // oninput: move |event| text_value.set(event.value()),
                oninput: move |event| {
                text_value.set(event.value());
                // 로그를 찍어 한글이 제대로 들어오는지 확인
                println!("입력값: {}", event.value());
                },
            }
        }
    }
}
