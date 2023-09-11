#[macro_use] extern crate rocket;
use api::{product_handler, customer_handler, order_handler};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            product_handler::list_products_handler, 
            product_handler::list_product_handler,
            product_handler::create_product_handler,
            product_handler::update_product_handler,
            product_handler::delete_product_handler,
            customer_handler::signup_customer_handler,
            customer_handler::login_customer_handler,
            order_handler::list_orders_handler,
            order_handler::list_order_handler,
            order_handler::create_order_handler,
        ])
}