use chrono::Utc;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool, Pool, Postgres};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> Result<(), sqlx::Error> {
    // 3. INSERT 쿼리 날리기
    // let note_content = "Rust에서 보낸 첫 번째 메모입니다.";

    let pool = connect_db().await.map_err(|e| {
        eprintln!("DB 연동 실패");
        e
    })?;

    // insert_data(note_content, pool.clone()).await.map_err(|e| {
    //     eprintln!("데이터 삽입 실패");
    //     e
    // })?;

    let window_attrs = screen_config();

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window_attrs))
        .with_context(pool)
        .launch(App);

    Ok(())
}

// 연습용 함수
async fn insert_data(note_content:String, pool: PgPool) -> Result<(), sqlx::Error> {
    // let note_content = "Rust에서 보낸 첫 번째 메모입니다.";

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
    // let pool = use_context::<sqlx::PgPool>();

    // let spawn_insert = move |_: MouseEvent| {
    //     spawn(async move {
    //         // 여기서 pool을 사용하여 DB 작업 수행
    //         // insert_data("새 메모", pool).await;
    //         insert_data(pool.clone()).await.map_err(|e| {
    //             eprintln!("데이터 삽입 실패");
    //             e
    //         });
    //     });
    // };

    rsx! {
        Note {}
        // Button {}
    }
}

#[component]
fn Note() -> Element {
    let mut text_value = use_signal(|| String::new());
    let pool = use_context::<sqlx::PgPool>();

    let on_save = move |_| {
        let current_text = text_value.read().clone();
        let pool_cloned = pool.clone();

        spawn(async move {
            if current_text.is_empty() {
                println!("내용이 비어있어 저장하지 않습니다.");
                return;
            }

            // insert_data 함수가 이제 text를 인자로 받도록 수정되어야 합니다.
            match insert_data(current_text, pool_cloned).await {
                Ok(_) => println!("DB 저장 성공!"),
                Err(e) => eprintln!("DB 저장 실패: {:?}", e),
            }
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS,}
        div {
            textarea {
                class: "textarea",
                oninput: move |event| {
                text_value.set(event.value());
                },
            }
            div {
                class: "save-button-container",
                button {
                    onclick: on_save,
                    "save"
                }
            }
        }
    }
}

// #[component]
// fn Button() -> Element {
//     rsx! {
//          div {
//             class: "save-button-container",

//             button {
//                 onclick: move |_| {
//                     println!("click");
//                     /*
//                         1. save를 누르면
//                         2. insert 쿼리가 실행되면서
//                         3.
//                      */
//                 },
//                 "save",
//             }
//          }

//     }
// }
