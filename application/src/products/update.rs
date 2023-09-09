use domain::models::{Product,NewProduct};
use shared::response_models::{ResponseProduct, ResponseProductBody};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use diesel::prelude::*;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
pub fn update_product(product_id:i32,product: Json<NewProduct>) -> Result<Product,NotFound<String>>{
    use domain::schema::products::dsl::*;
    match diesel::update(products.find(product_id)).set((
        product_name.eq(product.product_name.to_string()),
        product_price.eq(product.product_price),
        product_qty.eq(product.product_qty)
    )).get_result::<Product>(&mut establish_connection()){
        Ok(product)=>Ok(product),
        Err(err)=>match err{
            diesel::result::Error::NotFound => {
                let response = ResponseProduct { body: ResponseProductBody::Message(format!("Error update product with id {} - {}", product_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            } 
        }
    }
}