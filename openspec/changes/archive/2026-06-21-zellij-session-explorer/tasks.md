## 1. Setup and Initialization

- [x] 1.1 Initialize Rust WebAssembly project using Zellij plugin template
- [x] 1.2 Define `PluginState` struct with fields for tabs, panes, search query, search mode, and selection index
- [x] 1.3 Implement `zellij_tile::ZellijPlugin` trait for `PluginState` with empty stubs

## 2. Event Handling and State Sync

- [x] 2.1 Subscribe to `TabUpdate`, `PaneUpdate`, and `Keypress` events in `load()`
- [x] 2.2 Handle `TabUpdate` event to keep `PluginState.tabs` in sync
- [x] 2.3 Handle `PaneUpdate` event to keep `PluginState.panes` mapped to their respective tabs in sync

## 3. Search and Filtering Logic

- [x] 3.1 Define a `FilteredResults` data structure to hold the active list of items to render
- [x] 3.2 Implement filter function that matches tab/pane titles based on the `PluginState.search_query`
- [x] 3.3 Implement search mode toggling logic (`Tab & Pane`, `Tab Only`, `Pane Only`) that filters appropriately
- [x] 3.4 Ensure the selection index resets or clamps correctly when the filtered results change

## 4. View Rendering Architecture

- [x] 4.1 Define `ViewRenderer` trait with a `render()` method
- [x] 4.2 Implement `NestedViewRenderer` that groups matching panes under their respective tabs
- [x] 4.3 Add basic ANSI formatting for highlighting the currently selected row
- [x] 4.4 Hook the view renderer into the plugin's `render` loop, outputting the final string to stdout

## 5. User Interaction and Navigation

- [x] 5.1 Implement `Keypress` handling for appending characters to `search_query` and handling Backspace
- [x] 5.2 Implement `Keypress` handling for `Up` and `Down` arrows to mutate the selection index (ensuring it skips tab headers)
- [x] 5.3 Implement `Keypress` handling for `Tab` to toggle search mode configuration
- [x] 5.4 Implement `Keypress` handling for `Enter` to emit `FocusPane` (and `SwitchToTab` if necessary) for the selected pane
- [x] 5.5 Configure Zellij host keybinds to launch the plugin overlay with `Ctrl+y`
