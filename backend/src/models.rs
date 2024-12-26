use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::schema::*;

#[derive(Queryable, Selectable, Debug, Serialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: uuid::Uuid,
    pub username: String,
    pub permissions: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = session)]
#[diesel(belongs_to(Users))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expires_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = session)]
pub struct NewSession<'a> {
    pub user_id: &'a uuid::Uuid,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Serialize, Clone)]
#[diesel(table_name = songs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Songs {
    pub id: uuid::Uuid,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: i32,
    pub file_path: String,
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSong {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: i32,
    pub file_path: String,
}