use hyprland::event_listener::EventListener;
use tracing::{debug, span, trace, Level};

use crate::events::WORKSPACE_CHANGED;

pub async fn listen_to_events() {
    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |_| {
        let span = span!(Level::TRACE, "workspace changed event");
        let _ = span.enter();
        trace!("recieved workspace_change event");
        WORKSPACE_CHANGED
            .sender
            .send(true)
            .expect("The channel needs to be open.");
        trace!("sent event to channel");
    });

    debug!("starting hyprland event listener");
    let _ = event_listener.start_listener_async().await;
}

// TODO: actually make the thing compositor agnostic.
// pub fn get_current_workspaces() -> () {}
