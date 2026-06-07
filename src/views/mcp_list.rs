use gpui::*;

pub struct MCPList;

impl Render for MCPList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("MCPList")
    }
}
