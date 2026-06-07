use gpui::*;

pub struct Badge;

impl Render for Badge {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Badge")
    }
}
