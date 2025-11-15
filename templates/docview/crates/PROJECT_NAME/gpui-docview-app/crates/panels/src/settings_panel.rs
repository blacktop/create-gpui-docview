use docking::{DockPosition, Panel, PanelMetadata};
use gpui::{
    div, prelude::*, px, rems, App, Context, CursorStyle, Div, FocusHandle, Focusable, MouseButton,
    MouseDownEvent, Render, Window,
};
use theme::WorkspaceTheme;

pub struct SettingsPanel {
    theme: WorkspaceTheme,
    focus: FocusHandle,
    toggles: Vec<SettingToggle>,
}

#[derive(Clone)]
struct SettingToggle {
    label: &'static str,
    description: &'static str,
    enabled: bool,
}

impl Panel for SettingsPanel {
    const METADATA: PanelMetadata = PanelMetadata {
        id: "settings",
        title: "Settings",
        icon: "",
        position: DockPosition::Right,
    };

    fn new(theme: WorkspaceTheme, cx: &mut App) -> Self {
        Self {
            theme,
            focus: cx.focus_handle(),
            toggles: vec![
                SettingToggle {
                    label: "Auto Format",
                    description: "Runs `rustfmt` on save for Rust crates.",
                    enabled: true,
                },
                SettingToggle {
                    label: "Inline Diagnostics",
                    description: "Annotate editors with lints in real-time.",
                    enabled: true,
                },
                SettingToggle {
                    label: "Tab Drag Previews",
                    description: "Show live previews while dragging tabs.",
                    enabled: false,
                },
            ],
        }
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        let mut column = div()
            .flex_col()
            .size_full()
            .bg(colors.panel_bg)
            .rounded(px(8.0))
            .p(self.theme.gutter())
            .gap(self.theme.gutter())
            .track_focus(&self.focus_handle(cx))
            .child(
                div()
                    .text_sm()
                    .text_color(colors.text_muted)
                    .child("WORKSPACE PREFERENCES"),
            );

        for (index, toggle) in self.toggles.iter().enumerate() {
            column = column.child(
                div()
                    .flex_row()
                    .justify_between()
                    .items_start()
                    .gap(self.theme.gutter())
                    .bg(colors.panel_bg)
                    .border_1()
                    .border_color(colors.border_soft)
                    .rounded(px(10.0))
                    .p(self.theme.gutter())
                    .child(
                        div()
                            .flex_col()
                            .gap(rems(0.2))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.text_primary)
                                    .child(toggle.label),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.text_muted)
                                    .child(toggle.description),
                            )
                            .child(
                                div()
                                    .flex_row()
                                    .gap(rems(0.4))
                                    .text_xs()
                                    .text_color(colors.text_muted)
                                    .child("Shortcut")
                                    .child(
                                        div()
                                            .px(rems(0.3))
                                            .py(rems(0.1))
                                            .rounded(self.theme.radius())
                                            .bg(colors.accent_muted)
                                            .text_color(colors.accent)
                                            .child("cmd+,"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex_col()
                            .items_end()
                            .gap(rems(0.2))
                            .child(
                                self.switch(toggle.enabled).on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _: &MouseDownEvent, _, cx| {
                                        if let Some(item) = this.toggles.get_mut(index) {
                                            item.enabled = !item.enabled;
                                        }
                                        cx.notify();
                                    }),
                                ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.text_muted)
                                    .child(if toggle.enabled { "Enabled" } else { "Disabled" }),
                            ),
                    ),
            );
        }

        column
    }
}

impl gpui::Focusable for SettingsPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl SettingsPanel {
    fn switch(&self, enabled: bool) -> Div {
        let colors = self.theme.colors();
        let thumb_offset = if enabled { px(18.0) } else { px(2.0) };
        div()
            .w(px(36.0))
            .h(px(20.0))
            .rounded_full()
            .bg(if enabled { colors.accent } else { colors.border_soft })
            .border_1()
            .border_color(colors.border_soft)
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .size(px(16.0))
                    .rounded_full()
                    .bg(colors.panel_bg)
                    .ml(thumb_offset)
                    .mt(px(2.0)),
            )
    }
}
