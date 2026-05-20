use dioxus::prelude::*;
use crate::note::NoteSummary;
use crate::db::{fetch_note_list, update_data, insert_data};

pub fn use_list_resource () -> Resource<Vec<NoteSummary>> {

   let pool = use_context::<sqlx::PgPool>();

   let list_resource = use_resource(move || {
        let pool_cloned = pool.clone();

        async move {
            fetch_note_list(&pool_cloned).await
        }
    }); 

    list_resource
}

pub fn use_auto_save(text_value: Signal<String>, original_content: Signal<String>, current_note_id: Signal<Option<i64>>) {
   
    let pool = use_context::<sqlx::PgPool>();
    
    use_effect(move || {
        let current_text = text_value.read().clone();
        let old_text = original_content.read().clone();

        // 변경 없으면 실행 안함
        if current_text.is_empty() || current_text == old_text {
            return;
        }

        let pool_cloned = pool.clone();

        let mut id_state = current_note_id;
        let mut original_state = original_content;
        // let mut list_resource = list_resource;

        // 비동기 작업 실행
        spawn(async move {
            // debounce
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            let current_id = *id_state.read();

            match current_id {
                Some(id) => {
                    if update_data(id, &current_text, pool_cloned).await.is_ok() {
                        println!("업데이트 성공");
                        original_state.set(current_text);
                        // list_resource.restart();
                    }
                }
                None => {
                    if let Ok(new_id) = insert_data(&current_text, pool_cloned).await {
                        println!("최초 저장 성공");
                        id_state.set(Some(new_id));
                        original_state.set(current_text);
                        // list_resource.restart();
                    }
                }
            }
        });
    });

}