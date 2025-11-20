use leptos::prelude::*;

pub mod app;

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
            format!("OS: {}, Arch: {}", std::env::consts::OS, std::env::consts::ARCH)
        }
    };

    Ok(format!("Server System Info: {}", version_info))
}