use dioxus::core::Task;
use dioxus::prelude::*;
use tokio::task::JoinHandle;
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

// pub fn use_auto_save(text_value: Signal<String>, original_content: Signal<String>, current_note_id: Signal<Option<i64>>, mut list_resource: Resource<Vec<NoteSummary>>) {
pub fn use_auto_save(text_value: Signal<String>, original_content: Signal<String>, current_note_id: Signal<Option<i64>>, mut list_resource: Resource<Vec<NoteSummary>>, mut save_task: Signal<Option<Task>>) {
   
    let pool = use_context::<sqlx::PgPool>();
    

    // use_effect(move || {
        let current_text = text_value.read().clone();
        // let old_text = original_content.peek().clone();
        let old_text = original_content.read().clone();

        // 변경 없으면 실행 안함
        if current_text.is_empty() || current_text == old_text {
            return;
        }

        if let Some(task) = save_task.write().take() {
            task.cancel();
        }


        let pool_cloned = pool.clone();

        let mut id_state = current_note_id;
        let mut original_state = original_content;
        // let mut list_resource = list_resource;

        let current_id = *id_state.read();
        // 비동기 작업 실행
        let handle = spawn(async move {
            // debounce
            tokio::time::sleep(std::time::Duration::from_millis(700)).await;

            let saved = match current_id {
                Some(id) => {
                    update_data(id, &current_text, pool_cloned).await.is_ok()
                },
                None => { 
                    match insert_data(&current_text, pool_cloned).await {
                        Ok(new_id) => {
                            id_state.set(Some(new_id));
                            true
                        }, 
                        Err(_) => {
                            false
                        }
                    }
                }
            };

            if saved {
                original_state.set(current_text);
                list_resource.restart();
            }
        });

        save_task.set(Some(handle));
    // });

}