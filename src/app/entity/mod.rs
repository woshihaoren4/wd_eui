#[macro_use]
mod r#macro;
mod node;
mod task;

use crate::app::config::ConfigEntity;
use crate::infra::HttpClient;
use hyper::{HeaderMap, Method, StatusCode};
pub use node::*;
pub use r#macro::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub use task::*;
use wd_tools::sync::CopyLock;

pub trait CoordinateResponse {
    type Value;
    fn code(&self) -> i32;
    fn message(&self) -> &str;
    fn value(self) -> Self::Value;
}

pub struct Coordinate {
    hc: HttpClient,
    cfg: Arc<CopyLock<ConfigEntity>>,
}

impl Clone for Coordinate {
    fn clone(&self) -> Self {
        Self {
            hc: self.hc.clone(),
            cfg: self.cfg.clone(),
        }
    }
}

impl Coordinate {
    pub fn new(hc: HttpClient, cfg: Arc<CopyLock<ConfigEntity>>) -> Self {
        Self { hc, cfg }
    }
    fn handle<T, ReqBody>(
        &self,
        method: Method,
        url: &str,
        req_body: Option<ReqBody>,
        handle: impl FnOnce(anyhow::Result<T::Value>) + Send + Sync + 'static,
    ) where
        T: CoordinateResponse + for<'a> Deserialize<'a>,
        ReqBody: Serialize,
    {
        let url = format!("{}{}", self.cfg.share().http_url, url);
        let req_body = if let Some(body) = req_body {
            match serde_json::to_vec(&body) {
                Ok(o) => Some(o),
                Err(e) => {
                    let err = anyhow::anyhow!("Coordinate->json marshal error:{},", e);
                    wd_log::log_error_ln!("{}", err);
                    handle(Err(err));
                    return;
                }
            }
        } else {
            None
        };
        let resp_handle = move |res: anyhow::Result<(StatusCode, HeaderMap, Vec<u8>)>| {
            let (status, _headers, body) = match res {
                Ok(o) => o,
                Err(e) => {
                    wd_log::log_error_ln!("Coordinate->request error:{}", e);
                    handle(Err(e));
                    return;
                }
            };
            if status != StatusCode::OK {
                unsafe {
                    let err = anyhow::anyhow!(
                        "Coordinate->response error: code:{} body:{}",
                        status,
                        String::from_utf8_unchecked(body)
                    );
                    wd_log::log_error_ln!("{}", err);
                    handle(Err(err));
                }
                return;
            }
            let resp: T = match serde_json::from_slice(body.as_slice()) {
                Ok(o) => o,
                Err(e) => {
                    unsafe {
                        let err = anyhow::anyhow!(
                            "Coordinate->json unmarshal error:{} body:{}",
                            e,
                            String::from_utf8_unchecked(body)
                        );
                        wd_log::log_error_ln!("{}", err);
                        handle(Err(err));
                    }
                    return;
                }
            };
            if resp.code() != 0 {
                let err = anyhow::anyhow!(
                    "Coordinate->response failed: code:{} message:{}",
                    resp.code(),
                    resp.message()
                );
                wd_log::log_error_ln!("{}", err);
                handle(Err(err));
            } else {
                handle(Ok(resp.value()));
            }
        };
        if let Some(body) = req_body {
            self.hc.request((url, method, body, resp_handle))
        } else {
            self.hc.request((url, method, resp_handle))
        };
    }

    pub fn task_list(
        &self,
        handle: impl FnOnce(anyhow::Result<Vec<TaskEntity>>) + Send + Sync + 'static,
    ) {
        self.handle::<SearchTasksResponse, ()>(Method::GET, "/api/v1/task/search", None, handle);
    }
    pub fn create_task(
        &self,
        task: &TaskEntity,
        handle: impl FnOnce(anyhow::Result<String>) + Send + Sync + 'static,
    ) {
        let req = CreateTaskRequest {
            name: task.name.clone(),
            strategy: Some(Strategy::default()),
            slot: Some(task.slot.clone()),
        };
        self.handle::<CreateTaskResponse, CreateTaskRequest>(
            Method::POST,
            "/api/v1/task/create",
            Some(req),
            handle,
        );
    }
    pub fn join_task(
        &self,
        task_id: String,
        code: String,
        handle: impl FnOnce(anyhow::Result<String>) + Send + Sync + 'static,
    ) {
        let req = JoinTaskRequest {
            task_id,
            code,
            ..Default::default()
        };
        self.handle::<JoinTaskResponse, JoinTaskRequest>(
            Method::POST,
            "/api/v1/node/join",
            Some(req),
            handle,
        );
    }
    pub fn node_tags(
        &self,
        task_id: String,
        node_code: String,
        handle: impl FnOnce(anyhow::Result<Vec<i32>>) + Send + Sync + 'static,
    ) {
        let url = format!("/api/v1/node/{}/slot", node_code.as_str());
        let req = SlotDistributionsRequest {
            task_id,
            node_code,
            ..Default::default()
        };
        self.handle::<SlotDistributionsResponse, SlotDistributionsRequest>(
            Method::POST,
            &url,
            Some(req),
            handle,
        );
    }
    pub fn ping(
        &self,
        task_id: String,
        node_code: String,
        handle: impl FnOnce(anyhow::Result<String>) + Send + Sync + 'static,
    ) {
        let url = format!("/api/v1/node/ping?task_id={}&code={}", task_id, node_code);
        self.handle::<PingResponse, PingRequest>(Method::GET, url.as_str(), None, handle);
    }
}
