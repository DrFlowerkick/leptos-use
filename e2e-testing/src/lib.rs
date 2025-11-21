use leptos::prelude::*;

pub mod app;
pub mod simple_sse;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

pub mod use_broadcast_channel;

#[server]
async fn server_fn_example() -> Result<String, ServerFnError> {
    let version_info = match tokio::fs::read_to_string("/proc/version").await {
        Ok(content) => content.trim().to_string(),
        Err(_) => {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            format!(
                "OS: {}, Arch: {}",
                std::env::consts::OS,
                std::env::consts::ARCH
            )
        }
    };

    Ok(format!("Server System Info: {}", version_info))
}

#[server]
pub async fn trigger_sse_event(message: String) -> Result<(), ServerFnError> {
    use crate::simple_sse::{SimpleSseMsg, SimpleSseState};
    use axum::Extension;
    use leptos_axum::extract;

    // 1. extract state from Axum context
    let Extension(state) = extract::<Extension<SimpleSseState>>().await?;

    // 2. use the state to publish an event
    let event = SimpleSseMsg::new().data(message).build();

    state.publish(event);

    Ok(())
}