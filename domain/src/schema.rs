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
