use std::{fs, path::Path};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::{create_session, create_user, establish_connection, verify_user};
use serde::{Deserialize, Serialize};

#[get("/sample")]
async fn sample() -> impl Responder {
    // Reading the sample file
    // This path is relative to the place where the terminal runs the command
    let contents = fs::read(Path::new("./samples/Confidence Man - Holiday.mp3")).unwrap();
    HttpResponse::Ok().content_type("audio/mpeg").body(contents)
}

#[derive(Deserialize)]
struct PostedUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SessionReturn {
    session_id: uuid::Uuid,
}

#[post("/signup")]
async fn signup(user: web::Json<PostedUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let user = create_user(connection, &user.username, &user.password);
    // This handles if the user already exists
    if user.is_err() {
        return HttpResponse::InternalServerError().body(format!("{}", user.err().unwrap()));
    }
    let session = create_session(connection, &user.unwrap().id).expect("Error creating session!");
    let session_return: SessionReturn = SessionReturn {
        session_id: session.id
    };
    HttpResponse::Ok().json(session_return)
}

#[post("/login")]
async fn login(user: web::Json<PostedUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let user = verify_user(connection, &user.username, &user.password);

    // This handles if the user doesn't exist or the password is incorrect
    if user.is_err(){
        return HttpResponse::Unauthorized().body(user.err().unwrap());
    }

    // Creating a session
    let session = create_session(connection, &user.unwrap()).expect("Error creating session!");
    let session_return: SessionReturn = SessionReturn {
        session_id: session.id
    };

    return HttpResponse::Ok().json(session_return);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(sample)
            .service(signup)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}