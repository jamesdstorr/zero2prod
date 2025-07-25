use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // handles all the transport layer concerns
        App::new().route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
