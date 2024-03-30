use std::time::Duration;

use battery::{Manager, State};
use gio::glib;
use gtk4::{prelude::*, Box, Image, Label};

pub fn create() -> Box {
    gtk4::Settings::default()
        .unwrap()
        .set_gtk_icon_theme_name(Some("Fluent"));
    let container = Box::builder().css_classes(["battery"]).build();
    container.set_spacing(10);

    let (state, charge) = get_charge();
    let label = Label::builder()
        .label(format!("{}%", charge as i32))
        .build();
    let icon = Image::from_icon_name(&icon_name(&state, &charge));
    icon.set_pixel_size(25);

    container.append(&label);
    container.append(&icon);

    // Refresh clock every 1/10 seconds.
    glib::timeout_add_local(Duration::from_millis(100), move || {
        let (state, charge) = get_charge();
        label.set_text(&format!("{}%", charge as i32));
        icon.set_icon_name(Some(&icon_name(&state, &charge)));
        glib::ControlFlow::Continue
    });

    container
}

fn get_charge() -> (State, f32) {
    let manager = Manager::new().unwrap();
    if let Some(battery) = manager.batteries().unwrap().next() {
        let battery = battery.unwrap();
        return (
            battery.state(),
            battery
                .state_of_charge()
                .get::<battery::units::ratio::percent>(),
        );
    }
    unreachable!("no batteries found");
}

fn icon_name(state: &State, charge: &f32) -> String {
    let icon_name: &str = match charge {
        0.0..=5.0 => "battery-empty",
        5.0..=10.0 => "battery-caution",
        10.0..=20.0 => "battery-level-10",
        20.0..=30.0 => "battery-level-20",
        30.0..=40.0 => "battery-level-30",
        40.0..=50.0 => "battery-level-50", // no level 40
        50.0..=60.0 => "battery-level-50",
        60.0..=70.0 => "battery-level-70", // no level 60
        70.0..=80.0 => "battery-level-70",
        80.0..=100.0 => "battery-full",
        _ => "battery-action",
    };
    match state {
        State::Charging => format!("{}-charging", icon_name),
        _ => icon_name.to_string(),
    }
}
