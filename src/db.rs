use dotenvy::dotenv;
use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions};
use chrono::Utc;

use crate::note::NoteSummary;

/// .env 파일에 작성된 환경 변수를 읽어 PostgreSQL 커넥션 풀을 생성합니다.
/// 
/// DB 접속 실패시 sqlx::Error를 반환합니다.
pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    
    // 1. .env 파일로부터 환경 변수를 로드합니다.
    dotenv().ok(); 

    // 2 .env에 작성된 DATABASE_URL을 읽어오며, 없을시 에러 메시지와 함께 종료됩니다.
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL이 설정되지 않았습니다.");

    // 3. 비동기 방식으로 DB 커넥션 풀을 생성합니다.
    //    애플리케이션이 유지할 최대 커넥션 풀(DB 연결)은 5개 입니다. (max_connections)
    let pool:Pool<Postgres> = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;

    println!("DB 연결 성공");
    // 4. 커넥션 풀을 반환합니다. 
    // Result<PgPool, sqlx::Error>
    Ok(pool)
}

/// 사용자가 작성한 내용을 DB에 비동기적으로 저장합니다.
/// 성공시 id를 반환하며 실패시 sqlx::Error를 반환합니다.
/// 
pub async fn insert_data(note_content:&str, pool:PgPool) -> Result<i64, sqlx::Error> {

    // 1. 사용자가 입력한 내용과 시간을 db에 저장합니다.
    // RETURNING id 구문을 통해 삽입된 행의 id를 담은 레코드를 fetch_one() 메서드를 통해 가져옵니다.
    let result = sqlx::query!(
        r#"
        INSERT INTO notes (content, created_at, updated_at)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        note_content,
        Utc::now(),
        Utc::now()
    ).fetch_one(&pool).await?;

    println!("아이디 : {}", result.id);
    println!("데이터 저장 성공");
    // 2. id값을 반환합니다.
    Ok(result.id)

}

/// 사용자가 수정한 내용을 비동기적으로 DB에 저장합니다.
pub async fn update_data(id:i64, note_content:&str, pool:PgPool) -> Result<(), sqlx::Error> {

    // CASE WHEN ~ THEN ~ 문법을 통해 수정된 내용이 있을시에만 UPDATE 구문이 실행됩니다. 
    sqlx::query!(
        r#"
        UPDATE notes
        SET content = $1,
        updated_at = CASE WHEN content <> $1 THEN NOW() ELSE updated_at END
        WHERE id = $2
        "#,
        note_content,
        id
    ).execute(&pool).await?; // 반환받는 값이 없으므로 execute() 메서드를 사용합니다.

    println!("ID {} 업데이트 성공", id);
    Ok(())
}

pub async fn fetch_note_list(pool:&PgPool) -> Vec<NoteSummary> {
            
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
            .fetch_all(pool)
            .await
            .unwrap_or_default()

}