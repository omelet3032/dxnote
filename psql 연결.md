좋다 😎
**A 루트 + GUI 없이 CLI로 간다** → 이거 진짜 개발자 루트야.

DB는 PostgreSQL 기준으로 진행할게.

---

# 🎯 전체 목표

1️⃣ psql로 테이블 생성
2️⃣ Rust에서 연결
3️⃣ INSERT
4️⃣ SELECT
5️⃣ UPDATE
6️⃣ DELETE

---

# ✅ 1️⃣ 먼저 DB 접속 (CLI)

서버에서:

```bash
sudo docker exec -it practice-postgres psql -U user -d memo
```

성공하면 이런 프롬프트 나옴:

```
memo=#
```

---

# ✅ 2️⃣ 테이블 생성

CLI 안에서:

```sql
CREATE TABLE notes (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

확인:

```sql
\dt
```

---

# ✅ 3️⃣ Rust 연결 준비

Cargo.toml:

```toml
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
tokio = { version = "1", features = ["full"] }
dotenvy = "0.15"
```

---

# ✅ 4️⃣ .env 파일

```env
DATABASE_URL=postgres://user:password@localhost:5432/memo
```

---

# ✅ 5️⃣ Rust 기본 연결 코드

```rust
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await?;

    println!("Connected to DB");

    Ok(())
}
```

실행:

```bash
cargo run
```

---

# ✅ 6️⃣ INSERT 추가

```rust
sqlx::query("INSERT INTO notes (content) VALUES ($1)")
    .bind("first memo")
    .execute(&pool)
    .await?;
```

---

# ✅ 7️⃣ SELECT 조회

```rust
let rows = sqlx::query!("SELECT id, content FROM notes")
    .fetch_all(&pool)
    .await?;

for row in rows {
    println!("{}: {}", row.id, row.content);
}
```

---

# 🔥 이제 진짜 중요 포인트

CLI에서 직접 확인해봐:

```sql
SELECT * FROM notes;
```

👉 Rust → DB → CLI 확인
이 루프가 개발 기본 사이클임.

---

# 🚀 다음 단계 (A 루트 유지)

이제 할 것:

* UPDATE 구현
* DELETE 구현
* Rust 구조 정리 (repository 패턴)
* 에러 처리 개선

---

지금 바로 해볼 건 뭐부터?

1️⃣ 연결 테스트
2️⃣ insert/select까지 한 번에
3️⃣ 구조 설계 같이 정리

골라봐 😎
