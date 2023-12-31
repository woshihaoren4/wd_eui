use super::CoordinateResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JoinTaskRequest {
    pub task_id: String,
    pub code: String,
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JoinTaskResponse {
    pub token: String,
    #[serde(default)]
    pub code: i32,
    pub message: String,
}

coordinate_response_generate!(JoinTaskResponse, String, token);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlotDistributionsRequest {
    pub node_code: String,
    pub task_id: String,
    pub all_node_info: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlotDistributionsResponse {
    pub tags: Vec<i32>,
    pub version: String,
    pub nodes_slot: Vec<SlotAlloc>,
    #[serde(default)]
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlotAlloc {
    pub node_code: String,
    pub slots: Vec<i32>,
}

coordinate_response_generate!(SlotDistributionsResponse, Vec<i32>, tags);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PingRequest {
    pub task_id: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PingResponse {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub code: i32,
    pub message: String,
}

coordinate_response_generate!(PingResponse, String, version);
