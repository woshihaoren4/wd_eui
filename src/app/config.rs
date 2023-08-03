use eframe::egui;
use eframe::egui::{Context, TextEdit, Ui};
use std::sync::Arc;
use wd_tools::sync::CopyLock;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ConfigEntity {
    pub http_url: String,
    pub grpc_url: String,
    pub node_code: String,
    pub etcd_url: String,
    pub task_id: String,
}

impl Default for ConfigEntity {
    fn default() -> Self {
        Self {
            http_url: String::from("http://127.0.0.1:6789"),
            grpc_url: String::from(""),
            node_code: wd_tools::snowflake_id().to_string(),
            etcd_url: String::new(),
            task_id: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigTab {
    cfg: Arc<CopyLock<ConfigEntity>>,
}

impl ConfigTab {
    pub fn new(cfg: Arc<CopyLock<ConfigEntity>>) -> Self {
        Self { cfg }
    }
}

impl super::Tab for ConfigTab {
    fn name(&self) -> &'static str {
        "配置"
    }

    fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        egui::Grid::new("config")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                let raw_cfg = self.cfg.share();

                let mut cfg = (&*raw_cfg).clone();
                ui.label("HTTP PROXY:")
                    .on_hover_text("如果启用了http转gRPC代理，这里填入代理地址");
                ui.add(
                    TextEdit::singleline(&mut cfg.http_url).hint_text("rust-grpc-proxy address"),
                );
                ui.end_row();

                ui.label("GRPC URL:")
                    .on_hover_text("这里填入任意协调节点的grpc地址，当前不支持gRPC模式");
                ui.add(TextEdit::singleline(&mut cfg.grpc_url).hint_text("grpc address"));
                ui.end_row();

                ui.label("节点编号:")
                    .on_hover_text("如果启用了http转gRPC代理，这里填入代理地址");
                ui.horizontal(|ui| {
                    ui.add(
                        TextEdit::singleline(&mut cfg.node_code)
                            .desired_width(150.0)
                            .hint_text("rust-grpc-proxy address"),
                    );
                    if ui.button("随机生成").clicked() {
                        cfg.node_code = wd_tools::snowflake_id().to_string();
                    }
                });
                ui.end_row();

                ui.label("协调TaskId:").on_hover_text("协调任务的task id");
                ui.add(TextEdit::singleline(&mut cfg.task_id).hint_text("task id"));
                ui.end_row();

                if !(&*raw_cfg).eq(&cfg) {
                    self.cfg.update(|_| cfg);
                }
            });
    }
}
