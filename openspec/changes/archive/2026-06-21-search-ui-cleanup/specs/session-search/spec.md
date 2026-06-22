## ADDED Requirements

### Requirement: Minimal Terminal UI Layout
The plugin SHALL render a stable, minimal terminal UI for the search view that minimizes horizontal and vertical space usage.

#### Scenario: User types a search query
- **WHEN** the user types characters into the search field
- **THEN** the search prompt updates, but the search mode indicator remains statically pinned to the right side of the screen, preventing layout shift.

#### Scenario: System renders search results
- **WHEN** the system displays matched tabs and panes
- **THEN** the results are rendered using minimal space indentation without box-drawing tree characters, and tab names are displayed without redundant index labels.
