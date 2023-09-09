-- Your SQL goes here
CREATE TABLE customers(
   id SERIAL PRIMARY KEY,
   email VARCHAR(200) UNIQUE,
   password VARCHAR(8000) NOT NULL,
   first_name VARCHAR(200) NOT NULL,
   last_name VARCHAR(200) NOT NULL

)
