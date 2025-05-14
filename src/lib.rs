use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // handles all the transport layer concerns
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
