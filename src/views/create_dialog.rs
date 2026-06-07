use gpui::*;

pub struct CreateDialog;

impl Render for CreateDialog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("CreateDialog")
    }
}
