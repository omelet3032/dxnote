use chrono::Utc;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

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

// 연습용 함수
async fn insert_data(note_content:String, pool: PgPool) -> Result<i64, sqlx::Error> {

    let result = sqlx::query!(
        r#"
        INSERT INTO notes (content, created_at, updated_at)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        note_content,
        Utc::now(), // 현재 시간
        Utc::now()
    )
    .fetch_one(&pool) // 방금 넣은 ID를 가져오기 위해 fetch_one 사용
    .await?;

    println!("아이디 : {}", result.id);
    println!("데이터 저장 성공");
    Ok(result.id)
}

// async fn update_data(note_content:String, pool:PgPool, id:i64) -> Result<(), sqlx::Error> {
async fn update_data(id:i64, note_content:String, pool:PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE notes
        SET content = $1,
            updated_at = NOW()
        WHERE id = $2
        "#,
        note_content,
        id
    ).execute(&pool).await?;

    println!("ID {} 업데이트 성공", id);
    Ok(())
}

// async fn connect_db() -> Result<sqlx::Pool<Postgres>, sqlx::Error> {
async fn connect_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok(); // .env 파일을 읽어옵니다.

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL이 설정되지 않았습니다.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("DB 연결 성공!");
    Ok(pool)
}

fn screen_config() -> WindowBuilder {
    let window_attrs = WindowBuilder::new()
        .with_always_on_top(false)
        .with_title("dxnote")
        .with_visible(true)
        .with_focused(true); // 포커스 동작 보정 1순위 정적

    window_attrs
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


#[component]
fn Note() -> Element {
    let mut text_value = use_signal(|| String::new());
    let pool = use_context::<sqlx::PgPool>();
    let mut current_note_id = use_signal(|| None::<i64>);
    
    let list_pool = pool.clone();
    let save_pool = pool.clone();

    // 리스트 리소스 (변수명 앞의 _ 제거하여 나중에 restart 호출 가능하게 함)
    let mut list_resource = use_resource(move || {
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

    let _save_resource = use_resource(move || {
        let current_text = text_value.read().clone();
        let pool_cloned = save_pool.clone();
        // ID 시그널 자체를 넘겨서 내부에서 최신 값을 읽게 함
        let mut id_state = current_note_id;

        async move {
            if current_text.is_empty() { return; }
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            // 잠에서 깨어난 직후에 '최신' ID를 읽음 (매우 중요!)
            let current_id = *id_state.read();

            if let Some(existing_id) = current_id {
                if let Ok(_) = update_data(existing_id, current_text, pool_cloned).await {
                    println!("업데이트 성공");
                    list_resource.restart(); // 리스트 갱신
                }
            } else {
                if let Ok(new_id) = insert_data(current_text, pool_cloned).await {
                    println!("최초 저장 성공");
                    id_state.set(Some(new_id));
                    list_resource.restart(); // 리스트 갱신
                }
            }
        }
    });

    // rsx! 앞에 아무것도 붙이지 않고 마지막 줄에 배치
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-container",
            div { class: "sidebar",
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
                                    text_value.set(note.content.clone());
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
            div { class: "main-content",
                textarea {
                    class: "textarea",
                    value: "{text_value}",
                    oninput: move |event| {
                        text_value.set(event.value());
                    },
                }
            }
        }
    } // <--- 여기에 세미콜론(;)이 절대 없어야 합니다!

    // return rsx! {
    //     document::Link { rel: "stylesheet", href: MAIN_CSS }
    //     div { 
    //         class: "app-container",
    //         div { class: "sidebar",
    //             for note in list_resource.value().cloned().unwrap_or_default() {
    //                 div { 
    //                     class: "note-item",
    //                     onclick: move |_| {
    //                         text_value.set(note.content.clone());
    //                         current_note_id.set(Some(note.id));
    //                     },
    //                     // take(20)을 제거했습니다.
    //                     b { "{note.content.lines().next().unwrap_or(\"Empty\")}" }
    //                     p { 
    //                         style: "font-size: 0.8em; color: gray;",
    //                         "{note.updated_at.with_timezone(&chrono::Local).format(\"%m/%d %H:%M\")}" 
    //                     }
    //                 }
    //             }
    //         }

    //         div { class: "main-content",
    //             textarea {
    //                 class: "textarea",
    //                 value: "{text_value}",
    //                 oninput: move |event| {
    //                     text_value.set(event.value());
    //                 },
    //             }
    //         }
    //     }
    // }
}