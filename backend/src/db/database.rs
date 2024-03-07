use surrealdb::engine::remote::ws::{ Client, Ws };
use surrealdb::opt::auth::Root;
use surrealdb::{ Error, Surreal };
use uuid;

use crate::models::pizza::Pizza;
use crate::models::{ Comment, Task };

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    // implementing a database using surreal in the main.rs file
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        // sigining to the database
        client.signin(Root {
            username: "root",
            password: "root",
        }).await?;
        // creates or uses the same db
        client.use_ns("surreal").use_db("pizza_new").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("pizza_new"),
        })
    }

    // this returns a option contained vector of pizzas
    pub async fn get_all_pizza(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizza_new").await;
        match result {
            Ok(all_pizzas) => { Some(all_pizzas) }
            Err(_) => None,
        }
    }

    // general function to get all tasks from the collection of tasks
    pub async fn get_all_task(&self) -> Option<Vec<Task>> {
        let result_tasks = self.client.select("tasks").await;
        match result_tasks {
            Ok(all_tasks) => { Some(all_tasks) }
            Err(_) => None,
        }
    }

    // adding a new comment to the existing task after finding it based on task id
    pub async fn create_new_comment(
        &self,
        new_comment: Comment,
        update_id: &String
    ) -> Option<Task> {
        let task_id: String = update_id.to_string();
        let old_tasks = self.client.select("tasks").await;
        match old_tasks {
            Ok(all_tasks) => {
                let task_found = all_tasks
                    .iter()
                    .find(|task: &&Task| task.uuid.to_string() == task_id)
                    .map(|curr_task| {
                        let mut temp_task = curr_task.clone();
                        temp_task.comments.push(new_comment);
                        temp_task
                    });
                // updating the tasks after
                self.update_tasks(task_found.clone()).await;
                task_found
                // here the new task should be updated with the database ¥
            }
            Err(_) => None,
        }
    }

    // updating existing tasks from the surreal database series
    pub async fn update_tasks(&self, new_task: Option<Task>) -> Option<Vec<Task>> {
        if let Some(task_found) = new_task {
            let found_task_id = task_found.uuid.to_string();
            let older_tasks = self.client.update(("tasks", &found_task_id)).merge(Task {
                uuid: found_task_id.to_string(),
                status: task_found.status,
                task_name: task_found.task_name,
                comments: task_found.comments,
            }).await;
            match older_tasks {
                Ok(tasks) => { tasks }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    // update the existing data of tasks

    // adding a pizza
    pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
        let created_pizza = self.client
            .create(("pizza_new", new_pizza.uuid.clone()))
            .content(new_pizza).await;
        match created_pizza {
            Ok(created) => { created }
            Err(_) => None,
        }
    }
    // updating the pizza
    pub async fn update_pizza(&self, update_id: &String, updated_pizza: String) -> Option<Pizza> {
        let updated_pizza = self.client.update(("pizza_new", update_id)).merge(Pizza {
            uuid: update_id.to_string(),
            pizza_name: updated_pizza,
        }).await;
        match updated_pizza {
            Ok(updated) => { updated }
            Err(_) => None,
        }
    }

    // delete piza
    pub async fn delete_pizza(&self, delete_id: &String) -> Option<Pizza> {
        let delete_pizza_id = delete_id.to_string();
        let deleted_pizza = self.client.delete(("pizza_new", delete_pizza_id)).await;
        match deleted_pizza {
            Ok(deleted) => { deleted }
            Err(_) => None,
        }
    }

    // add a new task to the new task collection in the database
    pub async fn add_task(&self, new_task: Task) -> Option<Task> {
        let created_task = self.client
            .create(("tasks", new_task.uuid.clone()))
            .content(new_task).await;
        match created_task {
            Ok(created) => { created }
            Err(_) => None,
        }
    }

    // adding dummy pizzas
    pub async fn add_dummy_pizza(&self) {
        let pizza_counter = 10;
        let pizza_name: String = String::from("Rumon Pizza");
        for _ in 0..pizza_counter {
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = Pizza::new_pizza(String::from(new_uuid), pizza_name.clone());
            self.add_pizza(new_pizza).await;
        }
    }
}
