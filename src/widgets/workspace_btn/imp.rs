use glib::Properties;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::Align;
use gtk4::Box;
use hyprland::dispatch::WorkspaceIdentifierWithSpecial;
use hyprland::dispatch::{Dispatch, DispatchType::Workspace};
use std::cell::RefCell;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::WorkspaceButton)]
pub struct WorkspaceButton {
    #[property(get, set)]
    pub(super) active: RefCell<bool>,
    #[property(get, set)]
    pub(super) workspace: RefCell<String>,
    #[property(get, set)]
    pub(super) kind: RefCell<String>,
    // Keep a reference to the child.
    child: RefCell<Option<gtk4::Widget>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for WorkspaceButton {
    const NAME: &'static str = "WorkspaceButtonWidgets";
    type Type = super::WorkspaceButton;
    type ParentType = gtk4::Button;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for WorkspaceButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        let inner = &Box::builder().build();

        // add class for the type of workspace it is following WorkspaceKind
        // this is how i would want it to be added, but cannot temporarily since
        // obj.kind is not set before it is constructed
        // inner.add_css_class(&obj.kind());

        // Align the inner.
        inner.set_valign(Align::Center);
        inner.set_halign(Align::Center);
        obj.set_child(Some(inner));
        // set the button to have a width of 30px, so that resizing the inner
        // doesn't affect its overall size, and henceforth its position
        obj.set_width_request(30);
    }
    fn dispose(&self) {
        // Child widgets need to be manually unparented in `dispose()`.
        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }
    }
}

// Trait shared by all widgets
impl WidgetImpl for WorkspaceButton {}

// Trait shared by all buttons
impl ButtonImpl for WorkspaceButton {
    fn clicked(&self) {
        let obj = self.obj();
        let workspace = obj.workspace();
        let id = match workspace.parse::<i32>() {
            Ok(val) => WorkspaceIdentifierWithSpecial::Id(val),
            Err(_) => {
                if let Some((_, val)) = workspace.split_once("name:") {
                    // TODO: fix temp hacky fix
                    match val {
                        "social" => WorkspaceIdentifierWithSpecial::Id(11),
                        _ => WorkspaceIdentifierWithSpecial::Name(val),
                    }
                } else if let Some((_, val)) = workspace.split_once("special:") {
                    WorkspaceIdentifierWithSpecial::Name(val)
                } else {
                    unreachable!("invalid workspace name in config");
                }
            }
        };

        let _ = Dispatch::call(Workspace(id));
    }
}
