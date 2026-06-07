use gpui::*;

pub struct TextArea;

impl Render for TextArea {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("TextArea")
    }
}
