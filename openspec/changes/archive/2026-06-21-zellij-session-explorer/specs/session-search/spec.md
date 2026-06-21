## ADDED Requirements

### Requirement: Search Pane and Tab Titles
The plugin SHALL allow the user to filter the visible list of tabs and panes based on a search query.

#### Scenario: User types a query matching a pane
- **WHEN** the user types characters into the search field
- **THEN** the view updates to show only tabs that match the query OR tabs that contain panes matching the query, with non-matching panes hidden.

#### Scenario: User toggles search mode
- **WHEN** the user presses the `Tab` key
- **THEN** the search mode toggles between "Tab & Pane titles", "Tab titles only", and "Pane titles only", and the filtered results update immediately.

### Requirement: Navigate and Select Panes
The plugin SHALL allow keyboard navigation through the filtered list, allowing selection of panes only.

#### Scenario: User navigates the list
- **WHEN** the user presses the `Up` or `Down` arrow keys
- **THEN** the selection index moves to the previous or next pane in the list, skipping over any rows that represent a Tab header.

#### Scenario: User focuses a pane
- **WHEN** the user presses the `Enter` key on a selected pane
- **THEN** the plugin sends a `FocusPane` (and potentially `SwitchToTab` if the pane is in a different tab) command to Zellij, and the plugin overlay may be dismissed.
