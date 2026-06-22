## MODIFIED Requirements

### Requirement: Minimal Terminal UI Layout
The plugin SHALL render a stable, minimal terminal UI for the search view that minimizes horizontal and vertical space usage and strictly enforces terminal viewport bounds.

#### Scenario: User types a search query
- **WHEN** the user types characters into the search field
- **THEN** the search prompt updates, but the search mode indicator remains statically pinned to the right side of the screen, preventing layout shift.

#### Scenario: System renders search results
- **WHEN** the system displays matched tabs and panes
- **THEN** the results are rendered using minimal space indentation without box-drawing tree characters, and tab names are displayed without redundant index labels.

#### Scenario: Display exceeds terminal rows
- **WHEN** the number of search results exceeds the available terminal rows
- **THEN** the UI enforces a sliding viewport to keep the search bar fixed and the currently selected item visible within bounds.

#### Scenario: Names exceed terminal columns
- **WHEN** a tab or pane name exceeds the available terminal columns
- **THEN** the UI truncates the name with an ellipsis (`...`) so it occupies exactly one row.

#### Scenario: Items are rendered
- **WHEN** the UI draws tabs and panes
- **THEN** it applies distinct ANSI colors (e.g., Cyan for Tabs, Dim Gray for Panes, Green for Selection) to differentiate item types and selection state.

## ADDED Requirements

### Requirement: On-Demand Details Preview
The plugin SHALL provide a mechanism to view the full, un-truncated text of the currently selected item.

#### Scenario: User toggles preview box
- **WHEN** the user presses the `Ctrl+e` shortcut
- **THEN** the UI toggles a reserved preview box at the bottom of the screen displaying the full text of the currently selected item.
