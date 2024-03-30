use gio::prelude::*;
use gtk4::{gdk::Display, CssProvider};
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tracing::{span, trace, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::events::compositor_listeners;

mod assets;
mod backends;
mod config;
mod events;
mod widgets;
mod windows;

static APP_NAME: &str = "sh.derpdays.rgtk";

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

fn main() {
    // Initialise logging.
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .pretty();
    // Allow passing an env variable to set the log level.
    let filter_layer = tracing_subscriber::filter::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::filter::EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    trace!("creating the application");
    let application = gtk4::Application::new(Some(APP_NAME), Default::default());

    application.connect_startup(|_| {
        load_css();
    });

    application.connect_activate(|app| {
        let span = span!(Level::TRACE, "activation");
        let _ = span.enter();
        trace!("creating the bar");
        windows::bar::Bar::create(app);
        trace!("done creating the bar");
    });

    // Start listening for compositor events.
    compositor_listeners();

    trace!("running the application");
    application.run();
}

fn load_css() {
    let span = span!(Level::TRACE, "css");
    let _ = span.enter();
    trace!("loading css");
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("./styles/output.css"));

    // Add the provider to the default screen
    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    trace!("done loading css");
}
