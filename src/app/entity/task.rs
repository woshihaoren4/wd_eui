use serde::{Serialize, Deserialize};

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