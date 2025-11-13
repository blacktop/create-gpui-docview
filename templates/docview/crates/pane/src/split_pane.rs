use crate::pane_group::PaneGroup;
use gpui::{
    div, prelude::*, px, App, Context, CursorStyle, Div, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Render, Window,
};
use theme::WorkspaceTheme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

pub enum SplitNode {
    Leaf(PaneGroup),
    Split {
        direction: SplitDirection,
        first: Box<SplitNode>,
        second: Box<SplitNode>,
        ratio: f32, // 0.0 to 1.0, how much space first pane gets
    },
}

pub struct SplitContainer {
    theme: WorkspaceTheme,
    root: SplitNode,
    drag_state: Option<DragState>,
}

#[derive(Clone, Copy)]
struct DragState {
    _origin: f32,
    _ratio: f32,
}

impl SplitContainer {
    pub fn new(theme: WorkspaceTheme, cx: &mut App) -> Self {
        Self {
            theme: theme.clone(),
            root: SplitNode::Leaf(PaneGroup::new(theme, cx)),
            drag_state: None,
        }
    }

    pub fn with_split(theme: WorkspaceTheme, direction: SplitDirection, cx: &mut App) -> Self {
        let first = PaneGroup::new(theme.clone(), cx);
        let second = PaneGroup::new(theme.clone(), cx);

        Self {
            theme: theme.clone(),
            root: SplitNode::Split {
                direction,
                first: Box::new(SplitNode::Leaf(first)),
                second: Box::new(SplitNode::Leaf(second)),
                ratio: 0.5,
            },
            drag_state: None,
        }
    }

    pub fn split_vertical(&mut self, cx: &mut App) {
        self.root = SplitNode::Split {
            direction: SplitDirection::Vertical,
            first: Box::new(std::mem::replace(
                &mut self.root,
                SplitNode::Leaf(PaneGroup::new(self.theme.clone(), cx)),
            )),
            second: Box::new(SplitNode::Leaf(PaneGroup::new(self.theme.clone(), cx))),
            ratio: 0.5,
        };
    }

    pub fn split_horizontal(&mut self, cx: &mut App) {
        self.root = SplitNode::Split {
            direction: SplitDirection::Horizontal,
            first: Box::new(std::mem::replace(
                &mut self.root,
                SplitNode::Leaf(PaneGroup::new(self.theme.clone(), cx)),
            )),
            second: Box::new(SplitNode::Leaf(PaneGroup::new(self.theme.clone(), cx))),
            ratio: 0.5,
        };
    }

    fn render_node(&self, node: &SplitNode, cx: &mut Context<Self>) -> Div {
        match node {
            SplitNode::Leaf(_pane_group) => {
                // Render the pane group
                // Note: Since PaneGroup is not Copy, we can't directly render it here
                // In a real implementation, you'd use a model or handle
                div().flex_1().child("Pane")
            }
            SplitNode::Split {
                direction,
                first,
                second,
                ratio,
            } => {
                let _colors = self.theme.colors();
                let container = match direction {
                    SplitDirection::Horizontal => div().flex_col(),
                    SplitDirection::Vertical => div().flex_row(),
                };

                let first_size = *ratio;
                let second_size = 1.0 - ratio;

                container
                    .size_full()
                    .child(
                        div()
                            .when(*direction == SplitDirection::Horizontal, |d| {
                                d.h(gpui::relative(first_size))
                            })
                            .when(*direction == SplitDirection::Vertical, |d| {
                                d.w(gpui::relative(first_size))
                            })
                            .child(self.render_node(first, cx)),
                    )
                    .child(self.render_splitter(*direction, cx))
                    .child(
                        div()
                            .when(*direction == SplitDirection::Horizontal, |d| {
                                d.h(gpui::relative(second_size))
                            })
                            .when(*direction == SplitDirection::Vertical, |d| {
                                d.w(gpui::relative(second_size))
                            })
                            .child(self.render_node(second, cx)),
                    )
            }
        }
    }

    fn render_splitter(&self, direction: SplitDirection, cx: &mut Context<Self>) -> Div {
        let _colors = self.theme.colors();
        let handle = match direction {
            SplitDirection::Horizontal => div().w_full().h(px(4.0)).cursor(CursorStyle::ResizeRow),
            SplitDirection::Vertical => div().h_full().w(px(4.0)).cursor(CursorStyle::ResizeColumn),
        };

        handle
            .bg(_colors.border_soft)
            .hover(|style| style.bg(_colors.border_strong))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::start_drag))
    }

    fn start_drag(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        // Simplified drag handling
        self.drag_state = Some(DragState {
            _origin: f32::from(event.position.x),
            _ratio: 0.5,
        });
        cx.notify();
    }

    fn update_drag(&mut self, _event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(_drag) = self.drag_state {
            // Update the split ratio based on mouse position
            // This is simplified - in a real implementation, you'd calculate based on container size
            cx.notify();
        }
    }

    fn finish_drag(&mut self, _event: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.drag_state.take().is_some() {
            cx.notify();
        }
    }
}

impl Render for SplitContainer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_mouse_move(cx.listener(Self::update_drag))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::finish_drag))
            .child(self.render_node(&self.root, cx))
    }
}

