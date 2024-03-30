mod imp;

use gio::glib;
use glib::Object;
use gtk4::prelude::*;

use crate::config::WorkspaceItem;

glib::wrapper! {
    pub struct WorkspaceButton(ObjectSubclass<imp::WorkspaceButton>)
        @extends gtk4::Button, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl WorkspaceButton {
    pub(super) fn new(workspace: &WorkspaceItem) -> Self {
        let container: Self = Object::builder().build();
        container.set_cursor_from_name(Some("pointer"));
        // makes the btn container 30px wide so that there is no reshuffling of
        // workspaces when they expand.
        container.set_workspace(workspace.id);
        container.set_kind(workspace.kind.to_string());
        // temp until i find a better way.
        container.add_css_class(&workspace.kind.to_string());
        container.setup_signals();
        container
    }

    pub fn setup_signals(&self) {
        self.connect_active_notify(|button: &WorkspaceButton| {
            // println!("changed active, should dispatch");
            if button.active() {
                button.add_css_class("active");
            } else {
                button.remove_css_class("active");
            }
        });
    }
}
