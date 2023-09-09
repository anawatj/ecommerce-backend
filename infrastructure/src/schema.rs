// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 200]
        email -> Nullable<Varchar>,
        #[max_length = 8000]
        password -> Varchar,
        #[max_length = 200]
        first_name -> Varchar,
        #[max_length = 200]
        last_name -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 200]
        product_name -> Varchar,
        product_price -> Float8,
        product_qty -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    products,
);
