use serde::{Serialize, Deserialize};
use super::CoordinateResponse;

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct TaskEntity {
    pub id: String,
    // pub app_id: i32,
    pub name: String,
    // pub version: i32,
    // pub secret: String,
    // pub dead_timeout_sec: i32,
    // pub r#type : i32,
    pub slot: TaskSlot ,

    // pub created_at: i64,
    // pub updated_at: i64,
}

#[derive(Debug, Clone,Serialize,Deserialize,Default)]
pub struct TaskSlot {
    pub count: i32,
    pub node_max_count: i32,
    pub node_min_count: i32,
}
#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct SearchTasksResponse {
    pub tasks: Vec<TaskEntity>,
    #[serde(default)]
    pub code: i32,
    pub message: String,
}
coordinate_response_generate!(SearchTasksResponse,Vec<TaskEntity>,tasks);

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct Strategy {
    pub dead_timeout_sec: i32,
}

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct CreateTaskRequest {
    pub name: String,
    pub strategy: Option<Strategy>,
    pub slot: Option<TaskSlot>,
}

#[derive(Debug, Clone,Serialize,Deserialize, Default)]
pub struct CreateTaskResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: i32,
    pub message: String,
}
coordinate_response_generate!(CreateTaskResponse,String,id);