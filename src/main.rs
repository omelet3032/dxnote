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
async fn insert_data(note_content:String, pool: PgPool) -> Result<(), sqlx::Error> {

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

    rsx! {
        Note {}
    }
}

#[component]
fn Note() -> Element {
    let mut text_value = use_signal(|| String::new());
    
    let pool = use_context::<sqlx::PgPool>();

    // use_resource는 의존성(text_value)이 변할 때마다 클로저를 다시 실행합니다.
   /*  let _save_resource = use_resource(move || {
        let current_text = text_value.read().clone();
        let pool_cloned = pool.clone();

        async move {
            if current_text.is_empty() { return; }

            // 1. 디바운스: 700ms 동안 대기 (사용자가 타이핑을 멈출 때까지 기다림)
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            // 2. 실제 DB 저장
            match insert_data(current_text, pool_cloned).await {
                Ok(_) => println!("실시간 자동 저장 성공"),
                Err(e) => eprintln!("자동 저장 실패: {:?}", e),
            }
        }
    }); */

    let _save_resource = use_resource(move || {
        let current_text = text_value.read().clone();
        let pool_cloned = pool.clone();

        async move {
            if current_text.is_empty() {
                return;
            }

            // 1. 디바운스: 700ms동안 대기 (사용자가 타이핑을 멈출때까지 기다림)
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            // 2. 실제 DB 저장
            match insert_data(current_text, pool_cloned).await {
                Ok(_) => println!("실시간 자동 저장 성공"),
                Err(e) => eprintln!("자동 저장 실패: {:?}", e),
            }
        }
    });

/*     let on_save = move |_| {
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
 */
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS,}
        div {
            textarea {
                class: "textarea",
                oninput: move |event| {
                text_value.set(event.value());
                },
            }
            // div {
            //     class: "save-button-container",
            //     button {
            //         onclick: on_save,
            //         "save"
            //     }
            // }
        }
    }
}

