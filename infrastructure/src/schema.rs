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
    order_details (order_id, product_id) {
        order_id -> Int4,
        product_id -> Int4,
        qty -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        #[max_length = 200]
        order_customer -> Varchar,
        order_date -> Date,
        #[max_length = 100]
        order_status -> Varchar,
        #[max_length = 1000]
        address_1 -> Varchar,
        #[max_length = 1000]
        address_2 -> Varchar,
        ship_date -> Date,
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
    order_details,
    orders,
    products,
);
