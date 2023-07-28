use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::Duration;
use eframe::egui;
use eframe::egui::{Align, CollapsingHeader, Color32, Context, Layout, Sense, Ui, WidgetText};
use hyper::{StatusCode, Uri};
use crate::app::config::ConfigEntity;
use crate::app::entity::{Coordinate, TaskEntity};
use crate::infra::async_http_pool::*;
use wd_tools::sync::CopyLock;

pub struct TaskTab {
    tasks :  Arc<CopyLock<Vec<TaskEntity>>>,
    coordinate: Coordinate,
    cfg : Arc<CopyLock<ConfigEntity>>,
    index : usize,
    task:TaskEntity,
    info: Arc<CopyLock<String>>,
    show_mul_info : bool,
}

impl TaskTab {
    pub fn new(coordinate: Coordinate,cfg:Arc<CopyLock<ConfigEntity>>,tasks : Arc<CopyLock<Vec<TaskEntity>>>)->Self{

        let mut task = TaskEntity::default();
        task.slot.node_max_count = 1;
        task.slot.count = 1;
        let info =Arc::new(CopyLock::new( String::new()));
        let show_mul_info = false;
        Self{tasks, coordinate,cfg,index:0, task, info,show_mul_info }
    }

    fn render_tasks(&mut self,ctx: &Context, ui: &mut Ui){
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .auto_shrink([false;2])
            .show(ui,|ui|{
                let ts = self.tasks.share();
                for i in ts.iter(){
                    CollapsingHeader::new(format!("Task[{}]:{}",i.id,i.name))
                        .default_open(false)
                        .show(ui,|ui|{
                            egui::Grid::new(i.name.clone())
                                .num_columns(2)
                                .spacing([40.0, 4.0])
                                .striped(true)
                                .show(ui,|ui|{
                                    ui.label("所有的工作数量总和").on_hover_text("一份工作，称为一个槽，一个任务中的所有工作，依次编为若干槽");
                                    ui.label(format!("{}/个",i.slot.count));
                                    ui.end_row();
                                    ui.label("单节点最大工作数量").on_hover_text("每个节点最多分多少工作");
                                    ui.label(format!("{}/个",i.slot.node_max_count));
                                    ui.end_row();
                                    ui.label("单节点最小工作数量").on_hover_text("每个节点最少分多少,默认0，通常不需要设置");
                                    ui.label(format!("{}/个",i.slot.node_min_count));
                                    ui.end_row();
                                });
                        });
                }
            });

    }
    fn render_create_task(&mut self,ctx: &Context, ui: &mut Ui){
        let Self{
            coordinate,
            task, ..
        } = self;
        egui::Grid::new("create_task")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui,|ui|{
                ui.label("任务名称:");
                ui.add(egui::TextEdit::singleline(&mut task.name).hint_text("coordinate task name"));
                ui.end_row();
                ui.label("工作总数:");
                ui.add(egui::DragValue::new(&mut task.slot.count).speed(1));
                ui.end_row();
                ui.label(WidgetText::from("节点最大值:").color(Color32::BLUE)).on_hover_text("每个节点能够分配的最大值");
                ui.add(egui::DragValue::new(&mut task.slot.node_max_count).speed(1));
                ui.end_row();
            });
        if ui.button("创建任务").clicked(){
            let ctx = ctx.clone();
            let info = self.info.clone();
            self.coordinate.create_task(task,move|result|{
                match result {
                    Ok(o) => {
                        info.update(|_|format!("task create success:{}",o));
                    }
                    Err(e) => {
                        info.update(|_|format!("{}",e));
                    }
                }
                ctx.request_repaint();
            });
        }
    }
}

impl super::Tab for TaskTab {
    fn name(&self) -> &'static str {
        "任务"
    }

    fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui|{
            if ui.button("任务列表").clicked() {
                // 刷新任务
                let ts = self.tasks.clone();
                let info = self.info.clone();
                let ctx = ctx.clone();
                self.coordinate.task_list(move |result|{
                    match result {
                        Ok(o)=>{
                            ts.update(|_|o);
                            info.update(|_|"success".to_string());
                        }
                        Err(err)=>{
                            info.update(|_|err.to_string());
                        }
                    }
                    ctx.request_repaint();
                });
                self.index = 1;
            }
            if ui.button("新建任务").clicked() {

                self.index = 2;
            }
        });

        match self.index {
            1=>{
                self.render_tasks(ctx,ui);
            }
            2=>{
                self.render_create_task(ctx,ui);
            }
            _=>{}
        }
        ui.with_layout(Layout::bottom_up(Align::Min),|ui|{
            let info = self.info.share();
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
    }

}