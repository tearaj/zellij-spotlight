## Context

Zellij plugins are compiled to WebAssembly (WASI) and interact with the host via a specific API. The host sends events (like `TabUpdate`, `PaneUpdate`, `Keypress`), and the plugin responds by mutating its internal state, rendering its UI, or sending commands back to the host (like `FocusPane`). We need a plugin that acts as a fast session explorer, specifically for jumping between panes based on their title.

## Goals / Non-Goals

**Goals:**
- Implement a Rust-based Zellij plugin.
- Maintain a local state of all tabs and panes in the current session.
- Filter the list of tabs and panes based on user query and selected search mode.
- Render a nested UI showing matching panes grouped under their respective tabs.
- Support keyboard navigation (`Up`/`Down`, `Enter`, `Tab`).

**Non-Goals:**
- Discovering the actual running program command inside the pane (will just use pane titles for the initial version).
- Searching across multiple sessions (only the current session is supported).
- Mouse interaction.

## Decisions

**Decision 1: Model-Update-View Architecture**
- **Rationale**: Zellij plugins generally follow an event-driven loop. Using a structured State Model that processes updates and then hands off a filtered list to the View layer keeps the code testable and easy to reason about.
- **Alternatives**: Tangled state and rendering logic, which becomes hard to maintain when adding new views.

**Decision 2: View Renderer Trait**
- **Rationale**: We want to easily change the view (e.g., from a nested list to a flat fuzzy-finder list) in the future. By defining a `ViewRenderer` trait, we decouple the layout and terminal ANSI generation from the core logic. The trait takes `(query, filtered_items, selection_index)` and returns a `String` to be printed.

**Decision 3: Selection Logic**
- **Rationale**: The user explicitly wants to only land on panes. When navigating with Up/Down, the selection index will skip over Tab header rows and only highlight Pane rows.

## Risks / Trade-offs

- **Risk**: Performance degradation with a massive number of panes (100+).
  - *Mitigation*: The plugin will only render the visible portion of the list. Since terminal rendering is the bottleneck, we'll ensure we only generate string output for what fits on the screen (though `zellij-tile` often handles basic clipping, doing it in the plugin is safer).
- **Risk**: Out-of-sync state.
  - *Mitigation*: Ensure the plugin correctly subscribes to and processes all relevant Zellij events (`TabUpdate`, `PaneUpdate`) so the internal state always reflects the host.
