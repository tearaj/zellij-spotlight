## Context

The current `Zellij-Search` plugin renders filtered tabs and panes directly to stdout without considering the terminal's viewport bounds (`rows` parameter). When the list of panes exceeds the available rows, Zellij naturally buffers it, pushing the plugin's header (the search bar) completely off-screen. Furthermore, long pane names are allowed to wrap natively by Zellij, meaning a single item can consume multiple vertical lines, breaking any simple row-based viewport math. Both problems lead to a broken visual experience in busy sessions.

## Goals / Non-Goals

**Goals:**
- Keep the search bar fixed at the top.
- Confine the list of results within the terminal's `rows` boundary.
- Implement a sliding `scroll_offset` so the currently selected item is always visible.
- Ensure 1 item = 1 row by explicitly truncating long strings with `...`.
- Provide an on-demand "Preview Box" bound to a hotkey (`Ctrl+e`) to inspect the full text of truncated items.
- Use distinct ANSI colors to differentiate Tabs and Panes.

**Non-Goals:**
- Complex multi-line item rendering in the list. All items must fit on exactly one line to keep viewport math reliable.
- Extensive theme customization: We will hardcode a standard, high-contrast ANSI palette for this iteration.

## Decisions

**1. Sliding Viewport Math**
- **Decision:** The `render` loop will account for a specific `list_rows` limit. `list_rows` equals total `rows` minus the header (2 rows) and minus the preview box (if toggled on). 
- **Rationale:** A simple `scroll_offset` variable can be updated whenever the user's `selection_index` goes outside the `[scroll_offset, scroll_offset + list_rows - 1]` range.

**2. Truncation Logic**
- **Decision:** We will measure string length using `chars().count()` instead of bytes. Tabs will have a maximum length of `cols - 2` (accounting for the `> ` prefix). Panes will have `cols - 6` (`    > ` prefix). If the limit is exceeded, we slice using `.chars().take(limit - 3)` and append `...`.

**3. On-Demand Preview Box**
- **Decision:** A new field `show_preview: bool` will be added to `PluginState`. Toggled via `Ctrl+e` (since `Ctrl+p` might be used for other things, `e` for expand is safer). When true, the bottom 3 lines of the terminal are reserved: a `───` divider, and up to 2 lines of the un-truncated selected item's text.
- **Rationale:** This completely avoids the complexity of dynamic multi-line item heights inside the scrollable list.

**4. ANSI Color Hierarchy**
- **Decision:** Use ANSI escape codes directly in the output format strings. 
  - Tabs (unselected): `\x1b[36m` (Cyan)
  - Panes (unselected): `\x1b[90m` (Bright Black / Dim Gray)
  - Selected (any): `\x1b[32m` (Green)
- **Rationale:** Provides immediate visual scanning benefits. Cyan stands out as a structural element, dim gray recedes, and green highlights focus.

## Risks / Trade-offs

- **Risk:** `Ctrl+e` might conflict with a user's terminal muscle memory (e.g., Emacs-style 'end of line'). 
  - **Mitigation:** In Zellij's plugin mode, keystrokes are intercepted by the plugin first. If this is highly problematic, we can consider an alternative like `Ctrl+p` or `Alt+p`. For now, we will proceed with `Ctrl+e`.
- **Risk:** Wide-character (CJK) strings breaking the `cols` math. 
  - **Mitigation:** Standard Rust `.chars().count()` handles Unicode characters, but not strictly East Asian width. For the scope of this bug, `chars()` is a sufficient initial fix, though wide characters may still slightly overflow in edge cases.
