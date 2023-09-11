use diesel::sql_types::{Timestamp, Integer, Text};
use domain::models::{Order, NewOrder, OrderRead};
use domain::schema::orders::{order_customer, order_date, order_status, address_1, ship_date, address_2,order_desc};
use rocket::execute;
use shared::response_models::{ResponseOrder, ResponseOrderBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::jwt::ClaimData;



pub fn create_orders(token:ClaimData,order:Json<NewOrder>) -> Created<String>{
    use domain::schema::orders;

    let data = order.into_inner();

    match  diesel::insert_into(orders::table).values(&(
        order_desc.eq(data.order_desc),
        order_customer.eq(data.order_customer),
        order_date.eq(data.order_date),
        order_status.eq(data.order_status),
        address_1.eq(data.address_1),
        address_2.eq(data.address_2),
        ship_date.eq(data.ship_date)
    )).get_result::<Order>(&mut establish_connection()) {
        Ok(order) => {
            use domain::schema::order_details;
            use domain::schema::order_details::{order_id,product_id,qty};
            let details =data.products.iter().map(|product|{
                diesel::insert_into(order_details::table).values(&(
                    order_id.eq(order.id),
                    product_id.eq(product.product_id),
                    qty.eq(product.qty)
                )).execute(& mut establish_connection());
                (product.product_id,product.qty)
            }
             
            
            
            ).collect();
            let result:OrderRead=OrderRead { 
                id: order.id, 
                order_desc: order.order_desc, 
                order_customer: order.order_customer, 
                order_date: order.order_date, 
                order_status: order.order_status,
                 address_1: order.address_1, 
                 address_2: order.address_2, 
                 ship_date: order.ship_date, 
                 order_details: details
                };
            let response = ResponseOrder { body: ResponseOrderBody::Data(result) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        // doesn't seem like insert_into() will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}