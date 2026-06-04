use dioxus::core::Task;
use dxnote::db::*;
use dxnote::config::*;
use dxnote::hooks::*;

use dioxus::desktop::Config;
use dioxus::prelude::*;
use dxnote::note::NoteSummary;
use tokio::task::JoinHandle;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let pool = connect_db().await.map_err(|e| {
        eprintln!("DB 연동 실패");
        e
    })?;

    let window_attrs = screen_config();

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window_attrs))
        .with_context(pool)
        .launch(App);

    Ok(())
}


#[component]
fn App() -> Element {

    rsx! {

        Note {}
    }
}

#[component]
fn Note() -> Element {
    
    let text_value = use_signal(|| String::new());
    
    let mut current_note_id = use_signal(|| None::<i64>);
    let mut original_content = use_signal(|| String::new());
    
    let mut list_resource = use_list_resource();
    
    let mut save_task = use_signal(|| None::<Task>);
    
    // use_auto_save(text_value, original_content, current_note_id, list_resource);
    use_auto_save(text_value, original_content, current_note_id, list_resource, save_task);
    
    // use_effect(move || {
    //     let _ = original_content.read(); // 원본 변경 감지용
    //     list_resource.restart();         // 안전하게 리스트만 새로고침
    // });
    
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            class: "app-container",
            
            NoteList {text_value, original_content, current_note_id, list_resource}
            Textarea {text_value, list_resource}

        }        
    }
}

#[component]
fn NoteList(text_value: Signal<String>, original_content: Signal<String>, current_note_id: Signal<Option<i64>>, list_resource: Resource<Vec<NoteSummary>>) -> Element { 
// fn NoteList(text_value: Signal<String>, original_content: Signal<String>, current_note_id: Signal<Option<i64>>) -> Element { 

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
                        
                        let title = note.content.lines().next().unwrap_or("Empty");
                        
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
                                    // list_resource.restart();
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
fn Textarea(text_value:Signal<String>, list_resource: Resource<Vec<NoteSummary>>) -> Element {

    rsx! {

           div { 
                class: "main-content",
                
                textarea {
                    class: "textarea",
                    value: "{text_value}",
                    oninput: move |event| {
                        text_value.set(event.value());
                        // list_resource.restart();
                    },
                }
            }
    }
}

// #[component]
// fn Note1() -> Element {

//     let mut text_value = use_signal(|| String::new());
//     let mut current_note_id = use_signal(|| None::<i64>);
//     let mut original_content = use_signal(|| String::new());

//     let mut list_resource = use_list_resource();

//     let _ = use_auto_save(text_value, original_content, current_note_id);

//     use_effect(move || {
//         let _ = original_content.read(); // 원본 변경 감지용
//         list_resource.restart();         // 안전하게 리스트만 새로고침
//     });

//     // rsx! 앞에 아무것도 붙이지 않고 마지막 줄에 배치
//     rsx! {
//         document::Link { rel: "stylesheet", href: MAIN_CSS }
//         div { 
//             class: "app-container",
            
//             div { 
//                 class: "sidebar",
//                 for note in list_resource.value().cloned().unwrap_or_default() {
//                     // 복잡한 연산은 미리 변수로!
//                     {
//                         let time_str = note.updated_at
//                             .with_timezone(&chrono::Local)
//                             .format("%m/%d %H:%M")
//                             .to_string();
//                         let title = note.content.lines().next().unwrap_or("Empty");
                        
//                         rsx! {
//                             div { 
//                                 class: "note-item",
//                                 onclick: move |_| {
//                                     /* 
//                                     사용자가 단순 클릭만 했을땐 변동이 있어선 안된다.    
//                                     */
//                                     text_value.set(note.content.clone());
//                                     original_content.set(note.content.clone()); // 원본 내용 백업
//                                     current_note_id.set(Some(note.id));
//                                     // list_resource.restart();
//                                 },
//                                 b { "{title}" }
//                                 p { 
//                                     style: "font-size: 0.8em; color: gray;",
//                                     "{time_str}" 
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
            
//             div { 
//                 class: "main-content",
//                 textarea {
//                     class: "textarea",
//                     value: "{text_value}",
//                     oninput: move |event| {
//                         text_value.set(event.value());
//                     },
//                 }
//             }
//         }
//     }
// }