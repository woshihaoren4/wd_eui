mod task;

pub use task::*;
use std::collections::HashMap;
use std::sync::Arc;
use hyper::StatusCode;
use wd_tools::sync::CopyLock;
use crate::app::config::ConfigEntity;
use crate::infra::HttpClient;

pub struct Coordinate{
    hc:HttpClient,
    cfg : Arc<CopyLock<ConfigEntity>>
}

impl Coordinate {
    pub fn new(hc:HttpClient, cfg : Arc<CopyLock<ConfigEntity>>)->Self{
        Self{hc,cfg}
    }
    pub fn task_list(&self,handle:impl FnOnce(Vec<TaskEntity>)+Send+Sync+'static){
        let url = format!("{}/api/v1/task/search",self.cfg.share().http_url);
        self.hc.get(url,HashMap::new(),|res|{
            let (status,headers,body) = match res {
                Ok(o) => o,
                Err(e) => {
                    wd_log::log_error_ln!("Control->get task error:{}",e);
                    return
                }
            };
            if status != StatusCode::OK {
                unsafe {
                    wd_log::log_error_ln!("Control->get task failed: code:{} body:{}",status,String::from_utf8_unchecked(body));
                }
                return;
            }
            let resp:SearchTasksResponse = match serde_json::from_slice(body.as_slice()){
                Ok(o)=>o,
                Err(e)=>{
                    unsafe {
                        wd_log::log_error_ln!("Control->json unmarshal error:{} body:{}",e,String::from_utf8_unchecked(body));
                    }
                    return;
                }
            };
            if resp.code != 0 {
                wd_log::log_error_ln!("Control->response failed: code:{} message:{}",resp.code,resp.message);
            }
            handle(resp.tasks);
        })
    }
}