use gpui::*;

pub struct FilterBar;

impl Render for FilterBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("FilterBar")
    }
}
