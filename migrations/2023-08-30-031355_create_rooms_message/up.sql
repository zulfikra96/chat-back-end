-- Your SQL goes here
create table rooms_message (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id uuid not null,
    user_id uuid not null,
    message text,
    attachment_url text,
    created_at timestamp default now(),
    updated_at timestamp,
    
    CONSTRAINT room_fk 
        FOREIGN KEY(room_id)
            REFERENCES rooms(id)
                on delete set null,

    CONSTRAINT user_fk
        FOREIGN KEY(user_id)
            REFERENCES users(id)
                on delete set null
);
