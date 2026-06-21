## Why

The native session manager in Zellij is excellent for high-level management, but navigating within a single complex session with multiple tabs and panes can be cumbersome. This change introduces a fast, keyboard-driven plugin to search and quickly switch between panes and tabs within a session, boosting developer productivity.

## What Changes

- A new Zellij plugin that can be triggered via a hotkey (`Ctrl+y`).
- A search UI overlay that filters both tabs and panes by title.
- Configuration toggle (`Tab`) to switch the search mode between "Tab & Pane titles", "Tab titles only", or "Pane titles only".
- The ability to select and instantly focus a specific pane using `Up/Down` and `Enter`.
- An internal architecture that decouples the view renderer from state logic to support future view modes (e.g., flat list vs. nested list).

## Capabilities

### New Capabilities
- `session-search`: Capability to search across panes and tabs by title and focus a selected pane.

### Modified Capabilities
- 

## Impact

- Introduces a new WebAssembly (Rust) Zellij plugin.
- Relies on Zellij Plugin API (`TabUpdate`, `PaneUpdate`, `Keypress`) and commands (`SwitchToTab`, `FocusPane`).
