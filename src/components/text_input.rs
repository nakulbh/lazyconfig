use gpui::*;

pub struct TextInput;

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("TextInput")
    }
}
