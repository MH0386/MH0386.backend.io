use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
 
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
 
    let port: u16 = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap();
 
    HttpServer::new(|| {
        App::new()            .service(hello)
    })
    .bind(("0.0.0.0", port))?
        .run()
        .await
}