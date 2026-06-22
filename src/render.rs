use crate::filter::{FilteredItem, SearchMode};

mod ansi {
    pub const SELECTED: &str = "\x1b[32m";
    pub const TAB_HEADER: &str = "\x1b[36m";
    pub const PANE_DIM: &str = "\x1b[90m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RESET: &str = "\x1b[0m";
}

pub struct RenderContext<'a> {
    pub query: &'a str,
    pub items: &'a [FilteredItem],
    pub selection_index: usize,
    pub scroll_offset: usize,
    pub list_rows: usize,
    pub search_mode: &'a SearchMode,
    pub cols: usize,
    pub show_preview: bool,
}

fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    if text.chars().count() > max_chars {
        let mut t: String = text.chars().take(max_chars.saturating_sub(3)).collect();
        t.push_str("...");
        t
    } else {
        text.to_string()
    }
}

pub fn render(ctx: &RenderContext) -> String {
    let mut output = String::new();

    // Header: search prompt + mode indicator
    let prompt_str = format!("Find > {}_", ctx.query);
    let mode_display = format!("[{}]", ctx.search_mode.label());

    let padding_len = if ctx.cols > prompt_str.len() + mode_display.len() {
        ctx.cols - prompt_str.len() - mode_display.len()
    } else {
        1
    };
    let padding = " ".repeat(padding_len);

    output.push_str(&format!("{}{}{}\n", prompt_str, padding, mode_display));
    output.push_str(&format!("{}\n", "─".repeat(ctx.cols.max(1))));

    // Items list
    let visible_items = ctx
        .items
        .iter()
        .skip(ctx.scroll_offset)
        .take(ctx.list_rows)
        .enumerate();
    let mut selected_full_text = String::new();

    for (i, item) in visible_items {
        let actual_idx = i + ctx.scroll_offset;
        let is_selected = actual_idx == ctx.selection_index;

        match item {
            FilteredItem::Tab(tab) => {
                if is_selected {
                    selected_full_text = format!("Tab: {}", tab.name);
                }
                let display_name =
                    truncate_with_ellipsis(&tab.name, ctx.cols.saturating_sub(2));
                if is_selected {
                    output.push_str(&format!(
                        "> {}{}{}\n",
                        ansi::SELECTED, display_name, ansi::RESET
                    ));
                } else {
                    output.push_str(&format!(
                        "  {}{}{}\n",
                        ansi::TAB_HEADER, display_name, ansi::RESET
                    ));
                }
            }
            FilteredItem::Pane { pane, .. } => {
                if is_selected {
                    selected_full_text = format!("Pane: {}", pane.title);
                }
                let display_name =
                    truncate_with_ellipsis(&pane.title, ctx.cols.saturating_sub(6));
                if is_selected {
                    output.push_str(&format!(
                        "    > {}{}{}\n",
                        ansi::SELECTED, display_name, ansi::RESET
                    ));
                } else {
                    output.push_str(&format!(
                        "      {}{}{}\n",
                        ansi::PANE_DIM, display_name, ansi::RESET
                    ));
                }
            }
        }
    }

    // Preview
    if ctx.show_preview {
        let limit = ctx.cols.saturating_sub(2) * 2;
        let display_preview = truncate_with_ellipsis(&selected_full_text, limit);
        output.push_str(&format!("{}\n", "─".repeat(ctx.cols.max(1))));
        output.push_str(&format!(
            "  {}{}{}\n",
            ansi::BOLD, display_preview, ansi::RESET
        ));
    }

    output
}
