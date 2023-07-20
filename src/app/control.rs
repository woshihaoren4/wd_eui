use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
use eframe::egui::{CollapsingHeader, Context, Ui};
use crate::infra::async_http_pool::*;

pub struct Control{
    content: Arc<AtomicPtr<String>>,
    hc:HttpClient,
}

impl Control {
    pub fn new(hc:HttpClient)->Self{
        Self{content:Arc::new(AtomicPtr::default()),hc}
    }
}

impl super::Tap for Control {
    fn name(&self) -> &'static str {
        "任务"
    }

    fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        CollapsingHeader::new("task table")
            .default_open(false)
            .show(ui,|ui|{
                ui.label("test 1");
            });
        CollapsingHeader::new("task detail")
            .default_open(false)
            .show(ui,|ui|{
                ui.label("test 2");
            });
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