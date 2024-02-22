use actix_web::Resource;
use surrealdb::engine::remote::ws::{ Client, Ws };
use surrealdb::opt::auth::Root;
use futures::stream::StreamExt; // Import the StreamExt trait
use surrealdb::{ Error, Surreal };
use uuid;

use crate::models::pizza::Pizza;
use crate::models::Task;

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

    // delete piza
    pub async fn delete_pizza(&self, delete_id: &String) -> Option<Pizza> {
        let delete_pizza_id = delete_id.to_string();
        let deleted_pizza = self.client.delete(("pizza_new", delete_pizza_id)).await;
        match deleted_pizza {
            Ok(deleted) => { deleted }
            Err(_) => None,
        }
    }

    // add a new task to the new task collection
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
