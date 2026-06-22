## Why

Currently, the Zellij-Search UI ignores terminal row limits, pushing the search bar out of view if there are many tabs and panes. Additionally, long pane names break the interface alignment by wrapping text naturally, and everything uses standard terminal text colors which makes scanning difficult. This change fixes the UI rendering to provide a much smoother, contained, and visually hierarchy-driven search experience.

## What Changes

- Add a sliding viewport to keep the search bar fixed and the list contained within terminal rows.
- Truncate long tab and pane names with an ellipsis (`...`) so they occupy exactly one row.
- Introduce an on-demand "Preview Box" (toggled via shortcut, e.g., `Ctrl+e`) to view the full text of truncated names.
- Apply a color palette to differentiate items: Cyan/Bold White for Tabs, Dim Gray for Panes, and Green for the currently selected item.

## Capabilities

### New Capabilities

### Modified Capabilities
- `session-search`: UI must now enforce a maximum display limit, handle text truncation, toggle a detail preview pane, and render items with specific ANSI color codes to reflect state and type.

## Impact

- Modifies the rendering logic in `src/main.rs`.
- Modifies `PluginState` to track `show_preview`.
- Requires adding an additional keybinding (`Ctrl+e` or similar) to toggle the preview box.
