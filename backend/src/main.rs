use actix_web::{
    body,
    delete,
    get,
    http,
    patch,
    post,
    web::{ Data, Json, Path },
    App,
    HttpResponse,
    HttpServer,
    Responder,
};
use actix_cors::Cors;
use validator::Validate;
use uuid::{ self };
// models
mod models;
mod db;

// conneting to db
use crate::db::Database;
// connecting to pizza model
use crate::models::{
    BuyPizzaRequest,
    UpdatePizzaUrl,
    Pizza,
    DeletePizzaUrl,
    DeletePizzaResponse,
    CreatedPizzaResponse,
    ValidateCreateTask,
    Task,
    ValidateComment,
    Comment,
    VerifyTaskId,
};
//surreal start file:pizzashop2.db --user root --password root

// getting pizzas
#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    // fetched form surreal db
    let pizzas = db.get_all_pizza().await;
    //after finding its changing it to vec
    match pizzas {
        Some(found_pizzas) => {
            let parsed_pizzas: Vec<Pizza> = found_pizzas.to_vec();
            HttpResponse::Ok().json(parsed_pizzas)
        }
        None => HttpResponse::Ok().body("No pizzas found"),
    }
}

// getting all tasks
#[get("/tasks")]
async fn get_tasks(db: Data<Database>) -> impl Responder {
    let tasks = db.get_all_task().await;
    match tasks {
        Some(found_tasks) => {
            let parsed_tasks: Vec<Task> = found_tasks.to_vec();
            HttpResponse::Ok().json(parsed_tasks)
        }
        None => HttpResponse::Ok().body("Tasks unable to be retrieved at this time"),
    }
}

// adding comment to a particular task based on the id of the task
#[post("/add-comment/{uuid}")]
async fn add_comment_to_task(
    uuid: Path<VerifyTaskId>,
    body: Json<ValidateComment>,
    db: Data<Database>
) -> impl Responder {
    let task_id = uuid.into_inner().uuid;
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let comment_name: String = body.comment.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_commented_task = db.create_new_comment(
                Task::new_comment(new_uuid.to_string(), comment_name),
                &task_id
            );
            match new_commented_task.await {
                Some(found_new_task) => { 
                    // update the exising data
                    HttpResponse::Ok().json(found_new_task) }
                None => HttpResponse::Ok().body("No Comment Has been added"),
            }
        }
        Err(_) => { HttpResponse::Ok().body("Comment cannot be added") }
    }
}

// post a new task the collection of tasks in rust
#[post("/add-task")]
async fn add_task(body: Json<ValidateCreateTask>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let task_name: String = body.task_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_task: Task = Task {
                uuid: new_uuid.to_string(),
                task_name,
                status: false, // by default the task is set to false
                comments: Vec::new(),
            };
            let added_new_task = db.add_task(
                Task::new_task(
                    new_task.uuid,
                    new_task.task_name,
                    new_task.status,
                    new_task.comments
                )
            ).await;
            match added_new_task {
                Some(created_new_task) => { HttpResponse::Ok().json(created_new_task) }
                None => HttpResponse::Ok().json("Task failed to create within the colletion"),
            }
        }
        Err(_) => { HttpResponse::Ok().body("new task is invalid") }
    }
}

// post pizzas directly to the sql database
#[post("/buy")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = db.add_pizza(
                Pizza::new_pizza(String::from(new_uuid), pizza_name)
            ).await;
            match new_pizza {
                Some(created) => {
                    let created_success_response = CreatedPizzaResponse {
                        created_pizza: created,
                        created_message: "Pizza Has Been Created",
                    };
                    HttpResponse::Ok().json(created_success_response)
                }
                None => HttpResponse::Ok().json("Unable to create the pizza"),
            }
        }
        Err(_) => { HttpResponse::Ok().body("pizza name is required") }
    }
}

// route to delete a single pizza from the collection
#[delete("/delete_pizza/{uuid}")]
async fn delete_pizza(delete_url: Path<DeletePizzaUrl>, db: Data<Database>) -> impl Responder {
    let delete_uuid = delete_url.into_inner().uuid;
    let deleted_pizza = db.delete_pizza(&delete_uuid).await;
    match deleted_pizza {
        Some(deleted) => {
            // here for the frontend json needs to be returned in for deletion
            let delete_pizza_reponse = DeletePizzaResponse {
                deleted_item: deleted.to_owned(),
                delete_message: "Undo Delete",
            };
            HttpResponse::Ok().json(delete_pizza_reponse)
        }
        None => HttpResponse::Ok().json("Item has failed to be deleted"),
    }
}

// patching with a random uuid passed for editing
#[patch("/updatepizza/{uuid}")]
async fn update_pizza(
    update_pizza_url: Path<UpdatePizzaUrl>,
    body: Json<BuyPizzaRequest>,
    db: Data<Database>
) -> impl Responder {
    let updated_pizza_name = body.into_inner();
    let uuid = update_pizza_url.into_inner().uuid;
    let updated_pizza = db.update_pizza(&uuid, updated_pizza_name.pizza_name.clone()).await;
    match updated_pizza {
        Some(updated) => { HttpResponse::Ok().json(updated) }
        None => HttpResponse::Ok().json("Failed to update"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialising a new database
    let db = Database::init().await.expect("error in database");
    let db_data = Data::new(db);

    // creating a basic server
    let server_actions = HttpServer::new(move || {
        // getting cors permission for localhost endpoint access
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".rust-lang.org")
            })
            // the methods need to be allowed here in order for axios in the frontend to access it
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(db_data.clone()) // allows end point routes connection also
            .wrap(cors)
            .service(get_pizzas)
            .service(update_pizza)
            .service(buy_pizza)
            .service(delete_pizza)
            .service(add_task)
            .service(get_tasks)
            .service(add_comment_to_task)
    })
        .bind("127.0.0.1:8080")?
        .run().await;

    // server actions need to return something that allows a result to be returned
    match server_actions {
        Ok(_) => {
            println!("Server running in localhost 8080");
            Ok(())
        }
        Err(err) => {
            println!("Failed to run server");
            Err(err)
        }
    }
}
