-- Your SQL goes here
create type BookType as enum ('novel', 'short-story', 'poem');

CREATE TABLE Books (
       Id varchar(255) primary key,
       Title varchar(255) not null,
       Author varchar(255)
              references Authors(Slug)
              on update cascade
              on delete cascade,
       Isbn text[],
       Cover varchar(255),
       Publisher varchar(255),
       Published date,
       Genre text[],
       Synopsis text,
       BookType varchar(31)
);
