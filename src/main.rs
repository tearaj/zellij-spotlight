use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default, Debug, PartialEq)]
enum SearchMode {
    #[default]
    TabAndPane,
    TabOnly,
    PaneOnly,
}

#[derive(Default)]
struct PluginState {
    tabs: Vec<TabInfo>,
    panes: PaneManifest,
    search_query: String,
    search_mode: SearchMode,
    selection_index: usize,
    scroll_offset: usize,
    show_preview: bool,
}

#[derive(Clone)]
enum FilteredItem {
    Tab(TabInfo),
    Pane { pane: PaneInfo, tab_position: usize },
}

impl PluginState {
    fn filtered_results(&self) -> Vec<FilteredItem> {
        let mut results = Vec::new();
        let query = self.search_query.to_lowercase();
        
        for tab in &self.tabs {
            let tab_matches = tab.name.to_lowercase().contains(&query);
            let mut matched_panes = Vec::new();
            
            if let Some(tab_panes) = self.panes.panes.get(&tab.position) {
                for pane in tab_panes {
                    // Ignore Zellij's default UI plugins and this plugin itself
                    if pane.is_plugin {
                        let lower_title = pane.title.to_lowercase();
                        if lower_title.contains("tab-bar") 
                            || lower_title.contains("status-bar") 
                            || lower_title.contains("zellij-spotlight")
                            || lower_title.contains("session-manager")
                        {
                            continue;
                        }
                    }
                    
                    let pane_matches = pane.title.to_lowercase().contains(&query);
                    let should_include_pane = match self.search_mode {
                        SearchMode::TabAndPane => pane_matches || tab_matches,
                        SearchMode::TabOnly => tab_matches,
                        SearchMode::PaneOnly => pane_matches,
                    };
                    
                    if should_include_pane {
                        matched_panes.push(pane.clone());
                    }
                }
            }
            
            let should_include_tab = match self.search_mode {
                SearchMode::TabAndPane => tab_matches || !matched_panes.is_empty(),
                SearchMode::TabOnly => tab_matches,
                SearchMode::PaneOnly => !matched_panes.is_empty(),
            };
            
            if should_include_tab {
                results.push(FilteredItem::Tab(tab.clone()));
                if self.search_mode != SearchMode::TabOnly {
                    for pane in matched_panes {
                        results.push(FilteredItem::Pane { pane, tab_position: tab.position });
                    }
                }
            }
        }
        results
    }

