use docking::{DockPosition, Panel, PanelMetadata};
use gpui::{
    div, prelude::*, px, rems, App, Context, CursorStyle, Div, FocusHandle, Focusable, Render,
    Window,
};
use theme::WorkspaceTheme;

const ACTIVE_FILE: &str = "workspace.rs";

pub struct FileTreePanel {
    theme: WorkspaceTheme,
    focus: FocusHandle,
    nodes: Vec<FileNode>,
}

struct FileNode {
    name: &'static str,
    kind: NodeKind,
    children: Vec<FileNode>,
}

enum NodeKind {
    Directory,
    File(&'static str),
}

impl Panel for FileTreePanel {
    const METADATA: PanelMetadata = PanelMetadata {
        id: "file-tree",
        title: "Files",
        icon: "",
        position: DockPosition::Left,
    };

    fn new(theme: WorkspaceTheme, cx: &mut App) -> Self {
        Self {
            theme,
            focus: cx.focus_handle(),
            nodes: project_tree(),
        }
    }
}

impl Render for FileTreePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        div()
            .flex_col()
            .size_full()
            .bg(colors.panel_bg)
            .rounded(px(10.0))
            .p(self.theme.gutter())
            .gap(self.theme.gutter())
            .track_focus(&self.focus_handle(cx))
            .child(self.section_header("PROJECT"))
            .child(
                div()
                    .flex_col()
                    .gap(rems(0.2))
                    .children(self.nodes.iter().map(|node| self.render_node(node, 0))),
            )
            .child(self.section_header("RECENT FILES"))
            .child(
                div()
                    .flex_col()
                    .gap(rems(0.15))
                    .children(
                        ["workspace.rs", "pane_group.rs", "dock.rs"]
                            .into_iter()
                            .map(|file| {
                                div()
                                    .flex_row()
                                    .justify_between()
                                    .text_xs()
                        .text_color(colors.text_muted)
                                    .child(file)
                                    .child("Modified 2m ago")
                            }),
                    ),
            )
    }
}

impl gpui::Focusable for FileTreePanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl FileTreePanel {
    fn section_header(&self, title: &'static str) -> Div {
        let colors = self.theme.colors();
        div()
            .text_xs()
            .text_color(colors.text_muted)
            .child(title)
    }

    fn render_node(&self, node: &FileNode, depth: usize) -> impl IntoElement {
        let colors = self.theme.colors();
        let padding = self.theme.gutter() + rems(depth as f32 * 0.7);
        let icon = match node.kind {
            NodeKind::Directory => "",
            NodeKind::File(ext) => match ext {
                "rs" => "",
                "md" => "",
                "toml" => "",
                _ => "",
            },
        };
        let is_selected = matches!(node.kind, NodeKind::File("rs")) && node.name == ACTIVE_FILE;

        let mut row = div()
            .flex_col()
            .gap(rems(0.12))
            .child(
                div()
                    .flex_row()
                    .gap(rems(0.45))
                    .pl(padding)
                    .py(rems(0.15))
                    .text_color(if is_selected {
                        colors.text_primary
                    } else {
                        colors.text_muted
                    })
                    .text_sm()
                    .rounded(self.theme.radius())
                    .bg(if is_selected {
                        colors.accent_muted
                    } else {
                        colors.panel_bg
                    })
                    .hover(|style| style.cursor(CursorStyle::PointingHand).bg(colors.accent_muted))
                    .child(icon)
                    .child(node.name),
            );

        if !node.children.is_empty() {
            row = row.child(
                div()
                    .flex_col()
                    .gap(rems(0.15))
                    .children(node.children.iter().map(|child| self.render_node(child, depth + 1))),
            );
        }

        row
    }
}

fn project_tree() -> Vec<FileNode> {
    vec![
        FileNode {
            name: "crates",
            kind: NodeKind::Directory,
            children: vec![
                FileNode {
                    name: "pane",
                    kind: NodeKind::Directory,
                    children: vec![
                        FileNode { name: "src", kind: NodeKind::Directory, children: vec![] },
                        FileNode { name: "Cargo.toml", kind: NodeKind::File("toml"), children: vec![] },
                    ],
                },
                FileNode {
                    name: "docking",
                    kind: NodeKind::Directory,
                    children: vec![
                        FileNode { name: "src", kind: NodeKind::Directory, children: vec![] },
                        FileNode { name: "Cargo.toml", kind: NodeKind::File("toml"), children: vec![] },
                    ],
                },
            ],
        },
        FileNode { name: "README.md", kind: NodeKind::File("md"), children: vec![] },
    ]
}
