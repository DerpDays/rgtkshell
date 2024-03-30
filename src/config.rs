use strum::{Display, EnumString};

#[derive(Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum WorkspaceKind {
    #[default]
    Regular,
    Social,
}

#[derive(Clone)]
pub struct WorkspaceItem {
    pub id: &'static str,
    pub kind: WorkspaceKind,
}

pub static WORKSPACES: [WorkspaceItem; 11] = [
    WorkspaceItem {
        id: "1",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "2",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "3",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "4",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "5",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "6",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "7",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "8",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "9",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "10",
        kind: WorkspaceKind::Regular,
    },
    WorkspaceItem {
        id: "name:social",
        kind: WorkspaceKind::Social,
    },
];
