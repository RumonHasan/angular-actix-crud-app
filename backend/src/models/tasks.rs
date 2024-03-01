use serde::{ Deserialize, Serialize };
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct Task {
    pub uuid: String,
    pub task_name: String,
    pub status: bool,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Validate)] // will check the task name
pub struct ValidateTask {
    #[validate(length(min = 5, message = "pizza name required"))]
    pub task_name: String,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct Comment {
    pub comment_uuid: String,
    pub comment: String,
}

// validating task
#[derive(Serialize, Deserialize, Validate)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, message = "task name needs to have atleast one char"))]
    pub task_name: String,
}

// validate comment
#[derive(Serialize, Deserialize, Validate)]
pub struct ValidateComment{
    #[validate(length(min = 10, message = "Comment atleast needs to have 10 characters"))]
    pub comment: String
}

// confirm task id
#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyTaskId{
    pub uuid: String
}

// implementing task
impl Task {
    pub fn new_task(uuid: String, task_name: String, status: bool, comments: Vec<Comment>) -> Task {
        Task {
            uuid,
            task_name,
            status,
            comments,
        }
    }
    // making a new comment
    pub fn new_comment(uuid: String, comment: String) -> Comment {
        Comment {
            comment_uuid: uuid,
            comment,
        }
    }
}
