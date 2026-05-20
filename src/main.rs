use dxnote::db::*;
use dxnote::config::screen_config;

use dioxus::desktop::{Config};
use dioxus::prelude::*;
use sqlx::{PgPool, Pool, Postgres};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

// 메모 하나를 표현할 구조체 (sqlx 결과와 매칭)
#[derive(Clone, PartialEq, Debug)]
struct NoteSummary {
    id: i64,
    content: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}

fn get_list_resource (list_pool: Pool<Postgres>) -> Resource<Vec<NoteSummary>> {
   let list_resource = use_resource(move || {
        let pool = list_pool.clone();
        async move {
            sqlx::query_as!(
                NoteSummary,
                r#"
                SELECT 
                id, 
                content, 
                updated_at AS "updated_at!" 
                FROM notes 
                ORDER BY updated_at DESC
                "#
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        }
    }); 

    list_resource
}

struct SaveContext {
    text_value: Signal<String>,
    original_content: Signal<String>,
    current_note_id: Signal<Option<i64>>,
    save_pool: PgPool
}

// fn get_save_resource (text_value:Signal<String>, original_content: Signal<String>, current_note_id:Signal<Option<i64>>, save_pool:PgPool, mut list_resource:Resource<Vec<NoteSummary>>) {
fn use_auto_save(save_context: SaveContext, mut list_resource: Resource<Vec<NoteSummary>>) {
   
    // let _save_resource = use_resource(move || {
    use_resource(move || {
        let current_text = save_context.text_value.read().clone();
        let old_text = save_context.original_content.read().clone(); // 원본 읽기

        let pool_cloned = save_context.save_pool.clone();
        // ID 시그널 자체를 넘겨서 내부에서 최신 값을 읽게 함
        let mut id_state = save_context.current_note_id;
        let mut original_state = save_context.original_content; // 수정을 위한 캡처

        async move {
            if current_text.is_empty() || current_text == old_text { 
                return; 
            }

            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            // 잠에서 깨어난 직후에 '최신' ID를 읽음 (매우 중요!)
            let current_id = *id_state.read();

            if let Some(existing_id) = current_id {
                if let Ok(_) = update_data(existing_id, &current_text, pool_cloned).await {
                    println!("업데이트 성공");
                    original_state.set(current_text);
                    list_resource.restart(); // 리스트 갱신
                }
            } else {
                if let Ok(new_id) = insert_data(&current_text, pool_cloned).await {
                    println!("최초 저장 성공");
                    id_state.set(Some(new_id));
                    list_resource.restart(); // 리스트 갱신
                }
            }
        }
    }); 


}

#[component]
fn Note() -> Element {
    let pool = use_context::<sqlx::PgPool>();

    let mut text_value = use_signal(|| String::new());
    let mut current_note_id = use_signal(|| None::<i64>);
    let mut original_content = use_signal(|| String::new());
    let save_pool = pool.clone();

    let list_pool = pool.clone();

    let save_context = SaveContext { 
        text_value, 
        original_content, 
        current_note_id, 
        save_pool 
    };

    let list_resource = get_list_resource(list_pool);

    let _save_resource = use_auto_save(save_context, list_resource);

    // rsx! 앞에 아무것도 붙이지 않고 마지막 줄에 배치
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { 
            class: "app-container",
            
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
            
            div { 
                class: "main-content",
                textarea {
                    class: "textarea",
                    value: "{text_value}",
                    oninput: move |event| {
                        text_value.set(event.value());
                    },
                }
            }
        }
    }
}