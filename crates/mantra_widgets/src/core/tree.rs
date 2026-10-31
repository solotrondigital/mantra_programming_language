//! Widget hierarchy, tree traversal, node counting, and document generation.

use super::widget::{BuildContext, Widget};

/// Represents the top-level mounted widget tree in an application or screen.
pub struct WidgetTree {
    pub root: Box<dyn Widget>,
}

impl WidgetTree {
    pub fn new(root: Box<dyn Widget>) -> Self {
        Self { root }
    }

    /// Recursively counts all widget nodes in the tree.
    pub fn count_nodes(&self) -> usize {
        fn count(w: &dyn Widget) -> usize {
            1 + w.children().into_iter().map(count).sum::<usize>()
        }
        count(self.root.as_ref())
    }

    /// Renders the entire widget tree to an HTML fragment.
    pub fn render(&self, is_dark_mode: bool) -> String {
        let ctx = BuildContext::root(is_dark_mode);
        self.root.render(&ctx)
    }

    /// Wraps the rendered widget tree in a full HTML5 responsive document.
    pub fn render_full_document(&self, title: &str, theme_css: &str, is_dark_mode: bool) -> String {
        let content = self.render(is_dark_mode);
        let dark_class = if is_dark_mode { "mantra-theme-dark" } else { "mantra-theme-light" };

        format!(
            r#"<!DOCTYPE html>
<html lang="en" class="{dark_class}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>{title}</title>
    <style>
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            -webkit-tap-highlight-color: transparent;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background-color: var(--mantra-bg, #f8fafc);
            color: var(--mantra-text, #0f172a);
            line-height: 1.5;
            overflow-x: hidden;
            width: 100vw;
            min-height: 100vh;
        }}
        {theme_css}
    </style>
</head>
<body class="{dark_class}">
    <div id="mantra-root" class="mantra-app-container">
        {content}
    </div>
</body>
</html>"#
        )
    }
}

