-- Your SQL goes here
CREATE TYPE BookType AS ENUM ('novel', 'short-story', 'poem');

CREATE TABLE Books (
       Id VARCHAR(255) PRIMARY KEY,
       Title VARCHAR(255) NOT NULL,
       Author VARCHAR(255)
              REFERENCES Authors(Slug)
              ON UPDATE CASCADE
              ON DELETE CASCADE
              NOT NULL,
       Isbn TEXT[],
       Cover VARCHAR(255),
       Publisher VARCHAR(255),
       Published DATE,
       Genre TEXT[],
       Synopsis TEXT,
       BookType BookType NOT NULL
);
