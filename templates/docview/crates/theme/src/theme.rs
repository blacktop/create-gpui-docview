use gpui::{hsla, px, rems, App, AppContext, Context, Entity, EventEmitter, Pixels, Rems, Rgba};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
    HighContrast,
    Moonlight,
}

impl ThemeMode {
    pub fn all() -> Vec<ThemeMode> {
        vec![
            ThemeMode::Dark,
            ThemeMode::Light,
            ThemeMode::HighContrast,
            ThemeMode::Moonlight,
        ]
    }
}

impl fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeMode::Dark => write!(f, "Dark"),
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::HighContrast => write!(f, "High Contrast"),
            ThemeMode::Moonlight => write!(f, "Moonlight"),
        }
    }
}

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
    mode: ThemeMode,
    colors: WorkspaceColors,
    gutter: Rems,
    radius: Rems,
    tab_height: Rems,
    header_height: Rems,
    sidebar_width: Pixels,
    drawer_height: Pixels,
}

impl WorkspaceTheme {
    pub fn new(mode: ThemeMode) -> Self {
        let colors = match mode {
            ThemeMode::Dark => Self::dark_colors(),
            ThemeMode::Light => Self::light_colors(),
            ThemeMode::HighContrast => Self::high_contrast_colors(),
            ThemeMode::Moonlight => Self::moonlight_colors(),
        };

        Self {
            mode,
            colors,
            gutter: rems(0.8),
            radius: rems(0.35),
            tab_height: rems(2.3),
            header_height: rems(2.4),
            sidebar_width: px(240.0),
            drawer_height: px(220.0),
        }
    }

    fn dark_colors() -> WorkspaceColors {
        WorkspaceColors {
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
        }
    }

    fn light_colors() -> WorkspaceColors {
        WorkspaceColors {
            app_bg: hsla(0.0, 0.0, 0.98, 1.0).into(),
            sidebar_bg: hsla(0.0, 0.0, 0.96, 1.0).into(),
            editor_bg: hsla(0.0, 0.0, 1.0, 1.0).into(),
            panel_bg: hsla(0.0, 0.0, 0.97, 1.0).into(),
            border_soft: hsla(0.0, 0.0, 0.85, 0.5).into(),
            border_strong: hsla(0.0, 0.0, 0.75, 0.9).into(),
            text_primary: hsla(0.0, 0.0, 0.1, 1.0).into(),
            text_muted: hsla(0.0, 0.0, 0.4, 1.0).into(),
            accent: hsla(265.0, 0.7, 0.55, 1.0).into(),
            accent_muted: hsla(265.0, 0.5, 0.7, 0.2).into(),
            overlay_bg: hsla(0.0, 0.0, 0.95, 0.85).into(),
            code_selection: hsla(210.0, 0.8, 0.8, 0.3).into(),
        }
    }

    fn high_contrast_colors() -> WorkspaceColors {
        WorkspaceColors {
            app_bg: hsla(0.0, 0.0, 0.0, 1.0).into(),
            sidebar_bg: hsla(0.0, 0.0, 0.05, 1.0).into(),
            editor_bg: hsla(0.0, 0.0, 0.02, 1.0).into(),
            panel_bg: hsla(0.0, 0.0, 0.08, 1.0).into(),
            border_soft: hsla(0.0, 0.0, 0.3, 0.8).into(),
            border_strong: hsla(0.0, 0.0, 0.5, 1.0).into(),
            text_primary: hsla(0.0, 0.0, 1.0, 1.0).into(),
            text_muted: hsla(0.0, 0.0, 0.8, 1.0).into(),
            accent: hsla(200.0, 1.0, 0.6, 1.0).into(),
            accent_muted: hsla(200.0, 0.8, 0.4, 0.4).into(),
            overlay_bg: hsla(0.0, 0.0, 0.0, 0.9).into(),
            code_selection: hsla(200.0, 1.0, 0.5, 0.4).into(),
        }
    }

    fn moonlight_colors() -> WorkspaceColors {
        WorkspaceColors {
            app_bg: hsla(230.0, 0.38, 0.14, 1.0).into(),
            sidebar_bg: hsla(230.0, 0.35, 0.17, 1.0).into(),
            editor_bg: hsla(230.0, 0.33, 0.21, 1.0).into(),
            panel_bg: hsla(230.0, 0.3, 0.24, 1.0).into(),
            border_soft: hsla(230.0, 0.2, 0.35, 0.5).into(),
            border_strong: hsla(230.0, 0.25, 0.45, 0.85).into(),
            text_primary: hsla(210.0, 0.3, 0.92, 1.0).into(),
            text_muted: hsla(215.0, 0.2, 0.65, 1.0).into(),
            accent: hsla(180.0, 0.7, 0.65, 1.0).into(),
            accent_muted: hsla(180.0, 0.5, 0.45, 0.25).into(),
            overlay_bg: hsla(230.0, 0.4, 0.1, 0.8).into(),
            code_selection: hsla(180.0, 0.8, 0.5, 0.35).into(),
        }
    }

    pub fn docview() -> Self {
        Self::new(ThemeMode::Dark)
    }

    pub fn mode(&self) -> ThemeMode {
        self.mode
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

// ThemeManager is now a GPUI Entity that can emit events
pub struct ThemeManager {
    current: WorkspaceTheme,
    available_modes: Vec<ThemeMode>,
}

impl ThemeManager {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self {
            current: WorkspaceTheme::new(ThemeMode::Dark),
            available_modes: ThemeMode::all(),
        })
    }

    pub fn current(&self) -> &WorkspaceTheme {
        &self.current
    }

    pub fn current_mode(&self) -> ThemeMode {
        self.current.mode()
    }

    pub fn available_modes(&self) -> &[ThemeMode] {
        &self.available_modes
    }

    pub fn set_mode(&mut self, mode: ThemeMode, cx: &mut Context<Self>) {
        if self.current.mode() != mode {
            self.current = WorkspaceTheme::new(mode);
            cx.emit(ThemeChangedEvent { mode });
            cx.notify();
        }
    }
}

// Event emitted when theme changes
#[derive(Debug, Clone)]
pub struct ThemeChangedEvent {
    pub mode: ThemeMode,
}

impl EventEmitter<ThemeChangedEvent> for ThemeManager {}
