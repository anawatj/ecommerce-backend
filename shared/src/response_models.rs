// shared/src/response_models.rs

use domain::models::Product;
use domain::models::Customers;

use rocket::serde::{Serialize};

#[derive(Serialize)]
pub enum ResponseProductBody {
    Message(String),
    Data(Product),
    Products(Vec<Product>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseProduct {
    pub body: ResponseProductBody,
}

#[derive(Serialize)]
pub enum ResponseCustomerBody {
    Message(String),
    Data(Customers),
    Customers(Vec<Customers>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseCustomer {
    pub body: ResponseCustomerBody,
}



