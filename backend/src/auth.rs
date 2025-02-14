use argon2::{
    Argon2,
    PasswordHasher
};
use diesel::{
    prelude::*,
    result::Error
};
use password_hash::{
    rand_core::OsRng,
    SaltString
};
use uuid::Uuid;

use crate::models::*;

pub fn create_user(conn: &mut PgConnection, arg_username: &str, password: &str) -> Result<Users, &'static str> {
    use crate::schema::users;
    use crate::schema::users::dsl::*;

    // Checking if the user already exists
    let response = users.filter(username.eq(arg_username)).select(Users::as_select()).load(conn).expect("Error selecting from database!");
    if response.len() != 0 {
        return Err("Username already exists!");
    }

    // Hashing the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash2 = argon2.hash_password(password.as_bytes(), &salt).expect("Failed to hash password!").to_string();

    // Creating the user
    let new_user = NewUser {
        username: arg_username,
        password_hash: &password_hash2,
    };
    
    // Inserting the user into the database
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("Error saving new user");

    return Ok(result);
}

pub fn verify_user(conn: &mut PgConnection, arg_username: &str, arg_password: &str) -> Result<Uuid, &'static str> {
    use crate::schema::users::dsl::*;

    let response = users.filter(username.eq(arg_username)).select(Users::as_select()).load(conn).expect("Error loading users");
    let user = response.get(0);
    let user = match user {
        Some(user) => user,
        None => return Err("Username or password is incorrect!")
    };
    
    let encoded_hash = user.password_hash.clone();
    let vals = encoded_hash.split("$").collect::<Vec<&str>>();
    let salt = SaltString::from_b64(vals[4]).expect("Failed to decode salt");
    let argon2 = Argon2::default();
    let password_hashed = argon2.hash_password(arg_password.as_bytes(), &salt).expect("Failed to hash password!").to_string();
    if password_hashed == encoded_hash {
        return Ok(user.id);
    }
    return Err("Username or password is incorrect!");
}

pub fn create_session(conn: &mut PgConnection, arg_user_id: &uuid::Uuid) -> Result<Session, &'static str> {
    use crate::schema::session;

    let new_session = NewSession {
        user_id: arg_user_id,
        expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(30), // 30 days till expiration
    };

    let result = diesel::insert_into(session::table)
        .values(&new_session)
        .returning(Session::as_returning())
        .get_result(conn)
        .expect("Error saving new session");

    return Ok(result);
}

pub fn valid_session(conn: &mut PgConnection, arg_session_id: &uuid::Uuid) -> bool {
    use crate::schema::session::dsl::*;

    let response = session.filter(id.eq(arg_session_id)).select(Session::as_select()).load(conn).expect("Error loading sessions");
    let session_var = response.get(0);
    let session_var = match session_var {
        Some(session_var) => session_var,
        None => return false
    };

    if session_var.expires_at < chrono::Utc::now().naive_utc() {
        return false;
    }

    return true;
}

pub fn invalidate_session(conn: &mut PgConnection, arg_session_id: &uuid::Uuid) -> Result<usize, Error> {
    use crate::schema::session::dsl::*;

    diesel::delete(session.filter(id.eq(arg_session_id))).execute(conn)
}

pub fn get_user(conn: &mut PgConnection, arg_session_id: &uuid::Uuid) -> Result<Users, &'static str> {
    use crate::schema::{session, users};

    let resp = session::table
        .inner_join(users::table)
        .filter(session::id.eq(arg_session_id))
        .select((Session::as_select(), Users::as_select()))
        .load::<(Session, Users)>(conn);

    let resp = match resp {
        Ok(resp) => resp,
        Err(_) => return Err("Error loading users")
    };
    let resp = resp.get(0);
    let resp = match resp {
        Some(resp) => resp,
        None => return Err("Error getting user")
    };
    let user = resp.1.clone();
    return Ok(user);
}