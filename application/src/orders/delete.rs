use shared::response_models::{ResponseOrder, ResponseOrderBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::{response::status::NotFound, serde::de};
use domain::models::{OrderDetail, Order};
use shared::jwt::ClaimData;
pub fn delete_order(token:ClaimData,order_id: i32) -> Result<Vec<Order>, NotFound<String>> {
    use domain::schema::orders::dsl::*;
    use domain::schema::orders;
   

    let response: ResponseOrder;
   //  diesel::delete(order_details.filter(order_id.eq(id))).execute(&mut establish_connection());


    let num_deleted = match diesel::delete(orders.filter(id.eq(order_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseOrder { body: ResponseOrderBody::Message(format!("Error publishing order with id {} - {}", "", err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match orders::table.select(orders::all_columns).load::<Order>(&mut establish_connection()) {
            Ok( results) => {
                Ok(results)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = ResponseOrder { body: ResponseOrderBody::Message(format!("Error - no order with id {}", ""))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}