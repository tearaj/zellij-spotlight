## 1. Refactor Prompt Layout

- [x] 1.1 Update the search prompt string to use `Find >` instead of the emoji `🔍`
- [x] 1.2 Retrieve terminal columns width from the Zellij render context to calculate the padding space
- [x] 1.3 Modify the top bar rendering to correctly pad spaces so the prompt is left-aligned and the mode indicator is right-aligned

## 2. Refactor Search Results Rendering

- [x] 2.1 Remove all box-drawing characters (`├──▶`, `└──`, `▼`) from the results output logic
- [x] 2.2 Update tab header rendering to only display the tab name (removing redundant tab index identifiers)
- [x] 2.3 Update pane rendering to use a simple uniform indentation (e.g., 2 spaces) beneath tabs
- [x] 2.4 Verify that the active selection cursor (`> `) is clearly visible and correctly positioned with the new indentation
