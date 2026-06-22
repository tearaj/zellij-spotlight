use zellij_tile::prelude::*;

#[derive(Default, Debug, PartialEq)]
pub enum SearchMode {
    #[default]
    TabAndPane,
    TabOnly,
    PaneOnly,
}

impl SearchMode {
    pub fn cycle(&self) -> Self {
        match self {
            SearchMode::TabAndPane => SearchMode::TabOnly,
            SearchMode::TabOnly => SearchMode::PaneOnly,
            SearchMode::PaneOnly => SearchMode::TabAndPane,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SearchMode::TabAndPane => "TAB & PANE",
            SearchMode::TabOnly => "TAB ONLY",
            SearchMode::PaneOnly => "PANE ONLY",
        }
    }
}

#[derive(Clone)]
pub enum FilteredItem {
    Tab(TabInfo),
    Pane { pane: PaneInfo, tab_position: usize },
}

pub const DEFAULT_EXCLUSIONS: &[&str] = &[
    "tab-bar",
    "status-bar",
    "zellij-spotlight",
    "session-manager",
];

fn is_excluded_plugin(pane: &PaneInfo, exclusions: &[String]) -> bool {
    if !pane.is_plugin {
        return false;
    }
    let lower_title = pane.title.to_lowercase();
    exclusions.iter().any(|e| lower_title.contains(e.as_str()))
}

pub fn compute_filtered(
    tabs: &[TabInfo],
    panes: &PaneManifest,
    query: &str,
    mode: &SearchMode,
    exclusions: &[String],
) -> Vec<FilteredItem> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    for tab in tabs {
        let tab_matches = tab.name.to_lowercase().contains(&query_lower);
        let mut matched_panes = Vec::new();

        if let Some(tab_panes) = panes.panes.get(&tab.position) {
            for pane in tab_panes {
                if is_excluded_plugin(pane, exclusions) {
                    continue;
                }

                let pane_matches = pane.title.to_lowercase().contains(&query_lower);
                let should_include = match mode {
                    SearchMode::TabAndPane => pane_matches || tab_matches,
                    SearchMode::TabOnly => tab_matches,
                    SearchMode::PaneOnly => pane_matches,
                };

                if should_include {
                    matched_panes.push(pane.clone());
                }
            }
        }

        let should_include_tab = match mode {
            SearchMode::TabAndPane => tab_matches || !matched_panes.is_empty(),
            SearchMode::TabOnly => tab_matches,
            SearchMode::PaneOnly => !matched_panes.is_empty(),
        };

        if should_include_tab {
            results.push(FilteredItem::Tab(tab.clone()));
            if *mode != SearchMode::TabOnly {
                for pane in matched_panes {
                    results.push(FilteredItem::Pane {
                        pane,
                        tab_position: tab.position,
                    });
                }
            }
        }
    }
    results
}

pub fn compute_selectable(results: &[FilteredItem], mode: &SearchMode) -> Vec<usize> {
    results
        .iter()
        .enumerate()
        .filter_map(|(i, item)| match item {
            FilteredItem::Tab(_) if *mode == SearchMode::TabOnly => Some(i),
            FilteredItem::Pane { .. } if *mode != SearchMode::TabOnly => Some(i),
            _ => None,
        })
        .collect()
}
