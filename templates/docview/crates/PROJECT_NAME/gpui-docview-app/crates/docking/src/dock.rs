use crate::panel_trait::{DockPosition, PanelRegistration};
use gpui::{
    div, prelude::*, px, rems, Context, CursorStyle, Div, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Render, Window,
};
use theme::WorkspaceTheme;

pub struct DockRail {
    theme: WorkspaceTheme,
    position: DockPosition,
    panels: Vec<PanelRegistration>,
    active_panel: Option<usize>,
    extent: f32,
    drag: Option<ResizeDrag>,
}

#[derive(Clone, Copy)]
struct ResizeDrag {
    origin: f32,
    extent: f32,
}

impl DockRail {
    pub fn new(position: DockPosition, theme: WorkspaceTheme, panels: Vec<PanelRegistration>) -> Self {
        Self {
            theme,
            position,
            extent: match position {
                DockPosition::Bottom => 220.0,
                _ => 280.0,
            },
            active_panel: if panels.is_empty() { None } else { Some(0) },
            panels,
            drag: None,
        }
    }

    fn render_vertical(&mut self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors().clone();
        let mut rail = div()
            .flex_row()
            .h_full()
            .flex_shrink_0()
            .bg(colors.sidebar_bg)
            .on_mouse_move(cx.listener(Self::update_resize))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::finish_resize));

        rail = rail.child(self.render_icon_column(cx));

        if let Some(active) = self.active_panel {
            rail = rail.child(
                div()
                    .flex_col()
                    .w(px(self.extent))
                    .bg(colors.panel_bg)
                    .border_1()
                    .border_color(colors.border_soft)
                    .child(
                        div()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .px(self.theme.gutter())
                            .py(self.theme.gutter())
                            .text_sm()
                            .text_color(colors.text_muted)
                            .child(self.panels[active].metadata.title)
                            .child(self.panels[active].metadata.id),
                    )
                    .child(
                        div()
                            .flex_col()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.panels[active].view.clone()),
                    ),
            );
            rail = rail.child(self.render_handle(cx));
        }

        rail
    }

    fn render_bottom(&mut self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors().clone();
        let mut bar = div()
            .flex_col()
            .flex_shrink_0()
            .bg(colors.app_bg)
            .on_mouse_move(cx.listener(Self::update_resize))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::finish_resize));

        bar = bar.child(self.render_bottom_tabstrip(cx));

        if let Some(active) = self.active_panel {
            bar = bar.child(
                div()
                    .flex_col()
                    .h(px(self.extent))
                    .bg(colors.panel_bg)
                    .border_t(px(1.0))
                    .border_color(colors.border_soft)
                    .child(self.panels[active].view.clone()),
            );
            bar = bar.child(self.render_horizontal_handle(cx));
        }

        bar
    }

    fn render_icon_column(&mut self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors().clone();
        let mut column = div()
            .flex_col()
            .w(rems(2.6))
            .gap(rems(0.4))
            .py(self.theme.gutter())
            .bg(colors.sidebar_bg);

        for (index, panel) in self.panels.iter().enumerate() {
            let is_active = self.active_panel == Some(index);
            column = column.child(
                div()
                    .flex_col()
                    .items_center()
                    .gap(rems(0.2))
                    .px(rems(0.5))
                    .py(rems(0.3))
                    .rounded(px(6.0))
                    .bg(if is_active { colors.accent_muted } else { colors.sidebar_bg })
                    .text_color(if is_active { colors.accent } else { colors.text_muted })
                    .text_sm()
                    .hover(|style| style.cursor(CursorStyle::PointingHand).bg(colors.accent_muted))
                    .child(panel.metadata.icon)
                    .child(div().text_xs().child(panel.metadata.title))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, event, window, cx| {
                        this.toggle_panel(index, event, window, cx);
                    })),
            );
        }

        column
    }

    fn render_bottom_tabstrip(&mut self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors().clone();
        let mut row = div()
            .flex_row()
            .gap(rems(0.5))
            .px(self.theme.gutter())
            .py(self.theme.gutter())
            .bg(colors.app_bg)
            .border_t(px(1.0))
            .border_color(colors.border_soft);

        for (index, panel) in self.panels.iter().enumerate() {
            let is_active = self.active_panel == Some(index);
            row = row.child(
                div()
                    .px(rems(0.75))
                    .py(rems(0.25))
                    .rounded(px(6.0))
                    .bg(if is_active { colors.accent_muted } else { colors.panel_bg })
                    .text_color(colors.text_primary)
                    .text_xs()
                    .hover(|style| style.cursor(CursorStyle::PointingHand).bg(colors.accent_muted))
                    .child(panel.metadata.title)
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, event, window, cx| {
                        this.toggle_panel(index, event, window, cx);
                    })),
            );
        }

        row
    }

    fn render_handle(&self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        div()
            .w(px(6.0))
            .h_full()
            .bg(colors.border_strong)
            .cursor(CursorStyle::ResizeColumn)
            .rounded_full()
            .on_mouse_down(MouseButton::Left, cx.listener(Self::start_resize))
    }

    fn render_horizontal_handle(&self, cx: &mut Context<Self>) -> Div {
        let colors = self.theme.colors();
        div()
            .h(px(6.0))
            .w_full()
            .bg(colors.border_strong)
            .cursor(CursorStyle::ResizeRow)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::start_resize))
    }

    fn toggle_panel(
        &mut self,
        index: usize,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_panel = if self.active_panel == Some(index) {
            None
        } else {
            Some(index)
        };
        cx.notify();
    }

    fn start_resize(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let origin = match self.position {
            DockPosition::Bottom => f32::from(event.position.y),
            _ => f32::from(event.position.x),
        };
        self.drag = Some(ResizeDrag {
            origin,
            extent: self.extent,
        });
        cx.notify();
    }

    fn update_resize(&mut self, event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(drag) = self.drag else {
            return;
        };
        let axis_value = match self.position {
            DockPosition::Bottom => f32::from(event.position.y),
            _ => f32::from(event.position.x),
        };
        let direction = match self.position {
            DockPosition::Right => -1.0,
            DockPosition::Bottom => -1.0,
            DockPosition::Left => 1.0,
        };
        let delta = (axis_value - drag.origin) * direction;
        let (min, max) = match self.position {
            DockPosition::Bottom => (140.0, 420.0),
            _ => (180.0, 520.0),
        };
        self.extent = (drag.extent + delta).clamp(min, max);
        cx.notify();
    }

    fn finish_resize(&mut self, _event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.drag.take().is_some() {
            cx.notify();
        }
    }
}

impl Render for DockRail {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if matches!(self.position, DockPosition::Bottom) {
            self.render_bottom(cx)
        } else {
            self.render_vertical(cx)
        }
    }
}
