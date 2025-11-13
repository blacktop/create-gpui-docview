use gpui::{
    div, prelude::*, px, rems, App, Application, Context, CursorStyle, Div, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Overflow, Render, SharedString, Window,
    WindowOptions,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use theme::{ThemeManager, WorkspaceTheme};

fn main() {
    Application::new().run(|app: &mut App| {
        app.open_window(WindowOptions::default(), |_window, cx| cx.new(|cx| AppView::new(cx)))
            .expect("failed to open window");
    });
}

// Left: file tree. Right: document viewer.
struct AppView {
    theme: WorkspaceTheme,
    root: PathBuf,
    tree: Vec<FsNode>,
    selected_path: Option<PathBuf>,
    content_lines: Vec<SharedString>,
    sidebar_width: f32,
    sidebar_drag: Option<SidebarDrag>,
}

#[derive(Clone)]
struct FsNode {
    name: SharedString,
    path: PathBuf,
    is_dir: bool,
    open: bool,
    children: Option<Vec<FsNode>>, // None => not loaded
}

#[derive(Clone, Copy)]
struct SidebarDrag {
    origin: f32,
    width: f32,
}

impl AppView {
    fn new(_cx: &mut App) -> Self {
        let theme = ThemeManager::dark().current().clone();
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let tree = read_dir_nodes(&root);
        let sidebar_width = f32::from(theme.sidebar_width());
        Self {
            theme,
            root,
            tree,
            selected_path: None,
            content_lines: vec![],
            sidebar_width,
            sidebar_drag: None,
        }
    }

    // Sidebar with a lazy-loading file tree.
    fn sidebar(&self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        let mut sidebar = div()
            .w(px(self.sidebar_width))
            .h_full()
            .bg(colors.sidebar_bg)
            .border_r(px(1.0))
            .border_color(colors.border_soft)
            .p(self.theme.gutter())
            .flex_col()
            .gap(rems(0.4))
            .child(
                div()
                    .text_xs()
                    .text_color(colors.text_muted)
                    .child("FILES"),
            )
            .child(
                div()
                    .flex_col()
                    .gap(rems(0.1))
                    .children(self.tree.iter().map(|n| self.render_node(n, 0, cx))),
            );
        sidebar.style().overflow.y = Some(Overflow::Scroll);
        sidebar
    }

    fn render_node(&self, node: &FsNode, depth: usize, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        let padding = self.theme.gutter() + rems(depth as f32 * 0.7);
        let is_selected = self
            .selected_path
            .as_ref()
            .map(|p| p == &node.path)
            .unwrap_or(false);
        let icon = if node.is_dir {
            if node.open { "⌄" } else { "›" }
        } else {
            ""
        };

        // Row
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
                        colors.sidebar_bg
                    })
                    .hover(|style| style.cursor(CursorStyle::PointingHand).bg(colors.accent_muted))
                    .child(icon)
                    .child(node.name.clone())
                    .on_mouse_down(MouseButton::Left, {
                        let path = node.path.clone();
                        let is_dir = node.is_dir;
                        cx.listener(move |this, _, _, cx| {
                            if is_dir {
                                this.toggle_dir(&path, cx);
                            } else {
                                this.select_file(&path, cx);
                            }
                        })
                    }),
            );

        if node.is_dir && node.open {
            if let Some(children) = &node.children {
                row = row.child(
                    div()
                        .flex_col()
                        .gap(rems(0.1))
                        .children(children.iter().map(|c| self.render_node(c, depth + 1, cx))),
                );
            }
        }

        row
    }

    fn toggle_dir(&mut self, path: &Path, cx: &mut Context<Self>) {
        // Find and mutate node in-place; load children lazily.
        fn toggle_in(nodes: &mut [FsNode], path: &Path) -> bool {
            for n in nodes {
                if n.path == path {
                    n.open = !n.open;
                    if n.open && n.children.is_none() {
                        n.children = Some(read_dir_nodes(&n.path));
                    }
                    return true;
                }
                if n.is_dir {
                    if let Some(children) = n.children.as_mut() {
                        if toggle_in(children, path) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        let _ = toggle_in(&mut self.tree, path);
        cx.notify();
    }

    fn select_file(&mut self, path: &Path, cx: &mut Context<Self>) {
        self.selected_path = Some(path.to_path_buf());
        self.content_lines = read_file_lines(path);
        cx.notify();
    }

    fn tab_row(&self) -> Div {
        let colors = self.theme.colors();
        let title = self
            .selected_path
            .as_ref()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "No file selected".to_string());
        div()
            .flex_row()
            .gap(rems(0.35))
            .px(self.theme.gutter())
            .py(rems(0.3))
            .border_b(px(1.0))
            .border_color(colors.border_soft)
            .child(self.tab_chip(title.into(), true))
    }

    fn tab_chip(&self, title: SharedString, active: bool) -> Div {
        let colors = self.theme.colors();
        div()
            .px(rems(0.7))
            .py(rems(0.25))
            .rounded(self.theme.radius())
            .text_color(if active {
                colors.text_primary
            } else {
                colors.text_muted
            })
            .bg(if active {
                colors.accent_muted
            } else {
                colors.app_bg
            })
            .border(if active { px(1.0) } else { px(0.0) })
            .border_color(colors.border_soft)
            .child(title)
    }

    fn editor_surface(&self) -> Div {
        let colors = self.theme.colors();
        let mut editor = div()
            .flex_col()
            .flex_1()
            .bg(colors.editor_bg)
            .rounded(self.theme.radius())
            .border_1()
            .border_color(colors.border_soft)
            .p(self.theme.gutter())
            .gap(rems(0.2));

        if self.content_lines.is_empty() {
            editor = editor.child(
                div()
                    .text_color(colors.text_muted)
                    .child("Select a file from the left."),
            );
        } else {
            editor = editor.children(self.content_lines.iter().enumerate().map(|(i, line)| {
                div()
                    .flex_row()
                    .gap(rems(0.6))
                    .child(
                        div()
                            .w(rems(2.0))
                            .text_xs()
                            .text_color(colors.text_muted)
                            .child(format!("{:>4}", i + 1)),
                    )
                    .child(div().flex_1().text_color(colors.text_primary).child(line.clone()))
            }));
        }
        editor.style().overflow.y = Some(Overflow::Scroll);
        editor
    }

    fn sidebar_handle(&self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        div()
            .w(px(4.0))
            .h_full()
            .bg(colors.border_soft)
            .cursor(CursorStyle::ResizeColumn)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::start_sidebar_drag))
    }

    fn start_sidebar_drag(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.sidebar_drag = Some(SidebarDrag {
            origin: f32::from(event.position.x),
            width: self.sidebar_width,
        });
        cx.notify();
    }

    fn update_sidebar_drag(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(drag) = self.sidebar_drag {
            let delta = f32::from(event.position.x) - drag.origin;
            let new_width = (drag.width + delta).clamp(180.0, 420.0);
            if (new_width - self.sidebar_width).abs() > 0.5 {
                self.sidebar_width = new_width;
                cx.notify();
            }
        }
    }

    fn finish_sidebar_drag(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.sidebar_drag.take().is_some() {
            cx.notify();
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        div()
            .flex_row()
            .size_full()
            .bg(colors.app_bg)
            .on_mouse_move(cx.listener(Self::update_sidebar_drag))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::finish_sidebar_drag))
            .child(self.sidebar(cx))
            .child(self.sidebar_handle(cx))
            .child(
                div()
                    .flex_col()
                    .flex_1()
                    .bg(colors.app_bg)
                    .child(self.tab_row())
                    .child(div().flex_1().p(self.theme.gutter()).child(self.editor_surface())),
            )
    }
}

fn read_dir_nodes(dir: &Path) -> Vec<FsNode> {
    let mut entries: Vec<FsNode> = vec![];
    let Ok(read_dir) = fs::read_dir(dir) else {
        return entries;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        // Skip heavy/hidden directories by default
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') || name == "target" {
                continue;
            }
        }
        let is_dir = path.is_dir();
        let name = entry
            .file_name()
            .to_string_lossy()
            .to_string()
            .into();
        entries.push(FsNode {
            name,
            path,
            is_dir,
            open: false,
            children: if is_dir { None } else { Some(vec![]) },
        });
    }
    // Sort: dirs first, then files; then by name
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_string().cmp(&b.name.to_string()),
        }
    });
    entries
}

fn read_file_lines(path: &Path) -> Vec<SharedString> {
    match fs::read_to_string(path) {
        Ok(s) => {
            let mut lines = Vec::with_capacity(1024);
            for (i, line) in s.lines().enumerate() {
                if i > 10_000 {
                    lines.push("… (truncated)".into());
                    break;
                }
                lines.push(SharedString::from(line.to_string()));
            }
            if lines.is_empty() {
                lines.push("(empty file)".into());
            }
            lines
        }
        Err(_) => vec!["(binary or unreadable file)".into()],
    }
}
