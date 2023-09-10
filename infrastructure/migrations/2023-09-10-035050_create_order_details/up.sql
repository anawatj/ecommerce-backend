-- Your SQL goes here
CREATE TABLE order_details (
    order_id int not null,
    product_id int not null,
    qty int NOT NULL,
    PRIMARY KEY (order_id, product_id)
)
