use aws_config::{
    BehaviorVersion,
    Region
};
use aws_sdk_s3::{
    config::{
        Builder,
        Credentials
    },
    Client
};
use std::env;
use dotenvy::dotenv;

/// Gets a file from a bucket from its file name.
/// 
/// # Panics
/// Will panic if any of the environment variables don't exist in a .env file 
/// or in the terminal running the programs environment. The environment variables required are listed below:
/// - `DO_ACCESS_KEY_ID` - This is the access key for the storage bucket
/// - `DO_SECRET_ACCESS_KEY` - This is the secret access key for the storage bucket
/// - `DO_REGION` - This is the region of the storage bucket
/// - `DO_ENDPOINT` - This is the endpoint of the storage bucket
/// - `DO_BUCKET_NAME` - This is the name of the storage bucket
/// # Returns
/// A vector of bytes representing the file `Vec<u8>`
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

    let resp = client.get_object().bucket(bucket_name).key(file_name).send().await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(_) => panic!("Failed to get object from bucket!")
    };

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