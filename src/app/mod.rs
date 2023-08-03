mod about;
mod config;
mod entity;
mod font;
mod node_tab;
mod task_tab;
mod tools;

use crate::app::config::ConfigEntity;
use crate::app::entity::Coordinate;
use crate::infra::HttpClient;
use eframe::egui::{Color32, Context, Ui, Widget};
use eframe::{egui, CreationContext, Frame};
use std::sync::Arc;
use wd_tools::sync::CopyLock;

pub trait Tab {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}

pub struct TapIndex {
    name: &'static str,
}

impl Tab for TapIndex {
    fn name(&self) -> &'static str {
        self.name
    }

    fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.label(format!("this page is {}", self.name));
    }
}

pub struct App {
    tap: Vec<Box<dyn Tab>>,
    tap_index: usize,
    ctx: Option<Context>,
}

impl App {
    pub fn new(cc: &CreationContext, hc: HttpClient) -> Self {
        App::setup_custom_fonts(&cc.egui_ctx);
        let tap = vec![];
        let tap_index = 0;
        let cfg = Arc::new(CopyLock::new(ConfigEntity::default()));

        let coordinate = Coordinate::new(hc.clone(), cfg.clone());
        let tasks = Arc::new(CopyLock::new(Vec::new()));

        App {
            tap,
            tap_index,
            ctx: None,
        }
        .add_tag(about::About::default())
        .add_tag(config::ConfigTab::new(cfg.clone()))
        .add_tag(task_tab::TaskTab::new(coordinate.clone(), tasks.clone()))
        .add_tag(node_tab::NodeTab::new(coordinate, cfg, tasks))
    }
    fn add_tag<T: Tab + 'static>(mut self, tag: T) -> App {
        self.tap.push(Box::new(tag));
        self
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.ctx.is_none() {
            self.ctx = Some(ctx.clone())
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            //显示第一排tap
            ui.horizontal(|ui| {
                for (i, tap) in self.tap.iter().enumerate() {
                    let mut bt = egui::Button::new(tap.name());
                    if i == self.tap_index {
                        bt = bt.fill(Color32::GREEN);
                    }
                    if bt.ui(ui).clicked() {
                        self.tap_index = i;
                    }
                }
            });
            //绘制对应的界面
            ui.separator();
            if self.tap_index < self.tap.len() {
                self.tap[self.tap_index].show(ctx, ui);
            }
        });
    }
}
