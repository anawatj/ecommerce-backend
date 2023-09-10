use shared::response_models::{ResponseProductBody, ResponseProduct};
use application::products::{create, read ,update ,delete};
use domain::models::{NewProduct};
use rocket::{get, post ,put ,delete};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;
use shared::jwt::ClaimData;
#[get("/products")]
pub fn list_products_handler(token:ClaimData) -> String {
    let products = read::list_products(token);

    let response = ResponseProduct { body: ResponseProductBody::Products(products) };

    serde_json::to_string(&response).unwrap()
}

#[get("/products/<product_id>")]
pub fn list_product_handler(token:ClaimData,product_id: i32) -> Result<String, NotFound<String>> {
    let product = read::list_product(token,product_id)?;

    let response = ResponseProduct { body: ResponseProductBody::Data(product) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/products", format = "application/json", data = "<product>")]
pub fn create_product_handler(token:ClaimData,product: Json<NewProduct>) -> Created<String> {
    create::create_product(token,product)
}
#[put("/products/<product_id>",format="application/json",data="<product>")]
pub fn update_product_handler(token:ClaimData,product:Json<NewProduct>,product_id:i32)->Result<String,NotFound<String>>{
   let product = update::update_product(token,product_id, product)?;
   let response : ResponseProduct= ResponseProduct { body: ResponseProductBody::Data(product) };
   Ok(serde_json::to_string(&response).unwrap())
}
#[delete("/products/<product_id>")]
pub fn delete_product_handler(token:ClaimData,product_id:i32)->Result<String,NotFound<String>>{
    let products = delete::delete_product(token,product_id)?;

    let response = ResponseProduct { body: ResponseProductBody::Products(products) };

    Ok(serde_json::to_string(&response).unwrap())
}
