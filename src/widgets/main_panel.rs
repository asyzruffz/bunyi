use eframe::egui::{self, Context};

use crate::widgets::view::View;

#[derive(Default)]
pub struct MainPanel {
    name: String,
    age: i32,
}

impl View for MainPanel {
    fn show(&mut self, ctx: &Context) {
        let Self { name, age } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                *age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    }
}
