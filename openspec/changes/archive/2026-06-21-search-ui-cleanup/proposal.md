## Why

The current search view UI is cluttered, uses emojis (🔍), and has layout stability issues where typing pushes the mode indicator off-screen. It also wastes horizontal space with heavy box-drawing characters and redundant tab information. A sleeker, minimal UI will improve readability and provide a more professional CLI user experience.

## What Changes

- Update the search prompt to use `Find >` instead of `🔍`.
- Pin the search mode indicator (e.g., `[TAB & PANE]`) to the far right of the terminal to prevent shifting while typing.
- Remove tree box-drawing characters (`├──▶`, `└──`, `▼`) from the results list.
- Simplify pane indentation to just a few spaces underneath their respective tabs.
- Remove redundant tab index information from the result headers (displaying only the tab name).

## Capabilities

### New Capabilities
None.

### Modified Capabilities
- `session-search`: The visual layout and formatting of the search view and results are being updated for minimalism and stability.
