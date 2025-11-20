use codee::string::FromToStringCodec;
use leptos::{prelude::*, web_sys};
use leptos_use::{use_broadcast_channel_old::{use_broadcast_channel, UseBroadcastChannelReturn}};

/// E2E test component for use_broadcast_channel
#[component]
pub fn BroadcastChannelDemo() -> impl IntoView {
    let UseBroadcastChannelReturn {
        is_supported,
        message,
        post,
        error,
        ..
    } = use_broadcast_channel::<String, FromToStringCodec>("leptos-use-e2e-testing-channel");

    let (input_value, set_input_value) = signal(String::new());

    let (is_mounted, set_mounted) = signal(false);
    Effect::new(move |_| set_mounted.set(true));

    let server_answer = Resource::new(
        move || message.get(),
        move |maybe_msg| async move {
            if let Some(msg) = maybe_msg {
                format!("Message received: {}, server answers: {}", msg, 
        super::server_fn_example().await.unwrap_or_else(|_| "Server error".to_string()))
            } else {
                "No message sent yet".to_string()
            }
        }
);

    let error_msg = move || error.with(|e| {
        e.as_ref()
            .map(|err| format!("{:?}", err))
            .unwrap_or_else(|| "No error".to_string())
    });

    view! {
        <h2>Broadcast Channel E2E Test</h2>
        <p>Please open this page in at least two tabs</p>
        <Transition fallback=move || view! { <p>"Checking for BroadcastChannel support..."</p> }>
            <p>"Server answer: " {move || server_answer.get().unwrap_or_else(|| "Loading...".to_string())}</p>

            <Show
                when=move || is_supported.get() && is_mounted.get()
                fallback=move || view! { <p>"BroadcastChannel not supported"</p> }
            >
                <form on:submit={
                    let post = post.clone();
                    move |ev: web_sys::SubmitEvent| {
                        ev.prevent_default();
                        let value = input_value.get();
                        post(&value);
                    }
                }>
                    <input
                        value=input_value
                        on:input=move |event| {
                            set_input_value.set(event_target_value(&event));
                        }
                        type="text"
                    />
                    <button type="submit">Send Message</button>
                </form>
                <Show when=move || message.get().is_some()>
                    <p>"Received message: " {move || message.get().as_ref().unwrap().to_string()}</p>
                </Show>
                <Show when=move || error.with(|e| e.is_some())>
                    <p>"Error: " {error_msg()}</p>
                </Show>
            </Show>
        </Transition>
    }
}
