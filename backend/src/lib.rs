use std::io::Write;

use serde::{
    Deserialize,
    Serialize
};
use flate2::{
    write::ZlibEncoder,
    Compression
};

pub mod models;
pub mod schema;
pub mod db;
pub mod spaces;
pub mod auth;
pub mod samples;

pub fn compress_data(data: Vec<u8>) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::new(6));
    e.write_all(&data).unwrap();
    let compressed_data = e.finish().unwrap();
    return compressed_data;
}

#[derive(Deserialize)]
pub struct PostedUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionReturn {
    pub session_id: uuid::Uuid,
    pub error: String
}

#[derive(Serialize, Deserialize)]
pub struct SessionInput {
    pub session_id: uuid::Uuid
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub permissions: String
}