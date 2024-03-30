use gtk4::{prelude::*, Align, Application, ApplicationWindow, Box};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tracing::{span, trace, Level};

use crate::widgets::workspaces::Workspaces;
use crate::widgets::{battery, clock};

pub(crate) struct Bar {
    // windows: Vec<ApplicationWindow>,
}

impl Bar {
    pub fn create(app: &Application) -> ApplicationWindow {
        span!(Level::INFO, "bar");
        trace!("creating the window");
        let window = gtk4::ApplicationWindow::new(app);
        // Setup the window to be a layer surface.
        trace!("initialising the layer shell");
        window.init_layer_shell();
        // Display above normal windows.
        window.set_layer(Layer::Top);
        // Make sure that the main background is transparent, we can color over
        // this later in our other classes.
        // Push other windows out of the way.
        trace!("enabling the auto exclusion zone and aligning the window");
        window.auto_exclusive_zone_enable();
        // Provide margins from other windows and the borders of the monitor.
        // window.set_margin(Edge::Left, 20);
        // window.set_margin(Edge::Right, 20);
        // window.set_margin(Edge::Top, 20);
        // Anchor the bar to the top.
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, false);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.set_decorated(false);

        trace!("making sure that the bar has no bg");
        window.add_css_class("transparent");
        trace!("setting the child to the container");
        // Set the window to show the workspaces.
        window.set_child(Some(&Self::container()));
        // Finally, set the bar as visible.
        window.set_focusable(true);
        window.set_visible(true);
        window
    }

    fn container() -> Box {
        let container = Box::builder()
            .valign(Align::Center)
            .css_classes(["bar"])
            .hexpand(true)
            .vexpand(true)
            .hexpand_set(true)
            .homogeneous(true)
            .build();
        trace!("appending the workspaces widget to the bar");
        let workspaces = &Workspaces::new();
        workspaces.set_halign(Align::Start);
        container.append(workspaces);
        trace!("appending the battery widget to the bar");
        let right_side = &Box::builder().build();
        right_side.set_halign(Align::End);
        let bat = &battery::create();
        bat.set_halign(Align::End);
        right_side.append(bat);
        // container.append(&sys_tray::create());
        trace!("appending the clock widget to the bar");
        let clock = &clock::create();
        clock.set_halign(Align::End);
        right_side.append(clock);
        container.append(right_side);

        container
    }
}
