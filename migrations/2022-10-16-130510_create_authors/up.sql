-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE Authors (
       Id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
       FirstName VARCHAR(127),
       LastName VARCHAR(127),
       PenName VARCHAR(255)
);
