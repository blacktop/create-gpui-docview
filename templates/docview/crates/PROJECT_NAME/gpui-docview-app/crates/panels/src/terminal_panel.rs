use docking::{DockPosition, Panel, PanelMetadata};
use gpui::{
    div, hsla, prelude::*, px, rems, App, Context, FocusHandle, Focusable, Render, Window,
};
use theme::WorkspaceTheme;

pub struct TerminalPanel {
    theme: WorkspaceTheme,
    focus: FocusHandle,
    lines: Vec<TermLine>,
}

struct TermLine {
    prompt: &'static str,
    text: &'static str,
    kind: TermKind,
}

enum TermKind {
    Normal,
    Success,
    Error,
}

impl Panel for TerminalPanel {
    const METADATA: PanelMetadata = PanelMetadata {
        id: "terminal",
        title: "Terminal",
        icon: "",
        position: DockPosition::Bottom,
    };

    fn new(theme: WorkspaceTheme, cx: &mut App) -> Self {
        Self {
            theme,
            focus: cx.focus_handle(),
            lines: vec![
                TermLine { prompt: "λ", text: "cargo check", kind: TermKind::Normal },
                TermLine { prompt: "", text: "Checking pane_group v0.1.0", kind: TermKind::Normal },
                TermLine { prompt: "", text: "Finished dev [unoptimized + debuginfo] target(s) in 0.82s", kind: TermKind::Success },
                TermLine { prompt: "λ", text: "gpui-inspector --attach", kind: TermKind::Normal },
                TermLine { prompt: "", text: "error: inspector already running", kind: TermKind::Error },
            ],
        }
    }
}

impl Render for TerminalPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        div()
            .flex_col()
            .size_full()
            .bg(colors.editor_bg)
            .rounded(px(10.0))
            .border_1()
            .border_color(colors.border_soft)
            .p(self.theme.gutter())
            .gap(rems(0.3))
            .track_focus(&self.focus_handle(cx))
            .child(
                div()
                    .flex_row()
                    .justify_between()
                    .text_xs()
                    .text_color(colors.text_muted)
                    .child("TERMINAL • cargo check")
                    .child("Press esc to close"),
            )
            .child(
                div()
                    .flex_col()
                    .gap(rems(0.15))
                    .children(self.lines.iter().map(|line| self.render_line(line))),
            )
    }
}

impl gpui::Focusable for TerminalPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl TerminalPanel {
    fn render_line(&self, line: &TermLine) -> impl IntoElement {
        let colors = self.theme.colors();
        let color = match line.kind {
            TermKind::Normal => colors.text_primary,
            TermKind::Success => colors.accent,
            TermKind::Error => hsla(2.0, 0.7, 0.68, 1.0).into(),
        };
        let bg = match line.kind {
            TermKind::Error => colors.code_selection,
            TermKind::Success => colors.accent_muted,
            TermKind::Normal => colors.panel_bg,
        };
        div()
            .flex_row()
            .gap(rems(0.45))
            .px(rems(0.3))
            .py(rems(0.2))
            .rounded(self.theme.radius())
            .bg(bg)
            .text_sm()
            .text_color(color)
            .child(line.prompt)
            .child(line.text)
    }
}