    fn selectable_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        for (i, item) in self.filtered_results().iter().enumerate() {
            match item {
                FilteredItem::Tab(_) if self.search_mode == SearchMode::TabOnly => indices.push(i),
                FilteredItem::Pane { .. } if self.search_mode != SearchMode::TabOnly => indices.push(i),
                _ => {}
            }
        }
        indices
    }

    fn clamp_selection(&mut self) {
        let indices = self.selectable_indices();
        if indices.is_empty() {
            self.selection_index = 0;
            return;
        }

        if !indices.contains(&self.selection_index) {
            let mut closest = indices[0];
            for &idx in &indices {
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
        let indices = self.selectable_indices();
        let mut prev_idx = None;
        for &idx in &indices {
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
        let indices = self.selectable_indices();
        for &idx in &indices {
            if idx > self.selection_index {
                self.selection_index = idx;
                break;
            }
        }
    }
}

trait ViewRenderer {
    #[allow(clippy::too_many_arguments)]
    fn render(&self, query: &str, items: &[FilteredItem], selection_index: usize, scroll_offset: usize, list_rows: usize, search_mode: &SearchMode, cols: usize, show_preview: bool) -> String;
}

struct NestedViewRenderer;

impl ViewRenderer for NestedViewRenderer {
    fn render(&self, query: &str, items: &[FilteredItem], selection_index: usize, scroll_offset: usize, list_rows: usize, search_mode: &SearchMode, cols: usize, show_preview: bool) -> String {
        let mut output = String::new();
        let mode_str = match search_mode {
            SearchMode::TabAndPane => "TAB & PANE",
            SearchMode::TabOnly => "TAB ONLY",
            SearchMode::PaneOnly => "PANE ONLY",
        };
        
        let prompt_str = format!("Find > {}_", query);
        let mode_display = format!("[{}]", mode_str);
        
        let padding_len = if cols > prompt_str.len() + mode_display.len() {
            cols - prompt_str.len() - mode_display.len()
        } else {
            1
        };
        let padding = " ".repeat(padding_len);
        
        output.push_str(&format!("{}{}{}\n", prompt_str, padding, mode_display));
        output.push_str(&format!("{}\n", "─".repeat(cols.max(1))));
        
        let visible_items = items.iter().skip(scroll_offset).take(list_rows).enumerate();
        let mut selected_full_text = String::new();

        for (i, item) in visible_items {
            let actual_idx = i + scroll_offset;
            let is_selected = actual_idx == selection_index;
            match item {
                FilteredItem::Tab(tab) => {
                    if is_selected {
                        selected_full_text = format!("Tab: {}", tab.name);
                    }
                    let limit = cols.saturating_sub(2);
                    let display_name = if tab.name.chars().count() > limit {
                        let mut t: String = tab.name.chars().take(limit.saturating_sub(3)).collect();
                        t.push_str("...");
                        t
                    } else {
                        tab.name.clone()
                    };

                    if is_selected {
                        output.push_str(&format!("> \x1b[32m{}\x1b[0m\n", display_name));
                    } else {
                        output.push_str(&format!("  \x1b[36m{}\x1b[0m\n", display_name));
                    }
                }
                FilteredItem::Pane { pane, .. } => {
                    if is_selected {
                        selected_full_text = format!("Pane: {}", pane.title);
                    }
                    let limit = cols.saturating_sub(6);
                    let display_name = if pane.title.chars().count() > limit {
                        let mut t: String = pane.title.chars().take(limit.saturating_sub(3)).collect();
                        t.push_str("...");
                        t
                    } else {
                        pane.title.clone()
                    };

                    if is_selected {
                        output.push_str(&format!("    > \x1b[32m{}\x1b[0m\n", display_name));
                    } else {
                        output.push_str(&format!("      \x1b[90m{}\x1b[0m\n", display_name));
                    }
                }
            }
        }

        if show_preview {
            let limit = cols.saturating_sub(2);
            let display_preview = if selected_full_text.chars().count() > limit * 2 {
                let mut t: String = selected_full_text.chars().take((limit * 2).saturating_sub(3)).collect();
                t.push_str("...");
                t
            } else {
                selected_full_text
            };
            output.push_str(&format!("{}\n", "─".repeat(cols.max(1))));
            output.push_str(&format!("  \x1b[1m{}\x1b[0m\n", display_preview));
        }

        output
    }
}

register_plugin!(PluginState);

impl ZellijPlugin for PluginState {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::TabUpdate,
            EventType::PaneUpdate,
            EventType::Key,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info;
                self.clamp_selection();
                should_render = true;
            }
            Event::PaneUpdate(pane_manifest) => {
                self.panes = pane_manifest;
                self.clamp_selection();
                should_render = true;
            }
            Event::Key(key) => {
                let has_ctrl = key.key_modifiers.contains(&KeyModifier::Ctrl);
                match key.bare_key {
                    BareKey::Up => {
                        self.move_selection_up();
                        should_render = true;
                    }
                    BareKey::Down => {
                        self.move_selection_down();
                        should_render = true;
                    }
                    BareKey::Tab => {
                        self.search_mode = match self.search_mode {
                            SearchMode::TabAndPane => SearchMode::TabOnly,
                            SearchMode::TabOnly => SearchMode::PaneOnly,
                            SearchMode::PaneOnly => SearchMode::TabAndPane,
                        };
                        self.clamp_selection();
                        should_render = true;
                    }
                    BareKey::Enter => {
                        let results = self.filtered_results();
                        if let Some(item) = results.get(self.selection_index) {
                            match item {
                                FilteredItem::Pane { pane, tab_position } => {
                                    switch_tab_to(*tab_position as u32 + 1);
                                    if pane.is_plugin {
                                        focus_plugin_pane(pane.id, true, false);
                                    } else {
                                        focus_terminal_pane(pane.id, true, false);
                                    }
                                }
                                FilteredItem::Tab(tab) => {
                                    switch_tab_to(tab.position as u32 + 1);
                                }
                            }
                            self.search_query.clear();
                            self.selection_index = 0;
                            hide_self();
                        }
                    }
                    BareKey::Backspace => {
                        self.search_query.pop();
                        self.clamp_selection();
                        should_render = true;
                    }
                    BareKey::Char('c') if has_ctrl => {
                        self.search_query.clear();
                        self.selection_index = 0;
                        hide_self();
                    }
                    BareKey::Char('e') if has_ctrl => {
                        self.show_preview = !self.show_preview;
                        should_render = true;
                    }
                    BareKey::Char(c) if !has_ctrl => {
                        self.search_query.push(c);
                        self.clamp_selection();
                        should_render = true;
                    }
                    BareKey::Esc => {
                        self.search_query.clear();
                        self.selection_index = 0;
                        hide_self();
                    }
                    _ => {}
                }
            }
            _ => (),
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let items = self.filtered_results();
        
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

        let renderer = NestedViewRenderer;
        let view_string = renderer.render(
            &self.search_query, 
            &items, 
            self.selection_index, 
            self.scroll_offset,
            list_rows,
            &self.search_mode, 
            cols,
            self.show_preview
        );
        print!("{}", view_string);
    }
}
