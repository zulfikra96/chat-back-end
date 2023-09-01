-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE role_type AS ENUM ('ADMIN','MEMBER');
CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar(40) not null,
    nrp varchar(20) not null unique,
    password text,
    role role_type not null,
    created_at timestamp default NOW(),
    updated_at timestamp
);