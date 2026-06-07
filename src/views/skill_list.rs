use gpui::*;

pub struct SkillList;

impl Render for SkillList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("SkillList")
    }
}
