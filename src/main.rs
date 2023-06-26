use std::io::Result;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct GenericResponse<T> {
    ok: bool,
    result: Option<T>,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(GenericResponse::<String> {
        ok: true,
        result: Some("API is running fine".to_string()),
    })
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(ping))
        .bind(("localhost", 8000))?
        .run()
        .await
}
