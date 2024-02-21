// pizza.rs
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
// validator when the pizza comes from the frontend request
pub struct BuyPizzaRequest{
    // validator crate for checking whether its a valid pizza name or not
    #[validate(length(min = 5, message = "pizza name required"))]
    pub pizza_name: String,
}

// for updating a pizza
#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePizzaUrl{
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct DeletePizzaUrl{
    pub uuid: String,
}

// delete pizza response
#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct DeletePizzaResponse{
    pub deleted_item : Pizza,
    pub delete_message:&'static str,
}

// struct for pizza
#[derive(Validate, Deserialize, Serialize, Debug, Clone)]
pub struct Pizza{
    pub uuid: String,
    pub pizza_name: String
}

impl Pizza{
    // just returns a new pizza
    pub fn new_pizza(uuid: String, pizza_name: String)-> Pizza{
        Pizza{
            uuid,
            pizza_name
        }
    }
}