use std::io::Cursor;

use diesel::prelude::*;
use hound::{
    WavReader,
    WavWriter
};

use crate::{models::*, spaces::{delete_file_from_bucket, get_file_from_bucket}};

pub fn get_sample(file: Vec<u8>, sample_number: u32) -> Result<Vec<u8>, &'static str> {
    let reader = Cursor::new(file);
    let mut reader = match WavReader::new(reader) {
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
        let mut writer = WavWriter::new(&mut buffer, spec).unwrap();
        for sample in &samples {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();
    }

    let audio_bytes = buffer.into_inner();

    Ok(audio_bytes)
}

pub async fn get_sample_from_bucket(song_id: &uuid::Uuid, sample_number: u32) -> Result<Vec<u8>, &'static str> {
    let file_name = format!("{}/{}.wav", song_id, sample_number);
    let resp = get_file_from_bucket(&file_name).await;
    match resp {
        Ok(resp) => return Ok(resp),
        Err(_) => return Err("Error getting file from bucket")
    };
}

pub fn get_all_samples(file: Vec<u8>) -> Result<Vec<Vec<u8>>, &'static str> {
    let reader = Cursor::new(file);
    let mut reader = match WavReader::new(reader) {
        Ok(r) => r,
        Err(_) => return Err("Error opening audio file"),
    };

    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let num_channels = spec.channels as usize;

    // 10 seconds per sample
    let samples_per_segment = sample_rate * 10 * num_channels as u32;

    // Get duration of the audio file
    let duration = reader.duration() / sample_rate;
    let mut num_samples = duration / 10;
    if (duration % 10) != 0 {
        num_samples += 1;
    }

    let mut samples: Vec<Vec<u8>> = vec![];

    for _ in 0..num_samples {
        let mut current_sample: Vec<i16> = Vec::new();
        for sample in reader.samples::<i16>() {
            match sample {
                Ok(s) => current_sample.push(s),
                Err(_) => return Err("Error reading samples")
            };

            if current_sample.len() >= samples_per_segment as usize {
                break;
            }
        }
        // Write the first segment to a new WAV file in memory
        let mut buffer = Cursor::new(Vec::new());
        {
            let mut writer = WavWriter::new(&mut buffer, spec).unwrap();
            for sample in &current_sample {
                writer.write_sample(*sample).unwrap();
            }
            writer.finalize().unwrap();
        }

        let audio_bytes = buffer.into_inner();
        samples.push(audio_bytes);
    }

    Ok(samples)
}

pub async fn get_songs_list(conn: &mut PgConnection) -> Result<Vec<Songs>, &'static str> {
    use crate::schema::songs::dsl::*;

    let response = songs.select(Songs::as_select()).load(conn);
    let response = match response {
        Ok(response) => response,
        Err(_) => return Err("Error loading songs")
    };

    Ok(response)
}

pub async fn get_song(conn: &mut PgConnection, song_id: &uuid::Uuid) -> Result<Songs, &'static str> {
    use crate::schema::songs::dsl::*;

    let response = songs.filter(id.eq(song_id)).select(Songs::as_select()).load(conn);
    let response = match response {
        Ok(response) => response,
        Err(_) => return Err("Error loading songs")
    };
    let response = response.get(0);
    let response = match response {
        Some(response) => response,
        None => return Err("Error loading song")
    };

    return Ok(response.clone());
}

pub async fn insert_song(conn: &mut PgConnection, song: NewSong) -> Result<Songs, &'static str> {
    use crate::schema::songs;

    let result = diesel::insert_into(songs::table)
        .values(&song)
        .returning(Songs::as_returning())
        .get_result(conn);
    let result = match result {
        Ok(result) => result,
        Err(_) => return Err("Error adding song")
    };

    return Ok(result);
}

pub async fn delete_song_from_server(conn: &mut PgConnection, song_id: &uuid::Uuid) -> Result<&'static str, &'static str> {
    use crate::schema::songs::dsl::*;

    let response = songs.filter(id.eq(song_id)).select(Songs::as_select()).load(conn);
    let response = match response {
        Ok(response) => response,
        Err(_) => return Err("Error loading songs")
    };
    let response = response.get(0);
    let response = match response {
        Some(response) => response,
        None => return Err("Error loading song")
    };
    let sample_num = response.num_samples;

    diesel::delete(songs.filter(id.eq(song_id))).execute(conn).expect("Error deleting song");
    for i in 0..sample_num {
        let response = delete_file_from_bucket(format!("{}/{}.wav", song_id, i)).await;
        match response {
            Ok(_) => true,
            Err(_) => return Err("Error deleting samples")
        };
    }

    Ok("Song deleted successfully")
}