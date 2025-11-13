use gpui::{
    div, prelude::*, px, rems, App, Context, CursorStyle, Div, EventEmitter, FocusHandle,
    Focusable, MouseButton, MouseDownEvent, Render, Window,
};
use theme::WorkspaceTheme;

pub struct SettingsModal {
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

impl SettingsModal {
    pub fn new(theme: WorkspaceTheme, cx: &mut App) -> Self {
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
                SettingToggle {
                    label: "Auto Save",
                    description: "Automatically save files after changes.",
                    enabled: false,
                },
                SettingToggle {
                    label: "Line Numbers",
                    description: "Show line numbers in the editor gutter.",
                    enabled: true,
                },
                SettingToggle {
                    label: "Word Wrap",
                    description: "Wrap long lines in the editor.",
                    enabled: false,
                },
            ],
        }
    }

    fn switch(&self, enabled: bool) -> Div {
        let colors = self.theme.colors();
        let thumb_offset = if enabled { px(18.0) } else { px(2.0) };
        div()
            .w(px(36.0))
            .h(px(20.0))
            .rounded_full()
            .bg(if enabled {
                colors.accent
            } else {
                colors.border_soft
            })
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

impl Render for SettingsModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();

        // Modal overlay
        div()
            .absolute()
            .inset_0()
            .bg(colors.overlay_bg)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                // Close modal when clicking overlay
                cx.emit(SettingsModalEvent::Close);
            }))
            .child(
                // Settings panel
                div()
                    .w(px(600.0))
                    .max_h(px(700.0))
                    .bg(colors.panel_bg)
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(colors.border_strong)
                    .shadow_lg()
                    .flex_col()
                    .on_mouse_down(MouseButton::Left, |event, _window, cx| {
                        // Prevent closing when clicking inside panel
                        cx.stop_propagation();
                    })
                    .track_focus(&self.focus_handle(cx))
                    // Header
                    .child(
                        div()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .px(self.theme.gutter())
                            .py(rems(1.0))
                            .border_b(px(1.0))
                            .border_color(colors.border_soft)
                            .child(
                                div()
                                    .text_xl()
                                    .text_color(colors.text_primary)
                                    .child("Settings"),
                            )
                            .child(
                                div()
                                    .px(rems(0.6))
                                    .py(rems(0.3))
                                    .rounded(self.theme.radius())
                                    .text_xl()
                                    .text_color(colors.text_muted)
                                    .hover(|style| {
                                        style
                                            .cursor(CursorStyle::PointingHand)
                                            .bg(colors.accent_muted)
                                    })
                                    .child("×")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _: &MouseDownEvent, _, cx| {
                                            cx.stop_propagation();
                                            cx.emit(SettingsModalEvent::Close);
                                        }),
                                    ),
                            ),
                    )
                    // Content
                    .child({
                        let mut content = div()
                            .flex_col()
                            .gap(self.theme.gutter())
                            .p(self.theme.gutter());
                        content.style().overflow.y = Some(gpui::Overflow::Scroll);
                        content
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.text_muted)
                                    .child("WORKSPACE PREFERENCES"),
                            )
                            .children(self.toggles.iter().enumerate().map(|(index, toggle)| {
                                div()
                                    .flex_row()
                                    .justify_between()
                                    .items_start()
                                    .gap(self.theme.gutter())
                                    .bg(colors.editor_bg)
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
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_col()
                                            .items_end()
                                            .gap(rems(0.2))
                                            .child(self.switch(toggle.enabled).on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _: &MouseDownEvent, _, cx| {
                                                    if let Some(item) = this.toggles.get_mut(index) {
                                                        item.enabled = !item.enabled;
                                                    }
                                                    cx.notify();
                                                }),
                                            ))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(colors.text_muted)
                                                    .child(if toggle.enabled {
                                                        "Enabled"
                                                    } else {
                                                        "Disabled"
                                                    }),
                                            ),
                                    )
                            }))
                    }),
            )
    }
}

impl Focusable for SettingsModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl EventEmitter<SettingsModalEvent> for SettingsModal {}

#[derive(Debug, Clone)]
pub enum SettingsModalEvent {
    Close,
}

