use std::sync::Arc;
use eframe::egui::vec2;
use crate::infra::HttpPool;

mod app;
mod infra;

fn main() -> eframe::Result<()> {

    let (hp,hc) =HttpPool::new_init();

    infra::Runtime::new().add_task(hp.task_handle()).run();

    let opt = eframe::NativeOptions{
        initial_window_size : Some(vec2(400.0,300.0)),
        ..Default::default()
    };
    eframe::run_native("♻️this is a test app ♻️",opt,Box::new(|x|{
        Box::new(app::App::new(x,hc))
    }))
}