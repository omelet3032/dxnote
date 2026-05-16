use dioxus::desktop::WindowBuilder;

pub fn screen_config() -> WindowBuilder {
    let window_attrs:WindowBuilder = WindowBuilder::new()
        .with_always_on_top(false)
        .with_title("dxnote")
        .with_visible(true)
        .with_focused(true);

    window_attrs

}