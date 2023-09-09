use application::products::update::update_product;
use shared::response_models::{ResponseProductBody, ResponseProduct};
use application::products::{create, read ,update ,delete};
use domain::models::{NewProduct, Product};
use rocket::{get, post ,put ,delete};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/products")]
pub fn list_products_handler() -> String {
    let products = read::list_products();

    let response = ResponseProduct { body: ResponseProductBody::Products(products) };

    serde_json::to_string(&response).unwrap()
}

#[get("/products/<product_id>")]
pub fn list_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let product = read::list_product(product_id)?;

    let response = ResponseProduct { body: ResponseProductBody::Data(product) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/products", format = "application/json", data = "<product>")]
pub fn create_product_handler(product: Json<NewProduct>) -> Created<String> {
    create::create_product(product)
}
#[put("/products/<product_id>",format="application/json",data="<product>")]
pub fn update_product_handler(product:Json<NewProduct>,product_id:i32)->Result<String,NotFound<String>>{
   let product = update::update_product(product_id, product)?;
   let response : ResponseProduct= ResponseProduct { body: ResponseProductBody::Data(product) };
   Ok(serde_json::to_string(&response).unwrap())
}
#[delete("/products/<product_id>")]
pub fn delete_product_handler(product_id:i32)->Result<String,NotFound<String>>{
    let products = delete::delete_product(product_id)?;

    let response = ResponseProduct { body: ResponseProductBody::Products(products) };

    Ok(serde_json::to_string(&response).unwrap())
}
