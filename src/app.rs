use eframe::egui;

pub struct App;

impl App {
    pub fn run(title: &str) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1024.0, 768.0])
                .with_position([448.0, 156.0 - 50.0]),
            ..Default::default()
        };

        // Our application state:
        let mut name = "Arthur".to_owned();
        let mut age = 42;

        eframe::run_simple_native(title, options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("My egui Application");
                ui.horizontal(|ui| {
                    let name_label = ui.label("Your name: ");
                    ui.text_edit_singleline(&mut name)
                        .labelled_by(name_label.id);
                });
                ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                if ui.button("Increment").clicked() {
                    age += 1;
                }
                ui.label(format!("Hello '{name}', age {age}"));
            });
        })
    }
}
