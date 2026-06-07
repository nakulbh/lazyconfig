use gpui::*;

pub struct DetailPanel;

impl Render for DetailPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("DetailPanel")
    }
}
