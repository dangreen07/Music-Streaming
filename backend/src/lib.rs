use argon2::{Argon2, PasswordHasher};
use diesel::{prelude::*, result::Error};
use dotenvy::dotenv;
use password_hash::{rand_core::OsRng, SaltString};
use uuid::Uuid;
use std::{env, io::Cursor};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::{Builder, Credentials}, Client};

use crate::models::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

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

/// Returns true if the session is valid and false if it is not
/// # Panics
/// Panics if there is an error with the database connection
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

pub fn get_sample(file: Vec<u8>, sample_number: u32) -> Result<Vec<u8>, &'static str> {
    let reader = Cursor::new(file);
    let mut reader = match hound::WavReader::new(reader) {
        Ok(r) => r,
        Err(_) => return Err("Error opening audio file"),
    };

    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let num_channels = spec.channels as usize;

    // 10 seconds per sample
    let samples_per_segment = sample_rate * 10 * num_channels as u32;

    let mut samples = vec![];
    let skip_num = usize::try_from(sample_number * samples_per_segment).unwrap();
    for sample in reader.samples::<i16>().skip(skip_num) {
        match sample {
            Ok(s) => samples.push(s),
            Err(_) => return Err("Error reading samples")
        }

        if samples.len() >= samples_per_segment as usize {
            break;
        }
    }

    if samples.is_empty() {
        return Err("Audio file is empty or too short");
    }

    // Write the first segment to a new WAV file in memory
    let mut buffer = Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut buffer, spec).unwrap();
        for sample in &samples {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();
    }

    let audio_bytes = buffer.into_inner();

    Ok(audio_bytes)
}

pub async fn get_file_from_bucket(file_name: &str) -> Vec<u8> {
    dotenv().ok();

    // Environment variables
    let access_key = env::var("DO_ACCESS_KEY_ID").expect("DO_ACCESS_KEY_ID must be set");
    let secret_key = env::var("DO_SECRET_ACCESS_KEY").expect("DO_SECRET_ACCESS_KEY must be set");
    let region = env::var("DO_REGION").expect("DO_REGION must be set");
    let endpoint = env::var("DO_ENDPOINT").expect("DO_ENDPOINT must be set");
    let bucket_name = env::var("DO_BUCKET_NAME").expect("DO_BUCKET_NAME must be set");

    let region = Region::new(region);

    let credentials = Credentials::new(access_key, secret_key, None, None, "Digital Ocean");
    
    let config = Builder::new()
        .region(region)
        .credentials_provider(credentials)
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(endpoint)
        .build();

    let client = Client::from_conf(config);

    let resp = client.get_object().bucket(bucket_name).key(file_name).send().await.unwrap();

    let output = resp.body.collect().await.unwrap().into_bytes().to_vec();

    output
}

pub async fn get_files_from_bucket() -> Vec<String> {
    dotenv().ok();

    // Environment variables
    let access_key = env::var("DO_ACCESS_KEY_ID").expect("DO_ACCESS_KEY_ID must be set");
    let secret_key = env::var("DO_SECRET_ACCESS_KEY").expect("DO_SECRET_ACCESS_KEY must be set");
    let region = env::var("DO_REGION").expect("DO_REGION must be set");
    let endpoint = env::var("DO_ENDPOINT").expect("DO_ENDPOINT must be set");
    let bucket_name = env::var("DO_BUCKET_NAME").expect("DO_BUCKET_NAME must be set");

    let region = Region::new(region);

    let credentials = Credentials::new(access_key, secret_key, None, None, "Digital Ocean");
    
    let config = Builder::new()
        .region(region)
        .credentials_provider(credentials)
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(endpoint)
        .build();

    let client = Client::from_conf(config);

    let resp = client.list_objects_v2().bucket(bucket_name).send().await.unwrap();

    let mut output: Vec<String> = vec![];
    let resp = resp.contents();
    for item in resp {
        let current = match item.key() {
            Some(key) => key,
            None => continue
        };
        output.push(current.to_string());
    }

    output
}