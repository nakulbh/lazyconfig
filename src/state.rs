use gpui::*;

#[derive(Clone)]
pub struct AppState {
    pub active_tab: usize,
    pub selected_item: Option<String>,
    pub filter_text: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            selected_item: None,
            filter_text: String::new(),
        }
    }
}

impl Global for AppState {}
