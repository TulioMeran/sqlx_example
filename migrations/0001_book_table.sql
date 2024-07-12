CREATE TABLE book (
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    isbn VARCHAR NOT NULL
);

CREATE UNIQUE INDEX book_isbn_idx on book (isbn);