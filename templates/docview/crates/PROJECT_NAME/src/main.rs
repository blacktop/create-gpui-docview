use gpui::{
    actions, div, prelude::*, px, rems, uniform_list, App, Application, Axis, Context,
    CursorStyle, Div, KeyBinding, ListSizingBehavior, Menu, MenuItem, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Render, ScrollHandle, SharedString, Styled, UniformListScrollHandle,
    Window, WindowOptions,
};
use modals::{SettingsModal, SettingsModalEvent};
use pane::PaneGroup;
use statusbar::StatusBar;
use std::{
    fs,
    path::{Path, PathBuf},
};
use theme::{ThemeManager, WorkspaceTheme};

// Helper functions for flex layouts
fn h_flex() -> Div {
    div().flex().flex_row()
}

fn v_flex() -> Div {
    div().flex().flex_col()
}

// Define all menu actions
actions!(
    docview,
    [
        About,
        CheckForUpdates,
        NewFile,
        OpenFile,
        Save,
        SaveAs,
        CloseTab,
        CloseWindow,
        Quit,
        Undo,
        Redo,
        Cut,
        Copy,
        Paste,
        Find,
        Replace,
        ZoomIn,
        ZoomOut,
        ZoomReset,
        ToggleFullscreen,
        ToggleSidebar,
        ToggleFooter,
        ToggleSettings,
        NewWindow,
        Minimize,
        SplitVertical,
        SplitHorizontal,
    ]
);

fn main() {
    Application::new().run(|app: &mut App| {
        // Set up native OS menus
        app.set_menus(vec![
            // App menu (PROJECT_NAME)
            Menu {
                name: "PROJECT_NAME".into(),
                items: vec![
                    MenuItem::action("About PROJECT_NAME", About),
                    MenuItem::Separator,
                    MenuItem::action("Check for Updates...", CheckForUpdates),
                    MenuItem::Separator,
                    MenuItem::action("Settings...", ToggleSettings),
                    MenuItem::Separator,
                    MenuItem::action("Quit PROJECT_NAME", Quit),
                ],
            },
            // File menu
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("New File", NewFile),
                    MenuItem::action("Open File...", OpenFile),
                    MenuItem::Separator,
                    MenuItem::action("Save", Save),
                    MenuItem::action("Save As...", SaveAs),
                    MenuItem::Separator,
                    MenuItem::action("Close Tab", CloseTab),
                    MenuItem::action("Close Window", CloseWindow),
                ],
            },
            // Edit menu
            Menu {
                name: "Edit".into(),
                items: vec![
                    MenuItem::action("Undo", Undo),
                    MenuItem::action("Redo", Redo),
                    MenuItem::Separator,
                    MenuItem::action("Cut", Cut),
                    MenuItem::action("Copy", Copy),
                    MenuItem::action("Paste", Paste),
                    MenuItem::Separator,
                    MenuItem::action("Find", Find),
                    MenuItem::action("Replace", Replace),
                ],
            },
            // View menu
            Menu {
                name: "View".into(),
                items: vec![
                    MenuItem::action("Zoom In", ZoomIn),
                    MenuItem::action("Zoom Out", ZoomOut),
                    MenuItem::action("Reset Zoom", ZoomReset),
                    MenuItem::Separator,
                    MenuItem::action("Toggle Fullscreen", ToggleFullscreen),
                    MenuItem::Separator,
                    MenuItem::action("Toggle Sidebar", ToggleSidebar),
                    MenuItem::action("Toggle Footer", ToggleFooter),
                ],
            },
            // Window menu
            Menu {
                name: "Window".into(),
                items: vec![
                    MenuItem::action("New Window", NewWindow),
                    MenuItem::action("Minimize", Minimize),
                    MenuItem::Separator,
                    MenuItem::action("Split Vertical", SplitVertical),
                    MenuItem::action("Split Horizontal", SplitHorizontal),
                ],
            },
        ]);

        // Bind keyboard shortcuts
        app.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-n", NewFile, None),
            KeyBinding::new("cmd-o", OpenFile, None),
            KeyBinding::new("cmd-s", Save, None),
            KeyBinding::new("cmd-shift-s", SaveAs, None),
            KeyBinding::new("cmd-w", CloseTab, None),
            KeyBinding::new("cmd-shift-w", CloseWindow, None),
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-f", Find, None),
            KeyBinding::new("cmd-shift-f", Replace, None),
            KeyBinding::new("cmd-=", ZoomIn, None),
            KeyBinding::new("cmd--", ZoomOut, None),
            KeyBinding::new("cmd-0", ZoomReset, None),
            KeyBinding::new("ctrl-cmd-f", ToggleFullscreen, None),
            KeyBinding::new("cmd-b", ToggleSidebar, None),
            KeyBinding::new("cmd-j", ToggleFooter, None),
            KeyBinding::new("cmd-,", ToggleSettings, None),
            KeyBinding::new("cmd-shift-n", NewWindow, None),
            KeyBinding::new("cmd-m", Minimize, None),
            KeyBinding::new("cmd-\\", SplitVertical, None),
            KeyBinding::new("cmd-shift-\\", SplitHorizontal, None),
        ]);

        app.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|cx| AppView::new(cx))
        })
            .expect("failed to open window");
    });
}

