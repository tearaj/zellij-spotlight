use zellij_tile::prelude::*;

use crate::filter::FilteredItem;

pub fn execute_selection(item: &FilteredItem) {
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
}
