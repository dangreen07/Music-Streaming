use std::{fs, path::Path};
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::{create_session, create_user, establish_connection, invalidate_session, valid_session, verify_user};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PostedUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SessionReturn {
    session_id: uuid::Uuid,
    error: String
}

#[derive(Serialize, Deserialize)]
struct SessionInput {
    session_id: uuid::Uuid
}

#[get("/sample")]
async fn sample() -> impl Responder {
    // Reading the sample file
    // This path is relative to the place where the terminal runs the command
    let contents = fs::read(Path::new("./samples/Confidence Man - Holiday.mp3")).unwrap();
    HttpResponse::Ok().content_type("audio/mpeg").body(contents)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Note: This is insecure and should not be used in production
            .allowed_headers(vec!["Content-Type"])
            .allow_any_method();
        App::new()
            .wrap(cors)
            .service(sample)
            .service(signup)
            .service(login)
            .service(validate_session)
            .service(logout)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}