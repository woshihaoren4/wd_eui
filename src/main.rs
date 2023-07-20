use std::sync::Arc;
use eframe::egui::vec2;
use eframe::IconData;
use crate::infra::HttpPool;

mod app;
mod infra;

const ICON:&'static [u8] = include_bytes!("../logo.png");

fn main() -> eframe::Result<()> {

    let (hp,hc) =HttpPool::new_init();

    infra::Runtime::new().add_task(hp.task_handle()).run();

    let opt = eframe::NativeOptions{
        initial_window_size : Some(vec2(400.0,300.0)),
        icon_data:Some(IconData::try_from_png_bytes(ICON).unwrap()),
        ..Default::default()
    };
    eframe::run_native("♻️this is a test app ♻️",opt,Box::new(|x|{
        Box::new(app::App::new(x,hc))
    }))
}