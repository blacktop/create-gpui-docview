use crate::{pane::Pane, pane::PaneTab, tab_bar::{close_button, tab_chip, TabVisual}};
use gpui::{
    div, prelude::*, px, rems, App, Context, CursorStyle, Div, MouseButton, MouseDownEvent,
    Render, Rgba, SharedString, Window,
};
use theme::WorkspaceTheme;

macro_rules! include_lines {
    ($($line:expr),* $(,)?) => {{
        vec![$(SharedString::from($line)),*]
    }};
}

pub struct PaneGroup {
    theme: WorkspaceTheme,
    pane: Pane,
    next_tab_id: usize,
}

impl PaneGroup {
    pub fn new(theme: WorkspaceTheme, _cx: &mut App) -> Self {
        let mut next_tab_id = 0;
        let mut tabs = vec![
            sample_tab(
                &mut next_tab_id,
                "workspace.rs",
                "src/workspace.rs",
                "Rust",
                theme.colors().accent,
                include_lines!(
                    "fn main() {",
                    "    Application::new().run(|app| {",
                    "        app.open_window(...)",
                    "    });",
                    "}",
                ),
                true,
                false,
            ),
            sample_tab(
                &mut next_tab_id,
                "pane_group.rs",
                "crates/pane/src/pane_group.rs",
                "Rust",
                theme.colors().accent,
                include_lines!(
                    "// Pane layout",
                    "pub struct PaneGroup {",
                    "    theme: WorkspaceTheme,",
                    "    pane: Pane,",
                    "}",
                ),
                false,
                false,
            ),
        ];

        tabs.push(sample_tab(
            &mut next_tab_id,
            "design_notes.md",
            "docs/design_notes.md",
            "Markdown",
            theme.colors().accent,
            include_lines!(
                "# Document View",
                "- Focus on the center",
                "- Keep sidebars light",
                "- Use overlays for commands",
            ),
            false,
            true,
        ));

        Self {
            theme,
            pane: Pane::new(0, "Primary", tabs),
            next_tab_id,
        }
    }

    fn render_tabs(&mut self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        let mut track = div()
            .flex_row()
            .items_center()
            .gap(rems(0.25))
            .h(self.theme.tab_height())
            .px(self.theme.gutter())
            .bg(colors.app_bg)
            .border_b(px(1.0))
            .border_color(colors.border_soft);

        let pane = &mut self.pane;
        for (tab_index, tab) in pane.tabs.iter().enumerate() {
            let is_active = pane.active_tab == tab_index;
            let visual = TabVisual {
                title: &tab.title,
                subtitle: &tab.subtitle,
                language: &tab.language,
                dirty: tab.dirty,
                preview: tab.preview,
                active: is_active,
                accent: tab.accent,
                theme: &self.theme,
            };

            let mut chip = tab_chip(visual)
                .hover(|style| style.cursor(CursorStyle::PointingHand))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                    this.pane.set_active(tab_index);
                    cx.notify();
                }));

            chip = chip.child(
                close_button(&self.theme, is_active).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _event: &MouseDownEvent, _, cx| {
                        cx.stop_propagation();
                        this.close_tab(tab_index);
                        cx.notify();
                    }),
                ),
            );

            track = track.child(chip);
        }

        track
    }

    fn render_editor(&self) -> Div {
        let colors = self.theme.colors();
        let mut editor = div()
            .flex_col()
            .flex_1()
            .bg(colors.editor_bg)
            .rounded(self.theme.radius())
            .p(self.theme.gutter())
            .gap(rems(0.2));

        if let Some(tab) = self.pane.active() {
            for (line_idx, line) in tab.body.iter().enumerate() {
                editor = editor.child(
                    div()
                        .flex_row()
                        .gap(rems(0.6))
                        .child(
                            div()
                                .w(rems(2.0))
                                .text_xs()
                                .text_color(colors.text_muted)
                                .child(format!("{:>2}", line_idx + 1)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .text_color(colors.text_primary)
                                .child(line.clone()),
                        ),
                );
            }
        } else {
            editor = editor.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_1()
                    .text_color(colors.text_muted)
                    .child("Select a document from the sidebar"),
            );
        }

        editor
    }

    fn close_tab(&mut self, tab_index: usize) {
        if self.pane.remove_tab(tab_index).is_none() {
            return;
        }

        if self.pane.tabs.is_empty() {
            let tab = sample_tab(
                &mut self.next_tab_id,
                "welcome.md",
                "docs/welcome.md",
                "Markdown",
                self.theme.colors().accent,
                include_lines!("Welcome to GPUI"),
                false,
                false,
            );
            self.pane.push_tab(tab);
        }
    }
}

impl Render for PaneGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .flex_1()
            .child(self.render_tabs(cx))
            .child(self.render_editor())
    }
}

fn sample_tab(
    next_tab_id: &mut usize,
    title: impl Into<SharedString>,
    subtitle: impl Into<SharedString>,
    language: impl Into<SharedString>,
    accent: Rgba,
    body: Vec<SharedString>,
    dirty: bool,
    preview: bool,
) -> PaneTab {
    let id = *next_tab_id;
    *next_tab_id += 1;
    PaneTab::new(id, title, subtitle, language, accent, dirty, preview, body)
}
