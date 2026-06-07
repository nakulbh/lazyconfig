use gpui::*;

pub struct SplitView;

impl Render for SplitView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("SplitView")
    }
}
