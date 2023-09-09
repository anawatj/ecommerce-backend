use crate::schema::customers;
use crate::schema::products;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Customers {
    pub id:i32 ,
    pub email:String,
    pub password:String,
    pub first_name:String,
    pub last_name:String,
}
#[derive( Deserialize)]
pub struct LoginCustomer{
    pub email:String,
    pub password:String,
}

#[derive(Queryable, Serialize, PartialEq, PartialOrd)]
pub struct Product {
    pub id:i32 ,
    pub product_name:String,
    pub product_price:f64 ,
    pub product_qty:i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub product_name:String,
    pub product_price:f64 ,
    pub product_qty:i32,
}
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = customers)]
pub struct NewCustomer{
    pub email:String,
    pub password:String,
    pub first_name:String,
    pub last_name:String,
}