
use shared::response_models::{ResponseOrderBody, ResponseOrder};
use application::orders::{create, read, delete };
use domain::models::{NewOrder};
use rocket::{get, post ,put ,delete};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;
use shared::jwt::ClaimData;
#[get("/orders")]
pub fn list_orders_handler(token:ClaimData)->String{
    let orders = read::list_orders(token);

    let response = ResponseOrder { body: ResponseOrderBody::Orders(orders) };

    serde_json::to_string(&response).unwrap()
}
#[get("/orders/<order_id>")]
pub fn list_order_handler(token:ClaimData,order_id: i32) -> Result<String, NotFound<String>> {
    let order = read::list_order(token, order_id)?;

    let response = ResponseOrder { body: ResponseOrderBody::Data(order) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/orders", format = "application/json", data = "<order>")]
pub fn create_order_handler(token:ClaimData,order: Json<NewOrder>) -> Created<String> {
    create::create_orders(token, order)
}
#[delete("/orders/<order_id>")]
pub fn delete_order(token:ClaimData,order_id:i32)->Result<String,NotFound<String>>{
    let orders = delete::delete_order(token,order_id)?;
    let response = ResponseOrder { body: ResponseOrderBody::Orders(orders) };

    Ok(serde_json::to_string(&response).unwrap())
}