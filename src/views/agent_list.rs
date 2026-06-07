use gpui::*;

pub struct AgentList;

impl Render for AgentList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("AgentList")
    }
}
