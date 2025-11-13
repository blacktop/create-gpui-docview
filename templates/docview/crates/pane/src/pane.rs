use gpui::{Rgba, SharedString};

#[derive(Debug, Clone)]
pub struct Pane {
    pub id: usize,
    pub label: SharedString,
    pub tabs: Vec<PaneTab>,
    pub active_tab: usize,
}

impl Pane {
    pub fn new(id: usize, label: impl Into<SharedString>, tabs: Vec<PaneTab>) -> Self {
        Self {
            id,
            label: label.into(),
            tabs,
            active_tab: 0,
        }
    }

    pub fn active(&self) -> Option<&PaneTab> {
        self.tabs.get(self.active_tab)
    }

    pub fn active_mut(&mut self) -> Option<&mut PaneTab> {
        self.tabs.get_mut(self.active_tab)
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    pub fn push_tab(&mut self, tab: PaneTab) {
        self.tabs.push(tab);
        self.active_tab = self.tabs.len().saturating_sub(1);
    }

    pub fn remove_tab(&mut self, index: usize) -> Option<PaneTab> {
        if index >= self.tabs.len() {
            return None;
        }
        let removed = self.tabs.remove(index);
        if self.tabs.is_empty() {
            self.active_tab = 0;
        } else if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
        Some(removed)
    }
}

#[derive(Debug, Clone)]
pub struct PaneTab {
    pub id: usize,
    pub title: SharedString,
    pub subtitle: SharedString,
    pub language: SharedString,
    pub dirty: bool,
    pub preview: bool,
    pub accent: Rgba,
    pub body: Vec<SharedString>,
}

impl PaneTab {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: usize,
        title: impl Into<SharedString>,
        subtitle: impl Into<SharedString>,
        language: impl Into<SharedString>,
        accent: Rgba,
        dirty: bool,
        preview: bool,
        body: Vec<SharedString>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            subtitle: subtitle.into(),
            language: language.into(),
            accent,
            dirty,
            preview,
            body,
        }
    }
}
