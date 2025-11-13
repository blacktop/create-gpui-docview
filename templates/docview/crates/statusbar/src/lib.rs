use gpui::{div, prelude::*, px, rems, Context, Div, Render, SharedString, Window};
use theme::WorkspaceTheme;

pub struct StatusBar {
    theme: WorkspaceTheme,
    file_path: Option<SharedString>,
    file_type: Option<SharedString>,
    line_count: usize,
    cursor_line: usize,
    cursor_column: usize,
    encoding: SharedString,
    eol: SharedString,
}

impl StatusBar {
    pub fn new(theme: WorkspaceTheme) -> Self {
        Self {
            theme,
            file_path: None,
            file_type: None,
            line_count: 0,
            cursor_line: 1,
            cursor_column: 1,
            encoding: "UTF-8".into(),
            eol: "LF".into(),
        }
    }

    pub fn set_file(&mut self, path: Option<SharedString>, file_type: Option<SharedString>) {
        self.file_path = path;
        self.file_type = file_type;
    }

    pub fn set_line_count(&mut self, count: usize) {
        self.line_count = count;
    }

    pub fn set_cursor(&mut self, line: usize, column: usize) {
        self.cursor_line = line;
        self.cursor_column = column;
    }

    fn status_item(&self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Div {
        let colors = self.theme.colors();
        div()
            .flex()
            .flex_row()
            .gap(rems(0.3))
            .items_center()
            .px(rems(0.5))
            .py(rems(0.2))
            .rounded(self.theme.radius())
            .child(
                div()
                    .text_xs()
                    .text_color(colors.text_muted)
                    .child(label.into()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(colors.text_primary)
                    .child(value.into()),
            )
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();

        div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .h(rems(1.8))
            .w_full()
            .bg(colors.sidebar_bg)
            .border_t(px(1.0))
            .border_color(colors.border_soft)
            .px(self.theme.gutter())
            .items_center()
            .justify_between()
            .child(
                // Left side - file info
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(rems(0.8))
                    .when_some(self.file_path.as_ref(), |this, path| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(colors.text_primary)
                                .child(path.clone()),
                        )
                    })
                    .when_some(self.file_type.as_ref(), |this, file_type| {
                        this.child(
                            div()
                                .px(rems(0.4))
                                .py(rems(0.1))
                                .rounded(self.theme.radius())
                                .bg(colors.accent_muted)
                                .text_xs()
                                .text_color(colors.accent)
                                .child(file_type.clone()),
                        )
                    }),
            )
            .child(
                // Right side - cursor position, encoding, etc.
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(rems(0.4))
                    .child(self.status_item(
                        "Ln",
                        format!("{}, Col {}", self.cursor_line, self.cursor_column),
                    ))
                    .when(self.line_count > 0, |this| {
                        this.child(self.status_item("Lines", self.line_count.to_string()))
                    })
                    .child(self.status_item("", self.encoding.clone()))
                    .child(self.status_item("", self.eol.clone())),
            )
    }
}

