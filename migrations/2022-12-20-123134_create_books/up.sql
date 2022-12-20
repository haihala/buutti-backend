-- Your SQL goes here
CREATE TABLE books (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title VARCHAR CHECK(title != '') NOT NULL,
  author VARCHAR CHECK(author != '') NOT NULL,
  year INTEGER NOT NULL,
  publisher VARCHAR CHECK(publisher != ''),
  description VARCHAR,
  CONSTRAINT unique_books UNIQUE (title, author, year)
)
