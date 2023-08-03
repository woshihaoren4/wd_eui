#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::infra::HttpPool;
use eframe::egui::vec2;
use eframe::IconData;

mod app;
mod infra;

const ICON: &'static [u8] = include_bytes!("../logo.png");

fn main() -> eframe::Result<()> {
    #[cfg(not(target_os = "windows"))]
    {
        use std::env;
        use std::process::Command;

        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            let child = Command::new(&args[0])
                .arg("run")
                .spawn()
                .expect("Child process failed to start.");
            println!("app pid: {}", child.id());
            return Ok(());
        }
    }

    let (hp, hc) = HttpPool::new_init();

    infra::Runtime::new().add_task(hp.task_handle()).run();

    let opt = eframe::NativeOptions {
        initial_window_size: Some(vec2(400.0, 300.0)),
        icon_data: Some(IconData::try_from_png_bytes(ICON).unwrap()),
        ..Default::default()
    };
    eframe::run_native(
        "♻️coordinate test app ♻️",
        opt,
        Box::new(|x| Box::new(app::App::new(x, hc))),
    )
}
