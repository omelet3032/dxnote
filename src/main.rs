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

#[component]
fn Note() -> Element {
    let mut text_value = use_signal(|| String::new());
    
    let pool = use_context::<sqlx::PgPool>();
    let current_note_id = use_signal(|| None::<i64>);

    let mut id_state = current_note_id;
    let current_id_opt = *id_state.read();
    
    let _save_resource = use_resource(move || {
        let current_text = text_value.read().clone();
        let pool_cloned = pool.clone();

        async move {
            if current_text.is_empty() {
                return;
            }

            // 1. 디바운스: 700ms동안 대기 (사용자가 타이핑을 멈출때까지 기다림)
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            // 3. ID가 이미 있는지 확인
            if let Some(existing_id) = current_id_opt {
                // 이미 ID가 있다면 UPDATE 실행
                match update_data(existing_id, current_text, pool_cloned).await {
                    Ok(_) => println!("업데이트 성공 (ID: {})", existing_id),
                    Err(e) => eprintln!("업데이트 실패: {:?}", e),
                }
            } else {
                // ID가 없다면 새로 INSERT
                if let Ok(new_id) = insert_data(current_text, pool_cloned).await {
                    println!("최초 저장 성공 (ID: {})", new_id);
                    // 4. 여기서 추출된 new_id를 Signal에 저장!
                    id_state.set(Some(new_id));
                }
            }
        }
    });

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

