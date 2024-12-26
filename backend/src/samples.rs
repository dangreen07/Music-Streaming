use std::io::Cursor;

use diesel::prelude::*;
use hound::{
    WavReader,
    WavWriter
};

use crate::models::*;

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