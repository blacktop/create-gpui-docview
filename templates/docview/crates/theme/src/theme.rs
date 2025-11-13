use gpui::{hsla, px, rems, Pixels, Rems, Rgba};

#[derive(Debug, Clone)]
pub struct WorkspaceColors {
    pub app_bg: Rgba,
    pub sidebar_bg: Rgba,
    pub editor_bg: Rgba,
    pub panel_bg: Rgba,
    pub border_soft: Rgba,
    pub border_strong: Rgba,
    pub text_primary: Rgba,
    pub text_muted: Rgba,
    pub accent: Rgba,
    pub accent_muted: Rgba,
    pub overlay_bg: Rgba,
    pub code_selection: Rgba,
}

#[derive(Debug, Clone)]
pub struct WorkspaceTheme {
    colors: WorkspaceColors,
    gutter: Rems,
    radius: Rems,
    tab_height: Rems,
    header_height: Rems,
    sidebar_width: Pixels,
    drawer_height: Pixels,
}

impl WorkspaceTheme {
    pub fn docview() -> Self {
        let colors = WorkspaceColors {
            app_bg: hsla(220.0, 0.32, 0.07, 1.0).into(),
            sidebar_bg: hsla(220.0, 0.28, 0.1, 1.0).into(),
            editor_bg: hsla(220.0, 0.24, 0.15, 1.0).into(),
            panel_bg: hsla(220.0, 0.2, 0.18, 1.0).into(),
            border_soft: hsla(220.0, 0.16, 0.28, 0.45).into(),
            border_strong: hsla(220.0, 0.2, 0.36, 0.8).into(),
            text_primary: hsla(210.0, 0.2, 0.95, 1.0).into(),
            text_muted: hsla(215.0, 0.12, 0.7, 1.0).into(),
            accent: hsla(265.0, 0.6, 0.72, 1.0).into(),
            accent_muted: hsla(265.0, 0.4, 0.5, 0.3).into(),
            overlay_bg: hsla(220.0, 0.35, 0.06, 0.75).into(),
            code_selection: hsla(210.0, 0.7, 0.5, 0.3).into(),
        };

        Self {
            colors,
            gutter: rems(0.8),
            radius: rems(0.35),
            tab_height: rems(2.3),
            header_height: rems(2.4),
            sidebar_width: px(240.0),
            drawer_height: px(220.0),
        }
    }

    pub fn colors(&self) -> &WorkspaceColors {
        &self.colors
    }

    pub fn gutter(&self) -> Rems {
        self.gutter
    }

    pub fn radius(&self) -> Rems {
        self.radius
    }

    pub fn tab_height(&self) -> Rems {
        self.tab_height
    }

    pub fn header_height(&self) -> Rems {
        self.header_height
    }

    pub fn sidebar_width(&self) -> Pixels {
        self.sidebar_width
    }

    pub fn drawer_height(&self) -> Pixels {
        self.drawer_height
    }
}

impl Default for WorkspaceTheme {
    fn default() -> Self {
        Self::docview()
    }
}

#[derive(Debug, Clone)]
pub struct ThemeManager {
    current: WorkspaceTheme,
}

impl ThemeManager {
    pub fn dark() -> Self {
        Self {
            current: WorkspaceTheme::docview(),
        }
    }

    pub fn current(&self) -> &WorkspaceTheme {
        &self.current
    }
}
