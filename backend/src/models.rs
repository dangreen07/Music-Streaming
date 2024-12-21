use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: uuid::Uuid,
    pub username: String,
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