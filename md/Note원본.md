#[component]
fn Note() -> Element {
    let mut text_value = use_signal(|| String::new());
    
    let pool = use_context::<sqlx::PgPool>();
    let current_note_id = use_signal(|| None::<i64>);

    let mut id_state = current_note_id;
    let current_id_opt = *id_state.read();
    
     // 1. 전체 메모 리스트를 저장할 상태
    let mut notes_list = use_signal(|| Vec::<NoteSummary>::new());

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
        document::Link { 
            rel: "stylesheet", href: MAIN_CSS 
        }
        div { 
            class: "app-container",
            // 사이드바: 메모 리스트
            div { 
                class: "sidebar",
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
            div { 
                class: "main-content",
                
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
}