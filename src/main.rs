// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::RichText;

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                if ui.button(RichText::new("chang")).clicked() {
                    println!("唱");
                }
                if ui.button(RichText::new("tiao")).clicked() {
                    println!("跳");
                }
                if ui.button(RichText::new("rap")).clicked() {
                    println!("rap");
                }
                //
                // let name_label = ui.label("Your name: ");
                // ui.text_edit_singleline(&mut name)
                //     .labelled_by(name_label.id);
            });
            ui.separator();
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}