
use domain::models::{Customers,NewCustomer, LoginCustomer};
use shared::response_models::{ResponseCustomer, ResponseCustomerBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::{Created, NotFound,Unauthorized};
use rocket::serde::json::Json;
use shared::jwt::*;

pub fn login_customer(login: Json<LoginCustomer>) -> Result<String,NotFound<String>>{
    use domain::schema::customers::dsl::*;
    use domain::schema::customers;
    match customers::table.filter(email.eq(login.email.to_string())).first::<Customers>(&mut establish_connection()){
        Ok(customer)=>{
            let token = generate_token(customer.email);
            Ok(token)
        },
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseCustomer { body: ResponseCustomerBody::Message(format!("Error selecting customer with email   {}", err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
         
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}