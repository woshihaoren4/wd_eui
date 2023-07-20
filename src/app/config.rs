use std::sync::atomic::AtomicPtr;
use eframe::egui;
use eframe::egui::{Context, TextEdit, Ui};

#[derive(Debug)]
pub struct Config{
    pub http_url:String,
    pub grpc_url:String,
    pub node_code: String,
    pub etcd_url: String,
    pub task_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Self{
            http_url: "http://127.0.0.1:666".into(),
            etcd_url: String::new(),
            grpc_url: String::new(),
            node_code: wd_tools::snowflake_id().to_string(),
            task_id: String::new(),
        }
    }
}

impl super::Tap for Config{
    fn name(&self) -> &'static str {
        "配置"
    }

    fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        egui::Grid::new("config")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui,|ui|{

                ui.label("HTTP PROXY:").on_hover_text("如果启用了http转gRPC代理，这里填入代理地址");
                ui.add(TextEdit::singleline(&mut self.http_url).hint_text("rust-grpc-proxy address"));
                ui.end_row();

                ui.label("GRPC URL:").on_hover_text("这里填入任意协调节点的grpc地址，当前不支持gRPC模式");
                ui.add(TextEdit::singleline(&mut self.grpc_url).hint_text("grpc address"));
                ui.end_row();

                ui.label("节点编号:").on_hover_text("如果启用了http转gRPC代理，这里填入代理地址");
                ui.horizontal(|ui|{
                    ui.add(TextEdit::singleline(&mut self.node_code).desired_width(150.0).hint_text("rust-grpc-proxy address"));
                    if ui.button("随机生成").clicked() {
                        self.node_code = wd_tools::snowflake_id().to_string();
                    }
                });
                ui.end_row();

                ui.label("协调TaskId:").on_hover_text("协调任务的task id");
                ui.add(TextEdit::singleline(&mut self.task_id).hint_text("task id"));
                ui.end_row();

            });

    }
}