use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Task{
    pub uuid: String,
    pub task_name: String,
    pub status: bool,
    pub comments: Vec<Comment>
}

#[derive(Serialize, Deserialize, Validate)] // will check the task name
pub struct ValidateTask{
    #[validate(length(min = 5, message = "pizza name required"))]
    pub task_name: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Comment{
    pub comment_uuid: String,
    pub comment: String
}

// implementing task
impl Task{
   pub fn new_task(uuid: String, task_name: String, status: bool, comments: Vec<Comment>)-> Task{
        Task{
            uuid,
            task_name,
            status,
            comments
        }
   }
}