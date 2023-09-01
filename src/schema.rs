// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_type"))]
    pub struct RoleType;
}

diesel::table! {
    rooms (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        is_group -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rooms_message (id) {
        id -> Uuid,
        room_id -> Uuid,
        user_id -> Uuid,
        message -> Nullable<Text>,
        attachment_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleType;

    users (id) {
        id -> Uuid,
        #[max_length = 40]
        name -> Varchar,
        #[max_length = 20]
        nrp -> Varchar,
        password -> Nullable<Text>,
        role -> RoleType,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users_room (id) {
        id -> Uuid,
        room_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(rooms_message -> rooms (room_id));
diesel::joinable!(rooms_message -> users (user_id));
diesel::joinable!(users_room -> rooms (room_id));
diesel::joinable!(users_room -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    rooms,
    rooms_message,
    users,
    users_room,
);
