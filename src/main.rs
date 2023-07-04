// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{Color32, RichText};

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    let mut page  = 1;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.separator();
            ui.horizontal(|ui| {
                let mut rt = RichText::new("chang");
                if page == 1 {
                    rt = rt.color(Color32::WHITE).background_color(Color32::BLUE);
                }
                if ui.button(rt).clicked() {
                    page = 1;
                }
                ui.separator();
                let mut rt = RichText::new("tiao");
                if page == 2 {
                    rt = rt.color(Color32::WHITE).background_color(Color32::BLUE);
                }
                if ui.button(rt).clicked() {
                    page = 2;
                }
                ui.separator();
                let mut rt = RichText::new("rap");
                if page == 3 {
                    rt = rt.color(Color32::WHITE).background_color(Color32::BLUE);
                }
                if ui.button(rt).clicked() {
                    page = 3;
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