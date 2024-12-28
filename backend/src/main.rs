use std::{
    collections::HashMap,
    io::Cursor
};
use actix_cors::Cors;
use actix_web::{
    delete, get, post, web, App, HttpResponse, HttpServer, Responder
};
use backend::{
    auth::{
        create_session,
        create_user,
        get_user,
        invalidate_session,
        valid_session,
        verify_user
    }, compress_data,
    db::establish_connection,
    models::NewSong,
    samples::{
        delete_song_from_server, get_all_samples, get_sample_from_bucket, get_song, get_songs_list, insert_song, mp3_to_wav
    },
    spaces::{get_file_from_bucket, upload_file_to_bucket},
    PostedUser,
    SessionInput,
    SessionReturn,
    SongInfo,
    UserResponse
};
use actix_multipart::Multipart;
use futures::stream::StreamExt;
use futures::TryStreamExt;
use hound::WavReader;
use image::{ImageFormat, ImageReader};

#[get("/songs_list")]
async fn songs_list() -> impl Responder {
    let connection = &mut establish_connection();
    
    let songs_list = get_songs_list(connection).await;

    let songs_list = match songs_list {
        Ok(songs_list) => songs_list,
        Err(_) => return HttpResponse::InternalServerError().body("Error loading songs list")
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(songs_list)
}

#[get("/song_image/{song_id}")]
async fn song_image(path: web::Path<uuid::Uuid>) -> impl Responder {
    let song_id = path.into_inner();
    let key = format!("{0}/{0}.png", song_id);
    let file = get_file_from_bucket(&key).await;
    let file = match file {
        Ok(file) => file,
        Err(_) => return HttpResponse::NotFound().body("Error loading song image")
    };
    HttpResponse::Ok()
        .content_type("image/png")
        .body(file)
}

#[get("/song_info/{song_id}")]
async fn song_info(path: web::Path<uuid::Uuid>) -> impl Responder {
    let song_id = path.into_inner();
    let connection = &mut establish_connection();
    let result = get_song(connection, &song_id).await;
    let result = match result {
        Ok(result) => result,
        Err(error) => return HttpResponse::InternalServerError().body(error)
    };

    let output = SongInfo {
        song_duration: u32::try_from(result.duration).unwrap()
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(output)
}

/// Gets a 10 second sample from a song
#[get("/sample/{song_id}/{sample_number}")]
async fn samples_endpoint(path: web::Path<(uuid::Uuid, u32)>) -> impl Responder {
    let (song_id, sample_number) = path.into_inner();

    let resp = get_sample_from_bucket(&song_id, sample_number).await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(error) => return HttpResponse::InternalServerError().body(error)
    };
    HttpResponse::Ok()
        .content_type("audio/wav")
        .body(resp)
}

/// Get a 10 second sample from a song compressed with zlib
#[get("/sample_compressed/{song_id}/{sample_number}")]
async fn samples_compressed_endpoint(path: web::Path<(uuid::Uuid, u32)>) -> impl Responder {
    let (song_id, sample_number) = path.into_inner();

    let resp = get_sample_from_bucket(&song_id, sample_number).await;
    let resp = match resp {
        Ok(resp) => resp,
        Err(error) => return HttpResponse::InternalServerError().body(error)
    };
    let resp = compress_data(resp);

    HttpResponse::Ok()
        .content_type("application/zlib")
        .body(resp)
}

#[post("/signup")]
async fn signup(user: web::Json<PostedUser>) -> impl Responder {
    if user.username.len() < 3 {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: "Username must be at least 3 characters long!".to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }
    if user.password.len() < 8 {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: "Password must be at least 8 characters long!".to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }
    let connection = &mut establish_connection();
    let user = create_user(connection, &user.username, &user.password);
    // This handles if the user already exists
    if user.is_err() {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: user.err().unwrap().to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }
    let session = create_session(connection, &user.unwrap().id).expect("Error creating session!");
    let session_return: SessionReturn = SessionReturn {
        session_id: session.id,
        error: "".to_string()
    };
    HttpResponse::Ok().json(session_return)
}

#[post("/login")]
async fn login(user: web::Json<PostedUser>) -> impl Responder {
    if user.username.len() < 3 || user.password.len() < 8 {
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: "Username or password is incorrect!".to_string()
        };
        return HttpResponse::InternalServerError().json(output);
    }
    let connection = &mut establish_connection();
    let user = verify_user(connection, &user.username, &user.password);

    // This handles if the user doesn't exist or the password is incorrect
    if user.is_err(){
        let output = SessionReturn {
            session_id: uuid::Uuid::nil(),
            error: user.err().unwrap().to_string()
        };
        return HttpResponse::Unauthorized().json(output);
    }

    // Creating a session
    let session = create_session(connection, &user.unwrap()).expect("Error creating session!");
    let session_return: SessionReturn = SessionReturn {
        session_id: session.id,
        error: "".to_string()
    };

    return HttpResponse::Ok().json(session_return);
}

#[post("/validate_session")]
async fn validate_session(session_data: web::Json<SessionInput>) -> impl Responder {
    let connection = &mut establish_connection();
    let is_valid = valid_session(connection, &session_data.session_id);
    if is_valid {
        return HttpResponse::Ok().body("true");
    }
    else {
        return HttpResponse::Ok().body("false");
    }
}

