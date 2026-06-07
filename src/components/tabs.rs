use gpui::*;

pub struct Tabs;

impl Render for Tabs {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Tabs")
    }
}
