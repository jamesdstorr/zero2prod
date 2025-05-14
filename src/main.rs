use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

#[tokio::main] //async runtime 
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        // handles all the transport layer concerns
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
