use gpui::{div, prelude::*, px, rems, CursorStyle, Div, FontWeight, Rgba, SharedString};
use theme::WorkspaceTheme;

pub struct TabVisual<'a> {
    pub title: &'a SharedString,
    pub subtitle: &'a SharedString,
    pub language: &'a SharedString,
    pub dirty: bool,
    pub preview: bool,
    pub active: bool,
    pub accent: Rgba,
    pub theme: &'a WorkspaceTheme,
}

pub fn tab_chip<'a>(visual: TabVisual<'a>) -> Div {
    let colors = visual.theme.colors();
    let background = if visual.active {
        colors.accent_muted
    } else {
        colors.panel_bg
    };

    div()
        .flex()
        .flex_col()
        .justify_center()
        .gap(rems(0.1))
        .px(rems(0.65))
        .py(rems(0.35))
        .rounded(visual.theme.radius())
        .bg(background)
        .border_b(px(1.0))
        .border_color(colors.border_soft)
        .text_color(colors.text_primary)
        .child(
            div()
                .flex()
                .items_center()
                .gap(rems(0.35))
                .child(
                    div()
                        .w(rems(0.3))
                        .h(rems(0.3))
                        .rounded_full()
                        .bg(if visual.dirty {
                            visual.accent
                        } else {
                            colors.border_soft
                        }),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(visual.title.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(colors.text_muted)
                        .child(visual.language.clone()),
                )
                .when_some(visual.preview.then_some(()), |builder, _| {
                    builder.child(
                        div()
                            .text_xs()
                            .px(rems(0.4))
                            .py(rems(0.1))
                            .rounded(rems(0.2))
                            .bg(colors.accent_muted)
                            .text_color(colors.accent)
                            .child("PREVIEW"),
                    )
                }),
        )
        .child(
            div()
                .text_xs()
                .text_color(colors.text_muted)
                .child(visual.subtitle.clone()),
        )
}

pub fn close_button(theme: &WorkspaceTheme, active: bool) -> Div {
    let colors = theme.colors();
    let fg = if active {
        colors.text_primary
    } else {
        colors.text_muted
    };
    div()
        .size(rems(1.2))
        .rounded(theme.radius())
        .items_center()
        .justify_center()
        .text_xs()
        .text_color(fg)
        .hover(|style| style.bg(colors.accent_muted).cursor(CursorStyle::PointingHand))
        .child("×")
}
