use gpui::{div, hsla, prelude::*, px, rems, CursorStyle};
use theme::WorkspaceTheme;

pub fn picker_overlay(theme: &WorkspaceTheme) -> impl IntoElement {
    let colors = theme.colors();
    div()
        .absolute()
        .size_full()
        .bg(colors.overlay_bg)
        .flex_col()
        .items_center()
        .justify_start()
        .pt(rems(4.0))
        .child(
            div()
                .w(px(560.0))
                .bg(colors.panel_bg)
                .rounded(px(10.0))
                .shadow_lg()
                .p(theme.gutter())
                .flex_col()
                .gap(rems(0.4))
                .child(
                    div()
                        .flex_row()
                        .items_center()
                        .gap(rems(0.5))
                        .bg(colors.editor_bg)
                        .rounded(px(8.0))
                        .px(rems(0.8))
                        .py(rems(0.6))
                        .child("⌘P")
                        .child("Search files, commands, or symbols"),
                )
                .child(
                    div()
                        .flex_col()
                        .gap(rems(0.25))
                        .children([
                            picker_row("Open Workspace", "workspace::open"),
                            picker_row("Toggle Terminal", "view::toggle_terminal"),
                            picker_row("Split Pane Right", "pane::split"),
                        ]),
                ),
        )
}

fn picker_row(title: &'static str, command: &'static str) -> impl IntoElement {
    div()
        .flex_row()
        .justify_between()
        .bg(hsla(220.0, 0.18, 0.15, 0.8))
        .rounded(px(6.0))
        .px(rems(0.7))
        .py(rems(0.4))
        .text_sm()
        .hover(|style| style.bg(hsla(265.0, 0.6, 0.4, 0.6)).cursor(CursorStyle::PointingHand))
        .child(title)
        .child(
            div()
                .text_xs()
                .text_color(hsla(210.0, 0.2, 0.8, 0.9))
                .child(command),
        )
}