#[post("/logout")]
async fn logout(session_data: web::Json<SessionInput>) -> impl Responder {
    let connection = &mut establish_connection();
    let _ = invalidate_session(connection, &session_data.session_id);
    return HttpResponse::Ok();
}

#[get("/user/{session_id}")]
async fn users_info(session_data: web::Path<uuid::Uuid>) -> impl Responder {
    let session_id = session_data.into_inner();
    let connection = &mut establish_connection();
    let user = get_user(connection, &session_id);
    let user = match user {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().body("Error getting user info")
    };
    let user = UserResponse {
        id: user.id,
        username: user.username,
        permissions: user.permissions
    };
    return HttpResponse::Ok().json(user);
}

#[post("/song")]
async fn add_song(mut payload: Multipart) -> impl Responder {
    let mut other_fields: HashMap<String, String> = HashMap::new();
    let mut duration = 0;
    let mut output_samples: Vec<Vec<u8>> = Vec::new();
    let mut album_cover: Vec<u8> = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let field_name = field.name().to_string();

        // If the field is the file, handle separately
        if field_name == "file" {
            let content_type = format!("{}", &mut field.content_type().essence_str());
            let mut file_bytes = Vec::new();
            // Store the uploaded file
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                file_bytes.extend_from_slice(&data);
            }
            if content_type == "audio/mpeg" {
                // If the file is an mp3, convert it to wav
                let wav_bytes = mp3_to_wav(file_bytes);
                file_bytes = match wav_bytes {
                    Ok(wav_bytes) => wav_bytes,
                    Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
                };
            }
            
            // Get the duration of the audio file
            let reader = Cursor::new(file_bytes.clone());
            let reader = match WavReader::new(reader) {
                Ok(r) => r,
                Err(_) => return HttpResponse::BadRequest().body("Invalid audio file"),
            };
            duration = (reader.duration() / reader.spec().sample_rate) as i32;

            // Get the samples for the audio file
            let samples = get_all_samples(file_bytes);
            output_samples = match samples {
                Ok(samples) => samples,
                Err(_) => return HttpResponse::BadRequest().body("Unable to get samples"),
            };
        }
        else if field_name == "image" {
            // Store the uploaded image
            let mut file_bytes = web::BytesMut::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                file_bytes.extend_from_slice(&data);
            }
            album_cover = file_bytes.to_vec();
        }
        else {
            // Add other fields to the HashMap
            let mut value_bytes = web::BytesMut::new();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                value_bytes.extend_from_slice(&data);
            }

            // Interpret bytes as a UTF-8 string and store it
            let value = String::from_utf8(value_bytes.to_vec()).unwrap_or_default();
            other_fields.insert(field_name, value);
        }
    }
    // Insert into the database using Diesel
    let new_song = NewSong {
        title: other_fields.get("title").unwrap_or(&"Unknown Title".to_string()).to_string(),
        artist: other_fields.get("artist").unwrap_or(&"Unknown Artist".to_string()).to_string(),
        album: other_fields.get("album").unwrap_or(&"Unknown Album".to_string()).to_string(),
        duration,
        num_samples: output_samples.len() as i32
    };

    let connection = &mut establish_connection();
    let response = insert_song(connection, new_song).await;
    let added_song = match response {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().body("Error adding song")
    };
    // Upload the album cover to the bucket
    let image = ImageReader::new(Cursor::new(album_cover)).with_guessed_format();
    let image = match image {
        Ok(image) => image,
        Err(_) => return HttpResponse::InternalServerError().body("Error reading album cover")
    };
    let image = image.decode();
    let image = match image {
        Ok(image) => image,
        Err(_) => return HttpResponse::InternalServerError().body("Error decoding album cover")
    };
    let mut png_data: Vec<u8> = Vec::new();
    let resp = image.write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png);
    match resp {
        Ok(_) => {},
        Err(_) => return HttpResponse::InternalServerError().body("Error uploading album cover")
    };
    let key = format!("{0}/{0}.png", added_song.id);
    let resp = upload_file_to_bucket(&key, png_data).await;
    if resp.is_err() {
        return HttpResponse::InternalServerError().body("Error uploading album cover");
    }

    // Upload the samples to the bucket
    // This may take a while
    for i in 0..output_samples.len() {
        let key = format!("{}/{}.wav", added_song.id, i);
        let current = output_samples.get(i).unwrap().clone();
        let resp = upload_file_to_bucket(&key, current).await;
        if resp.is_err() {
            return HttpResponse::InternalServerError().body("Error uploading samples");
        }
    }

    HttpResponse::Ok().body("File upload successful")
}

#[delete("/song/{song_id}")]
async fn delete_song(path: web::Path::<uuid::Uuid>) -> impl Responder {
    let song_id = path.into_inner();
    let connection = &mut establish_connection();
    let response = delete_song_from_server(connection, &song_id).await;
    let response = match response {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().body("Error deleting song")
    };
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Note: This is insecure and should not be used in production
            .allowed_headers(vec!["Content-Type"])
            .allow_any_method();

        App::new()
            .wrap(cors)
            .service(samples_endpoint)
            .service(signup)
            .service(login)
            .service(validate_session)
            .service(logout)
            .service(song_info)
            .service(songs_list)
            .service(users_info)
            .service(samples_compressed_endpoint)
            .service(add_song)
            .service(delete_song)
            .service(song_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}