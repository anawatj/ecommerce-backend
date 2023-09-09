use shared::response_models::{ResponseProduct, ResponseProductBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Product;

pub fn delete_product(product_id: i32) -> Result<Vec<Product>, NotFound<String>> {
    use domain::schema::products::dsl::*;
    use domain::schema::products;

    let response: ResponseProduct;

    let num_deleted = match diesel::delete(products.filter(id.eq(product_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseProduct { body: ResponseProductBody::Message(format!("Error publishing product with id {} - {}", product_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match products::table.select(products::all_columns).load::<Product>(&mut establish_connection()) {
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
        response = ResponseProduct { body: ResponseProductBody::Message(format!("Error - no product with id {}", product_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}