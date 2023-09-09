-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    product_name VARCHAR(200) NOT NULL ,
    product_price FLOAT NOT NULL ,
    product_qty  INTEGER NOT NULL  
)