use dxnote::db::connect_db;
use dxnote::config::screen_config;
use dxnote::components::Note;

use dioxus::desktop::Config;
use dioxus::prelude::*;


#[tokio::main]
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


#[component]
fn App() -> Element {

    rsx! {

        Note {}
    }
}

