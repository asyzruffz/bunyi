use eframe::egui::Context;

pub trait View {
    fn show(&mut self, ctx: &Context);
}
