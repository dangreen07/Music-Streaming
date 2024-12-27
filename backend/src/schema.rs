// @generated automatically by Diesel CLI.

diesel::table! {
    session (id) {
        id -> Uuid,
        user_id -> Uuid,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    songs (id) {
        id -> Uuid,
        title -> Varchar,
        artist -> Varchar,
        album -> Varchar,
        duration -> Int4,
        num_samples -> Int4
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        permissions -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(session -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    session,
    songs,
    users,
);
