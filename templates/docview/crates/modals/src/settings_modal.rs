use gpui::{
    div, prelude::*, px, rems, App, Context, CursorStyle, Div, Entity, EventEmitter, FocusHandle,
    Focusable, MouseButton, MouseDownEvent, Render, ScrollWheelEvent, Subscription, Window,
};
use theme::{ThemeChangedEvent, ThemeManager, ThemeMode, WorkspaceTheme};

pub struct SettingsModal {
    theme_manager: Entity<ThemeManager>,
    theme: WorkspaceTheme,
    focus: FocusHandle,
    toggles: Vec<SettingToggle>,
    theme_subscription: Option<Subscription>,
}

#[derive(Clone)]
struct SettingToggle {
    label: &'static str,
    description: &'static str,
    enabled: bool,
}

impl SettingsModal {
    pub fn new(theme_manager: Entity<ThemeManager>, cx: &mut Context<Self>) -> Self {
        let theme = theme_manager.read(cx).current().clone();

        // Subscribe to theme changes
        let theme_subscription = Some(cx.subscribe(&theme_manager, |this, _, _: &ThemeChangedEvent, cx| {
            this.theme = this.theme_manager.read(cx).current().clone();
            cx.notify();
        }));

        Self {
            theme_manager,
            theme,
            focus: cx.focus_handle(),
            theme_subscription,
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

    fn theme_selector(&self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        let current_mode = self.theme_manager.read(cx).current_mode();
        let available_modes = self.theme_manager.read(cx).available_modes().to_vec();

        div()
            .flex_col()
            .gap(self.theme.gutter())
            .child(
                div()
                    .text_sm()
                    .text_color(colors.text_muted)
                    .child("THEME"),
            )
            .children(available_modes.into_iter().map(|mode| {
                let is_selected = current_mode == mode;
                let theme_manager = self.theme_manager.clone();

                div()
                    .flex_row()
                    .items_center()
                    .gap(self.theme.gutter())
                    .w_full()
                    .bg(if is_selected {
                        colors.editor_bg
                    } else {
                        colors.panel_bg
                    })
                    .border_1()
                    .border_color(if is_selected {
                        colors.accent
                    } else {
                        colors.border_soft
                    })
                    .rounded(px(10.0))
                    .p(self.theme.gutter())
                    .cursor(CursorStyle::PointingHand)
                    .hover(|style| {
                        style.bg(colors.editor_bg)
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _: &MouseDownEvent, _, cx| {
                        theme_manager.update(cx, |manager, cx| {
                            manager.set_mode(mode, cx);
                        });
                        cx.stop_propagation();
                    }))
                    .child(
                        // Radio button indicator
                        div()
                            .size(px(16.0))
                            .rounded_full()
                            .border_2()
                            .border_color(if is_selected {
                                colors.accent
                            } else {
                                colors.border_strong
                            })
                            .when(is_selected, |this| {
                                this.child(
                                    div()
                                        .size(px(8.0))
                                        .rounded_full()
                                        .bg(colors.accent)
                                        .mt(px(2.0))
                                        .ml(px(2.0)),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex_col()
                            .gap(rems(0.2))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.text_primary)
                                    .child(mode.to_string()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.text_muted)
                                    .child(match mode {
                                        ThemeMode::Dark => "Dark theme with purple accents",
                                        ThemeMode::Light => "Light theme with clean backgrounds",
                                        ThemeMode::HighContrast => "High contrast for accessibility",
                                        ThemeMode::Moonlight => "Cool moonlight theme with cyan accents",
                                    }),
                            ),
                    )
            }))
    }
}

impl Render for SettingsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        let viewport = window.viewport_size();
        let gutter_px = self.theme.gutter().to_pixels(window.rem_size());
        let h_margin = f32::from(gutter_px) * 2.0;
        let top_margin = (f32::from(viewport.height) * 0.1).max(80.0);
        let bottom_margin = top_margin;
        let panel_width = (f32::from(viewport.width) - (2.0 * h_margin))
            .clamp(320.0, 640.0);
        let panel_max_height = (f32::from(viewport.height) - (top_margin + bottom_margin))
            .clamp(320.0, 720.0);
        let left_offset = ((f32::from(viewport.width) - panel_width) / 2.0).max(h_margin);

        // Modal overlay - captures all events to prevent interaction with content behind
        div()
            .absolute()
            .inset_0()
            .bg(colors.overlay_bg)
            .on_mouse_down(MouseButton::Left, cx.listener(|_, _, _, cx| {
                // Close modal when clicking overlay
                cx.emit(SettingsModalEvent::Close);
            }))
            .on_scroll_wheel(|_: &ScrollWheelEvent, _, cx| {
                // Stop scroll events from reaching the content behind the modal
                cx.stop_propagation();
            })
            .child(
                // Settings panel
                div()
                    .absolute()
                    .left(px(left_offset))
                    .top(px(top_margin))
                    .w(px(panel_width))
                    .max_w(px(640.0))
                    .max_h(px(panel_max_height))
                    .min_h(px(320.0))
                    .bg(colors.panel_bg)
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(colors.border_strong)
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                        // Prevent closing when clicking inside panel
                        cx.stop_propagation();
                    })
                    .on_scroll_wheel(|_: &ScrollWheelEvent, _, _cx| {
                        // Consume all scroll events at the panel level to prevent them from
                        // propagating to the document view behind the modal.
                        // The scrollable content area (.overflow_y_scroll) will handle its
                        // own scrolling before this handler is called (during the capture phase).
                    })
                    .track_focus(&self.focus_handle(cx))
                    // Header (fixed)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .px(self.theme.gutter())
                            .py(rems(1.0))
                            .border_b_1()
                            .border_color(colors.border_soft)
                            .flex_shrink_0()
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
                                        cx.listener(|_, _: &MouseDownEvent, _, cx| {
                                            cx.stop_propagation();
                                            cx.emit(SettingsModalEvent::Close);
                                        }),
                                    ),
                            ),
                    )
                    // Scrollable content area
                    .child(
                        div()
                            .id("settings-content")
                            .flex_1()
                            .overflow_y_scroll()
                            .child(
                                div()
                                    .flex_col()
                                    .gap(self.theme.gutter())
                                    .p(self.theme.gutter())
                                    // Theme selector section
                                    .child(self.theme_selector(cx))
                                    // Spacer
                                    .child(div().h(rems(0.5)))
                                    // Preferences section
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(colors.text_muted)
                                            .child("WORKSPACE PREFERENCES"),
                                    )
                                    .children(self.toggles.iter().enumerate().map(|(index, toggle)| {
                                        div()
                                            .flex_row()
                                            .items_start()
                                            .gap(self.theme.gutter())
                                            .w_full()
                                            .bg(colors.editor_bg)
                                            .border_1()
                                            .border_color(colors.border_soft)
                                            .rounded(px(10.0))
                                            .p(self.theme.gutter())
                                            .child(
                                                div()
                                                    .flex_1()
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
                            )
                    ),
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
