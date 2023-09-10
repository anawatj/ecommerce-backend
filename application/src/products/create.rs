use domain::models::{Product, NewProduct};
use shared::response_models::{ResponseProduct, ResponseProductBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::jwt::ClaimData;
pub fn create_product(token:ClaimData,product: Json<NewProduct>) -> Created<String> {
    use domain::schema::products;

    let data = product.into_inner();

    match diesel::insert_into(products::table).values(&data).get_result::<Product>(&mut establish_connection()) {
        Ok(product) => {
            let response = ResponseProduct { body: ResponseProductBody::Data(product) };
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