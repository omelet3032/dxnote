use chrono::Utc;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> Result<(), sqlx::Error> {
    // 3. INSERT 쿼리 날리기
    let note_content = "Rust에서 보낸 첫 번째 메모입니다.";
    // let pool = connect_db().await;

    if let Ok(pool) = connect_db().await {
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
    
        println!("아이디 : {}",  result.id);
        println!("데이터 저장 성공");
    }

    // sqlx::query! 매크로를 사용하면 컴파일 타임에 SQL 검사를 해줍니다.
    // let result = sqlx::query!(
    //     r#"
    //     INSERT INTO notes (content, created_at, updated_at)
    //     VALUES ($1, $2, $3)
    //     RETURNING id
    //     "#,
    //     note_content,
    //     Utc::now(), // 현재 시간
    //     Utc::now()
    // )
    // .fetch_one(&pool) // 방금 넣은 ID를 가져오기 위해 fetch_one 사용
    // .await?;

    // println!("성공적으로 저장되었습니다. 생성된 ID: {}", result.id);

    screen_config();
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

fn screen_config() {
    let window_attrs = WindowBuilder::new()
        .with_always_on_top(false)
        .with_title("dxnote")
        .with_visible(true)
        .with_focused(true); // 포커스 동작 보정 1순위 정적

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window_attrs))
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

    }
}
