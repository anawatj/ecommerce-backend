
use domain::models::{Order, NewOrder, OrderDetail, OrderRead};
use rocket::response::status::NotFound;
use shared::jwt::ClaimData;
use shared::response_models::{ResponseOrder, ResponseOrderBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
pub fn list_order(token:ClaimData,id:i32)->Result<OrderRead,NotFound<String>>{
    use domain::schema::orders;
    use domain::schema::order_details;

    match orders::table.find(id).first::<Order>(& mut establish_connection()) {
        Ok(order) => {
         use domain::schema::order_details::order_id;
          let result_detail =order_details::table.filter(order_id.eq(id)).load::<OrderDetail>(& mut establish_connection());
          match result_detail {
            Ok(details)=>{
                let result : OrderRead=OrderRead 
                { id: order.id, 
                  order_desc: order.order_desc, 
                  order_customer: order.order_customer, 
                  order_date: order.order_date,
                  order_status: order.order_status, 
                  address_1: order.address_1, 
                  address_2: order.address_2,
                  ship_date: order.ship_date, 
                  order_details: details.iter().map(|detail|(detail.product_id,detail.qty)).collect()
                  };
                  Ok(result)
            },
            Err(err)=> match err {
                diesel::result::Error::NotFound => {
                    let response = ResponseOrder { body: ResponseOrderBody::Message(format!("Error selecting order with id {} - {}", id, err))};
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                },
                _ => {
                    panic!("Database error - {}", err);
                }        
            }
          }
        
        },
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseOrder { body: ResponseOrderBody::Message(format!("Error selecting order with id {} - {}", id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}
pub fn list_orders(token:ClaimData)->Vec<Order> {
    use domain::schema::orders;
    match orders::table.select(orders::all_columns).load::<Order>(& mut establish_connection()) {
        Ok(orders)=>orders,
        Err(err)=>match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}