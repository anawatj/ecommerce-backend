use diesel::sql_types::{Timestamp, Integer, Text};
use domain::models::{Order, NewOrder};
use domain::schema::orders::{order_customer, order_date, order_status, address_1, ship_date, address_2};
use shared::response_models::{ResponseOrder, ResponseOrderBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::jwt::ClaimData;



pub fn create_orders(token:ClaimData,order:Json<NewOrder>){
    use domain::schema::orders;

    let data = order.into_inner();

    match  diesel::insert_into(orders::table).values(&(
        order_customer.eq(data.order_customer),
        order_date.eq(data.order_date),
        order_status.eq(data.order_status),
        address_1.eq(data.address_1),
        address_2.eq(data.address_2),
        ship_date.eq(data.ship_date)
    )).get_result::<Order>(&mut establish_connection()) {
        Ok(order) => {
            let response = ResponseOrder { body: ResponseOrderBody::Data(order) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap());
        },
        // doesn't seem like insert_into() will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}