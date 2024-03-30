use gio::glib::StaticType;
use glib::subclass::Signal;
use gtk4::glib;
use gtk4::subclass::prelude::*;
use std::sync::OnceLock;

// Object holding the state
#[derive(Default)]
pub struct Workspaces {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Workspaces {
    const NAME: &'static str = "WorkspacesWidgets";
    type Type = super::Workspaces;
    type ParentType = gtk4::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for Workspaces {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![Signal::builder("workspace-changed")
                .param_types([str::static_type()])
                .build()]
        })
    }
}

// Trait shared by all widgets
impl WidgetImpl for Workspaces {}

// Trait shared by all boxes
impl BoxImpl for Workspaces {}
