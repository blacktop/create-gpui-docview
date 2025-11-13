use gpui::{div, hsla, prelude::*, px, rems, CursorStyle, Div, FontWeight, Rgba, SharedString};
use theme::WorkspaceTheme;

pub fn confirm_dialog(
    theme: &WorkspaceTheme,
    title: impl Into<SharedString>,
    body: impl Into<SharedString>,
) -> Div {
    let colors = theme.colors();
    let title = title.into();
    let body = body.into();
    div()
        .absolute()
        .size_full()
        .bg(colors.overlay_bg)
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(420.0))
                .bg(colors.panel_bg)
                .rounded(px(10.0))
                .p(theme.gutter())
                .flex_col()
                .gap(theme.gutter())
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(colors.text_primary)
                        .child(title),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(colors.text_muted)
                        .child(body),
                )
                .child(
                    div()
                        .flex_row()
                        .gap(rems(0.5))
                        .justify_end()
                        .child(primary_button("Continue", colors.accent))
                        .child(secondary_button("Cancel", colors.border_soft)),
                ),
        )
}

fn primary_button(label: impl Into<SharedString>, color: Rgba) -> Div {
    let label = label.into();
    div()
        .px(rems(0.9))
        .py(rems(0.4))
        .rounded(px(6.0))
        .bg(color)
        .text_color(hsla(0.0, 0.0, 1.0, 0.95))
        .text_sm()
        .hover(|style| style.opacity(0.85).cursor(CursorStyle::PointingHand))
        .child(label)
}

fn secondary_button(label: impl Into<SharedString>, color: Rgba) -> Div {
    let label = label.into();
    div()
        .px(rems(0.9))
        .py(rems(0.4))
        .rounded(px(6.0))
        .border_1()
        .border_color(color)
        .text_sm()
        .text_color(color)
        .hover(|style| style.bg(hsla(0.0, 0.0, 1.0, 0.05)).cursor(CursorStyle::PointingHand))
        .child(label)
}
