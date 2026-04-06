use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 입력된 텍스트를 저장할 상태 (Signal)
    let mut text_value = use_signal(|| String::new());

    rsx! {
    div {
        textarea {
            style: "resize: none",
            value: "{text_value}",
            // 값이 변할 때마다 상태 업데이트
            oninput: move |event| text_value.set(event.value())
        }

    }
    }
}
