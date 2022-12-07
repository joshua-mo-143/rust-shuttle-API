DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS notes;
DROP TABLE IF EXISTS products;

CREATE TABLE IF NOT EXISTS users (
    user_id serial PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS notes (
    note_id serial PRIMARY KEY,
    note VARCHAR NOT NULL,
    user_id integer,

    CONSTRAINT FK_notes_users FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS products (
    product_id serial PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    price integer NOT NULL,
    imgsrc VARCHAR NOT NULL,
    gender VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    brand VARCHAR NOT NULL
);

INSERT INTO users (username, password) VALUES ('username', 'password');

INSERT INTO notes (user_id, note) VALUES (1, 'Meme');
INSERT INTO notes (user_id, note) VALUES (1, 'Do the dishes');

INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Blue-Pleated Jeans', 'Wide-leg non-stretch denim jeans.', 590, '1.webp','man', 'Pants', 'Maison Margiela');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Black Knot Down Vest', 'Down-filled quilted nylon satin vest.', 2315, '2.webp', 'man', 'Jacket', 'Rick Owens');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Taupe Sociality Down Jacket', 'Down-filled quilted polyester taffeta jacket.', 790, '3.webp','man', 'Jacket', 'Heliot Emil');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Black Quilted Down Vest', 'Layered quilted down-filled recycled nylon taffeta vest.', 554, '4.webp','man', 'Jacket', 'Heliot Emil');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Black Organic Cotton Shirt', 'Organic cotton poplin shirt.', 330, '5.webp', 'man', 'Button-up Shirt', 'Rick Owens');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Black Fogpocket Shirt', 'Stretch organic cotton canvas shirt.', 670, '6.webp', 'man', 'Button-up Shirt', 'Rick Owens');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Green & Beige Double Bomber Jacket', 'Insulated nylon twill bomber jacket.', 1010, '7.webp','man', 'Jacket', 'Ambush');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Grey Caban Coat', 'Long sleeve insulated nylon taffeta puffer coat in grey.', 2990, '8.webp','man', 'Coat', 'Maison Margiela');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Black Metamorfosi Jacket', 'Nylon canvas jacket.', 2550, '9.webp','man', 'Coat', 'Gucci');
INSERT INTO products (name, description, price, imgsrc, gender, category, brand) VALUES ('Black Iridescent Sivaan Top', 'Sleeveless stretch viscose jersey single-shoulder camisole in iridescent black.',  265, '10.webp', 'woman', 'Camisole', 'Rick Owens Lilies');


                        
                        
                        
                        
                        
                        
                        
                        
                        
