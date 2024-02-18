use actix_web::{
    get,
    patch,
    post,
    web::{ Json, Path, Data },
    App,
    HttpResponse,
    HttpServer,
    Responder,
};
use validator::Validate;
use uuid::{ self };
// models
mod models;
mod db;
// conneting to db
use crate::db::Database;
// connecting to pizza model
use crate::models::{ BuyPizzaRequest, UpdatePizzaUrl, Pizza };

// getting pizzas
#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    // fetched form surreal db
    let pizzas = db.get_all_pizza().await;
    match pizzas {
        Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        None => HttpResponse::Ok().body("No pizzas found"),
    }
}

// post pizzas
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
                    HttpResponse::Ok().body(format!("Created New Pizza, {:?}", created))
                }
                None => HttpResponse::Ok().body("Error buying em"),
            }
        }
        Err(_) => { HttpResponse::Ok().body("pizza name is required") }
    }
}

// patching with a random uuid passed for editing
#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaUrl>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("udating the pizza with this {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialising a new database
    let db = Database::init().await.expect("error in database");
    db.add_dummy_pizza().await; // adding dummy list of pizzas
    let db_data = Data::new(db);
    // creating a basic server
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) // allows end point routes connection also
            .service(get_pizzas)
            .service(update_pizza)
            .service(buy_pizza)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
