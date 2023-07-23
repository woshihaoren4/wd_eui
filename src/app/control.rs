use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::Duration;
use eframe::egui;
use eframe::egui::{CollapsingHeader, Context, Ui};
use hyper::{StatusCode, Uri};
use crate::app::config::ConfigEntity;
use crate::app::entity::{Coordinate, TaskEntity};
use crate::infra::async_http_pool::*;
use wd_tools::sync::CopyLock;

pub struct Control{
    tasks :  Arc<CopyLock<Vec<TaskEntity>>>,
    coordinate: Coordinate,
    cfg : Arc<CopyLock<ConfigEntity>>
}

impl Control {
    pub fn new(coordinate: Coordinate,cfg:Arc<CopyLock<ConfigEntity>>)->Self{
        let tasks = Arc::new(CopyLock::new(Vec::new()));
        Self{tasks, coordinate,cfg}
    }
}

impl super::Tab for Control {
    fn name(&self) -> &'static str {
        "任务"
    }

    fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        if ui.button("刷新任务列表").clicked() {
           // 刷新任务
            let ts = self.tasks.clone();
            self.coordinate.task_list(move |list|{
                ts.update(|_|list)
            });
        }
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

        // if ui.button("测试http请求").clicked(){
            // let ctn = self.content.clone();
            // self.hc.get("http://www.baidu.com",HashMap::new(),|result|{
            //     match result{
            //         Ok((status,headers,body)) => {
            //             let sbody = unsafe {String::from_utf8_unchecked(body)};
            //             println!("success {} {:?} {}",status,headers,sbody);
            //         }
            //         Err(err) => {
            //             println!("Control.http error --->{}",err);
            //         }
            //     }
            // })
        // }
    }
}