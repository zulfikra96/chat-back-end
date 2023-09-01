-- Your SQL goes here
create table rooms_message (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id uuid not null,
    user_id uuid not null,
    message text,
    attachment_url text,
    created_at timestamp default now(),
    updated_at timestamp
    foreign key room_id
)
