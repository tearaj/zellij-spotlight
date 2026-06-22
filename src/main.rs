mod action;
mod filter;
mod input;
mod render;

use std::collections::BTreeMap;
use zellij_tile::prelude::*;

use filter::{DEFAULT_EXCLUSIONS, FilteredItem, SearchMode, compute_filtered, compute_selectable};
use input::InputAction;

struct CachedResults {
    items: Vec<FilteredItem>,
    selectable: Vec<usize>,
}

struct PluginState {
    tabs: Vec<TabInfo>,
    panes: PaneManifest,
    search_query: String,
    search_mode: SearchMode,
    selection_index: usize,
    scroll_offset: usize,
    show_preview: bool,
    exclusions: Vec<String>,
    cached: Option<CachedResults>,
}

impl Default for PluginState {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            panes: PaneManifest::default(),
            search_query: String::new(),
            search_mode: SearchMode::default(),
            selection_index: 0,
            scroll_offset: 0,
            show_preview: false,
            exclusions: DEFAULT_EXCLUSIONS.iter().map(|s| s.to_string()).collect(),
            cached: None,
        }
    }
}

impl PluginState {
    fn invalidate_cache(&mut self) {
        self.cached = None;
    }

    fn ensure_cache(&mut self) {
        if self.cached.is_none() {
            let items = compute_filtered(
                &self.tabs,
                &self.panes,
                &self.search_query,
                &self.search_mode,
                &self.exclusions,
            );
            let selectable = compute_selectable(&items, &self.search_mode);
            self.cached = Some(CachedResults { items, selectable });
        }
    }

    fn clamp_selection(&mut self) {
        self.ensure_cache();
        let selectable = &self.cached.as_ref().unwrap().selectable;

        if selectable.is_empty() {
            self.selection_index = 0;
            return;
        }

        if !selectable.contains(&self.selection_index) {
            let mut closest = selectable[0];
            for &idx in selectable {
                if idx <= self.selection_index {
                    closest = idx;
                } else {
                    if self.selection_index == 0 {
                        closest = idx;
                    }
                    break;
                }
            }
            self.selection_index = closest;
        }
    }

    fn move_selection_up(&mut self) {
        self.ensure_cache();
        let selectable = &self.cached.as_ref().unwrap().selectable;
        let mut prev_idx = None;
        for &idx in selectable {
            if idx < self.selection_index {
                prev_idx = Some(idx);
            } else {
                break;
            }
        }
        if let Some(idx) = prev_idx {
            self.selection_index = idx;
        }
    }

    fn move_selection_down(&mut self) {
        self.ensure_cache();
        let selectable = &self.cached.as_ref().unwrap().selectable;
        for &idx in selectable {
            if idx > self.selection_index {
                self.selection_index = idx;
                break;
            }
        }
    }

    fn update_scroll(&mut self, rows: usize) {
        let mut list_rows = rows.saturating_sub(3);
        if self.show_preview {
            list_rows = list_rows.saturating_sub(3);
        }

        if list_rows > 0 {
            if self.selection_index >= self.scroll_offset + list_rows {
                self.scroll_offset = self.selection_index - list_rows + 1;
            } else if self.selection_index < self.scroll_offset {
                self.scroll_offset = self.selection_index;
            }
        } else {
            self.scroll_offset = 0;
        }
    }
}

impl PluginState {
    fn handle_action(&mut self, input: InputAction) -> bool {
        match input {
            InputAction::MoveUp => {
                self.move_selection_up();
                true
            }
            InputAction::MoveDown => {
                self.move_selection_down();
                true
            }
            InputAction::CycleMode => {
                self.search_mode = self.search_mode.cycle();
                self.invalidate_cache();
                self.clamp_selection();
                true
            }
            InputAction::Confirm => {
                self.ensure_cache();
                let item = self
                    .cached
                    .as_ref()
                    .and_then(|c| c.items.get(self.selection_index))
                    .cloned();
                if let Some(item) = item {
                    action::execute_selection(&item);
                    self.search_query.clear();
                    self.selection_index = 0;
                    self.invalidate_cache();
                    hide_self();
                }
                false
            }
            InputAction::DeleteChar => {
                self.search_query.pop();
                self.invalidate_cache();
                self.clamp_selection();
                true
            }
            InputAction::TogglePreview => {
                self.show_preview = !self.show_preview;
                true
            }
            InputAction::Cancel => {
                self.search_query.clear();
                self.selection_index = 0;
                self.invalidate_cache();
                hide_self();
                false
            }
            InputAction::TypeChar(c) => {
                self.search_query.push(c);
                self.invalidate_cache();
                self.clamp_selection();
                true
            }
        }
    }
}

register_plugin!(PluginState);

impl ZellijPlugin for PluginState {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::TabUpdate, EventType::PaneUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info;
                self.invalidate_cache();
                self.clamp_selection();
                true
            }
            Event::PaneUpdate(pane_manifest) => {
                self.panes = pane_manifest;
                self.invalidate_cache();
                self.clamp_selection();
                true
            }
            Event::Key(key) => input::map_key(&key)
                .map(|a| self.handle_action(a))
                .unwrap_or(false),
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.update_scroll(rows);
        self.ensure_cache();

        let mut list_rows = rows.saturating_sub(3);
        if self.show_preview {
            list_rows = list_rows.saturating_sub(3);
        }

        let cached = self.cached.as_ref().unwrap();
        let ctx = render::RenderContext {
            query: &self.search_query,
            items: &cached.items,
            selection_index: self.selection_index,
            scroll_offset: self.scroll_offset,
            list_rows,
            search_mode: &self.search_mode,
            cols,
            show_preview: self.show_preview,
        };
        print!("{}", render::render(&ctx));
    }
}
