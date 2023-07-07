use eframe::egui::vec2;

mod app;

fn main() -> eframe::Result<()> {
    let opt = eframe::NativeOptions{
        initial_window_size : Some(vec2(400.0,300.0)),
        ..Default::default()
    };
    eframe::run_native("this is a test app",opt,Box::new(|x|{
        Box::new(app::App::new(x))
    }))
}