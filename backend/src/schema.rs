// @generated automatically by Diesel CLI.

diesel::table! {
    session (id) {
        id -> Uuid,
        user_id -> Uuid,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(session -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    session,
    users,
);
