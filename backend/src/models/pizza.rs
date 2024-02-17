// pizza.rs
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
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

// struct for pizza
#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Pizza{
    pub uuid: String,
    pub pizza_name: String
}

impl Pizza{
    // creating a new pizza
    pub fn new_pizza(uuid: String, pizza_name: String)-> Pizza{
        Pizza{
            uuid,
            pizza_name
        }
    }
}