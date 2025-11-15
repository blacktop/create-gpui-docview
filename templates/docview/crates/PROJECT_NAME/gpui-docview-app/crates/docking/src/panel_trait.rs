use gpui::{AnyView, App, AppContext, Focusable, Render};
use theme::WorkspaceTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Right,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
pub struct PanelMetadata {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub position: DockPosition,
}

#[derive(Clone)]
pub struct PanelRegistration {
    pub metadata: PanelMetadata,
    pub view: AnyView,
    pub badge: Option<&'static str>,
}

impl PanelRegistration {
    pub fn new(metadata: PanelMetadata, view: AnyView) -> Self {
        Self {
            metadata,
            view,
            badge: None,
        }
    }

    pub fn with_badge(mut self, badge: &'static str) -> Self {
        self.badge = Some(badge);
        self
    }
}

pub trait Panel: Render + Focusable + Sized + 'static {
    const METADATA: PanelMetadata;

    fn new(theme: WorkspaceTheme, cx: &mut App) -> Self;

    fn registration(theme: WorkspaceTheme, cx: &mut App) -> PanelRegistration {
        let entity = cx.new(|cx| Self::new(theme.clone(), cx));
        PanelRegistration::new(Self::METADATA, entity.into())
    }
}
