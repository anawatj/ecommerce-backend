
use shared::response_models::{ResponseCustomerBody, ResponseCustomer};
use application::customers::{create,login};
use domain::models::{NewCustomer, Customers, LoginCustomer};
use rocket::{get, post ,put ,delete};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[post("/signup", format = "application/json", data = "<customer>")]
pub fn signup_customer_handler(customer: Json<NewCustomer>) -> Created<String> {
    create::create_customer(customer)
}
#[post("/login",format="application/json",data="<customer>")]
pub fn login_customer_handler(customer: Json<LoginCustomer>)->Result<String,NotFound<String>>{
    login::login_customer(customer)
}