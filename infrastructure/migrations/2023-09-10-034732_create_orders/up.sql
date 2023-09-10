-- Your SQL goes here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    order_customer  varchar(200) not null,
    order_date  date not null,
    order_status varchar(100) not null,
    address_1 varchar(1000) not null,
    address_2 varchar(1000) not null,
    ship_date date not null
)
