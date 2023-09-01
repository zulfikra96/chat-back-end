-- Your SQL goes here
CREATE TABLE users_room (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id uuid,
    user_id uuid,
    created_at timestamp default now(),
    updated_at timestamp,
    CONSTRAINT users_fk
        FOREIGN KEY(user_id)
            REFERENCES users(id)
                ON DELETE SET NULL,
    CONSTRAINT rooms_fk
        FOREIGN KEY (room_id)
            REFERENCES rooms(id)
                ON DELETE SET NULL
);