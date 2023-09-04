-- Your SQL goes here
CREATE TABLE rooms (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar(100),
    is_group boolean default FALSE,
    created_at timestamp default now(),
    updated_at timestamp
);

