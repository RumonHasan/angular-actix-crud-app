use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Task{
    pub uuid: String,
    pub task_name: String,
    pub status: bool
}


// implementing task
impl Task{
   pub fn new_task(uuid: String, task_name: String, status: bool)-> Task{
        Task{
            uuid,
            task_name,
            status
        }
   }
}