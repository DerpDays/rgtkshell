// use hyprland::{
//     event_listener::{
//         EventListener, LayoutEvent, MinimizeEventData, MonitorEventData, ScreencastEventData,
//         WindowEventData, WindowFloatEventData, WindowMoveEvent, WindowOpenEvent,
//         WorkspaceRenameEventData,
//     },
//     shared::{Address, WorkspaceType},
// };
use lazy_static::lazy_static;
use tokio::sync::{
    watch::{self, Receiver, Sender},
    Mutex,
};

pub struct Channel<T> {
    pub sender: Sender<T>,
    pub reciever: Mutex<Receiver<T>>,
}

lazy_static! {
    pub static ref WORKSPACE_CHANGED: Channel<bool> = {
        let (tx, rx) = watch::channel::<bool>(false);
        Channel {
            sender: tx,
            reciever: Mutex::new(rx),
        }
    };
}

pub(crate) fn compositor_listeners() {
    crate::runtime().spawn(async move {
        #[cfg(feature = "hyprland")]
        crate::backends::hyprland::listen_to_events().await;
    });
}

// #[derive(Debug)]
// pub enum HyprlandEvent {
//     ListenerInitialised,
//     // Actual hyprland events.
//     WorkspaceChangedEvents(WorkspaceType),
//     WorkspaceAddedEvents(WorkspaceType),
//     WorkspaceDestroyedEvents(WorkspaceType),
//     WorkspaceMovedEvents(MonitorEventData),
//     WorkspaceRenameEvents(WorkspaceRenameEventData),
//     ActiveMonitorChangedEvents(MonitorEventData),
//     WindowChangedEvents(Option<WindowEventData>),
//     FullscreenStateChangedEvents(bool),
//     MonitorRemovedEvents(String),
//     MonitorAddedEvents(String),
//     KeyboardLayoutChangeEvents(LayoutEvent),
//     SubMapChangedEvents(String),
//     WindowOpenEvents(WindowOpenEvent),
//     WindowCloseEvents(Address),
//     WindowMovedEvents(WindowMoveEvent),
//     LayerOpenEvents(String),
//     LayerClosedEvents(String),
//     FloatStateEvents(WindowFloatEventData),
//     UrgentStateEvents(Address),
//     MinimizeEvents(MinimizeEventData),
//     WindowTitleChangedEvents(Address),
//     ScreencastEvents(ScreencastEventData),
// }
