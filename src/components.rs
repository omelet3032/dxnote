use dioxus::core::Task;
use dioxus::prelude::*;

use crate::hooks::{use_auto_save, use_list_resource};
use crate::note::NoteSummary;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn Note() -> Element {
    
    let text_value = use_signal(|| String::new());
    
    let current_note_id = use_signal(|| None::<i64>);
    let original_content = use_signal(|| String::new());
    
    let list_resource = use_list_resource();
    
    let save_task = use_signal(|| None::<Task>);
    
    use_auto_save(text_value, original_content, current_note_id, list_resource, save_task);
    
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            class: "app-container",
            
            NoteList {text_value, original_content, current_note_id, list_resource}
            Textarea {text_value}

        }        
    }
}

#[component]
fn NoteList(text_value: Signal<String>, original_content: Signal<String>, current_note_id: Signal<Option<i64>>, list_resource: Resource<Vec<NoteSummary>>) -> Element { 

    rsx! {
           div { 
            div { 
                class: "sidebar",
                
                for note in list_resource.value().cloned().unwrap_or_default() {
                    // 복잡한 연산은 미리 변수로!
                    {
                        let time_str = note.updated_at
                            .with_timezone(&chrono::Local)
                            .format("%m/%d %H:%M")
                            .to_string();
                        
                        // 1. 공백이나 엔터를 제외하고 진짜 내용이 있는 첫 번째 줄을 찾습니다.
                        let first_content_line = note.content
                                                .lines()
                                                .map(|line| line.trim())
                                                .find(|line| !line.is_empty()); // 비어있지 않은 첫 줄을 find!

                        // 2. 타이틀 결정
                        let title = match first_content_line {
                            Some(text) => text,            // 내용이 있는 첫 줄을 타이틀로 사용
                            None => "새로운 메모",          // 전체가 다 비어있거나 엔터만 있을 때
                        };
                        
                        
                        // let title = note.content.lines().next().unwrap_or("Empty");
                        /* 
                            if not.content.lines().next() == ""
                                title = 모새로운 메모"
                         */

                        rsx! {
                            div { 
                                class: "note-item",
                                onclick: move |_| {
                                    /* 
                                    사용자가 단순 클릭만 했을땐 변동이 있어선 안된다.    
                                    */
                                    text_value.set(note.content.clone());
                                    original_content.set(note.content.clone()); // 원본 내용 백업
                                    current_note_id.set(Some(note.id));
                                },
                                b { "{title}" }
                                p { 
                                    style: "font-size: 0.8em; color: gray;",
                                    "{time_str}" 
                                }
                            }
                        }
                    }
                }
            } 
        }
}
}


#[component]
fn Textarea(text_value:Signal<String>) -> Element {

    rsx! {
           div { 
                class: "main-content",
                
                textarea {
                    class: "textarea",
                    value: "{text_value}",
                    oninput: move |event| {
                        text_value.set(event.value());
                        // println!("입력값 : {}", text_value);
                    },
                }
            }
    }
}