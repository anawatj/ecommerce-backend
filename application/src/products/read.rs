use domain::models::Product;
use shared::response_models::{ResponseProduct, ResponseProductBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_product(product_id: i32) -> Result<Product, NotFound<String>> {
    use domain::schema::products;

    match products::table.find(product_id).first::<Product>(&mut establish_connection()) {
        Ok(product) => Ok(product),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseProduct { body: ResponseProductBody::Message(format!("Error selecting product with id {} - {}", product_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}
pub fn list_products() -> Vec<Product> {
    use domain::schema::products;

    match products::table.select(products::all_columns).load::<Product>(&mut establish_connection()) {
        Ok(mut products) => {
           // products.sort();
            products
        },
        // doesn't seem like selecting everything will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}