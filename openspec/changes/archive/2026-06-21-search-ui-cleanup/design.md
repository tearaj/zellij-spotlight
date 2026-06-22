## Context

The current `zellij-search` plugin renders search results using box-drawing characters and redundant tab information, making the UI feel cluttered. Additionally, the search mode indicator (e.g., `[Mode: Tab & Pane]`) is separated from the search prompt by tabs (`\t\t`), which causes the mode text to jump around the screen dynamically as the user types their search query.

## Goals / Non-Goals

**Goals:**
- Implement a stable terminal UI layout where the mode indicator does not shift.
- Simplify the visual hierarchy by removing box-drawing characters and using plain indentation.
- Remove redundant tab labels.

**Non-Goals:**
- Altering the core search logic or matching algorithm.
- Modifying how navigation between results works.

## Decisions

- **Left/Right split for Prompt and Mode**:
  - We will render the search prompt (`Find > query_`) on the left and pin the mode (e.g., `[TAB & PANE]`) to the far right edge of the terminal width. This structurally prevents the mode from shifting as the query string length changes.
  - *Alternative Considered*: Placing the mode on a separate line. Rejected because it unnecessarily wastes vertical real estate in a constrained terminal environment.
- **Minimal Indentation**:
  - We will remove tree-drawing characters (`├──▶`, `└──`, `▼`). Tabs will be printed directly, and child panes will be indented uniformly (e.g., 2 spaces). A simple `> ` or `* ` prefix will indicate the active pane.
  - *Alternative Considered*: A flattened breadcrumb view (`Tab > Pane`). Rejected as it creates too much horizontal noise when multiple panes belong to the same tab.

## Risks / Trade-offs

- **[Risk]** Terminal width calculation for right-aligning the mode indicator might be inaccurate, causing text wrapping or misalignment.
  - **Mitigation**: Rely on Zellij's provided terminal columns/rows from the rendering context rather than hardcoded widths or tabs.
