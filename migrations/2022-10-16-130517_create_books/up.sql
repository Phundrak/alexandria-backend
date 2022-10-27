-- Your SQL goes here
CREATE TYPE BookType AS ENUM ('novel', 'short-story', 'poem');

CREATE TABLE Books (
       Id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
       Title VARCHAR(255) NOT NULL,
       Author UUID
              REFERENCES Authors(Id)
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