struct AppView {
    theme: WorkspaceTheme,
    status_bar: gpui::Entity<StatusBar>,
    pane_group: gpui::Entity<PaneGroup>,
    
    // File tree state
    root: PathBuf,
    tree: Vec<FsNode>,
    selected_path: Option<PathBuf>,
    content_lines: Vec<SharedString>,
    
    // Scroll handles
    sidebar_scroll: ScrollHandle,
    editor_scroll: UniformListScrollHandle,
    
    // Layout state
    sidebar_visible: bool,
    footer_visible: bool,
    sidebar_width: f32,
    sidebar_drag: Option<SidebarDrag>,
    
    // Modal state
    settings_modal: Option<gpui::Entity<SettingsModal>>,
}

#[derive(Clone)]
struct FsNode {
    name: SharedString,
    path: PathBuf,
    is_dir: bool,
    open: bool,
    children: Option<Vec<FsNode>>,
}

#[derive(Clone, Copy)]
struct SidebarDrag {
    origin: f32,
    width: f32,
}

impl AppView {
    fn new(cx: &mut App) -> Self {
        let theme = ThemeManager::dark().current().clone();
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let tree = read_dir_nodes(&root);
        let sidebar_width = f32::from(theme.sidebar_width());
        
        let status_bar = cx.new(|_cx| StatusBar::new(theme.clone()));
        let pane_group = cx.new(|cx| PaneGroup::new(theme.clone(), cx));
        
        Self {
            theme,
            status_bar,
            pane_group,
            root,
            tree,
            selected_path: None,
            content_lines: vec![],
            sidebar_scroll: ScrollHandle::new(),
            editor_scroll: UniformListScrollHandle::new(),
            sidebar_visible: true,
            footer_visible: true,
            sidebar_width,
            sidebar_drag: None,
            settings_modal: None,
        }
    }


    // Action handlers
    fn on_toggle_sidebar(&mut self, _: &ToggleSidebar, _window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_visible = !self.sidebar_visible;
        cx.notify();
    }
    
    fn on_toggle_footer(&mut self, _: &ToggleFooter, _window: &mut Window, cx: &mut Context<Self>) {
        self.footer_visible = !self.footer_visible;
        cx.notify();
    }
    
    fn on_toggle_settings(&mut self, _: &ToggleSettings, _window: &mut Window, cx: &mut Context<Self>) {
        if self.settings_modal.is_some() {
            self.settings_modal = None;
        } else {
            let modal = cx.new(|cx| SettingsModal::new(self.theme.clone(), cx));
            self.settings_modal = Some(modal);
        }
        cx.notify();
    }
    
