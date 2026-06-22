## 1. Update State and Keybindings

- [x] 1.1 Add `show_preview: bool` and `scroll_offset: usize` fields to `PluginState`.
- [x] 1.2 Update `PluginState::update` to toggle `show_preview` when `Ctrl+e` (or `Ctrl+Char('e')`) is pressed.

## 2. Implement Viewport Logic

- [x] 2.1 Calculate `list_rows` based on terminal `rows` (e.g., `rows - 2` minus `3` if `show_preview` is true).
- [x] 2.2 Update `PluginState::clamp_selection`, `move_selection_up`, and `move_selection_down` to adjust `scroll_offset` so that `selection_index` is always within the visible `list_rows`.

## 3. Update Renderer

- [x] 3.1 Modify `NestedViewRenderer::render` (and `ViewRenderer` trait) to accept `rows: usize`, `scroll_offset: usize`, and `show_preview: bool`.
- [x] 3.2 Slice the `items` array using `scroll_offset` and `list_rows` to only iterate over visible items.
- [x] 3.3 Implement truncation logic for Tab (`cols - 2`) and Pane (`cols - 6`) strings using `.chars().take()` and appending `...`.
- [x] 3.4 Apply ANSI colors during string formatting (`\x1b[36m` for Tabs, `\x1b[90m` for Panes, `\x1b[32m` for Selected).
- [x] 3.5 Render the Preview Box (a separator line and the full text of the currently selected item) at the bottom if `show_preview` is true.
