mod imp;

use super::workspace_btn::WorkspaceButton;
use crate::events::WORKSPACE_CHANGED;
use glib::Object;
use gtk4::glib;
use gtk4::prelude::{BoxExt, WidgetExt};
use hyprland::{data, shared::HyprData};
use tracing::{span, trace, Level};

glib::wrapper! {
    pub struct Workspaces(ObjectSubclass<imp::Workspaces>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl Workspaces {
    pub fn new() -> Self {
        let span = span!(Level::DEBUG, "workspaces");
        let _ = span.enter();
        trace!("building the workspace widget");
        let container: Workspaces = Object::builder().build();
        trace!("adding the css");
        container.add_css_class("workspaces");
        // container.set_vexpand(false);
        // container.set_vexpand_set(false);
        trace!("adding the buttons");
        let buttons = container.add_buttons();
        trace!("setting up the signals");
        container.setup_signals(buttons);
        container
    }

    pub fn setup_signals(&self, buttons: Vec<WorkspaceButton>) {
        glib::spawn_future_local(async move {
            let rx = &WORKSPACE_CHANGED.reciever;
            loop {
                let mut rx = rx.lock().await;
                if rx.changed().await.is_err() {
                    break;
                }
                if *rx.borrow_and_update() {
                    Self::update_state(&buttons)
                };
            }
        });
    }

    fn add_buttons(&self) -> Vec<WorkspaceButton> {
        let mut buttons: Vec<WorkspaceButton> = vec![];
        for ws in crate::config::WORKSPACES.iter() {
            let workspace_button = WorkspaceButton::new(ws);
            self.append(&workspace_button);
            buttons.push(workspace_button);
        }
        Self::update_state(&buttons);
        buttons
    }
    fn update_state(buttons: &[WorkspaceButton]) {
        trace!("updating the state of the buttons");
        let monitors = data::Monitors::get().expect("unable to get workspaces");
        for btn in buttons.iter() {
            let active = monitors.iter().any(|monitor| {
                let ws = btn.workspace();
                let workspace: &str = if let Some(val) = ws.strip_prefix("name:") {
                    val
                } else if let Some(val) = ws.strip_prefix("special:") {
                    val
                } else {
                    &ws
                };
                monitor.active_workspace.name == workspace
            });
            btn.set_active(active);
        }
        trace!("done updating the state of the buttons");
    }
}

impl Default for Workspaces {
    fn default() -> Self {
        Self::new()
    }
}

// OLD LISTENER CODE
// let (sender, receiver) = async_channel::bounded::<()>(1);

// let mut event_listener = EventListener::new();
// event_listener.add_workspace_change_handler(move |_| {
//     let span = span!(Level::TRACE, "workspace changed event");
//     let _ = span.enter();
//     trace!("recieved workspace_change event");
//     sender
//         .send_blocking(())
//         .expect("The channel needs to be open.");
//     trace!("sent event to channel");
// });
// runtime().spawn(async move {
//     let _ = event_listener
//         .start_listener_async()
//         .await
//         .expect("failed to start hyprland event_listener");
// });
// glib::spawn_future_local(async move {
//     while let Ok(_response) = receiver.recv().await {
//         trace!("recieved, updating buttons");
//         Self::update_state(&buttons);
//         trace!("done updating buttons");
//     }
// });
