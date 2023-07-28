use serde::{Serialize, Deserialize};
use super::CoordinateResponse;

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct JoinTaskRequest {
    pub task_id: String,
    pub code: String,
    pub addr: String,
}

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct JoinTaskResponse {
    pub token: String,
    pub code: i32,
    pub message: String,
}

coordinate_response_generate!(JoinTaskResponse,String,token);

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct SlotDistributionsRequest {
    pub node_code: String,
    pub task_id: String,
    pub all_node_info: bool,
}
#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct SlotDistributionsResponse {
    pub tags: Vec<i32>,
    pub version: String,
    pub nodes_slot: Vec<SlotAlloc>,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct SlotAlloc {
    pub node_code:String,
    pub slots: Vec<i32>,
}

coordinate_response_generate!(SlotDistributionsResponse,Vec<i32>,tags);