use domain::models::{Customers,NewCustomer};
use shared::response_models::{ResponseCustomer, ResponseCustomerBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::jwt::*;
pub fn create_customer(customer: Json<NewCustomer>) -> Created<String> {
    use domain::schema::customers;

    let data = customer.into_inner();

    match diesel::insert_into(customers::table).values(&data).get_result::<Customers>(&mut establish_connection()) {
        Ok(customer) => {
            //let response = ResponseCustomer { body: ResponseCustomerBody::Data(customer) };
            let token = generate_token(customer.email);
            Created::new("").tagged_body(serde_json::to_string(&token).unwrap())
        },
        // doesn't seem like insert_into() will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}