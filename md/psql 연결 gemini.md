드디어 본격적인 Rust 백엔드 개발 단계로 들어오셨네요! Rust에서 PostgreSQL을 다룰 때는 **`sqlx`** 라이브러리를 사용하는 것이 현재 표준이자 가장 권장되는 방식입니다.

`sqlx`는 컴파일 타임에 SQL 문법 검사를 해주기 때문에 에러를 미리 잡을 수 있다는 강력한 장점이 있습니다.

---

## 1. 프로젝트 설정 (`Cargo.toml`)

먼저 비동기 런타임인 `tokio`와 데이터베이스 드라이버인 `sqlx`가 필요합니다.

```toml
[dependencies]
# 비동기 실행을 위해 필요합니다.
tokio = { version = "1", features = ["full"] }
# Postgres 기능을 켜줍니다.
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "chrono"] }
# 시간을 다루기 위해 추가합니다.
chrono = { version = "0.4", features = ["serde"] }
```

---

## 2. Rust에서 INSERT 실행하기

가장 기본적인 형태의 코드를 작성해 보겠습니다. `dxnote` 프로젝트의 `main.rs`나 별도의 테스트 파일에서 연습해 보세요.

```rust
use sqlx::postgres::PgPoolOptions;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 1. 연결 문자열 설정 (아까 확인한 WireGuard IP를 사용하세요!)
    let database_url = "postgres://postgres:password@10.x.x.x:5432/dxnote";

    // 2. 데이터베이스 커넥션 풀 생성
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("DB 연결 성공!");

    // 3. INSERT 쿼리 날리기
    let note_content = "Rust에서 보낸 첫 번째 메모입니다.";
    
    // sqlx::query! 매크로를 사용하면 컴파일 타임에 SQL 검사를 해줍니다.
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

    println!("성공적으로 저장되었습니다. 생성된 ID: {}", result.id);

    Ok(())
}
```

---

## 💡 핵심 포인트 설명

### 1. `$1, $2, $3` (매개변수 바인딩)
MySQL에서는 `?`를 썼겠지만, PostgreSQL은 **`$1`, `$2`** 처럼 숫자가 붙은 달러 기호를 사용합니다. 이는 SQL Injection 공격을 방지하는 안전한 방식입니다.

### 2. `sqlx::query!` 매크로
* `query!` 뒤에 느낌표가 붙은 것은 매크로입니다. 
* 실제 DB에 연결되어 있다면 컴파일할 때 테이블 명이나 컬럼 명이 맞는지 미리 체크해 줍니다. (이 기능을 쓰려면 `.env` 파일에 `DATABASE_URL`이 설정되어 있어야 합니다.)

### 3. `RETURNING id`
아까 `psql`에서 연습한 것처럼, 데이터를 넣자마자 방금 생긴 PK(`id`)를 가져올 수 있어 매우 편리합니다.

---

## 🛠️ 실습 과제
현재 작성 중인 `dxnote` 코드의 `Button` 컴포넌트 안에서 이 로직을 실행하려면, 먼저 `App` 레벨에서 `PgPool`을 생성하고 이를 **`use_context_provider`**로 하위 컴포넌트들에 공유해주는 작업이 필요합니다.



일단 위 코드를 별도의 `examples/db_test.rs` 같은 파일에 넣고 `cargo run`을 해서 실제로 DB에 행이 추가되는지 확인해 보시겠어요? 

연결 주소(IP)와 비밀번호가 `docker-compose` 설정과 일치하는지 다시 한번 꼭 확인해 주세요! 혹시 에러가 발생한다면 어떤 메시지가 뜨나요?