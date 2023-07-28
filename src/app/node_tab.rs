use std::sync::Arc;
use eframe::egui;
use eframe::egui::{Align, Color32, Context, Layout, Sense, Ui, WidgetText};
use wd_tools::sync::CopyLock;
use crate::app::config::ConfigEntity;
use crate::app::entity::TaskEntity;
use crate::app::tools;

pub struct NodeTab {
    tasks : Arc<CopyLock<Vec<TaskEntity>>>,
    select_task_code: String,
    cfg:Arc<CopyLock<ConfigEntity>>,
    info: Arc<String>,
    show_mul_info : bool,
    update_slot : bool,
    ping : bool,
    timepiece: i64,

}

impl NodeTab {
    pub fn new(cfg:Arc<CopyLock<ConfigEntity>>,tasks : Arc<CopyLock<Vec<TaskEntity>>>)->Self{
        let select_task_code = String::new();
        let info = Arc::new(String::new());
        let update_slot = true;
        let timepiece = wd_tools::time::utc_timestamp();
        Self{cfg,tasks,info,select_task_code,update_slot,timepiece,show_mul_info:false,ping:true}
    }
}
impl super::Tab for NodeTab {
    fn name(&self) -> &'static str {
        "节点"
    }

    fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        let cfg = self.cfg.share();
        self.timepiece = wd_tools::time::utc_timestamp();
        egui::Grid::new("node mg")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui,|ui|{
                ui.label("node code:");
                ui.label(cfg.node_code.as_str());
                ui.end_row();
                ui.label("task code:");
                let ts = self.tasks.share();
                let stc = &mut self.select_task_code;
                if ts.len() > 0 {
                    if stc.is_empty() {
                        *stc = ts[0].name.clone();
                    }
                    egui::ComboBox::from_label("选择一个 task code")
                        .selected_text(stc.as_str())
                        .show_ui(ui,|ui|{
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            for t in ts.iter() {
                                if ui.selectable_value(stc, t.name.clone(), t.name.as_str()).clicked(){
                                    wd_log::log_debug_ln!("当前的值是：{}",stc);
                                }
                            }
                        });
                }else{
                    ui.label(WidgetText::from("任务为空").color(Color32::RED)).on_hover_text("在任务列表中刷新，或新建任务");
                }
                ui.end_row();
                ui.label("监控工作分配");
                tools::toggle_ui(ui,&mut self.update_slot);
                ui.end_row();
                ui.label("保持心跳");
                tools::toggle_ui(ui,&mut self.ping);
                ui.end_row();
                ui.label("节点工作()")

            });
        ui.with_layout(Layout::bottom_up(Align::Min),|ui|{
            let info = self.info.clone();
            let hover_text = if self.show_mul_info {
                "click -> show brief info"
            }else{
                "click -> show detail info"
            };
            if ui.add(egui::Label::new(info.as_str()).wrap(self.show_mul_info).sense(Sense::click())).on_hover_text(hover_text).clicked() {
                self.show_mul_info = !self.show_mul_info
            }
            ui.separator();
        });
        ctx.request_repaint();
    }
}