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
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Order{
    pub id:i32,
    pub order_desc:String,
    pub order_customer:String ,
    pub order_date:String,
    pub order_status:String ,
    pub address_1:String ,
    pub address_2:String ,
    pub ship_date:String
}

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct OrderRead{
    pub id:i32,
    pub order_desc:String,
    pub order_customer:String ,
    pub order_date:String,
    pub order_status:String ,
    pub address_1:String ,
    pub address_2:String ,
    pub ship_date:String,
    pub order_details:Vec<(i32,i32)>
}
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct OrderDetail{
    pub order_id : i32,
    pub product_id :i32,
    pub qty:i32
}
#[derive( Deserialize)]
pub struct NewOrderDetail{
    pub product_id :i32 ,
    pub qty : i32,
    pub product_price:f64
}
#[derive( Deserialize)]
pub struct NewOrder{
    pub order_desc:String,
    pub order_customer:String,
    pub order_date:String,
    pub order_status:String ,
    pub address_1:String ,
    pub address_2:String ,
    pub ship_date:String,
    pub products:Vec<NewOrderDetail>
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