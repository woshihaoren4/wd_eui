use crate::app::config::ConfigEntity;
use crate::app::entity::{Coordinate, TaskEntity};
use crate::app::tools;
use eframe::egui;
use eframe::egui::{Align, Color32, Context, Layout, Sense, Ui, WidgetText};
use std::sync::Arc;
use wd_tools::sync::{Acl, CopyLock};

pub struct NodeTab {
    tasks: Arc<CopyLock<Vec<TaskEntity>>>,
    coordinate: Coordinate,
    select_task_code: String,
    select_task_id: String,
    cfg: Arc<CopyLock<ConfigEntity>>,
    info: Acl<String>,
    show_mul_info: bool,
    // update_slot : bool,
    ping: bool,
    timepiece: i64,
    update_time: i64,
    ping_time: i64,
    tags: Acl<Vec<i32>>,
}

impl NodeTab {
    pub fn new(
        coordinate: Coordinate,
        cfg: Arc<CopyLock<ConfigEntity>>,
        tasks: Arc<CopyLock<Vec<TaskEntity>>>,
    ) -> Self {
        let select_task_code = String::new();
        let select_task_id = String::new();
        let info = Acl::new(String::new());
        // let update_slot = true;
        let timepiece = wd_tools::time::utc_timestamp();
        let tags = Acl::new(Vec::new());
        let ping_time = 0;
        Self {
            cfg,
            select_task_id,
            coordinate,
            tasks,
            info,
            select_task_code,
            timepiece,
            ping_time,
            tags,
            show_mul_info: false,
            ping: true,
            update_time: 0,
        }
    }
    pub fn join_task(&mut self) {
        if self.select_task_id.is_empty() {
            self.info
                .update(|_| "加入节点失败，任务编码不能为空，请先刷新任务列表".into());
            return;
        }
        let info = self.info.clone();
        self.coordinate.join_task(
            self.select_task_id.clone(),
            self.cfg.share().node_code.clone(),
            move |result| match result {
                Ok(o) => {
                    info.update(|_| format!("join task success -> {}", o));
                }
                Err(err) => info.update(|_| format!("join task false:{}", err)),
            },
        );
    }
}
impl super::Tab for NodeTab {
    fn name(&self) -> &'static str {
        "节点"
    }

    fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        let cfg = self.cfg.share();
        self.timepiece = wd_tools::time::utc_timestamp();
        if ui.button("将当前节点加入任务").clicked() {
            self.join_task();
        }
        egui::Grid::new("node mg")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("node code:");
                ui.label(cfg.node_code.as_str());
                ui.end_row();
                ui.label("task code:");
                let ts = self.tasks.share();
                let stc = &mut self.select_task_code;
                if ts.len() > 0 {
                    if stc.is_empty() {
                        *stc = ts[0].name.clone();
                        self.select_task_id = ts[0].id.clone();
                    }
                    egui::ComboBox::from_label("选择一个 task code")
                        .selected_text(stc.as_str())
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            ui.set_min_width(60.0);
                            for t in ts.iter() {
                                if ui
                                    .selectable_value(stc, t.name.clone(), t.name.as_str())
                                    .clicked()
                                {
                                    self.select_task_id = t.id.clone()
                                }
                            }
                        });
                } else {
                    ui.label(WidgetText::from("任务为空").color(Color32::RED))
                        .on_hover_text("在任务列表中刷新，或新建任务");
                }
                ui.end_row();
                if self.ping_time == 0 {
                    self.ping_time = self.timepiece;
                }
                ui.label(format!(
                    "保持心跳({})",
                    60 - (self.timepiece - self.ping_time)
                ));
                tools::toggle_ui(ui, &mut self.ping);
                if self.timepiece - self.ping_time > 15 {
                    self.ping_time = self.timepiece;
                    if self.ping && !self.select_task_id.is_empty() {
                        let info = self.info.clone();
                        println!(
                            "--> {} {}",
                            self.select_task_id.clone(),
                            self.cfg.share().node_code.clone()
                        );
                        self.coordinate.ping(
                            self.select_task_id.clone(),
                            self.cfg.share().node_code.clone(),
                            move |result| match result {
                                Ok(o) => info.update(|_| format!("ping success -> version:{}", o)),
                                Err(err) => info.update(|_| format!("ping error:{}", err)),
                            },
                        );
                    }
                }
                ui.end_row();
                if self.update_time == 0 || self.update_time < self.timepiece - 30 {
                    //需要刷新
                    let tags = self.tags.clone();
                    let info = self.info.clone();
                    self.coordinate.node_tags(
                        self.select_task_id.clone(),
                        self.cfg.share().node_code.clone(),
                        move |result| match result {
                            Ok(ts) => {
                                tags.update(|_| ts);
                                info.update(|_| "update node tag success".to_string());
                            }
                            Err(err) => {
                                info.update(move |_| format!("update error:{}", err));
                            }
                        },
                    );
                    self.update_time = self.timepiece;
                }

                ui.label(format!(
                    "工作标签({})",
                    (self.timepiece - self.update_time) % 30
                ));
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        for i in self.tags.share().iter() {
                            ui.add(egui::Label::new(
                                WidgetText::from(i.to_string())
                                    .color(Color32::BLACK)
                                    .background_color(Color32::GREEN),
                            ));
                        }
                    });
                });
                ui.end_row();
            });
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            let info = self.info.share();
            let hover_text = if self.show_mul_info {
                "click -> show brief info"
            } else {
                "click -> show detail info"
            };
            if ui
                .add(
                    egui::Label::new(info.as_str())
                        .wrap(self.show_mul_info)
                        .sense(Sense::click()),
                )
                .on_hover_text(hover_text)
                .clicked()
            {
                self.show_mul_info = !self.show_mul_info
            }
            ui.separator();
        });
        ctx.request_repaint();
    }
}
