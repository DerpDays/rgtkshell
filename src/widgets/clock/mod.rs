use std::time::Duration;

use chrono::Local;
use gio::glib;
use gtk4::{prelude::*, Box, Label};

pub fn create() -> Box {
    let container = Box::builder().css_classes(["clock"]).build();

    let label = Label::builder().label(get_time().as_str()).build();
    container.append(&label);

    // Refresh clock every 1/10 seconds.
    glib::timeout_add_local(Duration::from_millis(100), move || {
        let time = get_time();
        label.set_text(time.as_str());
        glib::ControlFlow::Continue
    });

    container
}

fn get_time() -> String {
    format!("{}", Local::now().format("%H:%M:%S"))
}
