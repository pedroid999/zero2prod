use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    // Creating a response that echoes the form data.
    let response_body = format!("Received: Name = {}, Email = {}", form.name, form.email);
    // Return the form data in the response.
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(response_body)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    println!("Server running at 127.0.0.1:8000");
    Ok(server)
}
