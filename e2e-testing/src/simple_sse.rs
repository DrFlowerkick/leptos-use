#![cfg(feature = "ssr")]

use axum::response::sse::Event;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Clone)]
pub struct SimpleSseState {
    pub tx: broadcast::Sender<Event>,
}

impl SimpleSseState {
    pub fn new(buffer: usize) -> Self {
        let (tx, _rx) = broadcast::channel(buffer);
        SimpleSseState { tx }
    }
    pub fn publish(&self, event: Event) {
        let _ = self.tx.send(event);
    }
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }
}

pub struct SseComment {}
pub struct SseMsg {}

#[derive(Clone, Debug)]
pub struct SimpleSseMsg<S> {
    state: S,
    event_name: Option<String>,
    event_id: Option<String>,
    data: Option<String>,
    comment: Option<String>,
    retry: Option<Duration>,
}

impl SimpleSseMsg<()> {
    pub fn new() -> Self {
        SimpleSseMsg {
            state: (),
            event_name: None,
            event_id: None,
            data: None,
            comment: None,
            retry: None,
        }
    }
    pub fn comment(self, comment: impl Into<String>) -> SimpleSseMsg<SseComment> {
        SimpleSseMsg {
            state: SseComment {},
            event_name: None,
            event_id: None,
            data: None,
            comment: Some(comment.into()),
            retry: None,
        }
    }
    pub fn data(self, data: impl Into<String>) -> SimpleSseMsg<SseMsg> {
        SimpleSseMsg {
            state: SseMsg {},
            event_name: None,
            event_id: None,
            data: Some(data.into()),
            comment: None,
            retry: None,
        }
    }
    pub fn event_name(self, event_name: impl Into<String>) -> SimpleSseMsg<SseMsg> {
        SimpleSseMsg {
            state: SseMsg {},
            event_name: Some(event_name.into()),
            event_id: None,
            data: None,
            comment: None,
            retry: None,
        }
    }
}

impl SimpleSseMsg<SseComment> {
    pub fn build(self) -> Event {
        let mut event = Event::default();
        if let Some(comment) = self.comment {
            event = event.comment(comment);
        }
        event
    }
}

impl SimpleSseMsg<SseMsg> {
    pub fn data(self, data: impl Into<String>) -> Self {
        SimpleSseMsg {
            state: self.state,
            event_name: self.event_name,
            event_id: self.event_id,
            data: Some(data.into()),
            comment: None,
            retry: self.retry,
        }
    }
    pub fn event_name(self, event_name: impl Into<String>) -> Self {
        SimpleSseMsg {
            state: self.state,
            event_name: Some(event_name.into()),
            event_id: self.event_id,
            data: self.data,
            comment: None,
            retry: self.retry,
        }
    }
    pub fn event_id(self, event_id: impl Into<String>) -> Self {
        SimpleSseMsg {
            state: self.state,
            event_name: self.event_name,
            event_id: Some(event_id.into()),
            data: self.data,
            comment: None,
            retry: self.retry,
        }
    }
    pub fn retry(self, retry: Duration) -> Self {
        SimpleSseMsg {
            state: self.state,
            event_name: self.event_name,
            event_id: self.event_id,
            data: self.data,
            comment: None,
            retry: Some(retry),
        }
    }
    pub fn build(self) -> Event {
        let mut event = Event::default();
        if let Some(name) = self.event_name {
            event = event.event(name);
        }
        if let Some(id) = self.event_id {
            event = event.id(id);
        }
        if let Some(data) = self.data {
            event = event.data(data);
        }
        if let Some(retry) = self.retry {
            event = event.retry(retry);
        }
        event
    }
}

pub async fn api_sse_subscribe(
    state: axum::extract::Extension<SimpleSseState>,
) -> impl axum::response::IntoResponse {
    let rx = state.subscribe();

    let stream = BroadcastStream::new(rx);

    axum::response::sse::Sse::new(stream)
}