    fn on_close_window(&mut self, _: &CloseWindow, _window: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }
    
    fn on_quit(&mut self, _: &Quit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.quit();
    }


    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        
        v_flex()
            .flex_shrink_0()
            .w(px(self.sidebar_width))
            .h_full()
            .bg(colors.sidebar_bg)
            .border_r(px(1.0))
            .border_color(colors.border_soft)
            .child(
                div()
                    .p(self.theme.gutter())
                    .text_xs()
                    .text_color(colors.text_muted)
                    .child("FILES"),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .id("sidebar-scroll")
                            .track_scroll(&self.sidebar_scroll)
                            .overflow_scroll()
                            .size_full()
                            .p(self.theme.gutter())
                            .child(
                                v_flex()
                    .gap(rems(0.1))
                    .children(self.tree.iter().map(|n| self.render_node(n, 0, cx))),
                            )
                    )
            )
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
            if node.open {
                "▼"
            } else {
                "▶"
            }
        } else {
            "📄"
        };

        let path = node.path.clone();
        let is_dir = node.is_dir;

        let mut container = v_flex()
            .gap(rems(0.05));
        
        // File/folder row
        container = container.child(
            h_flex()
                .gap(rems(0.4))
                    .pl(padding)
                .py(rems(0.2))
                .items_center()
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
                .hover(|style| {
                    style
                        .cursor(CursorStyle::PointingHand)
                        .bg(colors.accent_muted)
                })
                .child(
                    div()
                        .flex_shrink_0()
                        .w(rems(0.8))
                    .child(icon)
                )
                .child(
                    div()
                        .flex_1()
                    .child(node.name.clone())
                )
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                            if is_dir {
                                this.toggle_dir(&path, cx);
                            } else {
                                this.select_file(&path, cx);
                            }
                })),
            );

        // Children (if directory is open)
        if node.is_dir && node.open {
            if let Some(children) = &node.children {
                container = container.child(
                    v_flex()
                        .gap(rems(0.05))
                        .children(children.iter().map(|c| self.render_node(c, depth + 1, cx))),
                );
            }
        }

        container
    }

    fn toggle_dir(&mut self, path: &Path, cx: &mut Context<Self>) {
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
        
        // Update status bar
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string().into());
        let file_type = path
            .extension()
            .map(|e| e.to_string_lossy().to_uppercase().to_string().into());
        
        self.status_bar.update(cx, |status_bar, _cx| {
            status_bar.set_file(file_name, file_type);
            status_bar.set_line_count(self.content_lines.len());
        });
        
        cx.notify();
    }

    fn tab_row(&self) -> Div {
        let colors = self.theme.colors();
        
        if let Some(path) = &self.selected_path {
            let title = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".to_string());
            
        div()
            .flex_row()
            .gap(rems(0.35))
            .px(self.theme.gutter())
            .py(rems(0.3))
                .bg(colors.app_bg)
            .border_b(px(1.0))
            .border_color(colors.border_soft)
            .child(self.tab_chip(title.into(), true))
        } else {
            div()
                .h(self.theme.tab_height())
                .w_full()
                .bg(colors.app_bg)
                .border_b(px(1.0))
                .border_color(colors.border_soft)
        }
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

    fn editor_surface(&self) -> gpui::AnyElement {
        let colors = self.theme.colors();

        if self.content_lines.is_empty() {
                div()
                .flex()
                .size_full()
                .items_center()
                .justify_center()
                .bg(colors.editor_bg)
                    .text_color(colors.text_muted)
                .child("Select a file from the left.")
                .into_any_element()
        } else {
            let content_lines = self.content_lines.clone();
            
            uniform_list(
                "editor-list",
                self.content_lines.len(),
                {
                    let colors = colors.clone();
                    let gutter = self.theme.gutter();
                    move |visible_range, _window, _cx| {
                        visible_range
                            .map(|ix| {
                                let line: &SharedString = &content_lines[ix];
                                h_flex()
                                    .w_full()
                                    .h(rems(1.3))
                                    .px(gutter)
                                    .gap(rems(1.0))
                                    .bg(colors.editor_bg)
                    .child(
                        div()
                                            .w(rems(3.5))
                                            .flex_shrink_0()
                            .text_xs()
                                            .text_right()
                            .text_color(colors.text_muted)
                                            .child(format!("{}", ix + 1)),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_sm()
                                            .font_family("Monaco")
                                            .text_color(colors.text_primary)
                                            .child(line.clone())
                                    )
                            })
                            .collect()
                    }
                },
            )
            .size_full()
            .track_scroll(self.editor_scroll.clone())
            .with_sizing_behavior(ListSizingBehavior::Infer)
            .bg(colors.editor_bg)
            .into_any_element()
        }
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

    fn start_sidebar_drag(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_drag = Some(SidebarDrag {
            origin: f32::from(event.position.x),
            width: self.sidebar_width,
        });
        cx.notify();
    }

    fn update_sidebar_drag(&mut self, event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(drag) = self.sidebar_drag {
            let delta = f32::from(event.position.x) - drag.origin;
            let new_width = (drag.width + delta).clamp(180.0, 420.0);
            if (new_width - self.sidebar_width).abs() > 0.5 {
                self.sidebar_width = new_width;
                cx.notify();
            }
        }
    }

    fn finish_sidebar_drag(&mut self, _event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.sidebar_drag.take().is_some() {
            cx.notify();
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();

        // Subscribe to settings modal events
        if let Some(modal) = &self.settings_modal {
            cx.subscribe(
                modal,
                |this, _modal, event: &SettingsModalEvent, cx| match event {
                    SettingsModalEvent::Close => {
                        this.settings_modal = None;
                        cx.notify();
                    }
                },
            );
        }
        
        // Root: vertical layout (main content | status)
        v_flex()
            .size_full()
            .bg(colors.app_bg)
            .on_action(cx.listener(Self::on_toggle_sidebar))
            .on_action(cx.listener(Self::on_toggle_footer))
            .on_action(cx.listener(Self::on_toggle_settings))
            .on_action(cx.listener(Self::on_close_window))
            .on_action(cx.listener(Self::on_quit))
            .on_mouse_move(cx.listener(Self::update_sidebar_drag))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::finish_sidebar_drag))
            // Main content: HORIZONTAL LAYOUT (sidebar | document)
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .min_h_0()
                    .overflow_hidden()
                    // Sidebar (if visible)
                    .when(self.sidebar_visible, |this| {
                        this.child(self.sidebar(cx))
                            .child(self.sidebar_handle(cx))
                    })
                    // Document area: vertical layout (tabs | editor)
                    .child(
                        v_flex()
                            .flex_1()
                            .h_full()
                            .min_w_0()
                            .overflow_hidden()
                            // Tab bar
                    .child(self.tab_row())
                            // Editor
                            .child(
                                div()
                                    .flex_1()
                                    .w_full()
                                    .overflow_hidden()
                                    .child(self.editor_surface())
                            ),
                    ),
            )
            // Status bar (if visible)
            .when(self.footer_visible, |this| {
                this.child(self.status_bar.clone())
            })
            // Settings modal (if visible)
            .when_some(self.settings_modal.as_ref(), |this, modal| {
                this.child(modal.clone())
            })
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
        let name = entry.file_name().to_string_lossy().to_string().into();
        entries.push(FsNode {
            name,
            path,
            is_dir,
            open: false,
            children: if is_dir { None } else { Some(vec![]) },
        });
    }
    // Sort: dirs first, then files; then by name
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_string().cmp(&b.name.to_string()),
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

