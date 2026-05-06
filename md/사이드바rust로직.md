실시간 자동 저장 로직까지 성공적으로 구현하셨네요! 이제 앱의 형태가 단순한 연습장을 넘어 실제 **노트 서비스**의 모습으로 진화할 차례입니다.

아이폰 메모앱처럼 사이드바에 메모 리스트를 띄우려면 크게 세 가지가 필요합니다.
1.  **데이터 구조:** 모든 메모를 담을 `Vector` 상태
2.  **데이터 조회:** DB에서 `updated_at` 기준 내림차순으로 가져오기
3.  **UI 레이아웃:** 사이드바와 메인 입력창 분리

---

## 1. 전역 상태 및 데이터 불러오기 로직

먼저 모든 메모 리스트를 관리할 `Signal`과, 이를 DB에서 동기화할 `use_resource`가 필요합니다.

```rust
// 메모 하나를 표현할 구조체 (sqlx 결과와 매칭)
#[derive(Clone, PartialEq, Debug)]
struct Note Summary {
    id: i64,
    content: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[component]
fn Note() -> Element {
    let mut text_value = use_signal(|| String::new());
    let mut current_note_id = use_signal(|| None::<i64>);
    // 1. 전체 메모 리스트를 저장할 상태
    let mut notes_list = use_signal(|| Vec::<NoteSummary>::new());
    
    let pool = use_context::<sqlx::PgPool>();

    // 2. 앱 실행 시 및 저장 시 리스트를 새로고침하는 리소스
    let _list_resource = use_resource(move || {
        let pool = pool.clone();
        async move {
            let rows = sqlx::query_as!(
                NoteSummary,
                "SELECT id, content, updated_at FROM notes ORDER BY updated_at DESC"
            )
            .fetch_all(&pool)
            .await;
            
            if let Ok(data) = rows {
                data
            } else {
                vec![]
            }
        }
    });

    // ... 생략 (저장 로직은 그대로 유지)
}
```

---

## 2. 레이아웃 설계 (CSS Flexbox 추천)

사이드바를 배치하려면 `main.css`에서 화면을 좌우로 나누어야 합니다.

```css
/* assets/main.css 예시 */
.app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
}

.sidebar {
    width: 250px;
    border-right: 1px solid #ddd;
    overflow-y: auto;
    background: #f9f9f9;
}

.main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
}

.note-item {
    padding: 15px;
    border-bottom: 1px solid #eee;
    cursor: pointer;
}

.note-item:hover { background: #e0e0e0; }
```

---

## 3. UI 구현 (RSX)

사용자가 리스트의 항목을 클릭했을 때, 해당 메모의 내용이 `textarea`에 나타나도록 연결합니다.

```rust
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "app-container",
            // 사이드바: 메모 리스트
            div { class: "sidebar",
                for note in _list_resource.value().cloned().unwrap_or_default() {
                    div { 
                        class: "note-item",
                        onclick: move |_| {
                            // 리스트 클릭 시 해당 내용으로 전환
                            text_value.set(note.content.clone());
                            current_note_id.set(Some(note.id));
                        },
                        // 제목은 첫 줄만 보여주기
                        b { "{note.content.lines().next().unwrap_or(\"Empty\").take(20)}" }
                        p { 
                            style: "font-size: 0.8em; color: gray;",
                            "{note.updated_at.with_timezone(&chrono::Local).format(\"%m/%d %H:%M\")}" 
                        }
                    }
                }
            }

            // 메인 편집 영역
            div { class: "main-content",
                textarea {
                    class: "textarea",
                    value: "{text_value}", // 중요: value를 text_value와 연결
                    oninput: move |event| {
                        text_value.set(event.value());
                    },
                }
            }
        }
    }
```

---

## 4. 주의사항 및 팁

1.  **`value: "{text_value}"`**: 리스트를 클릭했을 때 `textarea`의 내용이 바뀌게 하려면, `textarea`에 `value` 속성을 명시적으로 할당해야 합니다.
2.  **리스트 새로고침**: 현재는 `insert_data`가 성공해도 왼쪽 리스트가 즉시 변하지 않습니다. `_list_resource.restart()`를 호출하여 저장이 끝날 때마다 리스트를 갱신하게 만들면 좋습니다.
3.  **데이터 요약**: 리스트에는 전체 내용을 다 보여주기보다 `lines().next()`를 사용해 첫 줄(제목)만 보여주는 것이 깔끔합니다.



### **다음 구현 포인트**
*   **새 메모 추가 버튼:** `current_note_id`를 `None`으로 만들고 `text_value`를 비우는 버튼을 상단에 추가해 보세요.
*   **메모 삭제 기능:** 리스트 각 항목 옆에 작은 'X' 버튼을 만들어 `DELETE` 쿼리를 날릴 수 있습니다.

사이드바 레이아웃까지 잡히면 이제 정말 "앱" 같아질 거예요! 이 구조로 한 번 코드를 옮겨보시겠어요?