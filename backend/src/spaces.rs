use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::{Builder, Credentials}, Client};
use std::{env, io::Cursor};
use dotenvy::dotenv;

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