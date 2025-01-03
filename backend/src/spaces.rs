use aws_config::{
    BehaviorVersion,
    Region
};
use aws_sdk_s3::{
    config::{
        Builder,
        Credentials
    }, primitives::ByteStream, Client
};
use std::env;
use dotenvy::dotenv;

/// Gets a file from a bucket from its file name.
pub async fn get_file_from_bucket(file_name: &str) -> Result<Vec<u8>, &'static str> {
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

    let resp = client.get_object().bucket(bucket_name).key(file_name).send().await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(_) => return Err("Failed to get object from bucket!")
    };

    let output = resp.body.collect().await;
    let output = match output {
        Ok(output) => output.into_bytes().to_vec(),
        Err(_) => return Err("Failed to collect body from response")
    };

    Ok(output)
}

pub async fn upload_file_to_bucket(file_name: &str, file_bytes: Vec<u8>) -> Result<&'static str, &'static str> {
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

    let body = ByteStream::from(file_bytes);
    let response = client
        .put_object()
        .bucket(bucket_name)
        .key(file_name)
        .body(body)
        .send()
        .await;
    match response {
        Ok(response) => response,
        Err(err) => {
            println!("Error uploading file to bucket: {:?}", err);
            return Err("Error uploading file to bucket");
        }
    };
    Ok("File uploaded successfully")
}

pub async fn delete_file_from_bucket(file_name: String) -> Result<&'static str, &'static str> {
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

    let resp = client.delete_object().bucket(bucket_name).key(file_name).send().await;
    match resp {
        Ok(resp) => resp,
        Err(_) => return Err("Failed to delete object from bucket!")
    };
    Ok("File deleted successfully")
}