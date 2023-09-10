diesel::table! {
    customers(id){
        id -> Int4,
        email -> VarChar,
        password -> VarChar,
        first_name -> VarChar,
        last_name -> VarChar
    }
  
   
}
diesel::table! {
    products(id){
        id -> Int4,
        product_name -> VarChar,
        product_price -> Float8 ,
        product_qty -> Int4
    }
}
diesel::table! {
    orders(id){
        id->Int4,
        order_customer->VarChar,
        order_date->Date,
        order_status->VarChar,
        address_1->VarChar,
        address_2->VarChar,
        ship_date->Date
    }
}
diesel::table! {
    order_details(order_id,product_id){
        order_id->Int4,
        product_id->Int4,
        qty->Int4
    }
}
